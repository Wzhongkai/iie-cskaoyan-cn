ALTER TABLE article_comments
    ADD COLUMN IF NOT EXISTS parent_id UUID;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'article_comments_parent_id_fkey') THEN
        ALTER TABLE article_comments
            ADD CONSTRAINT article_comments_parent_id_fkey
            FOREIGN KEY (parent_id) REFERENCES article_comments (id) ON DELETE CASCADE;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS article_comments_parent_idx ON article_comments (parent_id, created_at ASC);
