-- Your SQL goes here
-- initial topics based on https://www.alphavantage.co/query?function=NEWS_SENTIMENT&tickers=AAPL&apikey=demo

CREATE TABLE topicrefs (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Populate the topic_reference table with the predefined topics will add more as needed
INSERT INTO topicrefs (name) VALUES
('Blockchain'),
('Earnings'),
('Economy - Fiscal'),
('Economy - Macro'),
('Economy - Monetary'),
('Energy & Transportation'),
('Finance'),
('Financial Markets'),
('IPO'),
('Life Sciences'),
('Manufacturing'),
('Real Estate & Construction'),
('Retail & Wholesale'),
('Technology');