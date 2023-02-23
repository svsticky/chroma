CREATE TABLE album_metadata (
    id VARCHAR(32) NOT NULL,
    name VARCHAR(64) NOT NULL,
    created_at BIGINT NOT NULL,
    cover_photo_id VARCHAR(32) DEFAULT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (cover_photo_id) REFERENCES photo_metadata (id)
) ENGINE=INNODB CHARACTER SET utf8 COLLATE utf8_general_ci;

CREATE TABLE photo_metadata (
    id VARCHAR(32) NOT NULL,
    album_id VARCHAR(32) NOT NULL,
    created_at BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (album_id) REFERENCES album_metadata (id),
    INDEX (album_id)
) ENGINE=INNODB CHARACTER SET utf8 COLLATE utf8_general_ci;