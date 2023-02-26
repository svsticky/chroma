CREATE TABLE photo_metadata (
    id VARCHAR(32) NOT NULL,
    album_id VARCHAR(32) NOT NULL,
    created_at BIGINT NOT NULL,
    PRIMARY KEY (id),
    INDEX (album_id)
) ENGINE=INNODB CHARACTER SET utf8 COLLATE utf8_general_ci;

CREATE TABLE album_metadata (
    id VARCHAR(32) NOT NULL,
    name VARCHAR(64) NOT NULL,
    created_at BIGINT NOT NULL,
    cover_photo_id VARCHAR(32) DEFAULT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (cover_photo_id) REFERENCES photo_metadata (id)
) ENGINE=INNODB CHARACTER SET utf8 COLLATE utf8_general_ci;

ALTER TABLE photo_metadata
    ADD FOREIGN KEY (album_id) REFERENCES album_metadata(id);

CREATE TABLE users (
    koala_id INT(32) UNSIGNED NOT NULL,
    access_token VARCHAR(128) NOT NULL,
    refresh_token VARCHAR(128) NOT NULL,
    expires_at BIGINT NOT NULL,
    is_admin BOOL NOT NULL,
    PRIMARY KEY (koala_id)
) ENGINE=INNODB CHARACTER SET utf8 COLLATE utf8_general_ci;

CREATE TABLE user_sessions (
    id VARCHAR(32) NOT NULL,
    koala_id INT(32) UNSIGNED NOT NULL,
    expires_at BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (koala_id) REFERENCES users(koala_id)
) ENGINE=INNODB CHARACTER SET utf8 COLLATE utf8_general_ci;