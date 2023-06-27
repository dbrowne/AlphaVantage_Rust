// @generated automatically by Diesel CLI.

diesel::table! {
    symbols (sid) {
        sid -> Int8,
        symbol -> Text,
        compname -> Text,
        secname -> Text,
        exch -> Text,
        cqssym -> Text,
        etf -> Bool,
        rltsz -> Float4,
        istest -> Bool,
        nasdaqsym -> Text,
        hasprice -> Bool,
        c_time -> Timestamp,
        m_time -> Timestamp,
    }
}
