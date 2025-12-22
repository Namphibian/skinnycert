-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- ============================================================
-- Function to set updated_on on inserts/updates
-- ============================================================
CREATE OR REPLACE FUNCTION set_updated_on()
    RETURNS TRIGGER AS
$$
BEGIN
    new.updated_on := NOW();
    RETURN new;
END;
$$ LANGUAGE plpgsql;
-- ============================================================
-- Parent table: key_algorithms (polymorphic base)
-- ============================================================
DROP TABLE IF EXISTS key_algorithms CASCADE;
CREATE TABLE key_algorithms
(
    id         uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    algorithm  TEXT        NOT NULL, -- 'RSA', 'ECDSA', future types
    created_on timestamptz NOT NULL DEFAULT NOW(),
    updated_on timestamptz NULL,     -- auto-managed by trigger
    deprecated BOOLEAN     NOT NULL DEFAULT FALSE
);
-- ============================================================
-- Child table: RSA (inherits key_algorithms)
-- ============================================================
DROP TABLE IF EXISTS rsa_key_algorithm CASCADE;
CREATE TABLE rsa_key_algorithm
(
    key_size     INTEGER NOT NULL, -- e.g., 2048, 3072, 4096
    display_name TEXT GENERATED ALWAYS AS (
        'RSA ' || key_size || '-bit'
        ) STORED,
    CONSTRAINT unique_rsa_key_size UNIQUE (key_size)
) INHERITS (key_algorithms);
-- Trigger to enforce the 'algorithm' column equals 'RSA' on child inserts/updates
CREATE OR REPLACE FUNCTION rsa_insert_trigger()
    RETURNS TRIGGER AS
$$
BEGIN
    -- Enforce RSA key size rule (example: multiple of 1024)
    IF new.algorithm <> 'RSA' THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514', -- data exception
            MESSAGE = FORMAT(
                    'Algorithm mismatch in rsa_key_algorithm: expected RSA, got %s',
                    new.algorithm
                      );
    END IF;
    -- Enforce RSA key size multiple of 1024
    IF new.key_size % 1024 <> 0 THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514', -- or a more specific code like '23514' (check_violation)
            MESSAGE = FORMAT(
                    'RSA key size (%s) must be a multiple of 1024',
                    new.key_size
                      );
    END IF;

    RETURN new;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER rsa_before_insert
    BEFORE INSERT
    ON rsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION rsa_insert_trigger();

CREATE TRIGGER rsa_before_update
    BEFORE UPDATE
    ON rsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION set_updated_on();
-- INSERT Some RSA keys:
INSERT INTO rsa_key_algorithm
(
    algorithm,
    key_size
)
VALUES
    (
        'RSA',
        2048

    ),
    (
        'RSA',
        3072

    ),
    (
        'RSA',
        4096

    );
-- ============================================================
-- Child table: ECDSA
-- ============================================================
DROP TABLE IF EXISTS ecdsa_key_algorithm CASCADE;
CREATE TABLE ecdsa_key_algorithm
(
    -- Specific ECDSA parameters
    display_name TEXT,             -- e.g., 'NIST P-256'
    nid_value    INTEGER NOT NULL, -- OpenSSL internal numeric ID
    CONSTRAINT unique_nid_value UNIQUE (nid_value)
) INHERITS (key_algorithms);
ALTER TABLE ecdsa_key_algorithm
    ADD COLUMN curve_size INTEGER NOT NULL DEFAULT 0;

-- Trigger to enforce the 'algorithm' column equals 'ECDSA' on child inserts/updates
CREATE OR REPLACE FUNCTION ecdsa_insert_trigger()
    RETURNS TRIGGER AS
$$
BEGIN
    IF new.algorithm IS NULL THEN
        new.algorithm := 'ECDSA';
    ELSIF new.algorithm <> 'ECDSA' THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514', -- data exception
            MESSAGE = FORMAT(
                    'Algorithm mismatch in ecdsa_key_algorithm: expected ECDSA, got %s',
                    new.algorithm
                      );
    END IF;
    RETURN new;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER ecdsa_before_insert
    BEFORE INSERT OR UPDATE
    ON ecdsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION enforce_ecdsa_algorithm_child();


CREATE TRIGGER ecdsa_before_update
    BEFORE UPDATE
    ON ecdsa_key_algorithm
    FOR EACH ROW
EXECUTE FUNCTION set_updated_on();
-- ECDSA rows (common NIDs)
INSERT INTO ecdsa_key_algorithm
(
    algorithm,
--     curve,
--     nid_name,
    nid_value,
    display_name
--     standard

)
VALUES
    (
        'ECDSA',
--         'P256',
--         'X9_62_PRIME256V1',
        415,
        'NIST P-256'
--         'X9.62'

    ),
    (
        'ECDSA',
--         'P384',
--         'SECP384R1',
        715,
        'NIST P-384'
--         'SECG'

    ),
    (
        'ECDSA',
--         'P521',
--         'SECP521R1',
        716,
        'NIST P-521'
--         'SECG'

    );
-- ============================================================
-- Main certificates table
-- ============================================================
DROP TABLE IF EXISTS certificates CASCADE;
CREATE TABLE certificates
(
    id                  uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),

    -- PEM data
    csr_pem             TEXT        NOT NULL,
    cert_pem            TEXT, -- NULL until signed by CA
    key_pem             TEXT        NOT NULL,
    public_key_pem      TEXT        NOT NULL,
    chain_pem           TEXT,

    -- Link to polymorphic base algorithm row (points to either RSA or ECDSA child row)
    key_algorithm_id    uuid        NOT NULL REFERENCES key_algorithms (id),

    -- Subject details
    organization        VARCHAR(255),
    organizational_unit VARCHAR(255),
    country             CHAR(2),
    state_or_province   VARCHAR(255),
    locality            VARCHAR(255),
    email               VARCHAR(255),

    -- Certificate metadata
    fingerprint         VARCHAR(64) UNIQUE,
    valid_from          timestamptz,
    expires_on          timestamptz,

    -- Audit timestamps
    created_on          timestamptz NOT NULL DEFAULT NOW(),
    updated_on          timestamptz NOT NULL DEFAULT NOW(),
    cert_uploaded_on    timestamptz,
    deleted_on          timestamptz
);

-- ============================================================
-- Integrity & audit triggers
-- ============================================================

-- Auto-update updated_on timestamp on legacy_certificates
CREATE OR REPLACE FUNCTION update_cert_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    new.updated_on := NOW();
    RETURN new;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER cert_update_timestamp
    BEFORE UPDATE
    ON certificates
    FOR EACH ROW
EXECUTE FUNCTION update_cert_timestamp();

-- ============================================================
-- Views for polymorphic querying
-- ============================================================

-- Unified list of available key options (algorithm + parameter + display)
DROP VIEW IF EXISTS all_key_algorithms CASCADE;

CREATE OR REPLACE VIEW all_key_algorithms AS
SELECT 'RSA'            AS algorithm,
       rsa.key_size     AS key_size,
       rsa.display_name AS display_name,
       rsa.id           AS key_algorithm_id,
       rsa.deprecated   AS deprecated
FROM rsa_key_algorithm rsa
UNION ALL
SELECT 'ECDSA'            AS algorithm,
       ecdsa.curve_size   AS key_size,
       ecdsa.display_name AS display_name,
       ecdsa.id           AS key_algorithm_id,
       ecdsa.deprecated   AS deprecated
FROM ecdsa_key_algorithm ecdsa;

CREATE OR REPLACE VIEW certificate_complete AS
SELECT
    c.id,
    c.csr_pem,
    c.cert_pem,
    c.key_pem,
    c.public_key_pem,
    c.chain_pem,
    c.key_algorithm_id,
    all_key.algorithm,
    all_key.key_size,
    all_key.display_name,
    all_key.deprecated,
    c.organization,
    c.organizational_unit,
    c.country,
    c.state_or_province,
    c.locality,
    c.email,
    COALESCE(
                    ARRAY_AGG(cs.san_value ORDER BY cs.san_order)
                    FILTER (WHERE cs.san_value IS NOT NULL),
                    ARRAY[]::VARCHAR[]
    ) AS sans,
    (ARRAY_AGG(cs.san_value ORDER BY cs.san_order))[1] AS common_name,
    c.fingerprint,
    c.valid_from,
    c.expires_on,
    c.created_on,
    c.updated_on,
    c.cert_uploaded_on,
    c.deleted_on
FROM certificates c
         JOIN all_key_algorithms all_key
              ON c.key_algorithm_id = all_key.key_algorithm_id
         LEFT JOIN certificate_sans cs
                   ON c.id = cs.certificate_id
GROUP BY
    c.id,
    c.csr_pem,
    c.cert_pem,
    c.key_pem,
    c.public_key_pem,
    c.chain_pem,
    c.key_algorithm_id,
    all_key.algorithm,
    all_key.key_size,
    all_key.display_name,
    all_key.deprecated,
    c.organization,
    c.organizational_unit,
    c.country,
    c.state_or_province,
    c.locality,
    c.email,
    c.fingerprint,
    c.valid_from,
    c.expires_on,
    c.created_on,
    c.updated_on,
    c.cert_uploaded_on,
    c.deleted_on;




-- ============================================================
-- Useful indexes
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_key_algorithms_algorithm ON key_algorithms (algorithm);
CREATE INDEX IF NOT EXISTS idx_rsa_key_size ON rsa_key_algorithm (key_size);
CREATE INDEX IF NOT EXISTS idx_ecdsa_curve ON ecdsa_key_algorithm (curve_size);
CREATE INDEX IF NOT EXISTS idx_certificates_algorithm_id ON certificates (key_algorithm_id);
CREATE INDEX IF NOT EXISTS idx_certificates_fingerprint ON certificates (fingerprint);


-- Subject Alternative Names (many-to-many relationship)
CREATE TABLE certificate_sans
(
    id             uuid PRIMARY KEY      DEFAULT uuid_generate_v4(),
    certificate_id uuid         NOT NULL REFERENCES certificates (id) ON DELETE CASCADE,
    san_value      VARCHAR(255) NOT NULL,
    san_order      INTEGER      NOT NULL DEFAULT 0, -- First SAN becomes CN
    created_on     timestamptz  NOT NULL DEFAULT NOW(),
    updated_on     timestamptz  NOT NULL DEFAULT NOW(),
    UNIQUE (certificate_id, san_value)
);

-- Indexes for common queries
CREATE INDEX idx_certificates_fingerprint ON certificates (fingerprint);
CREATE INDEX idx_certificates_expires_on ON certificates (expires_on);
CREATE INDEX idx_certificates_created_on ON certificates (created_on);
CREATE INDEX idx_certificates_deleted_on ON certificates (deleted_on) WHERE deleted_on IS NULL;
CREATE INDEX idx_certificate_sans_value ON certificate_sans (san_value);
CREATE INDEX idx_certificate_sans_cert_order ON certificate_sans (certificate_id, san_order);

-- Composite index for finding legacy_certificates by subject fields
CREATE INDEX idx_certificates_subject ON certificates (
                                                       organization,
                                                       organizational_unit,
                                                       country,
                                                       state_or_province,
                                                       locality
    ) WHERE deleted_on IS NULL;

-- Function to automatically update updated_on timestamp
CREATE OR REPLACE FUNCTION update_updated_on_column()
    RETURNS TRIGGER AS
$$
BEGIN
    new.updated_on = NOW();
    RETURN new;
END;
$$ LANGUAGE plpgsql;

-- Trigger for updated_on
CREATE TRIGGER update_certificates_updated_on
    BEFORE UPDATE
    ON certificates
    FOR EACH ROW
EXECUTE FUNCTION update_updated_on_column();

-- View for easy querying with all SANs as array
CREATE VIEW certificates_with_sans AS
SELECT c.*,
       COALESCE(
                       ARRAY_AGG(cs.san_value ORDER BY cs.san_order)
                       FILTER (WHERE cs.san_value IS NOT NULL),
                       ARRAY []::VARCHAR[]
       )                                                  AS sans,
       (ARRAY_AGG(cs.san_value ORDER BY cs.san_order))[1] AS common_name
FROM certificates c
         LEFT JOIN certificate_sans cs ON c.id = cs.certificate_id
GROUP BY c.id;

-- View for active (non-deleted, valid) legacy_certificates
CREATE VIEW active_certificates AS
SELECT *
FROM certificates_with_sans
WHERE deleted_on IS NULL
  AND cert_pem IS NOT NULL
  AND expires_on > NOW();

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
         created_on DESC;

-- View for legacy_certificates that will expire soon (useful for renewal planning)
CREATE VIEW expiring_certificates AS
SELECT id,
       fingerprint,
       common_name,
       organization,
       expires_on,
       expires_on - NOW()                                                  AS time_until_expiry,
       -- Check if there's a newer cert for the same subject
       EXISTS (SELECT 1
               FROM active_certificates newer
               WHERE newer.organization = certificates_with_sans.organization
                 AND newer.organizational_unit = certificates_with_sans.organizational_unit
                 AND newer.country = certificates_with_sans.country
                 AND newer.state_or_province = certificates_with_sans.state_or_province
                 AND newer.locality = certificates_with_sans.locality
                 AND newer.common_name = certificates_with_sans.common_name
                 AND newer.created_on > certificates_with_sans.created_on) AS has_renewal
FROM certificates_with_sans
WHERE deleted_on IS NULL
  AND expires_on IS NOT NULL
  AND expires_on > NOW()
  AND expires_on < NOW() + INTERVAL '30 days'
ORDER BY expires_on;

-- View for overlapping legacy_certificates (useful for monitoring zero-downtime rotation)
CREATE VIEW overlapping_certificates AS
SELECT c1.id                                                                        AS cert_id_1,
       c1.common_name,
       c1.fingerprint                                                               AS fingerprint_1,
       c1.valid_from                                                                AS valid_from_1,
       c1.expires_on                                                                AS expires_on_1,
       c2.id                                                                        AS cert_id_2,
       c2.fingerprint                                                               AS fingerprint_2,
       c2.valid_from                                                                AS valid_from_2,
       c2.expires_on                                                                AS expires_on_2,
       LEAST(c1.expires_on, c2.expires_on) - GREATEST(c1.valid_from, c2.valid_from) AS overlap_duration
FROM active_certificates c1
         JOIN active_certificates c2 ON
    c1.organization = c2.organization
        AND c1.organizational_unit = c2.organizational_unit
        AND c1.country = c2.country
        AND c1.state_or_province = c2.state_or_province
        AND c1.locality = c2.locality
        AND c1.common_name = c2.common_name
        AND c1.id < c2.id -- Avoid duplicates
WHERE c1.valid_from < c2.expires_on
  AND c2.valid_from < c1.expires_on;

CREATE VIEW certificate_with_sans AS
SELECT c.id,
       c.csr_pem,
       c.cert_pem,
       c.key_pem,
       c.public_key_pem,
       c.chain_pem,
--        c.key_algorithm,
--        c.rsa_key_size,
--        c.ecdsa_curve,
       c.organization,
       c.organizational_unit,
       c.country,
       c.state_or_province,
       c.locality,
       c.email,
       c.fingerprint,
       c.valid_from,
       c.expires_on,
       c.created_on,
       c.updated_on,
       c.cert_uploaded_on,
       c.deleted_on,
       COALESCE(ARRAY_AGG(cs.san_value ORDER BY cs.san_order) FILTER (WHERE cs.san_value IS NOT NULL),
                ARRAY []::CHARACTER VARYING[])            AS sans,
       (ARRAY_AGG(cs.san_value ORDER BY cs.san_order))[1] AS common_name
FROM certificates c
         LEFT JOIN certificate_sans cs ON c.id = cs.certificate_id
GROUP BY c.id



