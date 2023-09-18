CREATE TABLE photo_metadata (
    id VARCHAR(32) NOT NULL,
    album_id VARCHAR(32) NOT NULL,
    created_at BIGINT NOT NULL,
    PRIMARY KEY (id)
);

CREATE INDEX idx_photo_metaddata_album_id ON photo_metadata(album_id);

CREATE TABLE album_metadata (
    id VARCHAR(32) NOT NULL,
    name VARCHAR(64) NOT NULL,
    created_at BIGINT NOT NULL,
    cover_photo_id VARCHAR(32) DEFAULT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (cover_photo_id) REFERENCES photo_metadata (id)
);

ALTER TABLE photo_metadata
    ADD FOREIGN KEY (album_id) REFERENCES album_metadata(id);

CREATE TABLE users (
    koala_id INT NOT NULL,
    access_token VARCHAR(128) NOT NULL,
    refresh_token VARCHAR(128) NOT NULL,
    expires_at BIGINT NOT NULL,
    is_admin BOOL NOT NULL,
    PRIMARY KEY (koala_id)
);

CREATE TABLE user_sessions (
    id VARCHAR(32) NOT NULL,
    koala_id INT NOT NULL,
    expires_at BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (koala_id) REFERENCES users(koala_id)
);