# db-engines-benchmark

This repo is used to test various queries against different databases from a Rust Backend, only rust-native db clients will be taken into account.

Every backend uses only pure queries, meaning that most of the work will be done from a client (if not implemented already I'll implement it).

The client which simulate the load is in Python.

Entity means both tables or documents or anything equivalent for the given database.

I suppose that the database schema (or equivalent) has already been done so for the SQL part the tables already exist with the referential integrity constraint/s.

TODOS:

- [x] DB Preparation (if present)
- [x] Random DB clients tests
- [x] Base Actix server (see postgres implementation)
- [ ] GraphQL API
- [ ] Python client (WIP)

## Tests and descriptions

### Databases

SQL

- [ ] MySQL ! - https://docs.rs/mysql/16.1.0/mysql/
- [ ] MariaDB - https://docs.rs/mysql/16.1.0/mysql/
- [x] Postgres (WIP) - https://docs.rs/postgres/0.16.0-rc.2/postgres/

NoSQL

- [ ] Mongo ! - https://docs.rs/mongodb/0.4.0/mongodb/
- [ ] Neo4j - https://docs.rs/rusted_cypher/1.1.0/rusted_cypher/
- [ ] ArangoDB - The db client is WIP, repo: https://github.com/simoneromano96/chameleon-db-client
- [ ] CouchDB - https://couchdb-rs.github.io/couchdb/doc/v0.6.0/couchdb/

RAM/Cache

- [ ] Redis - https://docs.rs/redis/0.12.0/redis/

**Note: `!` means top priority**

### Queries

#### Single entity operations

Select an entity x times:

- 1
- 10
- 100
- 1'000
- 10'000
- 100'000
- 1'000'000
- 10'000'000
- 100'000'000

Insert an entity x times:

- 1
- 10
- 100
- 1'000
- 10'000
- 100'000
- 1'000'000
- 10'000'000
- 100'000'000

Delete an entity x times:

- 1
- 10
- 100
- 1'000
- 10'000
- 100'000
- 1'000'000
- 10'000'000
- 100'000'000

## Requirements

- Sail - Used for developement
- Docker - Used to run and compile everything
