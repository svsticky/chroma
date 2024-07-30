CREATE TABLE chroma_scopes (
    koala_id INT NOT NULL,
    scope VARCHAR(128) NOT NULL,
    granted_by INT NOT NULL,
    granted_at BIGINT NOT NULL,
    PRIMARY KEY (koala_id, scope),
    FOREIGN KEY (koala_id) REFERENCES users(koala_id),
    FOREIGN KEY (granted_by) REFERENCES users(koala_id)
);