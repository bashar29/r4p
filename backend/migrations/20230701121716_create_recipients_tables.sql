-- Add migration script here
CREATE TABLE companies(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    company_name TEXT NOT NULL UNIQUE
);

CREATE TABLE recipients(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    company_id uuid NOT NULL
        REFERENCES companies(id)
);

ALTER TABLE companies ADD COLUMN main_recipient_id uuid REFERENCES recipients(id);

CREATE TABLE consulted_recipients(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    recipient_id uuid NOT NULL
        REFERENCES recipients(id),
    r4p_id uuid NOT NULL
        REFERENCES r4p(id),
    answered boolean NOT NULL
)