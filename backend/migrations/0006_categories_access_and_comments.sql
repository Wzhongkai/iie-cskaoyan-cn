CREATE TABLE IF NOT EXISTS article_categories (
    slug TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    sort_order INTEGER NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (slug ~ '^[a-z0-9-]{2,60}$'),
    CHECK (char_length(name) BETWEEN 2 AND 30)
);

INSERT INTO article_categories (slug, name, sort_order) VALUES
    ('initial', '初试经验', 10),
    ('reexam', '复试经验', 20),
    ('career', '就业分享', 30),
    ('policy', '政策资料', 40),
    ('data', '数据纠错', 50)
ON CONFLICT (slug) DO NOTHING;

ALTER TABLE articles DROP CONSTRAINT IF EXISTS articles_category_check;
ALTER TABLE submissions DROP CONSTRAINT IF EXISTS submissions_category_check;
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'articles_category_fkey') THEN
        ALTER TABLE articles
            ADD CONSTRAINT articles_category_fkey FOREIGN KEY (category)
            REFERENCES article_categories (slug) ON UPDATE CASCADE;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'submissions_category_fkey') THEN
        ALTER TABLE submissions
            ADD CONSTRAINT submissions_category_fkey FOREIGN KEY (category)
            REFERENCES article_categories (slug) ON UPDATE CASCADE;
    END IF;
END $$;

ALTER TABLE articles ADD COLUMN IF NOT EXISTS password_hash TEXT;

CREATE TABLE IF NOT EXISTS github_users (
    github_id BIGINT PRIMARY KEY,
    login TEXT NOT NULL,
    avatar_url TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS github_oauth_states (
    state UUID PRIMARY KEY,
    return_path TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS github_sessions (
    token_hash BYTEA PRIMARY KEY,
    github_id BIGINT NOT NULL REFERENCES github_users (github_id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS article_comments (
    id UUID PRIMARY KEY,
    article_id UUID NOT NULL REFERENCES articles (id) ON DELETE CASCADE,
    github_id BIGINT NOT NULL REFERENCES github_users (github_id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (char_length(trim(body)) BETWEEN 1 AND 2000)
);

CREATE INDEX IF NOT EXISTS article_comments_article_idx ON article_comments (article_id, created_at ASC);
CREATE INDEX IF NOT EXISTS github_sessions_expiry_idx ON github_sessions (expires_at);
