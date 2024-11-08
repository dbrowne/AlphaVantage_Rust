ALTER TABLE overviews
    ALTER COLUMN MarketCapitalization TYPE Int,
    ALTER COLUMN EBITDA TYPE Int,
    ALTER COLUMN BookValue TYPE REAL;

-- Update the overviewexts table
ALTER TABLE overviewexts
    ALTER COLUMN RevenueTTM TYPE Int,
    ALTER COLUMN GrossProfitTTM TYPE Int,
    ALTER COLUMN Beta TYPE REAL,
    ALTER COLUMN annWeekHigh TYPE REAL,
    ALTER COLUMN annWeekLow TYPE REAL,
    ALTER COLUMN fiftyDayMovingAverage TYPE REAL,
    ALTER COLUMN twohDayMovingAverage TYPE REAL,
    ALTER COLUMN SharesOutstanding TYPE REAL;-- This file should undo anything in `up.sql`