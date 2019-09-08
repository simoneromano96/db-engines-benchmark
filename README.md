# db-engines-benchmark

This repo is used to test various queries against different databases from a Rust Backend.

Every backend uses both simple, pure queries and an ORM or ODM to have a layer of abstraction and better control of types from the database.

The client which simulate the load is in Python.

Entity means both tables or documents or anything equivalent for the given database.

I suppose that the database schema (or equivalent) has already been done so for the SQL part the tables already exist with the referential integrity constraint/s.

TODOS:

* [ ] Base Actix server
* [ ] REST API
* [ ] GraphQL API
* [ ] Python client

## Tests and descriptions

### Databases

SQL
* [ ] MySQL
* [ ] MariaDB
* [ ] Postgres
* [ ] Yugabyte
* [ ] TiDB
* [ ] Firebird

NoSQL
* [ ] Mongo
* [ ] Cassandra
* [ ] Neo4j
* [ ] Couchbase
* [ ] ArangoDB
* [ ] HBase
* [ ] CouchDB
* [ ] JanusGraph

RAM/Cache
* [ ] Redis
* [ ] Memcached

### Queries

#### Single entity operations

Select an entity x times:

* 1
* 10
* 100
* 1'000
* 10'000
* 100'000
* 1'000'000
* 10'000'000
* 100'000'000

Insert an entity x times:

* 1
* 10
* 100
* 1'000
* 10'000
* 100'000
* 1'000'000
* 10'000'000
* 100'000'000

Delete an entity x times:

* 1
* 10
* 100
* 1'000
* 10'000
* 100'000
* 1'000'000
* 10'000'000
* 100'000'000


## Requirements

* Sail - Used for developement
* Docker - Used to run and compile everything
