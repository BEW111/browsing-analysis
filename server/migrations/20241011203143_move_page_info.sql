INSERT INTO page (url)
SELECT DISTINCT page_url
FROM page_info;

INSERT INTO embedding_run (id, description)
VALUES (0, 'default_embedding');

INSERT INTO preprocessed_page_embedding (page_id, embedding_run_id, embedding)
SELECT p.id, 0, pi.page_embedding
FROM page_info pi
JOIN page p ON pi.page_url = p.url;