ALTER TABLE cluster_assignment
    ADD COLUMN page_id INTEGER;

UPDATE cluster_assignment
SET page_id = (
    SELECT id FROM page
    WHERE page.url = cluster_assignment.page_url
);

ALTER TABLE cluster_assignment
    DROP COLUMN page_url;

ALTER TABLE cluster_assignment
    ADD CONSTRAINT fk_page_id FOREIGN KEY (page_id) REFERENCES page(id) ON DELETE CASCADE;