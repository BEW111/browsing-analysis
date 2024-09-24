ALTER TABLE cluster
ALTER COLUMN clustering_run_id SET NOT NULL,
ADD CONSTRAINT fk_cluster_clustering_run FOREIGN KEY (clustering_run_id) REFERENCES clustering_run(id);