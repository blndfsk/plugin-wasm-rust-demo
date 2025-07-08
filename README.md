# plugin-wasm-rust-demo

Demo Traefik plugin written in rust. Inspired by https://github.com/juliens/traefik-plugin-rust-demo

This plugin filters incoming requests dependent on the configuration.

## Building

if not already installed, add the wasm-target

```shell
rustup target add wasm32-wasip1
```

Build the plugin with

```shell
make
```

The artifacts are found in target/plugin/

## Installation

This Demo is not meant for installation, it is just a showcase. However, traefik supports a manual installation.

```shell
mkdir -p <traefik>/plugins-local/src/plugindemowasm/
cp target/plugin/plugin.wasm <traefik>/plugins-local/src/plugindemowasm/

```
Configure the static configuration (and restart traefik)
```yaml
# Static configuration

experimental:
  localPlugins:
    plugindemowasm:
      moduleName: plugindemowasm
```
Call the middleware from one of your routers
```yaml
# Dynamic configuration

http:
  routers:
    my-router:
    [...]
      middlewares:
        - plugindemowasm-mw
[...]
  middlewares:
    plugindemowasm-mw:
      plugin:
        plugindemowasm:
          rules:
            - ""
```            