# Installation
>Note: This page is a work in progress 

Chroma is packaged as one application. The backend webserver serves the API routes, of course. But it also serves the UI, which is bundled during compilation.

It has the following runtime dependencies:
- A MySQL database
- An S3-compatible storage bucket (E.g. [MinIO](https://min.io) works toos)

Chroma can be deployed as a Docker container.
```yaml
{{#include ../../server/docker-compose.yml}}
```

Before starting the container, one must make sure to create an OAuth client in Koala.
If Koala is running locally, you can navigate to [http://koala.rails.local:3000/api/oauth/applications](http://koala.rails.local:3000/api/oauth/applications) and create a new client.
The redirect uri *must* match what is specified in the `KOALA_OAUTH_REDIRECT_URI` variable. Only the scope `member-read` is required. Koala will then give you an OAuth client and secret key, these must be given to Chroma.

A list of all configuration variables:
```rust,no_run,noplayground
{{#rustdoc_include ../../server/chroma/src/config.rs:config}}
```