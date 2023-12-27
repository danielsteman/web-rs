CREATE TABLE blog (
    id SERIAL PRIMARY KEY,
    title TEXT,
    summary TEXT,
    body TEXT,
    date DATE,
    tags TEXT[]
);
