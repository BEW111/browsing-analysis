-- Step 6: Create cluster_assignment table without constraints initially
CREATE TABLE IF NOT EXISTS cluster_assignment (
    id SERIAL PRIMARY KEY,
    page_url TEXT NOT NULL,
    cluster_id TEXT NOT NULL
);

-- Step 7: Migrate existing assignments to the new table
INSERT INTO cluster_assignment (page_url, cluster_id)
SELECT page_url, page_cluster_id
FROM page_info
WHERE page_cluster_id IS NOT NULL;

