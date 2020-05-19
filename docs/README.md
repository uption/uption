# Uption documentation

Uption is a tool to collect different metrics from network and export them to external location. Uption supports multiple different collectable metrics which can be configured for different needs. Uption also supports multiple different export methods and formats.

## Architecture

Uption has a concept of _collectors_ and _exporters_. Collectors generate metrics based on different tests and exporters export the generated data.

### Collecting data

Uption runs as a service and it has an internal scheduler that starts different collection jobs based on the configured interval. By design, only one collection job runs at a time so that different collection jobs would not intefere with each other.

### Exporting data

Data export starts as soon as new metrics become available from collectors. Internally Uption passes messages between collector scheduler and exporter scheduler. Exporter scheduler automatically retries to export a message if exporting fails, for example, because of a network error. Exponential backoff and jitter features are used to avoid network congestion.

## Collectors

Currently available collectors are listed here and planned collectors are listed in [issues](https://github.com/uption/uption/issues?q=is%3Aopen+is%3Aissue+label%3A%22Category%3A+Collector%22). Please open a new issue if you can not find a suitable collector for your needs.

### HTTP

### Ping

## Exporters

Currently available exporters are listed here and planned exporters are listed in [issues](https://github.com/uption/uption/issues?q=is%3Aopen+is%3Aissue+label%3A%22Category%3A+Exporter%22). Please open a new issue if you can not find a suitable exporter for your needs.

### InfluxDB

### Stdout
