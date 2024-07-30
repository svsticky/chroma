CREATE TYPE photo_quality as ENUM (
       'Original', 'W400', 'W1600'
);

CREATE TABLE photo_s3_urls (
    photo_id VARCHAR(32) NOT NULL,
    s3_url TEXT NOT NULL,
    quality photo_quality NOT NULL,
    PRIMARY KEY (photo_id, quality)
);