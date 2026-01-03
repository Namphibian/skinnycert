SELECT *
FROM key_algorithm_info;

INSERT INTO certificates(
                            csr_pem,
                            key_pem,
                            public_key_pem,
                            chain_pem,
                            key_algorithm_id,
                            organization,
                            organizational_unit,
                            country,
                            state_or_province,
                            locality,
                            email,
                            fingerprint

)
VALUES
    (
        'csr',
        'key',
        'public_key',
        'chain',
        (SELECT key_algorithm_id
         FROM key_algorithm_info
         WHERE algorithm_type_name = 'RSA'
         LIMIT 1),
        'organization',
        'organizational_unit',
        'ZA',
        'state_or_province',
        'locality',
        'email',
        'fingerprint'

    );

TRUNCATE TABLE certificates CASCADE;
TRUNCATE TABLE certificate_sans;