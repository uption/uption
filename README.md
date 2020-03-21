# Uption

A tool to collect and export network performance metrics on devices like WLAN Pi and Raspberry Pi.
This tool was inspired by [wiperf](https://github.com/wifinigel/wiperf) project.

## Project status

🚧 This project is in very early stage of development 🚧

### Features
Uption has a concept of *receivers* and *exporters*. Receivers generate metrics based on different tests and exporters export the generated data.

#### Exporters
- [x] Stdout
- [ ] InfluxDB (https://github.com/uption/uption/issues/6)

#### Receivers
- [ ] HTTP 🚧 In progress 🚧
- [ ] Ping 🚧 In progress 🚧

### Feature parity with Wiperf

#### Exporters
- [ ] Splunk (https://github.com/uption/uption/issues/13)
- [ ] JSON file (https://github.com/uption/uption/issues/11)
- [ ] CSV file (https://github.com/uption/uption/issues/12)

#### Receivers 
- [ ] DHCP (https://github.com/uption/uption/issues/14)
- [ ] DNS (https://github.com/uption/uption/issues/15)
- [ ] HTTP (https://github.com/uption/uption/issues/4)
- [ ] Iperf3 (https://github.com/uption/uption/issues/16)
- [ ] Ookla Speedtest (https://github.com/uption/uption/issues/17)
- [ ] Ping (https://github.com/uption/uption/issues/3)
- [ ] Wireless adapter (https://github.com/uption/uption/issues/18)

## Contributions

We welcome and appreciate pull requests. Please check the issues if you wish to contribute.
