// @generated automatically by Diesel CLI.

diesel::table! {
    articles (hashid) {
        hashid -> Text,
        sourceid -> Int4,
        category -> Text,
        title -> Text,
        url -> Text,
        summary -> Text,
        banner -> Text,
        author -> Int4,
        ct -> Timestamp,
    }
}

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
        newsoverviewid -> Int4,
        articleid -> Text,
        sourceid -> Int4,
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
        hashid -> Text,
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
        revenuettm -> Int8,
        grossprofitttm -> Int8,
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
        beta -> Float8,
        annweekhigh -> Float8,
        annweeklow -> Float8,
        fiftydaymovingaverage -> Float8,
        twohdaymovingaverage -> Float8,
        sharesoutstanding -> Float8,
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
        marketcapitalization -> Int8,
        ebitda -> Int8,
        peratio -> Float4,
        pegratio -> Float4,
        bookvalue -> Float8,
        dividendpershare -> Float4,
        dividendyield -> Float4,
        eps -> Float4,
        c_time -> Timestamp,
        mod_time -> Timestamp,
    }
}

diesel::table! {
    procstates (spid) {
        spid -> Int4,
        proc_id -> Nullable<Int4>,
        start_time -> Timestamp,
        end_state -> Nullable<Int4>,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    proctypes (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    sources (id) {
        id -> Int4,
        source_name -> Text,
        domain -> Text,
    }
}

diesel::table! {
    states (id) {
        id -> Int4,
        name -> Text,
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
        feedid -> Int4,
        sid -> Int8,
        relevance -> Float8,
        tsentiment -> Float8,
        sentimentlable -> Text,
    }
}

diesel::table! {
    topicmaps (id) {
        id -> Int4,
        sid -> Int8,
        feedid -> Int4,
        topicid -> Int4,
        relscore -> Float8,
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

diesel::joinable!(articles -> authors (author));
diesel::joinable!(articles -> sources (sourceid));
diesel::joinable!(authormaps -> authors (authorid));
diesel::joinable!(authormaps -> feeds (feedid));
diesel::joinable!(feeds -> articles (articleid));
diesel::joinable!(feeds -> newsoverviews (newsoverviewid));
diesel::joinable!(feeds -> sources (sourceid));
diesel::joinable!(feeds -> symbols (sid));
diesel::joinable!(intradayprices -> symbols (sid));
diesel::joinable!(newsoverviews -> symbols (sid));
diesel::joinable!(overviewexts -> symbols (sid));
diesel::joinable!(overviews -> symbols (sid));
diesel::joinable!(procstates -> proctypes (proc_id));
diesel::joinable!(procstates -> states (end_state));
diesel::joinable!(summaryprices -> symbols (sid));
diesel::joinable!(tickersentiments -> feeds (feedid));
diesel::joinable!(tickersentiments -> symbols (sid));
diesel::joinable!(topicmaps -> feeds (feedid));
diesel::joinable!(topicmaps -> symbols (sid));
diesel::joinable!(topicmaps -> topicrefs (topicid));
diesel::joinable!(topstats -> symbols (sid));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    authormaps,
    authors,
    feeds,
    intradayprices,
    newsoverviews,
    overviewexts,
    overviews,
    procstates,
    proctypes,
    sources,
    states,
    summaryprices,
    symbols,
    tickersentiments,
    topicmaps,
    topicrefs,
    topstats,
);
