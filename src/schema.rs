// @generated automatically by Diesel CLI.

diesel::table! {
    overviewexts (sid) {
        sid -> Int8,
        revenuepersharettm -> Float4,
        profitmargin -> Float4,
        operatingmarginttm -> Float4,
        returnonassetsttm -> Float4,
        returnonequityttm -> Float4,
        revenuettm -> Int4,
        grossprofitttm -> Int4,
        dilutedepsttm -> Float4,
        quarterlyearningsgrowthyoy -> Float4,
        quarterlyrevenuegrowthyoy -> Float4,
        analysttargetprice -> Float4,
        trailingpe -> Float4,
        forwardpe -> Float4,
        pricetosalesratiottm -> Float4,
        pricetobookratio -> Float4,
        evtorevenue -> Float4,
        evtoebitda -> Float4,
        beta -> Float4,
        annweekhigh -> Float4,
        annweeklow -> Float4,
        fiftydaymovingaverage -> Float4,
        twohdaymovingaverage -> Float4,
        sharesoutstanding -> Float4,
        dividenddate -> Date,
        exdividenddate -> Date,
        c_time -> Timestamp,
        mod_time -> Timestamp,
    }
}

diesel::table! {
    overviews (sid) {
        sid -> Int8,
        symbol -> Text,
        name -> Text,
        description -> Text,
        cik -> Text,
        exch -> Text,
        curr -> Text,
        country -> Text,
        sector -> Text,
        industry -> Text,
        address -> Text,
        fiscalyearend -> Text,
        latestquarter -> Date,
        marketcapitalization -> Int4,
        ebitda -> Int4,
        peratio -> Float4,
        pegratio -> Float4,
        bookvalue -> Float4,
        dividendpershare -> Float4,
        dividendyield -> Float4,
        eps -> Float4,
        c_time -> Timestamp,
        mod_time -> Timestamp,
    }
}

diesel::table! {
    symbols (sid) {
        sid -> Int8,
        symbol -> Text,
        name -> Text,
        sec_type -> Text,
        region -> Text,
        marketopen -> Time,
        marketclose -> Time,
        timezone -> Text,
        currency -> Text,
        overview -> Bool,
        intraday -> Bool,
        summary -> Bool,
        c_time -> Timestamp,
        m_time -> Timestamp,
    }
}

diesel::joinable!(overviewexts -> symbols (sid));
diesel::joinable!(overviews -> symbols (sid));

diesel::allow_tables_to_appear_in_same_query!(overviewexts, overviews, symbols,);
