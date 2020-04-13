# Uption

A tool to collect and export network performance metrics on devices like WLAN Pi and Raspberry Pi.
This tool was inspired by [wiperf](https://github.com/wifinigel/wiperf) project.

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

## Project status

🚧 This project is in very early stage of development 🚧

### Features

Uption has a concept of _collectors_ and _exporters_. Collectors generate metrics based on different tests and exporters export the generated data.

#### Exporters

- [x] Stdout
- [x] InfluxDB

#### Collectors

- [x] HTTP
- [x] Ping

### Feature parity with Wiperf

#### Exporters

- [ ] Splunk ([issue #13](https://github.com/uption/uption/issues/13))
- [ ] JSON file ([issue #11](https://github.com/uption/uption/issues/11))
- [ ] CSV file ([issue #12](https://github.com/uption/uption/issues/12))

#### Collectors

- [ ] DHCP ([issue #14](https://github.com/uption/uption/issues/14))
- [ ] DNS ([issue #15](https://github.com/uption/uption/issues/15))
- [x] HTTP
- [ ] Iperf3 ([issue #16](https://github.com/uption/uption/issues/16))
- [ ] Ookla Speedtest ([issue #17](https://github.com/uption/uption/issues/17))
- [x] Ping
- [ ] Wireless adapter ([issue #18](https://github.com/uption/uption/issues/18))

## Contributions

We welcome and appreciate pull requests. Please check the issues if you wish to contribute.
