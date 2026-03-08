# plugin-wasm-rust-demo

This repository demonstrates a Traefik plugin implemented in Rust using the [http-wasm-guest](https://crates.io/crates/http-wasm-guest)-crate.
The plugin is designed to add a custom header to incoming HTTP requests.

## Build Instructions

To build the plugin, ensure the WebAssembly target is installed:

```shell
rustup target add wasm32-wasip1
```

Compile the plugin using:

```shell
cargo build --target wasm32-wasip1
```

## Usage

This project serves as a demonstration and is not intended for production deployment.

For testing and experimentation, utilize the included `run.sh` script. This script launches a Traefik container with the compiled plugin and connects the `whoami` service, configuring the plugin as a middleware.

Testing the whoami Service

Once the script is running, you can test the `whoami` service using `curl`:

```shell
curl http://whoami.localhost:8080/
```

This command sends a request to Traefik, which routes it to the `whoami` service. The plugin should add a custom header to the response. 

### Prerequisites

Before running the `run.sh` script, ensure the following dependencies are installed and properly configured:

- **Podman**
  - Install Podman for container and pod management ([installation guide](https://podman.io/getting-started/installation))
  - Ensure Podman is configured to run containers and pods

- **Buildah**
  - Install Buildah for building container images ([installation guide](https://github.com/containers/buildah/blob/main/install.md))
