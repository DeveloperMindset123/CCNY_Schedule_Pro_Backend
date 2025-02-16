-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  email TEXT NOT NULL,
  user_password TEXT NOT NULL,
  major VARCHAR NOT NULL,
  date_of_birth TEXT NOT NULL,
  pronouns VARCHAR NOT NULL,
  gender VARCHAR NOT NULL,
  degree_type TEXT NOT NULL,
  college_year VARCHAR NOT NULL
)