CREATE TABLE IF NOT EXISTS novel_details (
    id INTEGER PRIMARY KEY NOT NULL,
    novel_id INTEGER,
    title TEXT,
    author INTEGER,
    reviews INTEGER,
    collected INTEGER,
    first_chapter_clicks INTEGER,
    last_chapter_clicks INTEGER,
    udpated_at TEXT
);
