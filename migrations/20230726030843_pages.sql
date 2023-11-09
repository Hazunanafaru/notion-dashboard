-- Add migration script here
CREATE TABLE IF NOT EXISTS daily_journal_pages
(
    id BIGSERIAL PRIMARY KEY,
    date DATE NOT NULL,
    page_id VARCHAR (255),
    name VARCHAR (255),
    verdict VARCHAR (255),
    kCal INT,
    tags VARCHAR (255)
);
