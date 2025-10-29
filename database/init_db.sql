-- Enable UUID extension for primary keys
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enum types for key algorithms and strengths
CREATE TYPE key_algorithm AS ENUM ('RSA', 'ECDSA');
CREATE TYPE rsa_key_size AS ENUM ('2048', '3072', '4096');
CREATE TYPE ecdsa_curve AS ENUM ('P256', 'P384', 'P521');

-- Main certificates table
CREATE TABLE certificates
(
    id                  UUID PRIMARY KEY       DEFAULT uuid_generate_v4(),

    -- PEM data (encrypted private keys recommended in production)
    csr_pem             TEXT          NOT NULL,
    cert_pem            TEXT,        -- NULL until signed by CA
    key_pem             TEXT          NOT NULL,
    public_key_pem      TEXT          NOT NULL,
    chain_pem           TEXT,

    -- Key configuration
    key_algorithm       key_algorithm NOT NULL,
    rsa_key_size        rsa_key_size,
    ecdsa_curve         ecdsa_curve,

    -- Subject details (stored as separate columns for queryability)
    organization        VARCHAR(255),
    organizational_unit VARCHAR(255),
    country             CHAR(2),
    state_or_province   VARCHAR(255),
    locality            VARCHAR(255),
    email               VARCHAR(255),

    -- Certificate metadata
    fingerprint         VARCHAR(64) UNIQUE,
    valid_from          TIMESTAMPTZ,
    expires_at          TIMESTAMPTZ,

    -- Audit timestamps
    created_at          TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    cert_uploaded_at    TIMESTAMPTZ, -- When signed cert was uploaded
    deleted_at          TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT key_strength_check CHECK (
        (key_algorithm = 'RSA' AND rsa_key_size IS NOT NULL AND ecdsa_curve IS NULL) OR
        (key_algorithm = 'ECDSA' AND ecdsa_curve IS NOT NULL AND rsa_key_size IS NULL)
        )
);

-- Subject Alternative Names (many-to-many relationship)
CREATE TABLE certificate_sans
(
    id             UUID PRIMARY KEY      DEFAULT uuid_generate_v4(),
    certificate_id UUID         NOT NULL REFERENCES certificates (id) ON DELETE CASCADE,
    san_value      VARCHAR(255) NOT NULL,
    san_order      INTEGER      NOT NULL DEFAULT 0, -- First SAN becomes CN
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    UNIQUE (certificate_id, san_value)
);

-- Indexes for common queries
CREATE INDEX idx_certificates_fingerprint ON certificates (fingerprint);
CREATE INDEX idx_certificates_expires_at ON certificates (expires_at);
CREATE INDEX idx_certificates_created_at ON certificates (created_at);
CREATE INDEX idx_certificates_deleted_at ON certificates (deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX idx_certificate_sans_value ON certificate_sans (san_value);
CREATE INDEX idx_certificate_sans_cert_order ON certificate_sans (certificate_id, san_order);

-- Composite index for finding certificates by subject fields
CREATE INDEX idx_certificates_subject ON certificates (
                                                       organization,
                                                       organizational_unit,
                                                       country,
                                                       state_or_province,
                                                       locality
    ) WHERE deleted_at IS NULL;

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for updated_at
CREATE TRIGGER update_certificates_updated_at
    BEFORE UPDATE
    ON certificates
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- View for easy querying with all SANs as array
CREATE VIEW certificates_with_sans AS
SELECT c.*,
       COALESCE(
                       ARRAY_AGG(cs.san_value ORDER BY cs.san_order)
                       FILTER (WHERE cs.san_value IS NOT NULL),
                       ARRAY []::VARCHAR[]
       )                                                  as sans,
       (ARRAY_AGG(cs.san_value ORDER BY cs.san_order))[1] as common_name
FROM certificates c
         LEFT JOIN certificate_sans cs ON c.id = cs.certificate_id
GROUP BY c.id;

-- View for active (non-deleted, valid) certificates
CREATE VIEW active_certificates AS
SELECT *
FROM certificates_with_sans
WHERE deleted_at IS NULL
  AND cert_pem IS NOT NULL
  AND expires_at > NOW();

-- View to find the most recent certificate for each subject + SANs combination
-- This helps with zero-downtime renewals
CREATE VIEW latest_certificates_by_subject AS
SELECT DISTINCT ON (
    organization,
    organizational_unit,
    country,
    state_or_province,
    locality,
    common_name
    ) *
FROM active_certificates
ORDER BY organization,
         organizational_unit,
         country,
         state_or_province,
         locality,
         common_name,
         created_at DESC;

-- View for certificates that will expire soon (useful for renewal planning)
CREATE VIEW expiring_certificates AS
SELECT id,
       fingerprint,
       common_name,
       organization,
       expires_at,
       expires_at - NOW()                                                  as time_until_expiry,
       -- Check if there's a newer cert for the same subject
       EXISTS (SELECT 1
               FROM active_certificates newer
               WHERE newer.organization = certificates_with_sans.organization
                 AND newer.organizational_unit = certificates_with_sans.organizational_unit
                 AND newer.country = certificates_with_sans.country
                 AND newer.state_or_province = certificates_with_sans.state_or_province
                 AND newer.locality = certificates_with_sans.locality
                 AND newer.common_name = certificates_with_sans.common_name
                 AND newer.created_at > certificates_with_sans.created_at) as has_renewal
FROM certificates_with_sans
WHERE deleted_at IS NULL
  AND expires_at IS NOT NULL
  AND expires_at > NOW()
  AND expires_at < NOW() + INTERVAL '30 days'
ORDER BY expires_at;

-- View for overlapping certificates (useful for monitoring zero-downtime rotation)
CREATE VIEW overlapping_certificates AS
SELECT c1.id                                                                        as cert_id_1,
       c1.common_name,
       c1.fingerprint                                                               as fingerprint_1,
       c1.valid_from                                                                as valid_from_1,
       c1.expires_at                                                                as expires_at_1,
       c2.id                                                                        as cert_id_2,
       c2.fingerprint                                                               as fingerprint_2,
       c2.valid_from                                                                as valid_from_2,
       c2.expires_at                                                                as expires_at_2,
       LEAST(c1.expires_at, c2.expires_at) - GREATEST(c1.valid_from, c2.valid_from) as overlap_duration
FROM active_certificates c1
         JOIN active_certificates c2 ON
    c1.organization = c2.organization
        AND c1.organizational_unit = c2.organizational_unit
        AND c1.country = c2.country
        AND c1.state_or_province = c2.state_or_province
        AND c1.locality = c2.locality
        AND c1.common_name = c2.common_name
        AND c1.id < c2.id -- Avoid duplicates
WHERE c1.valid_from < c2.expires_at
  AND c2.valid_from < c1.expires_at;