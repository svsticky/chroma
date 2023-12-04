CREATE TABLE service_tokens (
    id VARCHAR(64) NOT NULL,
    token_hash VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    created_by INT NOT NULL,
    created_at BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (created_by) REFERENCES users(koala_id)
);

DROP TABLE service_token_user;

ALTER TABLE chroma_scopes
    DROP CONSTRAINT chroma_scopes_pkey,
    ADD COLUMN service_token_id VARCHAR(64) DEFAULT NULL,
    DROP CONSTRAINT chroma_scopes_koala_id_fkey,
    ALTER COLUMN koala_id SET DEFAULT NULL,
    ADD COLUMN owner_type user_type NOT NULL,
    ADD PRIMARY KEY (koala_id, service_token_id, scope),
    ADD CONSTRAINT chroma_scopes_koala_id_fkey FOREIGN KEY (koala_id) REFERENCES users(koala_id),
    ADD CONSTRAINT chroma_scopes_service_token_id_fkey FOREIGN KEY (service_token_id) REFERENCES service_tokens(id);

ALTER TABLE album_metadata RENAME created_by TO created_by_koala_id;
ALTER TABLE album_metadata RENAME published_by TO published_by_koala_id;
ALTER TABLE album_metadata
    ALTER COLUMN created_by_koala_id SET DEFAULT NULL,
    ALTER COLUMN published_by_koala_id SET DEFAULT NULL,
    ADD COLUMN created_by_service_token_id VARCHAR(64) DEFAULT NULL,
    ADD COLUMN published_by_service_token_id VARCHAR(64) DEFAULT NULL,
    ADD CONSTRAINT created_by_service_token_id_fkey FOREIGN KEY (created_by_service_token_id) REFERENCES service_tokens(id),
    ADD CONSTRAINT published_by_service_token_id_fkey FOREIGN KEY (published_by_service_token_id) REFERENCES service_tokens(id),
    ADD CONSTRAINT created_by_koala_id FOREIGN KEY (created_by_koala_id) REFERENCES users(koala_id),
    ADD CONSTRAINT published_by_koala_id FOREIGN KEY (published_by_koala_id) REFERENCES users(koala_id);

