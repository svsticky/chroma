# Colorizer
Tool to migrate an existing Pxl installation to Chroma.

## Requirments
- Access to the S3 bucket backing the Pxl installation
- A service token for Chroma (refer to the config option `service_tokens` in Chroma)

## Installation
There is no prebuild executable of colorizer. Thus, you need the Rust toolchain installed along with the protobuf compiler.
```
cargo install --git https://github.com/svsticky/chroma.git colorizer
```

## Usage
>Warning: Take a backup of Chroma's current S3 and database state!

The following environmental variables must be set:
- `CHROMA_SERVICE_TOKEN`: Valid service token for the Chroma API.
- `S3_ACCESS_KEY_ID`: The access key ID of the Pxl S3 bucket.
- `S3_SECRET_ACCESS_KEY`: The secret access key of the Pxl S3 bucket.
 
```bash
./colorizer \
  --pxl-metadadata-file <Name of the metadata file on S3> \
  --chroma-api <URL to Chroma, e.g. https://chroma.svsticky.nl> \
  --s3-bucket-name <The name of the Pxl bucket> \
  --s3-region <The region of the Pxl bucket> \
  --s3-endpoint-url <The URL of the Pxl bucket> \
  --s3-force-path-style <Optional, if using MinIO, set this to true>
  --start-at-dir <Optional, start at this directory name and skip all albums before it, chronologically>
```

This will run Colorizer, it'll convert albums one at a time.  
This might take a while, depending on the size of your Pxl instance.

>Warning: If for whatever reason Colorizer fails halfway through a migration, it is recommended to revert back to the backup.
>Colorizer has **NO** recovery mechanism whatsoever.

## Quitting halfway through
If you want to quit halfway through, use `Ctrl + \`, this will finish the current album and then exit.
You can then later continue with the ``--start-at-dir`` flag.

## License
MIT or Apache 2.0, at your option.