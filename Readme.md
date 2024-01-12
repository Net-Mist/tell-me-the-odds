# Tell me the odds

<!--toc:start-->

- [Tell me the odds](#tell-me-the-odds)
  - [Architecture](#architecture)
    - [Domain models](#domain-models)
    - [Domain services](#domain-services)
    - [Application services](#application-services)
    - [Infrastructure services](#infrastructure-services)
  - [Technology stack](#technology-stack)
  - [CI](#ci)
  - [Test](#test)
  - [TODO](#todo)
  <!--toc:end-->

Solution of the [developer-test](https://github.com/lioncowlionant/developer-test).

With a recent version of rust (tested with 1.75.0), you can build the project with `cargo build --release`. Then you can run the cli with `./target/release/give-me-the-odds examples/millennium-falcon.json examples/example2/empire.json` and the webserver with `./target/release/millennium_falcon examples/millennium-falcon.json`.

Note that when starting, the webserver will create a folder `logs` containing a file `millennium.log.{date}` with the log of the server. stdio will be pretty silent if everything goes well. The server will be listening on `0.0.0.0:8000`.

It is also possible to start the server and the cli with docker.

First

- Build the docker images locally by running `docker build -t millennium-falcon .` or `docker build -t give-me-the-odds -f cli.Dockerfile .`
- Or pull the images from dockerhub: `docker pull netmist/give-me-the-odds:0.3.0`, `docker pull netmist/millenium-falcon:0.3.0`

Then run the server with `docker run -it --rm -v ./logs:/app/logs -v ./examples:/app/examples -p 0.0.0.0:8000:8000 millennium-falcon examples/millennium-falcon.json`
and the cli with `docker run -it --rm -v ./examples:/app/examples give-me-the-odds examples/millennium-falcon.json examples/example1/empire.json`

## Architecture

The code follows the onion architecture. More specifically, the code is divided into 4 sections:

- Domain models
- Domain services
- Application services
- Infrastructure services

### Domain models

Contains the definitions of `PlanetId`, `GalaxyRoutes`, `PlanetCatalog` and `BountyHunterPlanning`.

### Domain services

Contains the public definition of `compute_probability_of_success`, and some private definitions for this function.

### Application services

Contains the definition of `MillenniumFalconData` and `EmpireData` matching the json formats specified in the requirements of the app, and `Route` matching the database data format (but without db-related types or field) and some code to bridge the data.

### Infrastructure services

Contains code to connect and read from the DB, process the CLI input and defining the webserver endpoints.

## Technology stack

This code was written in Rust 1.75.0. Notables dependencies are :

- Actix for the web server
- Anyhow for the error handling
- SQLX for database connection and compiled-time safety
- Tokio for the async engine
- Tracing for the tracing (logging with span)

## CI

4 GitHub workflows are defined.

- One that performs a security audit every day. More specifically:
  - Cargo-deny check for security vulnerabilities, license violation, unmaintained projects and several other things.
  - Cargo-audit for a second security audit. Seems to be more precise than cargo-deny, and also automatically open issue on security vulnerabilities. (see for instance [this issue](https://github.com/Net-Mist/tell-me-the-odds/issues/1))
- A second that run the classic steps at every push:
  - format with `cargo-fmt`
  - lint with `clippy`
  - build and test in dev mode
  - build and test again in release mode
  - compute and publish code coverage
- The third integrate `release-please` to automatically create GitHub releases, version bumps and changelog generation.
- The last one build and publish docker images to dockerhub when a new tag is pushed.

## Test

Unit-tests are defined directly inside the code. Look for the `mod test`. Integration tests are defined in the `tests` folder

## TODO

- release-please
- release protocol
