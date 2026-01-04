USE pt;

INSERT INTO departments (name) VALUES ("Information Technology");
INSERT INTO departments (name) VALUES ("Engineering");
INSERT INTO departments (name) VALUES ("Marketing");

INSERT INTO users (first_name, last_name, hired_at, department_id) VALUES ("John", "Doe", "2026-01-01", 1);
INSERT INTO users (first_name, last_name, hired_at, department_id) VALUES ("Jane", "Doe", "2026-01-02", 1);
INSERT INTO users (first_name, last_name, hired_at, department_id) VALUES ("Terry", "Davis", "2026-01-03", 2);
INSERT INTO users (first_name, last_name, hired_at, department_id) VALUES ("Jack", "Daniels", "2026-01-04", 3);

INSERT INTO products (name) VALUES ("Teenage Mutant Ninja Turtle Action Figure");
INSERT INTO products (name) VALUES ("Gun");

INSERT INTO statuses (name) VALUES ("Started");
INSERT INTO statuses (name) VALUES ("Waiting");
INSERT INTO statuses (name) VALUES ("In Progress");
INSERT INTO statuses (name) VALUES ("Completed");

INSERT INTO product_orders (address, product_id, status_id) VALUES ("The White House", 1, 1);
