extern crate postgres;

// Connection Pool manager
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
// Threading for r2d2
use std::sync::Arc;
use std::thread;
// Fake data
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::Fake;

// Uuid generation library
use uuid::Uuid;
// Decimal library
use rust_decimal::Decimal;

#[derive(Debug)]
struct Customer {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Debug)]
struct Order {
    id: Uuid,
    customer_id: Uuid,
}

#[derive(Debug)]
struct Item {
    id: Uuid,
    name: String,
    description: String,
    price: Decimal,
}

#[derive(Debug)]
struct OrderItem {
    order_id: Uuid,
    item_id: Uuid,
    quantity: i8,
}

fn main() {
    // DB Configuration
    // let mut config: Config = Config::new();
    // config.host("localhost");
    // config.port(5432);
    // config.dbname("benchmark");
    // config.password("bench");
    // config.user("bench");

    // Create a client
    // let mut client: Client = config.connect(NoTls).expect("Could not connect");

    // Create a pool
    let manager: PostgresConnectionManager = PostgresConnectionManager::new(
        "postgresql://bench:bench@localhost/benchmark",
        TlsMode::None,
    )
    .expect("Could not create a connection manager");

    let pool = Arc::new(Pool::new(manager).expect("Could not create connection pool"));

    // Run some test queries
    // Create some customers
    let mut customers: Vec<Customer> = Vec::new();
    for _ in 0..50 {
        customers.push(Customer {
            id: Uuid::new_v4(),
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            email: SafeEmail(EN).fake(),
        });
    }

    for customer in customers {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get().expect("Connection error");
            connection.prepare(
                "INSERT INTO \"Customer\" (\"id\", \"first_name\", \"last_name\", \"email\") values ($1, $2, $3, $4)",
            ).unwrap().execute(&[                        
                &customer.id,
                &customer.first_name,
                &customer.last_name,
                &customer.email,
            ]).unwrap();
            /*
            match connection.prepare(
                "INSERT INTO \"Customer\" (\"id\", \"first_name\", \"last_name\", \"email\") values ($1, $2, $3, $4)",
            ) {
                Ok(stmt) => stmt
                    .execute(&[
                        &customer.id,
                        &customer.first_name,
                        &customer.last_name,
                        &customer.email,
                    ])
                    .expect("Insertion error"),
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };
            */
        }).join().unwrap();
    }

    /*
    let stmt = match pool.prepare("INSERT INTO Customer (id, first_name, last_name, email) values ($1, $2, $3, $4)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {}", e);
            return;
        }
    };
    for i in range(1, 5u) {
        let title = format!("Blogpost number {}", i);
        let text = format!("Content of the blogpost #{}", i);
        stmt.execute(&[&title, &text]).ok().expect("Inserting blogposts failed");
    }
    */
}
