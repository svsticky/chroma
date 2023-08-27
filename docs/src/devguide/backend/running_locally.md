# Running Chroma locally
Useful while working on Chroma

## Requirments
- Rust toolchain
- Docker
- Docker compose
- A working Koala installation

## Running
Chroma supports two different storage engines:
- File
- S3

If you're using the `S3` engine, either use the MinIO setup provided in [server/docker-compose.yml], or use an Amazon S3 instance.
Much simpler is using the `File` engine, documented here. 

## Getting going
>Documented for and tested on Ubuntu 22.04

1. `cd server/`.
2. `docker compose up -d postgres`. 
3. Copy `sample.env` to `.env`.
4. With Koala running, navigate to [http://koala.rails.local:3000/api/oauth/applications](http://koala.rails.local:3000/api/oauth/applications) and create a new application.
5. As redirect URI, use the value of `KOALA_OAUTH_REDIRECT_URI` from your `.env` file. Make sure it matches exactly!
6. For scopes, use `member-read`.
7. Edit your `.env`, copy the `UID` from Koala to `KOALA_CLIENT_ID` and `Secret` to `KOALA_CLIENT_SECRET`.
8. Edit your `.env`, update `DB_HOST` to be the IP address of your computer.
9. Chroma can now be compiled and run with
```bash
export $(cat .env | xargs) && cargo run
```

The backend will now be available on port 8000. To change this, add `HTTP_PORT=<YOUR PORT>` to your `.env` file