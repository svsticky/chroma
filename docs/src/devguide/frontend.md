# Frontend

The frontend of Chroma is written in Vue 2 using the Vuetify framework.
Everything is written in TypeScript.
This was chosen mainly as I've got experience in Vue and Chroma was at the time of development on a schedule.

The frontend uses Yarn for package managment. As everything is JavaScript, you should install NodeJS.
I recommend using `nvm` (**N**ode **V**ersion **M**nager), you can find it [here](https://github.com/nvm-sh/nvm).

As of February 2023, NodeJS 16 is used.

After installing NodeJS, you can install Yarn from [here](https://yarnpkg.com/getting-started/install)

After installing the necessary tools, run `yarn install` in the `frontend/` directory. This will install all the node modules that are required.

The following yarn subcommands are available:
- `build` Build a production distribution
- `serve` Serve the frontend locally. It'll be recompiled as you change files
- `protos` Update the automatically generated typescript definitions from protobuf files (Requires Linux and `protoc`)