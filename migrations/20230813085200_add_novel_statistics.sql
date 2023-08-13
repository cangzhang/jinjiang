CREATE TABLE IF NOT EXISTS novel_statistics (
    id INTEGER PRIMARY KEY NOT NULL,
    novel_id INTEGER,
    first_chapter_clicks INTEGER,
    last_chapter_clicks INTEGER,
    reviews INTEGER,
    collected INTEGER,
    rewards INTEGER,
    updated_at TEXT
);