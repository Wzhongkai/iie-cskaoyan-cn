ALTER TABLE article_comments
    ADD COLUMN parent_id UUID REFERENCES article_comments (id) ON DELETE CASCADE;

CREATE INDEX article_comments_parent_idx ON article_comments (parent_id, created_at ASC);
