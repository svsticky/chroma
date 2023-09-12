ALTER TABLE album_metadata
    ADD COLUMN is_draft BOOLEAN NOT NULL,
    ADD COLUMN created_by INT NOT NULL,
    ADD COLUMN published_by INT DEFAULT NULL,
    ADD COLUMN published_at BIGINT DEFAULT NULL,
    ADD FOREIGN KEY (created_by) REFERENCES users(koala_id),
    ADD FOREIGN KEY (published_by) REFERENCES users(koala_id);

CREATE TABLE service_token_user (
    id INT NOT NULL AUTO_INCREMENT,
    service_token VARCHAR(64) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY (service_token)
);