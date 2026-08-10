CREATE TABLE article_categories (
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
    ('data', '数据纠错', 50);

ALTER TABLE articles DROP CONSTRAINT articles_category_check;
ALTER TABLE submissions DROP CONSTRAINT submissions_category_check;
ALTER TABLE articles
    ADD CONSTRAINT articles_category_fkey FOREIGN KEY (category)
    REFERENCES article_categories (slug) ON UPDATE CASCADE;
ALTER TABLE submissions
    ADD CONSTRAINT submissions_category_fkey FOREIGN KEY (category)
    REFERENCES article_categories (slug) ON UPDATE CASCADE;
ALTER TABLE articles ADD COLUMN password_hash TEXT;

CREATE TABLE github_users (
    github_id BIGINT PRIMARY KEY,
    login TEXT NOT NULL,
    avatar_url TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE github_oauth_states (
    state UUID PRIMARY KEY,
    return_path TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE github_sessions (
    token_hash BYTEA PRIMARY KEY,
    github_id BIGINT NOT NULL REFERENCES github_users (github_id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE article_comments (
    id UUID PRIMARY KEY,
    article_id UUID NOT NULL REFERENCES articles (id) ON DELETE CASCADE,
    github_id BIGINT NOT NULL REFERENCES github_users (github_id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (char_length(trim(body)) BETWEEN 1 AND 2000)
);

CREATE INDEX article_comments_article_idx ON article_comments (article_id, created_at ASC);
CREATE INDEX github_sessions_expiry_idx ON github_sessions (expires_at);
