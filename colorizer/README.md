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

Simple: 
```bash
./colorizer \
  --pxl-metadadata-file <Name of the metadata file on S3> \
  --chroma-api <URL to Chroma, e.g. https://chroma.svsticky.nl> \
  --chroma-service-token <Your service token> \
  --s3-bucket-name <The name of the Pxl bucket> \
  --s3-region <The region of the Pxl bucket> \
  --s3-endpoint-url <The URL of the Pxl bucket> \
  --s3-access-key-id <The access key ID of the Pxl bucket> \
  --s3-secret-access-key <The secret access key of the Pxl bucket> \
  --s3-force-path-style <Optional, if using MinIO, set this to true>
```

This will run Colorizer, it'll convert albums one at a time.  
This might take a while, depending on the size of your Pxl instance.

>Warning: If for whatever reason Colorizer fails halfway through a migration, it is recommended to revert back to the backup.
>Colorizer has **NO** recovery mechanism whatsoever.

## License
MIT or Apache 2.0, at your option.