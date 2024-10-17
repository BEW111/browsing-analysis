ALTER TABLE preprocessed_page_embedding
    DROP CONSTRAINT IF EXISTS preprocessed_page_embedding_embedding_run_id_fkey;

ALTER TABLE preprocessed_page_embedding
    DROP COLUMN IF EXISTS embedding_run_id;

DROP TABLE IF EXISTS embedding_run;

ALTER TABLE preprocessed_page_embedding
    ADD COLUMN embedding_run TEXT;