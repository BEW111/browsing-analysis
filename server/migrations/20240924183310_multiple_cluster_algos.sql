-- Step 1: Create clustering_run table
CREATE TABLE IF NOT EXISTS clustering_run (
    id SERIAL PRIMARY KEY,
    algorithm TEXT NOT NULL,
    run_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    type TEXT NOT NULL CHECK (type IN ('online', 'batch'))
);

-- Step 2: Insert the legacy clustering run with a specific ID
INSERT INTO clustering_run (id, algorithm, type)
VALUES (1, 'legacy_closest_neighbor_algorithm', 'online');

-- Step 3: Add clustering_run_id to cluster table without a default
ALTER TABLE cluster
ADD COLUMN clustering_run_id INT;
