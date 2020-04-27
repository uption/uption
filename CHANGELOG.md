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

- Replace `http_req` crate with `reqwest`.
- Add InfluxDB exporter and HTTP collector tests.
- Implement logging to a file and stdout.

### Fixed

- Libc version too new in release builds.

### Infrastructure

- Add lint and format steps in CI.
- Add scheduled dependency security audit.
- Build and test in release mode when making a release.

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
