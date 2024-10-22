INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'kimiyash',
    'kimiyash@gmail.com',
    '$2b$12$xrKA7yE3QF7Fxf1iR5brQepGpRDBMzzeNLHVHdXLtt1voJMSa37zi',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';