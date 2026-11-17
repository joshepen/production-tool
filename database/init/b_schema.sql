USE pt;

CREATE TABLE IF NOT EXISTS departments (
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(100) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_types (
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
	id INT PRIMARY KEY AUTO_INCREMENT,
	first_name VARCHAR(50) NOT NULL,
	last_name VARCHAR(50) NOT NULL,
	hired_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	department_id INT NOT NULL,
	user_type_id INT NOT NULL DEFAULT 1,
	FOREIGN KEY (department_id) REFERENCES departments(id),
	FOREIGN KEY (user_type_id) REFERENCES user_types(id)
);

CREATE TABLE IF NOT EXISTS products (
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(100) NOT NULL
);

CREATE TABLE IF NOT EXISTS statuses (
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS work_centers (
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(100) NOT NULL,
	description TEXT
);

CREATE TABLE IF NOT EXISTS product_work_centers (
	product_id INT,
	step_order INT,
	work_center_id INT NOT NULL,
	PRIMARY KEY (product_id, step_order),
	FOREIGN KEY (product_id) REFERENCES products(id),
	FOREIGN KEY (work_center_id) REFERENCES work_centers(id)
);

CREATE TABLE IF NOT EXISTS product_orders (
	id INT PRIMARY KEY AUTO_INCREMENT,
	address VARCHAR(100) NOT NULL,
	product_id INT NOT NULL,
	FOREIGN KEY (product_id) REFERENCES products(id),
	status_id INT DEFAULT 1 NOT NULL,
	FOREIGN KEY (status_id) REFERENCES statuses(id),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS work_orders (
	id INT PRIMARY KEY AUTO_INCREMENT,
	product_order_id INT NOT NULL,
	step_order INT NOT NULL,
	status_id INT DEFAULT 1 NOT NULL,
	started_at TIMESTAMP NULL,
	completed_at TIMESTAMP NULL,
	estimated_hours DOUBLE DEFAULT 0,
	FOREIGN KEY (product_order_id) REFERENCES product_orders(id) ON DELETE CASCADE,
	FOREIGN KEY (status_id) REFERENCES statuses(id)
);
