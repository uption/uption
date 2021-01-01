# Uption

[![Github Release](https://img.shields.io/github/v/release/uption/uption?include_prereleases)](https://github.com/uption/uption/releases)
[![Crates.io](https://img.shields.io/crates/v/uption)](https://crates.io/crates/uption)
[![Crates.io](https://img.shields.io/crates/l/uption)](./LICENSE)
[![CI](https://github.com/uption/uption/workflows/CI/badge.svg)](https://github.com/uption/uption/actions?query=workflow%3ACI)
[![docs](https://img.shields.io/badge/docs-Uption%20wiki-blue)](https://github.com/uption/uption/wiki)

A tool to collect and export network performance metrics on Linux devices.
This tool was inspired by [wiperf](https://github.com/wifinigel/wiperf) project.

## Documentation

Uption has a concept of _collectors_ and _exporters_. Collectors generate metrics based on different tests and exporters export the generated data. See documentation for all the supported collectors and exporters. Documentation can be found from [Uption wiki](https://github.com/uption/uption/wiki).

## Installation

### Cargo

When installed as a standalone executable, Uption will read configuration from the current folder or
alternatively you can move the configuration file to `/etc/uption/`.

```sh
cargo install -f uption
wget https://raw.githubusercontent.com/uption/uption/master/uption.toml
uption
```

### Debian package

Get the latest Debian package from [Releases](https://github.com/uption/uption/releases). Debian
package allows installing Uption as a systemd service and the default configuration file is copied
to `/etc/uption/`.

```sh
dpkg -i uption_x.y.z_amd64.deb
sudo systemctl enable uption
sudo systemctl start uption
```

## Contributions

We welcome and appreciate pull requests. Please check the issues if you wish to contribute.
