// @generated automatically by Diesel CLI.

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
