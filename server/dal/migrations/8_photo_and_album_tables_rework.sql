-- Setup new tables

CREATE TABLE photos
(
    id          VARCHAR NOT NULL PRIMARY KEY,
    hash        CHAR(32) UNIQUE,
    uploaded_at BIGINT  NOT NULL DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP),
    uploaded_by INTEGER NOT NULL,
    captured_at BIGINT
);


CREATE TABLE albums
(
    id             VARCHAR NOT NULL PRIMARY KEY,
    name           VARCHAR NOT NULL,
    cover_photo_id VARCHAR REFERENCES photos (id) ON DELETE SET NULL,
    created_at     BIGINT  NOT NULL DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP),
    created_by     INTEGER NOT NULL,
    published      BOOLEAN NOT NULL,
    published_at   BIGINT,
    published_by   INTEGER
);


CREATE TABLE photo_urls
(
    photo_id VARCHAR NOT NULL REFERENCES photos (id) ON DELETE CASCADE,
    url      TEXT    NOT NULL,
    size     VARCHAR NOT NULL,
    width    INTEGER NOT NULL,
    height   INTEGER NOT NULL
);


CREATE TABLE photo_albums
(
    photo_id VARCHAR NOT NULL REFERENCES photos (id) ON DELETE CASCADE,
    album_id VARCHAR NOT NULL REFERENCES albums (id) ON DELETE CASCADE
);


CREATE TABLE photo_exif
(
    photo_id VARCHAR NOT NULL REFERENCES photos (id) ON DELETE CASCADE,
    key      VARCHAR NOT NULL,
    value    TEXT
);

-- Transfer data

INSERT INTO photos (id, uploaded_at, uploaded_by, captured_at)
SELECT id, created_at, 0, created_at
FROM photo_metadata;

INSERT INTO albums (id, name, cover_photo_id, created_at, created_by, published, published_at, published_by)
SELECT id,
       name,
       cover_photo_id,
       created_at,
       created_by,
       NOT is_draft AS published,
       published_at,
       published_by
FROM album_metadata;

DELETE
FROM photo_s3_urls
WHERE photo_id NOT IN (SELECT id
                       FROM photos);

INSERT INTO photo_urls (photo_id, url, size, width, height)
SELECT photo_id, s3_url, quality, 0, 0
FROM photo_s3_urls;

UPDATE photo_urls
SET size = 'Preview'
WHERE size = 'W1600';
UPDATE photo_urls
SET size = 'Thumbnail'
WHERE size = 'W400';

INSERT INTO photo_albums
SELECT id, album_id
FROM photo_metadata;

-- Remove the old tables

ALTER TABLE album_metadata
    DROP CONSTRAINT album_metadata_cover_photo_id_fkey;

DROP TABLE photo_s3_urls;
DROP TABLE photo_metadata;
DROP TABLE album_metadata;