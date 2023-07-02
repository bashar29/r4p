-- Add migration script here
CREATE TABLE r4p_status(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    code smallint NOT NULL UNIQUE,
    status TEXT NOT NULL UNIQUE
);

CREATE TABLE r4p(
    id uuid NOT NULL,
    PRIMARY KEY (id),    
    status_code smallint NOT NULL
        REFERENCES r4p_status (code),
    pitch TEXT NOT NULL,
    tj_max MONEY
);