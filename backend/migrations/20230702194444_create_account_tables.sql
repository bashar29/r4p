-- Add migration script here
CREATE TABLE accounts(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    account_name TEXT NOT NULL UNIQUE
);

CREATE TABLE roles(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    code smallint NOT NULL UNIQUE,
    user_role TEXT NOT NULL UNIQUE
);

CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),    
    account_id uuid NOT NULL
        REFERENCES accounts(id),
    lastname TEXT NOT NULL,
    firstname TEXT,
    position TEXT,
    role_code smallint NOT NULL
        REFERENCES roles(code),
    supervisor_id uuid
        REFERENCES users(id)
);