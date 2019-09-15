#[macro_use]
extern crate mysql;
// Mysql client
use mysql as my;
// Uuid generation library
use uuid::Uuid;
// Decimal library
use rust_decimal::Decimal;
// Fake data
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::Fake;

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
    // Setup client
    let mut options_builder: my::OptsBuilder = my::OptsBuilder::new();
    options_builder.ip_or_hostname(Some("localhost"));
    options_builder.user(Some("bench"));
    options_builder.pass(Some("bench"));
    options_builder.db_name(Some("benchmark"));
    // Connection
    let pool = my::Pool::new(options_builder).unwrap();

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

    // println!("{:?}", customers);

    // Put the customers in the db
    for mut statement in pool
        .prepare(
            r"INSERT INTO Customer 
            (id, first_name, last_name, email) VALUES 
            (:id, :first_name, :last_name, :email)",
        )
        .into_iter()
    {
        // println!("{:?}", statement);
        for customer in customers.iter() {
            // println!("{:?}", customer);

            statement
                .execute(params! {
                    "id" => customer.id.to_string(),
                    "first_name" => &customer.first_name,
                    "last_name" => &customer.last_name,
                    "email" => &customer.email,
                })
                .unwrap();
        }
    }

    /*
    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    pool.prep_exec(r"CREATE TABLE payment (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )", ()).unwrap();

    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)").into_iter() {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }).unwrap();
        }
    }

    // Let's select payments from database
    let selected_payments: Vec<Payment> =
    pool.prep_exec("SELECT customer_id, amount, account_name from payment", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
            let (customer_id, amount, account_name) = my::from_row(row);
            Payment {
                customer_id: customer_id,
                amount: amount,
                account_name: account_name,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    // Now make sure that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`
    // so assume we are lukky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");
    */
}
