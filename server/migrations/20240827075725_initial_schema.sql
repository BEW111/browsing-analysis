CREATE EXTENSION vector;

CREATE TABLE IF NOT EXISTS browse_event (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    tab_id INT NOT NULL,
    page_url TEXT NOT NULL,
    page_title TEXT NOT NULL,
    event_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS page_info (
    page_url TEXT PRIMARY KEY,
    page_embedding vector(384) NOT NULL,
    page_cluster_id TEXT NOT NULL
);