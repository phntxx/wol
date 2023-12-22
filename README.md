# wol

[![GitHub License][shield-license]][license]

![wol screenshot][screenshot]

`wol` (short for [Wake-on-LAN][wikipedia]) is a simple interface for sending magic packets, written in Rust and Svelte.

# Installation

## Installation using Docker

### Example

```sh
docker run -e CONFIG_FILE=<PATH-TO-CONFIG> -p 3000:3000 ghcr.io/phntxx/wol
```

### Example (Docker Compose)

```yml
version: "3.8"
services:

  ...

  wol:
    image: ghcr.io/phntxx/wol
    environment:
      - CONFIG_FILE=<PATH-TO-CONFIG>
    ports:
      - 3000:3000/tcp

  ...
```

## Installation from source

# Configuration

wol is configured using the config file and environment variables:

## Config file

The configuration file tells wol about the saved devices, so that these can be woken up easily.
For an example, see [config.yml][config].

## Environment variables

Environment variable | Docker default    | Description
---------------------|-------------------|--------------------------------------
`RUST_LOG`           | `wol`             | Used to configure backend logging
`CONFIG_FILE`        | `/app/config.yml` | Location of the configuration file
`FRONTEND_PATH`      | `/app/frontend`   | Location of the built frontend
`ADDRESS`            | `0.0.0.0:3000`    | The address the backend will bind to

# Build from source

You can also build wol from source. In order to do so, please run the following commands:

1. Clone this git repository:

    ```sh
    git clone https://github.com/phntxx/wol.git
    ```

2. Build the frontend:

    ```sh
    cd wol/frontend
    npm install
    npm run build
    ```

3. Build the backend:

    ```sh
    cd wol/backend
    cargo build
    ```

# Development

In order to run wol for development purposes, the following commands need to be run:

1. Clone this git repository:

    ```sh
    git clone https://github.com/phntxx/wol.git
    ```

2. Create an initial build of the frontend:

    ```sh
    cd wol/frontend
    npm install
    npm run build
    ```

3. Run the backend using your initial build of the frontend:

    ```sh
    cd wol/backend
    ADDRESS="0.0.0.0:3000" FRONTEND_PATH="../frontend/build" CONFIG_FILE="../config.yml" cargo run
    ```

4. In another terminal, run the vite development server:

    ```sh
    cd wol/frontend
    npm run dev
    ```

The backend should now be accessible on port `3000`, with the frontend preview being available on port `5173`.

[config]: config.yml
[screenshot]: img/screenshot.png
[license]: LICENSE
[shield-license]: https://img.shields.io/github/license/phntxx/wol.svg
[wikipedia]: https://en.wikipedia.org/wiki/Wake-on-LAN
