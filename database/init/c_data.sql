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

INSERT INTO statuses (name) VALUES ("Pending");
INSERT INTO statuses (name) VALUES ("Started");
INSERT INTO statuses (name) VALUES ("Waiting");
INSERT INTO statuses (name) VALUES ("In Progress");
INSERT INTO statuses (name) VALUES ("Completed");

INSERT INTO work_centers (name, description) VALUES ("Welding", "Welding station for metal fabrication");
INSERT INTO work_centers (name, description) VALUES ("Painting", "Paint booth for finishing");
INSERT INTO work_centers (name, description) VALUES ("Assembly", "Final assembly station");
INSERT INTO work_centers (name, description) VALUES ("Quality Control", "Inspection and testing station");
INSERT INTO work_centers (name, description) VALUES ("Machining", "CNC machining center");

INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (1, 1, 1);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (1, 2, 2);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (1, 3, 3);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (1, 4, 4);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (2, 1, 1);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (2, 2, 5);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (2, 3, 3);
INSERT INTO product_work_centers (product_id, step_order, work_center_id) VALUES (2, 4, 4);

INSERT INTO product_orders (address, product_id, status_id) VALUES ("The White House", 1, 1);