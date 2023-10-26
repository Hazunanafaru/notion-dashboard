-- Add migration script here
CREATE TABLE IF NOT EXISTS pages
(
    id BIGSERIAL PRIMARY KEY,
    date DATE NOT NULL,
    page_id VARCHAR (255)
);

CREATE TABLE IF NOT EXISTS monthly_datas
(
    id BIGSERIAL PRIMARY KEY,
    year INTEGER NOT NULL,
    month INTEGER NOT NULL,
    total_dj_pages INTEGER
);
