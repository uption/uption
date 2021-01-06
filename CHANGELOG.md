# Uption changelog

This project follows semantic versioning.

Types of changes:

- **Added**: New features.
- **Changed**: Changes in existing functionality.
- **Deprecated**: Soon-to-be removed features.
- **Removed**: Removed features.
- **Fixed**: Bug fixes.
- **Security**: Vulnerabilities.
- **Infrastructure**: Changes in build or deployment infrastructure.
- **Documentation**: Changes in documentation.

## [Unreleased]

### Added

-

### Changed

-

### Fixed

-

### Infrastructure

-

## v0.6.3 - 2021-01-06

### Changed

- Update reqwest to version 0.11.
- Do not retry InfluxDB export on bad request.

### Fixed

- Fix InfluxDB string metrics missing quotes.
- Fix wireless collector message without metrics. #90
- Fix wireless collector sends message with empty tags. #91

## v0.6.2 - 2021-01-05

### Fixed

- Fix DNS collector default timeout.

## v0.6.1 - 2021-01-05

### Fixed

- Fix wrong path to config file in `etc` folder.

## v0.6.0 - 2021-01-05

### Added

- Add default values to all configuration options.
- Implement wireless interface collector.

### Changed

- Improve configuration validation.

### Infrastructure

- Remove deprecated `set-env` from Github Actions workflows.

## v0.5.0 - 2020-07-15

### Added

- Implement InfluxDB v1 exporter.
- Implement logger exporter.
- Implement DNS collector.

### Changed

- Rename `logger` config section to `logging`.
- Update dependencies.
- Update logging format.

### Infrastructure

- Require up to date lock file in CI.

### Documentation

- Add links to documentation.

## v0.4.1 - 2020-05-04

### Changed

- Update dependencies.

### Fixed

- Required libgcc1 version too new in amd64 Debian package.

## v0.4.0 - 2020-05-04

### Added

- Replace `http_req` crate with `reqwest`.
- Add InfluxDB exporter and HTTP collector tests.
- Implement logging to a file and stdout.
- Add tests for ping collector.

### Fixed

- Libc version too new in release builds.

### Infrastructure

- Add lint and format steps in CI.
- Add scheduled dependency security audit.
- Build and test in release mode when making a release.
- Build Debian package for armv7.

### Documentation

- Improve code documentation.

## v0.3.1 - 2020-04-14

### Changed

- Update dependencies.

### Fixed

- Fix Debian package replaces configuration files.

## v0.3.0 - 2020-04-14

### Added

- Order inserted tags and metrics when exporting.
- Support tags in exported messages.
- Add hostname tag to exported messages.

### Changed

- Export message to InfluxDB as a single line.

### Infrastructure

- Add CI workflow to build Debian packages automatically.

### Documentation

- Add installation instructions to README.md.

## v0.2.1 - 2020-04-06

### Fixed

- Ping parsing fails if latency more than 99ms.

## v0.2.0 - 2020-04-05

### Added

- Error handling improvements.
- Exporter scheduler retry logic.

## v0.1.0 - 2020-03-30

### Added

- HTTP collector.
- Ping collector.
- Stdout exporter.
- InfluxDB exporter.
