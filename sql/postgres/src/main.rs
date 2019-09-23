extern crate postgres;

// Connection Pool manager
use actix_web::Responder;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
// Actix
use actix_web::{delete, get, post, put};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
// Log
use env_logger;
use std::io;
// Serde serialisation
use serde::{Serialize, Deserialize};

// Uuid generation library
use uuid::Uuid;
// Decimal library
use rust_decimal::Decimal;

// Models
#[derive(Serialize, Deserialize, Debug)]
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

// Routes
#[get("/ping")]
fn index() -> impl Responder {
    HttpResponse::Ok().json("pong")
}

#[get("/customers")]
fn get_customers(
    db: web::Data<Pool<PostgresConnectionManager>>, // From store
) -> impl Responder {
    let conn = db.get().unwrap();
    let customers: Vec<Customer> = Vec::new();
    for row in &conn.query("SELECT \"id\", \"first_name\", \"last_name\", \"email\" FROM \"Customer\"", &[]).unwrap() {
        let customer = Customer {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            email: row.get(3),
        };
        println!("{:?}", customer);
    }
    HttpResponse::Ok().json(customers)
}



fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("postgres-bench");

    // r2d2 pool
    let manager: PostgresConnectionManager = PostgresConnectionManager::new(
        "postgresql://bench:bench@localhost/benchmark",
        TlsMode::None,
    )
    .expect("Could not create a connection manager");

    let pool = r2d2::Pool::new(manager).expect("Could not create connection pool");

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_customers)
    })
    .bind("127.0.0.1:8083")?
    .start();

    sys.run()
}

/*
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
*/
