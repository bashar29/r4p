-- Add migration script here
CREATE TABLE offers_status(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    code smallint NOT NULL UNIQUE,
    status TEXT NOT NULL UNIQUE
);

CREATE TABLE offers(
    id uuid NOT NULL,
    PRIMARY KEY (id),    
    status_code smallint NOT NULL
        REFERENCES offers_status(code),
    pitch TEXT NOT NULL,
    tj MONEY,
    cv TEXT,
    recipient_id uuid NOT NULL
        REFERENCES recipients(id)
);