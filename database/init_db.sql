-- Enable UUID extension for primary keys
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enum types for key algorithms and strengths
CREATE TYPE key_algorithm AS ENUM ('RSA', 'ECDSA');
CREATE TYPE rsa_key_size AS ENUM ('2048', '3072', '4096');
CREATE TYPE ecdsa_curve AS ENUM ('P256', 'P384', 'P521');

-- Main certificates table
CREATE TABLE certificates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- PEM data (encrypted private keys recommended in production)
    cert_pem TEXT NOT NULL,
    key_pem TEXT NOT NULL,
    chain_pem TEXT,
    
    -- Key configuration
    key_algorithm key_algorithm NOT NULL,
    rsa_key_size rsa_key_size,
    ecdsa_curve ecdsa_curve,
    
    -- Subject details (stored as separate columns for queryability)
    organization VARCHAR(255),
    organizational_unit VARCHAR(255),
    country CHAR(2),
    state_or_province VARCHAR(255),
    locality VARCHAR(255),
    email VARCHAR(255),
    
    -- Certificate metadata
    fingerprint VARCHAR(64) UNIQUE,
    valid_from TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    
    -- Audit timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    -- Constraints
    CONSTRAINT key_strength_check CHECK (
        (key_algorithm = 'RSA' AND rsa_key_size IS NOT NULL AND ecdsa_curve IS NULL) OR
        (key_algorithm = 'ECDSA' AND ecdsa_curve IS NOT NULL AND rsa_key_size IS NULL)
    )
);

-- Subject Alternative Names (many-to-many relationship)
CREATE TABLE certificate_sans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    certificate_id UUID NOT NULL REFERENCES certificates(id) ON DELETE CASCADE,
    san_value VARCHAR(255) NOT NULL,
    san_order INTEGER NOT NULL DEFAULT 0,  -- First SAN becomes CN
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(certificate_id, san_value)
);

-- Certificate generation requests audit log (optional but useful)
CREATE TABLE certificate_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    certificate_id UUID REFERENCES certificates(id) ON DELETE SET NULL,
    
    -- Request details
    key_algorithm key_algorithm NOT NULL,
    rsa_key_size rsa_key_size,
    ecdsa_curve ecdsa_curve,
    validity_days INTEGER NOT NULL DEFAULT 365,
    
    -- Request metadata
    requested_by VARCHAR(255),
    request_ip INET,
    request_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    
    CONSTRAINT request_key_strength_check CHECK (
        (key_algorithm = 'RSA' AND rsa_key_size IS NOT NULL AND ecdsa_curve IS NULL) OR
        (key_algorithm = 'ECDSA' AND ecdsa_curve IS NOT NULL AND rsa_key_size IS NULL)
    )
);

-- Indexes for common queries
CREATE INDEX idx_certificates_fingerprint ON certificates(fingerprint);
CREATE INDEX idx_certificates_expires_at ON certificates(expires_at);
CREATE INDEX idx_certificates_created_at ON certificates(created_at);
CREATE INDEX idx_certificates_deleted_at ON certificates(deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX idx_certificate_sans_value ON certificate_sans(san_value);
CREATE INDEX idx_certificate_sans_cert_order ON certificate_sans(certificate_id, san_order);

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for updated_at
CREATE TRIGGER update_certificates_updated_at
    BEFORE UPDATE ON certificates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- View for easy querying with all SANs as array
CREATE VIEW certificates_with_sans AS
SELECT 
    c.*,
    COALESCE(
        ARRAY_AGG(cs.san_value ORDER BY cs.san_order) 
        FILTER (WHERE cs.san_value IS NOT NULL),
        ARRAY[]::VARCHAR[]
    ) as sans,
    (ARRAY_AGG(cs.san_value ORDER BY cs.san_order))[1] as common_name
FROM certificates c
LEFT JOIN certificate_sans cs ON c.id = cs.certificate_id
GROUP BY c.id;

-- View for expiring certificates
CREATE VIEW expiring_certificates AS
SELECT 
    id,
    fingerprint,
    common_name,
    expires_at,
    expires_at - NOW() as time_until_expiry
FROM certificates_with_sans
WHERE deleted_at IS NULL
  AND expires_at IS NOT NULL
  AND expires_at > NOW()
  AND expires_at < NOW() + INTERVAL '30 days'
ORDER BY expires_at;