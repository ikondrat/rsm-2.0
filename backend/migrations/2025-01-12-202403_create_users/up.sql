-- Your SQL goes here
CREATE TABLE
	users (
		id uuid PRIMARY KEY DEFAULT gen_random_uuid() UNIQUE,
		email VARCHAR(200) UNIQUE NOT NULL,
		first_name VARCHAR(200) NOT NULL,
		last_name VARCHAR(200) NOT NULL,
		username VARCHAR(50) NOT NULL,
		created_at timestamp with time zone DEFAULT now(),
		updated_at timestamp with time zone DEFAULT now()
	);