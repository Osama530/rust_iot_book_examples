-- Your SQL goes here
CREATE TABLE movies (
    movie_id SERIAL PRIMARY KEY,
    movie_name VARCHAR(20) NOT NULL,
    movie_gener VARCHAR(20) NOT NULL,
    -- idb_rating Float NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f' 
)