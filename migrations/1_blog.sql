CREATE TABLE blog (
    id INT4 PRIMARY KEY,
    title TEXT,
    summary TEXT,
    body TEXT,
    date DATE,
    tags TEXT[]
);
