CREATE TABLE expenses (
    id             UUID PRIMARY KEY NOT NULL,
    description    VARCHAR(255)     NOT NULL,
    amount         REAL             NOT NULL,
    payment_method VARCHAR(16)      NOT NULL,
    created        TIMESTAMP             NOT NULL,
    last_updated   TIMESTAMP             NOT NULL
);