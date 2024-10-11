CREATE TABLE page (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL UNIQUE,
    contents TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE embedding_run (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TABLE preprocessed_page_embedding (
    id SERIAL PRIMARY KEY,
    page_id INTEGER REFERENCES page(id),
    embedding_type_id INTEGER REFERENCES embedding_run(id),
    embedding vector(384),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(page_id, embedding_type_id)
);