/* Drop table if already exists */

DROP TABLE IF EXISTS users;

/* Create Table */
create table users (
    id serial primary key ,
    name varchar(200) not null,
    email varchar(200) not null
);

/* Load seed data for testing */
INSERT INTO users (id, name, email) 
VALUES (1, 'Alice Smith', 'alice.smith@example.com'), 
(2, 'Bob Johnson', 'bob.johnson@example.com'), 
(3, 'Charlie Lee', 'charlie.lee@example.com'), 
(4, 'Dana White', 'dana.white@example.com'), 
(5, 'Evan Brown', 'evan.brown@example.com');
