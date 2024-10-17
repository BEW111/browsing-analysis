ALTER TABLE cluster
DROP CONSTRAINT IF EXISTS cluster_clustering_run_id_fkey;

ALTER TABLE cluster
DROP COLUMN IF EXISTS clustering_run_id;

DROP TABLE IF EXISTS clustering_run;

ALTER TABLE cluster
ADD COLUMN clustering_run TEXT;
