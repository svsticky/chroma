ALTER TABLE photo_metadata
    ADD COLUMN width INT NOT NULL,
    ADD COLUMN height INT NOT NULL,
    ADD COLUMN exif_data JSONB;