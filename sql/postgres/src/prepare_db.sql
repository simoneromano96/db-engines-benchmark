/* Create Customer Table */
CREATE TABLE "Customer" (
  "id" uuid NOT NULL,
  "first_name" character varying(32) NOT NULL,
  "last_name" character varying(32) NOT NULL,
  "email" character varying(32) NOT NULL
);

/* Create Primary Key */
ALTER TABLE "Customer"
ADD CONSTRAINT "Customer_id" PRIMARY KEY ("id");  

/* Create Order Table */
CREATE TABLE "Order" (
  "id" uuid NOT NULL,
  "customer_id" uuid NOT NULL
);

/* Create Primary Key and Foreign Key */
ALTER TABLE "Order"
ADD CONSTRAINT "Order_id" PRIMARY KEY ("id"),
ADD FOREIGN KEY ("customer_id") REFERENCES "Customer" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

/* Create Item Table */
CREATE TABLE "Item"
(
    id uuid NOT NULL,
    name character varying(32) NOT NULL,
    description character varying(128) NOT NULL,
    price money NOT NULL
)

ALTER TABLE "Item"
ADD CONSTRAINT "item_id" PRIMARY KEY ("id");

