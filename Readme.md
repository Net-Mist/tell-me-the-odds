# Tell me the odds

<!--toc:start-->

- [Tell me the odds](#tell-me-the-odds)
  - [Running with Rust](#running-with-rust)
  - [Running with Docker](#running-with-docker)
  - [Architecture](#architecture)
    - [Domain models](#domain-models)
    - [Domain services](#domain-services)
    - [Application services](#application-services)
    - [Infrastructure services](#infrastructure-services)
  - [Technology stack](#technology-stack)
  - [CI](#ci)
  - [Test](#test)
  <!--toc:end-->

Solution of the [developer-test](https://github.com/lioncowlionant/developer-test).

## Running with Rust

With a recent version of rust (tested with 1.75.0), you can build the project with `cargo build --release`. Then you can run the cli with `./target/release/give-me-the-odds examples/millennium-falcon.json examples/example2/empire.json` and the webserver with `./target/release/millennium_falcon examples/millennium-falcon.json`.

Note that when starting, the webserver will create a folder `logs` containing a file `millennium.log.{date}` with the logs of the server. stdio will be pretty silent if everything goes well. The server will be listening on `0.0.0.0:8000`.

## Running with Docker

It is also possible to run the server and the cli with docker by running

```sh
docker run -it --rm \
  -v ./logs:/app/logs \
  -v ./examples:/app/examples \
  -p 0.0.0.0:8000:8000 \
  netmist/millennium-falcon:1.0.0 \
  examples/millennium-falcon.json
```

and

```sh
docker run -it --rm \
  -v ./examples:/app/examples \
  netmist/give-me-the-odds:1.0.0 \
  examples/millennium-falcon.json \
  examples/example2/empire.json
```

## Architecture

The code follows the onion architecture. More specifically, the code is divided into 4 sections:

- Domain models
- Domain services
- Application services
- Infrastructure services

### Domain models

Contains the definitions of `PlanetId`, `GalaxyRoutes`, `PlanetCatalog` and `BountyHunterPlanning`.

> Implementation notes:
> Graph are tricky to implement in Rust. Because of the only-one-owner rule, a node can't own its neighbors. A solution could be to wrap the node structure in a reference counter, but as there is no cycle detection in Rust reference counter, it could create memory leak.
> The solution adopted here is to create a flat data structure (`PlanetCatalog`) that contains all the planets' data (only the name for now) and create a `PlanetId` for each of them (think of it as a pointer).
> Then the other data structures work directly with `PlanetId`. As it is a small structure (only a `usize`), it can be copied or cloned for free.

### Domain services

Contains the `compute_probability_of_success` function.

### Application services

Contains the definition of `MillenniumFalconData` and `EmpireData` matching the json formats specified in the requirements of the app, and `Route` matching the database data format (but without db-related types or field) and some code to bridge the data.

### Infrastructure services

Contains code to connect and read from the DB, process the CLI arguments and defining the webserver endpoints.

> Implementation notes:
> We consider that all the data in the database fit in the memory of the rust program, and that the content of the database is immutable. This is why we are doing a single query to get all the data.
> In case this assumption doesn't hold anymore, we could create a `Planet` trait that provide a `get_route` method that gives all the routes starting from a specific planet. Then we could implement this trait both for our internal types and for a structure responsible to maintaining the DB connection and type our code with this trait instead of using explicit structure.

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
