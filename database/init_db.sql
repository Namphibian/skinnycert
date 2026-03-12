-- ============================================================
-- UUID extension
-- ============================================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================
-- TLS Statuses
-- ============================================================
DROP TABLE IF EXISTS key_algorithm_type_tls_statuses CASCADE;

CREATE TABLE key_algorithm_type_tls_statuses
(
    id          uuid PRIMARY KEY      DEFAULT uuid_generate_v4(),
    name        VARCHAR(64)  NOT NULL,
    description VARCHAR(256) NOT NULL,

    created_on  timestamptz  NOT NULL DEFAULT NOW(),
    updated_on  timestamptz
);

COMMENT ON TABLE key_algorithm_type_tls_statuses
    IS 'TLS compatibility statuses for algorithm types (RSA, ECDSA, Ed25519, etc.). Determines whether an algorithm family is usable for browser-trusted TLS certificates.';

COMMENT ON COLUMN key_algorithm_type_tls_statuses.id
    IS 'Primary key for the TLS status entry.';

COMMENT ON COLUMN key_algorithm_type_tls_statuses.name
    IS 'Short identifier for the TLS status (e.g., supported, not_supported, experimental). Must be unique.';

COMMENT ON COLUMN key_algorithm_type_tls_statuses.description
    IS 'Human-readable explanation of the TLS status and its intended use.';

COMMENT ON COLUMN key_algorithm_type_tls_statuses.created_on
    IS 'Timestamp when this TLS status entry was created.';

COMMENT ON COLUMN key_algorithm_type_tls_statuses.updated_on
    IS 'Timestamp when this TLS status entry was last updated.';

ALTER TABLE key_algorithm_type_tls_statuses
    ADD CONSTRAINT unq_key_algorithm_type_tls_statuses_name UNIQUE (name);

COMMENT ON CONSTRAINT unq_key_algorithm_type_tls_statuses_name ON key_algorithm_type_tls_statuses
    IS 'Ensures that each TLS status name is unique.';


-- Seed TLS statuses
INSERT INTO key_algorithm_type_tls_statuses
(
    name,
    description
)
VALUES
    (
        'supported',
        'Usable for browser-trusted TLS certificates'
    ),
    (
        'not_supported',
        'Not usable for TLS certificates'
    ),
    (
        'experimental',
        'Future or internal-only algorithms'
    );

DROP TABLE IF EXISTS key_algorithm_statuses CASCADE;

CREATE TABLE key_algorithm_statuses
(
    id          uuid PRIMARY KEY      DEFAULT uuid_generate_v4(),
    name        VARCHAR(64)  NOT NULL,
    description VARCHAR(256) NOT NULL,

    created_on  timestamptz  NOT NULL DEFAULT NOW(),
    updated_on  timestamptz
);

COMMENT ON TABLE key_algorithm_statuses
    IS 'Operational statuses for specific key algorithms (e.g., RSA-2048, P-256). Indicates whether a concrete algorithm is TLS-secure, deprecated, internal-only, forbidden, etc.';

COMMENT ON COLUMN key_algorithm_statuses.id
    IS 'Primary key for the key algorithm status entry.';

COMMENT ON COLUMN key_algorithm_statuses.name
    IS 'Short identifier for the key algorithm status (e.g., tls_secure, tls_insecure, internal_only, deprecated, forbidden, experimental). Must be unique.';

COMMENT ON COLUMN key_algorithm_statuses.description
    IS 'Human-readable explanation of the operational status and its intended use.';

COMMENT ON COLUMN key_algorithm_statuses.created_on
    IS 'Timestamp when this key algorithm status entry was created.';

COMMENT ON COLUMN key_algorithm_statuses.updated_on
    IS 'Timestamp when this key algorithm status entry was last updated.';

ALTER TABLE key_algorithm_statuses
    ADD CONSTRAINT unq_key_algorithm_statuses_name UNIQUE (name);

COMMENT ON CONSTRAINT unq_key_algorithm_statuses_name ON key_algorithm_statuses
    IS 'Ensures that each key algorithm status name is unique.';


INSERT INTO key_algorithm_statuses
(
    name,
    description
)
VALUES
    (
        'tls_secure',
        'Safe for browser-trusted TLS certificates'
    ),
    (
        'tls_insecure',
        'Cryptographically weak for TLS but still functional'
    ),
    (
        'internal_only',
        'Safe for internal PKI but not accepted by browsers'
    ),
    (
        'deprecated',
        'Supported but discouraged for new deployments'
    ),
    (
        'forbidden',
        'Must not be used'
    ),
    (
        'experimental',
        'Future or PQC algorithms not yet standardized'
    );

-- ============================================================
-- Algorithm Types
-- ============================================================
DROP TABLE IF EXISTS key_algorithm_types CASCADE;

CREATE TABLE key_algorithm_types
(
    id                uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    name              VARCHAR(64) NOT NULL,
    description       VARCHAR(256),

    requires_nid      BOOLEAN     NOT NULL DEFAULT FALSE,
    requires_strength BOOLEAN     NOT NULL DEFAULT TRUE,

    tls_status_id     uuid        NOT NULL,

    created_on        timestamptz NOT NULL DEFAULT NOW(),
    updated_on        timestamptz
);

COMMENT ON TABLE key_algorithm_types
    IS 'Defines algorithm families (RSA, ECDSA, Ed25519, X25519). Each entry describes the behavior and TLS compatibility of an algorithm type.';

COMMENT ON COLUMN key_algorithm_types.id
    IS 'Primary key for the algorithm type entry.';

COMMENT ON COLUMN key_algorithm_types.name
    IS 'Short identifier for the algorithm type (e.g., RSA, ECDSA, Ed25519, X25519). Must be unique.';

COMMENT ON COLUMN key_algorithm_types.description
    IS 'Human-readable description of the algorithm type.';

COMMENT ON COLUMN key_algorithm_types.requires_nid
    IS 'Indicates whether this algorithm type requires an OpenSSL NID (true for ECDSA curves).';

COMMENT ON COLUMN key_algorithm_types.requires_strength
    IS 'Indicates whether this algorithm type requires a key strength value (true for RSA and ECDSA).';

COMMENT ON COLUMN key_algorithm_types.tls_status_id
    IS 'Foreign key referencing key_algorithm_type_tls_statuses.id, describing TLS compatibility for this algorithm type.';

COMMENT ON COLUMN key_algorithm_types.created_on
    IS 'Timestamp when this algorithm type entry was created.';

COMMENT ON COLUMN key_algorithm_types.updated_on
    IS 'Timestamp when this algorithm type entry was last updated.';

ALTER TABLE key_algorithm_types
    ADD CONSTRAINT unq_key_algorithm_types_name UNIQUE (name);

COMMENT ON CONSTRAINT unq_key_algorithm_types_name ON key_algorithm_types
    IS 'Ensures that each algorithm type name is unique.';

ALTER TABLE key_algorithm_types
    ADD CONSTRAINT fk_key_algorithm_types_tls_status
        FOREIGN KEY (tls_status_id) REFERENCES key_algorithm_type_tls_statuses (id);

COMMENT ON CONSTRAINT fk_key_algorithm_types_tls_status ON key_algorithm_types
    IS 'Links each algorithm type to its TLS compatibility status.';


-- Seed algorithm types
INSERT INTO key_algorithm_types
(
    name,
    description,
    requires_nid,
    requires_strength,
    tls_status_id
)
SELECT 'RSA', 'Rivest–Shamir–Adleman', FALSE, TRUE, id
FROM key_algorithm_type_tls_statuses
WHERE name = 'supported'
UNION ALL
SELECT 'ECDSA', 'Elliptic Curve Digital Signature Algorithm', TRUE, TRUE, id
FROM key_algorithm_type_tls_statuses
WHERE name = 'supported'
UNION ALL
SELECT 'Ed25519', 'Edwards-curve Digital Signature Algorithm', FALSE, FALSE, id
FROM key_algorithm_type_tls_statuses
WHERE name = 'not_supported'
UNION ALL
SELECT 'X25519', 'Montgomery curve Diffie–Hellman key exchange', FALSE, FALSE, id
FROM key_algorithm_type_tls_statuses
WHERE name = 'not_supported';

-- ============================================================
-- Key Algorithms
-- ============================================================
DROP TABLE IF EXISTS key_algorithms CASCADE;

CREATE TABLE key_algorithms
(
    id                uuid PRIMARY KEY      DEFAULT uuid_generate_v4(),
    algorithm_type_id uuid         NOT NULL,
    status_id         uuid         NOT NULL,
    key_strength      INTEGER      NULL,
    nid_value         INTEGER      NULL,
    display_name      VARCHAR(256) NOT NULL,
    created_on        timestamptz  NOT NULL DEFAULT NOW(),
    updated_on        timestamptz
);

COMMENT ON TABLE key_algorithms
    IS 'Concrete algorithm configurations (e.g., RSA-2048, P-256, Ed25519). Each row represents a specific usable algorithm instance.';

COMMENT ON COLUMN key_algorithms.id
    IS 'Primary key for the key algorithm entry.';

COMMENT ON COLUMN key_algorithms.algorithm_type_id
    IS 'Foreign key referencing key_algorithm_types.id, identifying the algorithm family (RSA, ECDSA, Ed25519, etc.).';

COMMENT ON COLUMN key_algorithms.status_id
    IS 'Foreign key referencing key_algorithm_statuses.id, describing the operational status of this specific algorithm (e.g., tls_secure, deprecated, internal_only).';

COMMENT ON COLUMN key_algorithms.key_strength
    IS 'Key strength value (e.g., RSA bit length or ECDSA curve size). NULL for algorithms that do not use strength.';

COMMENT ON COLUMN key_algorithms.nid_value
    IS 'OpenSSL NID for ECDSA curves. NULL for non-ECDSA algorithms.';

COMMENT ON COLUMN key_algorithms.display_name
    IS 'Human-readable name for the algorithm instance (e.g., "RSA 2048-bit", "NIST P-256").';

COMMENT ON COLUMN key_algorithms.created_on
    IS 'Timestamp when this key algorithm entry was created.';

COMMENT ON COLUMN key_algorithms.updated_on
    IS 'Timestamp when this key algorithm entry was last updated.';


-- ============================================================
-- Foreign Keys
-- ============================================================

ALTER TABLE key_algorithms
    ADD CONSTRAINT fk_key_algorithms_algorithm_type
        FOREIGN KEY (algorithm_type_id)
            REFERENCES key_algorithm_types (id);

COMMENT ON CONSTRAINT fk_key_algorithms_algorithm_type ON key_algorithms
    IS 'Links each key algorithm to its algorithm type definition.';

ALTER TABLE key_algorithms
    ADD CONSTRAINT fk_key_algorithms_status
        FOREIGN KEY (status_id)
            REFERENCES key_algorithm_statuses (id);

COMMENT ON CONSTRAINT fk_key_algorithms_status ON key_algorithms
    IS 'Links each key algorithm to its operational status classification.';

CREATE UNIQUE INDEX udix_key_algorithms_key_strength_algorithm_type_id_nid_value
    ON key_algorithms (key_strength, algorithm_type_id, nid_value)
    NULLS NOT DISTINCT;


-- ============================================================
-- Validation Trigger (cross-table rules)
-- ============================================================

CREATE OR REPLACE FUNCTION validate_key_algorithm()
    RETURNS TRIGGER AS
$$
DECLARE
    at RECORD;
BEGIN
    SELECT requires_nid, requires_strength, name
    INTO at
    FROM key_algorithm_types
    WHERE id = new.algorithm_type_id;

    -- Enforce nid_value rules
    IF at.requires_nid AND new.nid_value IS NULL THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514',
            MESSAGE = 'Algorithm type requires nid_value but none was provided';
    END IF;

    IF NOT at.requires_nid AND new.nid_value IS NOT NULL THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514',
            MESSAGE = 'Algorithm type forbids nid_value but one was provided';
    END IF;

    -- Enforce key_strength rules
    IF at.requires_strength AND new.key_strength IS NULL THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514',
            MESSAGE = 'Algorithm type requires key_strength but none was provided';
    END IF;

    -- RSA-specific rule: key size must be a multiple of 1024
    IF at.name = 'RSA' AND new.key_strength IS NOT NULL AND new.key_strength % 1024 <> 0 THEN
        RAISE EXCEPTION USING
            ERRCODE = '23514',
            MESSAGE = FORMAT(
                    'RSA key size (%s) must be a multiple of 1024',
                    new.key_strength
                      );
    END IF;

    RETURN new;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION validate_key_algorithm()
    IS 'Validates cross-table rules for key algorithms, enforcing NID requirements, strength requirements, and RSA-specific constraints.';

CREATE TRIGGER key_algorithms_validate
    BEFORE INSERT OR UPDATE
    ON key_algorithms
    FOR EACH ROW
EXECUTE FUNCTION validate_key_algorithm();

COMMENT ON TRIGGER key_algorithms_validate ON key_algorithms
    IS 'Ensures that inserted or updated key algorithms comply with algorithm type rules.';

DROP VIEW IF EXISTS key_algorithm_info CASCADE;
CREATE OR REPLACE VIEW key_algorithm_info AS
SELECT
    -- key_algorithms
    ka.id                 AS key_algorithm_id,
    ka.algorithm_type_id  AS key_algorithm_type_id,
    ka.status_id          AS key_algorithm_status_id,
    ka.key_strength       AS key_algorithm_strength,
    ka.nid_value          AS key_algorithm_nid_value,
    ka.display_name       AS key_algorithm_display_name,
    ka.created_on         AS key_algorithm_created_on,
    ka.updated_on         AS key_algorithm_updated_on,

    -- key_algorithm_types
    kat.id                AS algorithm_type_id,
    kat.name              AS algorithm_type_name,
    kat.description       AS algorithm_type_description,
    kat.requires_nid      AS algorithm_type_requires_nid,
    kat.requires_strength AS algorithm_type_requires_strength,
    kat.tls_status_id     AS algorithm_type_tls_status_id,
    kat.created_on        AS algorithm_type_created_on,
    kat.updated_on        AS algorithm_type_updated_on,

    -- key_algorithm_statuses
    kas.id                AS status_id,
    kas.name              AS status_name,
    kas.description       AS status_description,
    kas.created_on        AS status_created_on,
    kas.updated_on        AS status_updated_on,

    -- key_algorithm_type_tls_statuses
    katts.id              AS tls_status_id,
    katts.name            AS tls_status_name,
    katts.description     AS tls_status_description,
    katts.created_on      AS tls_status_created_on,
    katts.updated_on      AS tls_status_updated_on

FROM key_algorithms ka
         LEFT JOIN key_algorithm_types kat
                   ON ka.algorithm_type_id = kat.id
         INNER JOIN key_algorithm_statuses kas
                    ON ka.status_id = kas.id
         INNER JOIN key_algorithm_type_tls_statuses katts
                    ON kat.tls_status_id = katts.id;

COMMENT ON VIEW key_algorithm_info
    IS 'Flattened view combining key algorithms, algorithm types, operational statuses, and TLS compatibility metadata.';

-- key_algorithms columns
COMMENT ON COLUMN key_algorithm_info.key_algorithm_id
    IS 'Primary key of the key_algorithms entry.';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_type_id
    IS 'Foreign key referencing key_algorithm_types.id.';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_status_id
    IS 'Foreign key referencing key_algorithm_statuses.id.';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_strength
    IS 'Key strength (e.g., RSA bit length or ECDSA curve size).';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_nid_value
    IS 'OpenSSL NID for ECDSA curves; NULL for non-ECDSA algorithms.';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_display_name
    IS 'Human-readable name for the key algorithm.';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_created_on
    IS 'Timestamp when the key algorithm entry was created.';
COMMENT ON COLUMN key_algorithm_info.key_algorithm_updated_on
    IS 'Timestamp when the key algorithm entry was last updated.';

-- key_algorithm_types columns
COMMENT ON COLUMN key_algorithm_info.algorithm_type_id
    IS 'Primary key of the key_algorithm_types entry.';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_name
    IS 'Name of the algorithm type (RSA, ECDSA, Ed25519, etc.).';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_description
    IS 'Human-readable description of the algorithm type.';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_requires_nid
    IS 'Whether this algorithm type requires an OpenSSL NID.';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_requires_strength
    IS 'Whether this algorithm type requires a key strength value.';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_tls_status_id
    IS 'Foreign key referencing key_algorithm_type_tls_statuses.id.';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_created_on
    IS 'Timestamp when the algorithm type entry was created.';
COMMENT ON COLUMN key_algorithm_info.algorithm_type_updated_on
    IS 'Timestamp when the algorithm type entry was last updated.';

-- key_algorithm_statuses columns
COMMENT ON COLUMN key_algorithm_info.status_id
    IS 'Primary key of the key_algorithm_statuses entry.';
COMMENT ON COLUMN key_algorithm_info.status_name
    IS 'Operational status name (TLS_SECURE, TLS_INSECURE, etc.).';
COMMENT ON COLUMN key_algorithm_info.status_description
    IS 'Human-readable description of the operational status.';
COMMENT ON COLUMN key_algorithm_info.status_created_on
    IS 'Timestamp when the status entry was created.';
COMMENT ON COLUMN key_algorithm_info.status_updated_on
    IS 'Timestamp when the status entry was last updated.';

-- key_algorithm_type_tls_statuses columns
COMMENT ON COLUMN key_algorithm_info.tls_status_id
    IS 'Primary key of the key_algorithm_type_tls_statuses entry.';
COMMENT ON COLUMN key_algorithm_info.tls_status_name
    IS 'TLS compatibility status name (supported, not_supported, etc.).';
COMMENT ON COLUMN key_algorithm_info.tls_status_description
    IS 'Human-readable description of the TLS compatibility status.';
COMMENT ON COLUMN key_algorithm_info.tls_status_created_on
    IS 'Timestamp when the TLS status entry was created.';
COMMENT ON COLUMN key_algorithm_info.tls_status_updated_on
    IS 'Timestamp when the TLS status entry was last updated.';

DROP VIEW IF EXISTS key_algorithm_type_info CASCADE;
CREATE OR REPLACE VIEW key_algorithm_type_info AS
SELECT
    -- key_algorithm_types
    kat.id                AS key_algorithm_type_id,
    kat.name              AS key_algorithm_type_name,
    kat.description       AS key_algorithm_type_description,
    kat.requires_nid      AS key_algorithm_type_requires_nid,
    kat.requires_strength AS key_algorithm_type_requires_strength,
    kat.created_on        AS key_algorithm_type_created_on,
    kat.updated_on        AS key_algorithm_type_updated_on,
    -- key_algorithm_type_tls_statuses
    katts.id              AS key_algorithm_type_tls_status_id,
    katts.name            AS key_algorithm_type_tls_status_name,
    katts.description     AS key_algorithm_type_tls_status_description,
    katts.created_on      AS key_algorithm_type_tls_status_created_on,
    katts.updated_on      AS key_algorithm_type_tls_status_updated_on

FROM key_algorithm_types kat
         INNER JOIN key_algorithm_type_tls_statuses katts
                    ON kat.tls_status_id = katts.id;
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
-- Main certificates table
-- ============================================================
DROP TABLE IF EXISTS certificates CASCADE;
CREATE TABLE certificates
(
    id                  uuid PRIMARY KEY      DEFAULT uuid_generate_v4(),

    -- PEM data
    csr_pem             TEXT         NOT NULL,
    cert_pem            TEXT         NULL, -- NULL until signed by CA
    key_pem             TEXT         NOT NULL,
    public_key_pem      TEXT         NOT NULL,
    chain_pem           TEXT         NULL, -- NULL until signed by CA

    -- Link to polymorphic base algorithm row (points to either RSA or ECDSA child row)
    key_algorithm_id    uuid         NOT NULL REFERENCES key_algorithms (id),

    -- Subject details
    organization        VARCHAR(256) NOT NULL,
    organizational_unit VARCHAR(128) NULL,
    country             CHAR(2)      NOT NULL,
    state_or_province   VARCHAR(128) NULL,
    locality            VARCHAR(128) NULL,
    email               VARCHAR(256) NULL,

    -- Certificate metadata
    fingerprint         VARCHAR(64) UNIQUE,
    valid_from          timestamptz,
    valid_to            timestamptz,

    -- Audit timestamps
    created_on          timestamptz  NOT NULL DEFAULT NOW(),
    updated_on          timestamptz  NOT NULL DEFAULT NOW(),
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
-- Useful indexes
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_certificates_algorithm_id ON certificates (key_algorithm_id);
CREATE INDEX IF NOT EXISTS idx_certificates_fingerprint ON certificates (fingerprint);

-- Subject Alternative Names (many-to-many relationship)
DROP TABLE IF EXISTS certificate_sans CASCADE;
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
CREATE INDEX idx_certificate_sans_value ON certificate_sans (san_value);
CREATE INDEX idx_certificate_sans_cert_order ON certificate_sans (certificate_id, san_order);
-- Indexes for common queries
CREATE INDEX idx_certificates_fingerprint ON certificates (fingerprint);
CREATE INDEX idx_certificates_valid_to ON certificates (valid_to);
CREATE INDEX idx_certificates_created_on ON certificates (created_on);
CREATE INDEX idx_certificates_deleted_on ON certificates (deleted_on) WHERE deleted_on IS NULL;


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
  AND valid_to > NOW();

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
       valid_to,
       valid_to - NOW()                                                    AS time_until_expiry,
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
  AND valid_to IS NOT NULL
  AND valid_to > NOW()
  AND valid_to < NOW() + INTERVAL '30 days'
ORDER BY valid_to;

-- View for overlapping legacy_certificates (useful for monitoring zero-downtime rotation)
CREATE VIEW overlapping_certificates AS
SELECT c1.id                                                                    AS cert_id_1,
       c1.common_name,
       c1.fingerprint                                                           AS fingerprint_1,
       c1.valid_from                                                            AS valid_from_1,
       c1.valid_to                                                              AS valid_to_1,
       c2.id                                                                    AS cert_id_2,
       c2.fingerprint                                                           AS fingerprint_2,
       c2.valid_from                                                            AS valid_from_2,
       c2.valid_to                                                              AS valid_to_2,
       LEAST(c1.valid_to, c2.valid_to) - GREATEST(c1.valid_from, c2.valid_from) AS overlap_duration
FROM active_certificates c1
         JOIN active_certificates c2 ON
    c1.organization = c2.organization
        AND c1.organizational_unit = c2.organizational_unit
        AND c1.country = c2.country
        AND c1.state_or_province = c2.state_or_province
        AND c1.locality = c2.locality
        AND c1.common_name = c2.common_name
        AND c1.id < c2.id -- Avoid duplicates
WHERE c1.valid_from < c2.valid_to
  AND c2.valid_from < c1.valid_to;

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
       c.valid_to,
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

DROP VIEW IF EXISTS certificate_info CASCADE;

CREATE OR REPLACE VIEW certificate_info AS
SELECT
    -- Certificate core fields
    c.id,
    c.csr_pem,
    c.cert_pem,
    c.key_pem,
    c.public_key_pem,
    c.chain_pem,
    c.key_algorithm_id,

    -- Expanded algorithm metadata from key_algorithm_info
    all_key.key_algorithm_display_name                AS key_algorithm_display_name,
    all_key.key_algorithm_strength                    AS key_algorithm_key_strength,
    all_key.key_algorithm_nid_value                   AS key_algorithm_nid_value,
    all_key.key_algorithm_created_on,
    all_key.key_algorithm_updated_on,

    -- Algorithm status
    all_key.status_id,
    all_key.status_name,
    all_key.status_description,
    all_key.status_created_on,
    all_key.status_updated_on,

    -- Algorithm type
    all_key.algorithm_type_id,
    all_key.algorithm_type_name,
    all_key.algorithm_type_description,
    all_key.algorithm_type_requires_nid,
    all_key.algorithm_type_requires_strength,
    all_key.algorithm_type_created_on,
    all_key.algorithm_type_updated_on,

    -- TLS status
    all_key.tls_status_id,
    all_key.tls_status_name,
    all_key.tls_status_description,
    all_key.tls_status_created_on,
    all_key.tls_status_updated_on,
    -- Subject details
    c.organization,
    c.organizational_unit,
    c.country,
    c.state_or_province,
    c.locality,
    c.email,

    -- SANs (ordered array)
    COALESCE(
                    ARRAY_AGG(cs.san_value ORDER BY cs.san_order)
                    FILTER (WHERE cs.san_value IS NOT NULL),
                    ARRAY []::VARCHAR[]
    )                                                 AS sans,

    -- Common Name = first SAN (san_order = 0)
    MIN(cs.san_value) FILTER (WHERE cs.san_order = 0) AS common_name,

    -- Certificate metadata
    c.fingerprint,
    c.valid_from,
    c.valid_to,

    -- Derived metadata
    COALESCE((c.cert_pem IS NOT NULL), FALSE)         AS is_signed,
    COALESCE((NOW() > c.valid_to), FALSE)             AS is_expired,

    -- Audit timestamps
    c.created_on,
    c.updated_on,
    c.cert_uploaded_on,
    c.deleted_on

FROM certificates c
         JOIN key_algorithm_info all_key
              ON c.key_algorithm_id = all_key.key_algorithm_id
         LEFT JOIN certificate_sans cs
                   ON c.id = cs.certificate_id

GROUP BY c.id,
         c.csr_pem,
         c.cert_pem,
         c.key_pem,
         c.public_key_pem,
         c.chain_pem,
         c.key_algorithm_id,

         -- All fields from key_algorithm_info
         all_key.key_algorithm_display_name,
         all_key.key_algorithm_strength,
         all_key.key_algorithm_nid_value,
         all_key.key_algorithm_created_on,
         all_key.key_algorithm_updated_on,
         all_key.status_id,
         all_key.status_name,
         all_key.status_description,
         all_key.status_created_on,
         all_key.status_updated_on,
         all_key.algorithm_type_id,
         all_key.algorithm_type_name,
         all_key.algorithm_type_description,
         all_key.algorithm_type_requires_nid,
         all_key.algorithm_type_requires_strength,
         all_key.algorithm_type_created_on,
         all_key.algorithm_type_updated_on,
         all_key.tls_status_id,
         all_key.tls_status_name,
         all_key.tls_status_description,
         all_key.tls_status_created_on,
         all_key.tls_status_updated_on,
         all_key.tls_status_updated_on,
         -- Certificate subject + metadata
         c.organization,
         c.organizational_unit,
         c.country,
         c.state_or_province,
         c.locality,
         c.email,
         c.fingerprint,
         c.valid_from,
         c.valid_to,
         c.created_on,
         c.updated_on,
         c.cert_uploaded_on,
         c.deleted_on;

