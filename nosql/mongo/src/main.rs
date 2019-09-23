#[macro_use]
extern crate serde_derive;

// MongoDB driver
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

// Fake data
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::Fake;

// Decimal
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    // id: bson::oid::ObjectId,
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Debug)]
struct Order {
    id: bson::oid::ObjectId,
    customer_id: bson::oid::ObjectId,
}

#[derive(Debug)]
struct Item {
    id: bson::oid::ObjectId,
    name: String,
    description: String,
    price: Decimal,
}

#[derive(Debug)]
struct OrderItem {
    order_id: bson::oid::ObjectId,
    item_id: bson::oid::ObjectId,
    quantity: i8,
}

fn main() {
    // Create Client
    let uri = "mongodb://bench:bench@localhost:27017";
    let client = Client::with_uri(uri).expect("Failed to initialize client.");
    // Authenticate in the db
    client.db("admin").auth("bench", "bench").expect("Could not authenticate");
    // Run some insertions
    let coll = client.db("benchmark").collection("Customer");
    
    // Run some test queries
    // Create some customers
    let mut customers: Vec<Customer> = Vec::new();
    for _ in 0..50 {
        customers.push(Customer {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            email: SafeEmail(EN).fake(),
        });
    }

    for deserialized_customer in customers {
        // Serialize
        let customer = bson::to_bson(&deserialized_customer).expect("Could not serialize customer");  
    
        if let bson::Bson::Document(document) = customer {
            coll.insert_one(document, None).expect("Could not insert customer document");  // Insert into a MongoDB collection
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }
    }
}
