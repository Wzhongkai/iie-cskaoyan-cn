ALTER TABLE submissions ADD COLUMN published_article_id UUID REFERENCES articles(id);
