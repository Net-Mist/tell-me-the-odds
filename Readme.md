# Tell me the odds

Solution of the [developer-test](https://github.com/lioncowlionant/developer-test).

## Architecture

The code follows the onion architecture. More specifically, the code is divided into 4 sections:

- Domain model
- Domain services
- Application services
- Infrastructure services

### Domain model

Contains the definitions of `PlanetId`, `GalaxyRoutes`, `PlanetCatalog` and `BountyHunterPlanning`.

### Domain services

Contains the public definition of `compute_probability_of_success`, and some private definitions for this function.

### Application services

Contains the definition of `MillenniumFalconData` and `EmpireData` matching the json formats specified in the requirements of the app, and `Route` matching the database data format (but without db-related types or field) and some code to bridge the data.

### Infrastructure service

Contains code to connect and read from the DB, process the CLI input and defining the webserver endpoints.

## Technology stack

- This code was written in Rust 1.75.0.

### CI

- A github action perform a security audit every day. More specifically:
  - cargo-deny check for security vulnerabilities, license violation, unmaintained projects and several other things
  - cargo-audit for a second security audit. Seems to find more stuff than cargo-deny
- A second CI run the classic steps:

  - format with `cargo-fmt`
  - lint with `clippy`
  - build and test in dev mode
  - build and test again in release mode
  - build the docker image
  - push the docker image to dockerhub

## TODO

- coverage is missing
- bulding the docker image
- pushing to dockerhub with a dev tag
- release-please
- release protocol
- front
- Instrumented
- observability
- test:
  - full black-box tests for the endpoints
  - some scenario tests for the example provided in the documentation
  - unit-test inside the code
