# db-engines-benchmark

This repo is used to test various queries against different databases from a Rust Backend.

Every backend uses only pure queries, meaning that most of the work will be done from a client (if not implemented already I'll implement it).

The client which simulate the load is in Python.

Entity means both tables or documents or anything equivalent for the given database.

I suppose that the database schema (or equivalent) has already been done so for the SQL part the tables already exist with the referential integrity constraint/s.

TODOS:

- [ ] Base Actix server
- [ ] REST API
- [ ] GraphQL API
- [ ] Python client

## Tests and descriptions

### Databases

SQL

- [ ] MySQL ! - https://docs.rs/mysql/16.1.0/mysql/
- [ ] MariaDB ! - https://docs.rs/mysql/16.1.0/mysql/
- [ ] Postgres ! - https://docs.rs/postgres/0.16.0-rc.2/postgres/

NoSQL

- [ ] Mongo ! - https://docs.rs/mongodb/0.4.0/mongodb/
- [ ] Cassandra ! - https://docs.rs/cdrs/2.2.0/cdrs/
- [ ] Neo4j - https://docs.rs/rusted_cypher/1.1.0/rusted_cypher/
- [ ] ArangoDB - Client is WIP
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
