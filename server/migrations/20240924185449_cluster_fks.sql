DELETE FROM cluster_assignment
WHERE cluster_id NOT IN (
    SELECT id FROM cluster
);

ALTER TABLE cluster_assignment
ADD CONSTRAINT fk_cluster_assignment_page_url FOREIGN KEY (page_url) REFERENCES page_info(page_url),
ADD CONSTRAINT fk_cluster_assignment_cluster_id FOREIGN KEY (cluster_id) REFERENCES cluster(id);