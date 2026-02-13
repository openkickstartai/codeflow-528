CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE review_status AS ENUM ('draft', 'open', 'in_review', 'approved', 'merged', 'closed');

CREATE TABLE reviews (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title       TEXT NOT NULL,
    description TEXT,
    author_id   UUID NOT NULL,
    status      review_status NOT NULL DEFAULT 'draft',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE comments (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    review_id   UUID NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    author_id   UUID NOT NULL,
    content     TEXT NOT NULL,
    line_number INTEGER,
    file_path   TEXT,
    parent_id   UUID REFERENCES comments(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_comments_review ON comments(review_id);
CREATE INDEX idx_comments_parent ON comments(parent_id) WHERE parent_id IS NOT NULL;
CREATE INDEX idx_reviews_status  ON reviews(status);
CREATE INDEX idx_reviews_author  ON reviews(author_id);
