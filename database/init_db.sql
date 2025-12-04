-- Enable UUID extension for primary rsa_keys
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enum types for key algorithms and strengths
-- CREATE TYPE key_algorithm AS ENUM ('RSA', 'ECDSA');
-- CREATE TYPE rsa_key_size AS ENUM ('2048', '3072', '4096');
-- CREATE TYPE ecdsa_curve AS ENUM ('P256', 'P384', 'P521');


-- ============================================================
-- Parent table: rsa_keys (polymorphic base)
-- ============================================================
CREATE TABLE key_algorithms
(
    id         UUID PRIMARY KEY     DEFAULT uuid_generate_v4(),
    algorithm  TEXT        NOT NULL, -- 'RSA', 'ECDSA', future types
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- Child table: RSA (inherits rsa_keys)
-- ============================================================
CREATE TABLE rsa_key_algorithm
(
    -- Specific RSA parameters
    rsa_key_size INTEGER NOT NULL, -- e.g., 2048, 3072, 4096
    display_name TEXT
) INHERITS (key_algorithms);

-- Trigger to enforce the 'algorithm' column equals 'RSA' on child inserts/updates
CREATE OR REPLACE FUNCTION enforce_rsa_algorithm_child()
    RETURNS TRIGGER AS
$$
BEGIN
    IF NEW.algorithm IS NULL THEN
        NEW.algorithm := 'RSA';
    ELSIF NEW.algorithm <> 'RSA' THEN
        RAISE EXCEPTION 'Algorithm mismatch in rsa_key_algorithm: expected RSA, got %', NEW.algorithm;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rsa_child_algorithm_check
    BEFORE INSERT OR UPDATE
    ON rsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION enforce_rsa_algorithm_child();


-- ============================================================
-- Child table: ECDSA (inherits rsa_keys)
-- ============================================================
CREATE TABLE ecdsa_key_algorithm
(
    -- Specific ECDSA parameters
    curve        TEXT    NOT NULL, -- e.g., 'P256', 'P384', 'P521'
    nid_name     TEXT    NOT NULL, -- e.g., 'X9_62_PRIME256V1', 'SECP384R1', 'SECP521R1'
    nid_value    INTEGER NOT NULL, -- OpenSSL internal numeric ID
    display_name TEXT,             -- e.g., 'NIST P-256'
    standard     TEXT,             -- e.g., 'X9.62', 'SECG', 'NIST'
    deprecated   BOOLEAN NOT NULL DEFAULT FALSE
) INHERITS (key_algorithms);

-- Trigger to enforce the 'algorithm' column equals 'ECDSA' on child inserts/updates
CREATE OR REPLACE FUNCTION enforce_ecdsa_algorithm_child()
    RETURNS TRIGGER AS
$$
BEGIN
    IF NEW.algorithm IS NULL THEN
        NEW.algorithm := 'ECDSA';
    ELSIF NEW.algorithm <> 'ECDSA' THEN
        RAISE EXCEPTION 'Algorithm mismatch in ecdsa_key_algorithm: expected ECDSA, got %', NEW.algorithm;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER ecdsa_child_algorithm_check
    BEFORE INSERT OR UPDATE
    ON ecdsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION enforce_ecdsa_algorithm_child();

-- ============================================================
-- Main certificates table (links only to base rsa_keys)
-- ============================================================
CREATE TABLE certificates
(
    id                  UUID PRIMARY KEY     DEFAULT uuid_generate_v4(),

    -- PEM data
    csr_pem             TEXT        NOT NULL,
    cert_pem            TEXT, -- NULL until signed by CA
    key_pem             TEXT        NOT NULL,
    public_key_pem      TEXT        NOT NULL,
    chain_pem           TEXT,

    -- Link to polymorphic base algorithm row (points to either RSA or ECDSA child row)
    key_algorithm_id    UUID        NOT NULL REFERENCES key_algorithms (id),

    -- Subject details
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
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    cert_uploaded_at    TIMESTAMPTZ,
    deleted_at          TIMESTAMPTZ
);

-- ============================================================
-- Integrity & audit triggers
-- ============================================================

-- Auto-update updated_at timestamp on certificates
CREATE OR REPLACE FUNCTION update_cert_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER cert_update_timestamp
    BEFORE UPDATE
    ON certificates
    FOR EACH ROW
EXECUTE FUNCTION update_cert_timestamp();

-- Ensure referenced key_algorithm_id exists in a child table (RSA or ECDSA)
-- Note: The FK ensures existence in parent; this trigger ensures it is realized in a child.
CREATE OR REPLACE FUNCTION enforce_algorithm_has_child()
    RETURNS TRIGGER AS
$$
DECLARE
    is_rsa   BOOLEAN;
    is_ecdsa BOOLEAN;
BEGIN
    -- Check if the referenced id exists in either child
    SELECT EXISTS(SELECT 1 FROM rsa_key_algorithm WHERE id = NEW.key_algorithm_id) INTO is_rsa;
    SELECT EXISTS(SELECT 1 FROM ecdsa_key_algorithm WHERE id = NEW.key_algorithm_id) INTO is_ecdsa;

    IF NOT (is_rsa OR is_ecdsa) THEN
        RAISE EXCEPTION 'key_algorithm_id % must reference a child row in rsa_key_algorithm or ecdsa_key_algorithm', NEW.key_algorithm_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER certificates_algorithm_child_check
    BEFORE INSERT OR UPDATE
    ON certificates
    FOR EACH ROW
EXECUTE FUNCTION enforce_algorithm_has_child();


-- Optional: prevent creating child rows with duplicate IDs across children
-- (rare unless manually setting IDs). Typically the DEFAULT uuid_generate_v4() avoids collision.
-- This guard ensures a key_algorithm id is unique across the inheritance hierarchy.
CREATE OR REPLACE FUNCTION enforce_unique_child_ids()
    RETURNS TRIGGER AS
$$
BEGIN
    -- If inserting into RSA, ensure the same id does not exist in ECDSA, and vice versa
    IF TG_TABLE_NAME = 'rsa_key_algorithm' THEN
        IF EXISTS (SELECT 1 FROM ecdsa_key_algorithm WHERE id = NEW.id) THEN
            RAISE EXCEPTION 'Algorithm id % already used in ecdsa_key_algorithm', NEW.id;
        END IF;
    ELSIF TG_TABLE_NAME = 'ecdsa_key_algorithm' THEN
        IF EXISTS (SELECT 1 FROM rsa_key_algorithm WHERE id = NEW.id) THEN
            RAISE EXCEPTION 'Algorithm id % already used in rsa_key_algorithm', NEW.id;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_unique_child_ids_rsa
    BEFORE INSERT
    ON rsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION enforce_unique_child_ids();

CREATE TRIGGER enforce_unique_child_ids_ecdsa
    BEFORE INSERT
    ON ecdsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION enforce_unique_child_ids();


-- ============================================================
-- Views for polymorphic querying
-- ============================================================

-- Unified list of available key options (algorithm + parameter + display)
CREATE OR REPLACE VIEW available_key_options AS
SELECT 'RSA'                                                            AS algorithm,
       rsa.rsa_key_size::TEXT                                           AS option,
       COALESCE(rsa.display_name, 'RSA ' || rsa.rsa_key_size || '-bit') AS display_name,
       rsa.id                                                           AS key_algorithm_id
FROM rsa_key_algorithm rsa

UNION ALL
SELECT 'ECDSA'                                               AS algorithm,
       ecdsa.curve                                           AS option,
       COALESCE(ecdsa.display_name, 'ECDSA ' || ecdsa.curve) AS display_name,
       ecdsa.id                                              AS key_algorithm_id
FROM ecdsa_key_algorithm ecdsa
WHERE ecdsa.deprecated = FALSE;

-- Certificates resolved to their algorithm specifics via polymorphic join
CREATE OR REPLACE VIEW certificates_with_options AS
SELECT c.id               AS certificate_id,
       c.key_algorithm_id,
       ka.algorithm       AS algorithm,
       rsa.rsa_key_size,
       rsa.display_name   AS rsa_display,
       ecdsa.curve        AS ecdsa_curve,
       ecdsa.nid_name,
       ecdsa.nid_value,
       ecdsa.display_name AS ecdsa_display,
       ecdsa.deprecated   AS ecdsa_deprecated,
       c.organization,
       c.organizational_unit,
       c.country,
       c.state_or_province,
       c.locality,
       c.email,
       c.fingerprint,
       c.valid_from,
       c.expires_at,
       c.created_at,
       c.updated_at,
       c.cert_uploaded_at,
       c.deleted_at
FROM certificates c
         JOIN key_algorithms ka ON c.key_algorithm_id = ka.id
         LEFT JOIN rsa_key_algorithm rsa ON c.key_algorithm_id = rsa.id
         LEFT JOIN ecdsa_key_algorithm ecdsa ON c.key_algorithm_id = ecdsa.id;


-- ============================================================
-- Useful indexes
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_key_algorithms_algorithm ON key_algorithms (algorithm);
CREATE INDEX IF NOT EXISTS idx_rsa_key_size ON rsa_key_algorithm (rsa_key_size);
CREATE INDEX IF NOT EXISTS idx_ecdsa_curve ON ecdsa_key_algorithm (curve);
CREATE INDEX IF NOT EXISTS idx_certificates_algorithm_id ON certificates (key_algorithm_id);
CREATE INDEX IF NOT EXISTS idx_certificates_fingerprint ON certificates (fingerprint);


-- ============================================================
-- Initial data: RSA sizes and ECDSA curves with OpenSSL NID
-- ============================================================

-- RSA rows (algorithm is enforced by trigger; included explicitly for clarity)
INSERT INTO rsa_key_algorithm (algorithm, rsa_key_size, display_name)
VALUES ('RSA', 2048, 'RSA 2048-bit'),
       ('RSA', 3072, 'RSA 3072-bit'),
       ('RSA', 4096, 'RSA 4096-bit');

-- ECDSA rows (common NIDs)
INSERT INTO ecdsa_key_algorithm (algorithm, curve, nid_name, nid_value, display_name, standard, deprecated)
VALUES ('ECDSA', 'P256', 'X9_62_PRIME256V1', 415, 'NIST P-256', 'X9.62', FALSE),
       ('ECDSA', 'P384', 'SECP384R1', 715, 'NIST P-384', 'SECG', FALSE),
       ('ECDSA', 'P521', 'SECP521R1', 716, 'NIST P-521', 'SECG', FALSE);

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



