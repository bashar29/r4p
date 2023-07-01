-- Add migration script here
CREATE TABLE r4p_status(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    status TEXT NOT NULL UNIQUE
);

CREATE TABLE r4p(
    id uuid NOT NULL,
    PRIMARY KEY (id),    
    status_id uuid NOT NULL
        REFERENCES r4p_status (id),
    pitch TEXT NOT NULL,
    tj_max MONEY
);