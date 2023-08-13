-- migrate:up
CREATE TABLE IF NOT EXISTS novel_statistics (
    id SERIAL PRIMARY KEY,
    novel_id INT NOT NULL,
    first_chapter_clicks INT,
    last_chapter_clicks INT,
    reviews INT,
    collected INT,
    rewards INT,
    updated_at TEXT
);

-- migrate:down
DROP TABLE IF EXISTS novel_statistics;
