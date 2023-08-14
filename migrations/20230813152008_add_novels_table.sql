-- migrate:up
CREATE TABLE novels (
    id SERIAL PRIMARY KEY, 
    novel_id INT NOT NULL UNIQUE,
    author_id INT NOT NULL, 
    title VARCHAR(255) NOT NULL,
    created_at VARCHAR(128),
    updated_at VARCHAR(128)
);

-- migrate:down
DROP TABLE IF EXISTS novels;
