/* Create Customer table */
CREATE TABLE `Customer` (
  `id` varchar(36) NOT NULL,
  `first_name` varchar(32) NOT NULL,
  `last_name` varchar(32) NOT NULL,
  `email` varchar(32) NOT NULL
) ENGINE='InnoDB' COLLATE 'utf8mb4_unicode_ci';

/* Add the primary key */
ALTER TABLE `Customer`
ADD PRIMARY KEY `id` (`id`);

/* Create Order Table */
CREATE TABLE `Order` (
  `id` varchar(36) NOT NULL,
  `customer_id` varchar(36) NOT NULL
) ENGINE='InnoDB' COLLATE 'utf8mb4_unicode_ci';

/* Add the primary key and the foreign key */
ALTER TABLE `Order`
ADD PRIMARY KEY `id` (`id`),
ADD FOREIGN KEY (`customer_id`) REFERENCES `Customer` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

/* Create Item Table */
CREATE TABLE `Item` (
  `id` varchar(36) NOT NULL,
  `name` varchar(32) NOT NULL,
  `description` varchar(128) NOT NULL,
  `price` decimal(8,2) NOT NULL
) ENGINE='InnoDB' COLLATE 'utf8mb4_unicode_ci';

/* Add the primary key */
ALTER TABLE `Item`  
ADD PRIMARY KEY `id` (`id`);

/* Create Order_Item Table */
CREATE TABLE `Order_Item` (
  `order_id` varchar(36) NOT NULL,
  `item_id` varchar(36) NOT NULL,
  `quantity` int(4) unsigned NOT NULL
) ENGINE='InnoDB' COLLATE 'utf8mb4_unicode_ci';

/* Add the primary key and the foreign keys */
ALTER TABLE `Order_Item`
ADD PRIMARY KEY `order_id_item_id` (`order_id`, `item_id`),
ADD FOREIGN KEY (`order_id`) REFERENCES `Order` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
ADD FOREIGN KEY (`item_id`) REFERENCES `Item` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;