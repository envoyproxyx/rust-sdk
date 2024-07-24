
# EnvoyX Rust SDK

This is the Rust SDK for the EnvoyX modules. The modules are shared libraries that can be loaded by the EnvoyX proxy to extend HTTP filtering capabilities.

The shared library must be compiled with the same environment as EnvoyX, that means the programs must be compiled
on amd64 Linux with the same version of glibc as the EnvoyX proxy.

This SDK facilitates the creation of Rust-based shared libraries that can be loaded at multiple HTTP filter chain
in Envoy configuration. See the [example](./example) for more details.

## On an amd64 Linux machine

To install the EnvoyX binary locally, the easiest way is to copy the binary from the Docker container:
```bash
docker run --entrypoint=/bin/bash --rm -v $(pwd):/work/envoyx -w /work/envoyx ghcr.io/envoyproxyx/envoy:v1.31-latest-envoyx-main -c "cp /usr/local/bin/envoy /work/envoyx/envoy-bin"
mv envoy-bin /usr/local/bin/envoy
```

where `v1.31` is the Envoy version, and `main` is the [envoyproxyx/envoy](https://github.com/envoyproxyx/envoyx) repository's version (main or tags).
See [github/workflows/commit.yaml](.github/workflows/commit.yaml) for the currently supported versions.

You can build the example and run tests with the following commands:

```bash
cargo test
cargo test --examples
```
and if the test passes, you can assume the shared library is compatible with the EnvoyX.
