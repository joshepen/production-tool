CREATE TABLE `users`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `first_name` VARCHAR(255) NOT NULL,
    `last_name` VARCHAR(255) NOT NULL,
    `department_id` BIGINT NOT NULL,
    `hired_at` DATE NOT NULL
);
CREATE TABLE `departments`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name` BIGINT NOT NULL
);
CREATE TABLE `products`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(255) NOT NULL
);
CREATE TABLE `statuses`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name` BIGINT NOT NULL
);
CREATE TABLE `product_orders`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `address` VARCHAR(255) NOT NULL,
    `product_id` BIGINT NOT NULL,
    `status_id` BIGINT NOT NULL,
    `created_at` TIMESTAMP NOT NULL,
    `completed_by` BIGINT NOT NULL
);
CREATE TABLE `issues`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `created_by` BIGINT NOT NULL,
    `created_at` BIGINT NOT NULL,
    `completed_at` TIMESTAMP NULL,
    `completed_by` BIGINT NULL,
    `department_id` BIGINT NOT NULL,
    `title` VARCHAR(255) NOT NULL,
    `description` TEXT NOT NULL
);
ALTER TABLE
    `product_orders` ADD CONSTRAINT `product_orders_product_id_foreign` FOREIGN KEY(`product_id`) REFERENCES `products`(`id`);
ALTER TABLE
    `users` ADD CONSTRAINT `users_department_id_foreign` FOREIGN KEY(`department_id`) REFERENCES `departments`(`id`);
ALTER TABLE
    `product_orders` ADD CONSTRAINT `product_orders_status_id_foreign` FOREIGN KEY(`status_id`) REFERENCES `statuses`(`id`);
ALTER TABLE
    `product_orders` ADD CONSTRAINT `product_orders_completed_by_foreign` FOREIGN KEY(`completed_by`) REFERENCES `users`(`id`);
ALTER TABLE
    `issues` ADD CONSTRAINT `issues_department_id_foreign` FOREIGN KEY(`department_id`) REFERENCES `departments`(`id`);
ALTER TABLE
    `issues` ADD CONSTRAINT `issues_created_by_foreign` FOREIGN KEY(`created_by`) REFERENCES `users`(`id`);
ALTER TABLE
    `issues` ADD CONSTRAINT `issues_completed_by_foreign` FOREIGN KEY(`completed_by`) REFERENCES `users`(`id`);