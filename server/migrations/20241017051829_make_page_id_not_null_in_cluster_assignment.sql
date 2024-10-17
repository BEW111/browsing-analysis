DELETE FROM cluster_assignment
WHERE page_id IS NULL;

ALTER TABLE cluster_assignment
ALTER COLUMN page_id SET NOT NULL;