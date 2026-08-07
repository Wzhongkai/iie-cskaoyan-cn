UPDATE articles SET category = 'policy' WHERE category = 'notice';
UPDATE submissions SET category = 'policy' WHERE category = 'notice';

ALTER TABLE articles DROP CONSTRAINT articles_category_check;
ALTER TABLE articles
    ADD CONSTRAINT articles_category_check
    CHECK (category IN ('initial', 'reexam', 'career', 'policy', 'data'));

ALTER TABLE submissions DROP CONSTRAINT submissions_category_check;
ALTER TABLE submissions
    ADD CONSTRAINT submissions_category_check
    CHECK (category IN ('initial', 'reexam', 'career', 'policy', 'data'));
