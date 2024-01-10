# Tell me the odds

Solution of the [developer-test](https://github.com/lioncowlionant/developer-test).

## Architecture

The code follows the onion architecture. More specifically, the code is divided into 4 sections:

- Domain modeld
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

This code was written in Rust 1.75.0.

For linter, formatter, test, code coverage and code security audit, please refer to the CI in `.github/workflow`

given the number of star in the galaxy, the code may handle a large amont of data => Rust

test:

- full black-box tests for the endpoints
- some scenario tests for the example provided in the documentation
- unit-test inside the code

or the backend of the technical test.

Instrumented
observability
database migrations
automated tests
benchmark

coverage

Questions:

Hello !
Thanks a lot. I've had a look at the test and I have a couple of question regarding the technical requirements:

- Is it possible to have an upper-bound for the number of planets, the number of bounty hunters, the number of routes and the countdown ?
- Should I consider that the database containing the ROUTES table can be load in memory, or should I consider that in the real application this database is too big, and I should only query it (as if it wasn't sqlite)?
- Is it possible to have some details on the hardware of the Millennium Falcon ? Is it running x86-64 or ARM ? Is it running Linux, MacOS or Windows ? Is there a maximum limit of CPU/RAM/Time that I should be aware of?
- How many simultaneous connections are expected to the server ?
- Are you expecting 2 different servers for serving the front and the back ? Or could the backend be responsible serving the frontend ?

- How strong are the DB assumptions ? When executing a `.schema ROUTES` on the DB file, it gave: `CREATE TABLE routes (    origin TEXT,    destination TEXT,    travel_time UNSIGNED INTEGER);`, so nothing prevent the origin and destination to be null or empty.
