CREATE TABLE blog (
    id INTEGER PRIMARY KEY,
    title TEXT,
    summary TEXT,
    body TEXT,
    date DATE,
    tags TEXT[]
);

INSERT INTO blog (id, title, summary, body, date, tags)
VALUES (1, 'Sample Title', 'Brief summary here', 'Full body text here', '2023-12-27', ARRAY['tag1', 'tag2']);