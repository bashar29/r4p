-- Add migration script here
CREATE TABLE offers_status(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    status TEXT NOT NULL UNIQUE
);

CREATE TABLE offers(
    id uuid NOT NULL,
    PRIMARY KEY (id),    
    status_id uuid NOT NULL
        REFERENCES offers_status(id),
    pitch TEXT NOT NULL,
    tj MONEY,
    cv text,
    recipient_id uuid NOT NULL
        REFERENCES recipients(id)
);