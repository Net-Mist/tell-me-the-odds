# Tell me the odds

Solution of the [developer-test](https://github.com/lioncowlionant/developer-test).

With a recent version of rust (tested with 1.75.0), you can build the project with `cargo build --release`. Then you can run the cli with `./target/release/give-me-the-odds examples/millennium-falcon.json examples/example2/empire.json` and the webserver with `./target/release/millennium_falcon examples/millennium-falcon.json`.

Note that when starting, the webserver will create a folder `logs` containing a file `millennium.log.{date}` with the log of the server. stdio will be pretty silent if everything goes well. The server will be listening on `127.0.0.1:8000`.

It is also possible to start the server with docker by running `docker build -t millennium-falcon .` then `docker run -it --rm -v ./logs:/app/logs -v ./examples:/app/examples -p 0.0.0.0:8000:8000 millennium-falcon examples/millennium-falcon.json`

and the cli by running `docker build -t give-me-the-odds -f cli.Dockerfile .` then `docker run -it --rm -v ./examples:/app/examples give-me-the-odds examples/millennium-falcon.json examples/example1/empire.json`

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

This code was written in Rust 1.75.0. Notables dependencies are :

- Actix for the web server
- Anyhow for the error handling
- SQLX for database connection and compiled-time safety
- Tokio for the async engine
- Tracing for the tracing (logging with span)

### CI

2 GitHub workflows are defined.

- One that performs a security audit every day. More specifically:
  - Cargo-deny check for security vulnerabilities, license violation, unmaintained projects and several other things.
  - Cargo-audit for a second security audit. Seems to be more precise than cargo-deny, and also automatically open issue on security vulnerabilities. (see for instance [this issue](https://github.com/Net-Mist/tell-me-the-odds/issues/1))
- A second that run the classic steps:
  - format with `cargo-fmt`
  - lint with `clippy`
  - build and test in dev mode
  - build and test again in release mode
  - compute and publish code coverage

## Test

Unit-tests are defined directly inside the code. Look for the `mod test`. Integration tests are defined in the `tests` folder

## TODO

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
