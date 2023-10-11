// @generated automatically by Diesel CLI.

diesel::table! {
    authormaps (id) {
        id -> Int4,
        feedid -> Int4,
        authorid -> Int4,
    }
}

diesel::table! {
    authors (id) {
        id -> Int4,
        author_name -> Text,
    }
}

diesel::table! {
    feeds (id) {
        id -> Int4,
        sid -> Int8,
        overviewid -> Nullable<Int4>,
        title -> Text,
        url -> Text,
        publishedt -> Timestamp,
        summary -> Text,
        banner -> Text,
        source -> Text,
        sourcecategory -> Text,
        sourcedomain -> Text,
        osentiment -> Float8,
        sentlabel -> Text,
    }
}

diesel::table! {
    intradayprices (eventid) {
        eventid -> Int4,
        tstamp -> Timestamp,
        sid -> Int8,
        symbol -> Text,
        open -> Float4,
        high -> Float4,
        low -> Float4,
        close -> Float4,
        volume -> Int4,
    }
}

diesel::table! {
    newsoverviews (id) {
        id -> Int4,
        sid -> Int8,
        items -> Int4,
        sentiment -> Text,
        relevance -> Text,
        creation -> Timestamp,
    }
}

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
    summaryprices (eventid) {
        eventid -> Int4,
        date -> Date,
        sid -> Int8,
        symbol -> Text,
        open -> Float4,
        high -> Float4,
        low -> Float4,
        close -> Float4,
        volume -> Int4,
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

diesel::table! {
    tickersentiments (id) {
        id -> Int4,
        feedid -> Nullable<Int4>,
        ticker -> Text,
        sid -> Int8,
        relevance -> Float8,
        tsentiment -> Float8,
        sentimentlable -> Text,
    }
}

diesel::table! {
    topicrefs (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    topstats (eventid) {
        eventid -> Int4,
        date -> Timestamp,
        event_type -> Text,
        sid -> Int8,
        symbol -> Text,
        price -> Float4,
        change_val -> Float4,
        change_pct -> Float4,
        volume -> Int4,
    }
}

diesel::joinable!(authormaps -> authors (authorid));
diesel::joinable!(authormaps -> feeds (feedid));
diesel::joinable!(feeds -> newsoverviews (overviewid));
diesel::joinable!(overviewexts -> symbols (sid));
diesel::joinable!(overviews -> symbols (sid));
diesel::joinable!(tickersentiments -> feeds (feedid));

diesel::allow_tables_to_appear_in_same_query!(
    authormaps,
    authors,
    feeds,
    intradayprices,
    newsoverviews,
    overviewexts,
    overviews,
    summaryprices,
    symbols,
    tickersentiments,
    topicrefs,
    topstats,
);
