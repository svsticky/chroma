CREATE TYPE user_type as ENUM (
    'Koala', 'Service'
);

ALTER TABLE album_metadata
    ADD COLUMN is_draft BOOLEAN NOT NULL,
    ADD COLUMN created_by INT NOT NULL,
    ADD COLUMN created_by_type user_type NOT NULL,
    ADD COLUMN published_by INT DEFAULT NULL,
    ADD COLUMN published_by_type user_type DEFAULT NULL,
    ADD COLUMN published_at BIGINT DEFAULT NULL,
    ADD FOREIGN KEY (created_by) REFERENCES users(koala_id),
    ADD FOREIGN KEY (published_by) REFERENCES users(koala_id);

CREATE TABLE service_token_user (
    id SERIAL NOT NULL,
    service_token VARCHAR(64) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (service_token)
);

ALTER TABLE users
    ADD COLUMN name VARCHAR(64) NOT NULL;