// from http://www.nasdaqtrader.com/trader.aspx?id=symboldirdefs

// NASDAQ Symbol Directory
/*
Symbol	The one to four or five character identifier for each NASDAQ-listed security.
Security Name	Company issuing the security.
Market Category	The category assigned to the issue by NASDAQ based on Listing Requirements. Values:
Q = NASDAQ Global Select MarketSM
G = NASDAQ Global MarketSM
S = NASDAQ Capital Market

Test Issue	Indicates whether or not the security is a test security. Values: Y = yes, it is a test issue. N = no, it is not a test issue.
Financial Status	Indicates when an issuer has failed to submit its regulatory filings on a timely basis, has failed to meet NASDAQ's continuing listing standards, and/or has filed for bankruptcy. Values include:
D = Deficient: Issuer Failed to Meet NASDAQ Continued Listing Requirements
E = Delinquent: Issuer Missed Regulatory Filing Deadline
Q = Bankrupt: Issuer Has Filed for Bankruptcy
N = Normal (Default): Issuer Is NOT Deficient, Delinquent, or Bankrupt.
G = Deficient and Bankrupt
H = Deficient and Delinquent
J = Delinquent and Bankrupt
K = Deficient, Delinquent, and Bankrupt
Round Lot	Indicates the number of shares that make up a round lot for the given security.

ACT Symbol

Field Name Change effective 2/12/2010

Identifier for each security used in ACT and CTCI connectivity protocol. Typical identifiers have 1-5 character root symbol and then 1-3 characters for suffixes. Allow up to 14 characters.

More information regarding the symbology convention can be found on this website.

Security Name	The name of the security including additional information, if applicable. Examples are security type (common stock, preferred stock, etc.) or class (class A or B, etc.). Allow up to 255 characters.
Exchange
The listing stock exchange or market of a security.

Allowed values are:
A = NYSE MKT
N = New York Stock Exchange (NYSE)
P = NYSE ARCA
Z = BATS Global Markets (BATS)
V = Investors' Exchange, LLC (IEXG)
CQS Symbol
Identifier of the security used to disseminate data via the SIAC Consolidated Quotation System (CQS) and Consolidated Tape System (CTS) data feeds. Typical identifiers have 1-5 character root symbol and then 1-3 characters for suffixes. Allow up to 14 characters.

More information regarding the symbology convention can be found on this website.

ETF	Identifies whether the security is an exchange traded fund (ETF). Possible values:
Y = Yes, security is an ETF
N = No, security is not an ETF
For new ETFs added to the file, the ETF field for the record will be updated to a value of "Y".
Round Lot Size	Indicates the number of shares that make up a round lot for the given security. Allow up to 6 digits.
Test Issue	Indicates whether the security is a test security.
Y = Yes, it is a test issue.
N = No, it is not a test issue.

*/

/*
See https://www.tickdata.com/product/nbbo/
Exchange codes
Exchange on which the trade occurred.
A AMEX (NYSE MKT)
B NASDAQ OMX BX (Boston)
C National Stock Exchange (Cincinnati)
D/1 NASD ADF (FINRA)
E Market Independent (SIP - Generated)
I ISE (International Securities Exchange)
J DirectEdge A
K DirectEdge X
L Long-Term Stock Exchange (Starting 8/1/2020)
M Chicago
N NYSE
O Instinet (Valid only during January and February 1993)
P ARCA (formerly Pacific)
S Consolidated Tape System
T/Q NASDAQ
V IEX
W CBOE (Valid through 04/30/14)
X NASDAQ OMX PSX (Philadelphia)
Y BATS Y-Exchange, Inc.
Z BATS
*/



pub mod sec_types {
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use serde::Deserialize;


    const EQTY: &str = "equity";
    const OPTN: &str = "option";
    const FUTURE: &str = "future";
    const WAR: &str = "wrnt";
    const MUTF: &str = "mutual fund";
    const COMMON_STOCK: &str = "common stock";
    const ADR_FULL: &str = "american depositary shares";
    const ADR: &str = "adr";
    const BOND: &str = "bond";
    const SUB_DEBENS: &str = "subordinated debentures";
    const PREFERRED: &str = "preferred";
    const PFD: &str = "pfd";
    const ETF: &str = "etf";
    const ETN: &str = "etn";
    const ETN1: &str = "exchange traded note";
    const LPCOMM: &str = "lp common units representing limited partner interests";
    const WARRANT: &str = "warrant";
    const WARRANT1: &str = "wt";
    const ORDSHRS: &str = "ordinary shares";
    const DEPSHRS: &str = "depositary sh";
    const DEPSHRS1: &str = "dep shs";
    const COMMSHR: &str = "common chares";
    const SRNOTES: &str = "senior notes";
    const COMMU: &str = "common units representing limited partner interests";
    const FLOATR: &str = "floating rate";
    const NOTES: &str = "notes";
    const TRUST: &str = "trust";
    const FUND: &str = "fund";
    const CRYPT: &str = "crypto";
    const FX: &str = "forei";
    const FX2: &str = "fx";
    const SWAP: &str = "swap";

    pub const OTHER: i8 = 127;
    const COMMON_V: i8 = 0;
    const ADR_V: i8 = 1;
    const PFD_V: i8 = 2;
    const ETF_V: i8 = 3;
    const SUBDEB_V: i8 = 4;
    const ETN_V: i8 = 5;
    const LPCOMM_V: i8 = 6;
    const WARRANT_V: i8 = 7;
    const ORDSHRS_V: i8 = 8;
    const DEPSHRS_V: i8 = 9;
    const COMMSHR_V: i8 = 10;
    const SRNOTES_V: i8 = 11;
    const COMMU_V: i8 = 12;
    const FLOATR_V: i8 = 13;
    const NOTES_V: i8 = 14;
    const TRUST_V: i8 = 15;
    const FUND_V: i8 = 16;
    const BOND_V: i8 = 17;
    const OPTN_V: i8 = 18;
    const FUTURE_V: i8 = 19;
    const MUTUAL_FUND_V: i8 = 20;
    const  CRYPT_V: i8 = 21;
    const FX_V: i8 = 22;
    const SWAP_V: i8 = 23;

    lazy_static! {
    pub static ref SEC_TYPES: HashMap<&'static str, i8> = [
        (EQTY,COMMON_V),
        (COMMON_STOCK,COMMON_V),
        (ADR_FULL,ADR_V),
        (ADR,ADR_V),
        (BOND,BOND_V),
        (PREFERRED,PFD_V),
        (PFD,PFD_V),
        (ETF,ETF_V),
        (SUB_DEBENS,SUBDEB_V),
        (ETN,ETN_V),
        (ETN1,ETN_V),
        (LPCOMM,LPCOMM_V),
        (WAR, WARRANT_V),
        (WARRANT, WARRANT_V),
        (WARRANT1,WARRANT_V),
        (ORDSHRS,COMMON_V),
        (DEPSHRS,DEPSHRS_V),
        (DEPSHRS1,DEPSHRS_V),
        (COMMSHR,COMMSHR_V),
        (SRNOTES,SRNOTES_V),
        (COMMU,COMMU_V),
        (FLOATR,FLOATR_V),
        (NOTES,NOTES_V),
        (TRUST,TRUST_V),
        (FUND,FUND_V),
        (OPTN,OPTN_V),
        (FUTURE,FUTURE_V),
        (MUTF,MUTUAL_FUND_V),
        (CRYPT,CRYPT_V),
        (FX,FX_V),
        (FX2,FX_V),
        (SWAP,SWAP_V),


        ].iter().copied().collect();
}


    #[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, Deserialize)]
    pub enum SymboltFlag {
        Overview,
        Intraday,
        Summary,
        All,
    }


    #[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, Deserialize)]
    pub enum SecurityType {
        Equity,
        Bond,
        Option,
        Future,
        ETF,
        MutualF,
        Crypto,
        FX,
        Swap,
        Wrnt,
        Adr,
        Pfd,
        Other,
    }

    lazy_static! {
        pub static ref SEC_TYPE_MAP: HashMap<i8, SecurityType> = [
            (COMMON_V, SecurityType::Equity),
            (ADR_V, SecurityType::Adr),
            (BOND_V, SecurityType::Bond),
            (ETF_V, SecurityType::ETF),
            (SUBDEB_V, SecurityType::Bond),
            (ETN_V, SecurityType::ETF),
            (LPCOMM_V, SecurityType::MutualF),
            (WARRANT_V, SecurityType::Wrnt),
            (ORDSHRS_V, SecurityType::Equity),
            (DEPSHRS_V, SecurityType::Adr),
            (COMMSHR_V, SecurityType::Equity),
            (SRNOTES_V, SecurityType::Bond),
            (COMMU_V, SecurityType::MutualF),
            (FLOATR_V, SecurityType::Bond),
            (NOTES_V, SecurityType::Bond),
            (TRUST_V, SecurityType::MutualF),
            (FUND_V, SecurityType::MutualF),
            (OPTN_V, SecurityType::Option),
            (FUTURE_V, SecurityType::Future),
            (MUTUAL_FUND_V, SecurityType::MutualF),
            (CRYPT_V, SecurityType::Crypto),
            (FX_V, SecurityType::FX),
            (SWAP_V, SecurityType::Swap),
        ].iter().cloned().collect();
    }


    const MASK32: i64 = 0x1FFFF_FFFF;
    const SHIFT: u8 = 47;
    const EQUITY_M: i64 = 0b0000_0000;
    const PFD_M: i64 = 0b0000_0010;
    const ADR_M: i64 = 0b0000_0100;
    const WRNT_M: i64 = 0b0000_0110;
    const BOND_M: i64 = 0b0001_0000;
    const OPT_M: i64 = 0b0010_0000;
    const FUT_M: i64 = 0b0011_0000;
    const ETF_M: i64 = 0b0100_0000;
    const MUTF_M: i64 = 0b0101_0000;
    const CRYPT_M: i64 = 0b0110_0000;
    const FX_M: i64 = 0b0111_0000;
    const SWAP_M: i64 = 0b1000_0000;

    const OTHER_M: i64 = 0b1111_0000;

    pub struct SecurityIdentifier {
        security_type: SecurityType,
        raw_id: u32,
    }

    pub type SecTypeCounts = HashMap<SecurityType, u32>;

    impl SecurityType {
        pub fn encode(st: SecurityType, id: u32) -> i64 {
            match st {
                SecurityType::Equity => EQUITY_M << SHIFT | id as i64,
                SecurityType::Bond => BOND_M << SHIFT | id as i64,
                SecurityType::Option => OPT_M << SHIFT | id as i64,
                SecurityType::Future => FUT_M << SHIFT | id as i64,
                SecurityType::ETF => ETF_M << SHIFT | id as i64,
                SecurityType::MutualF => MUTF_M << SHIFT | id as i64,
                SecurityType::Crypto => CRYPT_M << SHIFT | id as i64,
                SecurityType::FX => FX_M << SHIFT | id as i64,
                SecurityType::Swap => SWAP_M << SHIFT | id as i64,
                SecurityType::Wrnt => WRNT_M << SHIFT | id as i64,
                SecurityType::Adr => ADR_M << SHIFT | id as i64,
                SecurityType::Pfd => PFD_M << SHIFT | id as i64,
                SecurityType::Other => OTHER_M << SHIFT | id as i64,
            }
        }


        pub fn get_sec_type(sid: i64) -> SecurityType {
            let sectype = sid >> SHIFT;
            match sectype {
                EQUITY_M => SecurityType::Equity,
                BOND_M => SecurityType::Bond,
                OPT_M => SecurityType::Option,
                FUT_M => SecurityType::Future,
                ETF_M => SecurityType::ETF,
                MUTF_M => SecurityType::MutualF,
                CRYPT_M => SecurityType::Crypto,
                FX_M => SecurityType::FX,
                SWAP_M => SecurityType::Swap,
                WRNT_M => SecurityType::Wrnt,
                ADR_M => SecurityType::Adr,
                PFD_M => SecurityType::Pfd,
                OTHER_M => SecurityType::Other,
                _ => SecurityType::Other,
            }
        }




        pub fn get_detailed_sec_type(s_typ: &str, s_name: &str) -> (SecurityType, String) {
            let s_name_lower = s_name.to_lowercase();
            let s_typ_lower = s_typ.to_lowercase();

            let mut security_type = SecurityType::Other;
            let mut sec_type_str = "Other".to_string();

            match s_typ_lower.as_str() {
                EQTY => {
                    security_type = SecurityType::Equity;
                    sec_type_str = "Eqty".to_string();
                }
                ETF => {
                    security_type = SecurityType::ETF;
                    sec_type_str = "ETF".to_string();
                }
                MUTF => {
                    security_type = SecurityType::MutualF;
                    sec_type_str = "MutF".to_string();
                }
                _ => {}
            }

            if s_name_lower.contains(ADR) {
                security_type = SecurityType::Adr;
                sec_type_str = "Adr".to_string();
            } else if s_name_lower.contains(WARRANT) || s_name_lower.contains(WAR) {
                security_type = SecurityType::Wrnt;
                sec_type_str = "Wrnt".to_string();
            } else if s_name_lower.contains(PFD) {
                security_type = SecurityType::Pfd;
                sec_type_str = "Pfd".to_string();
            }

            (security_type, sec_type_str)
        }


        pub fn get_sec_type_from_string(s_typ: &str) -> SecurityType {
            if s_typ == EQTY {
                return SecurityType::Equity;
            } else if s_typ == ETF {
                return SecurityType::ETF;
            } else if s_typ == MUTF {
                return SecurityType::MutualF;
            }

            SecurityType::Other
        }

        pub fn get_sec_type_fs(security: &str) -> SecurityType {
            let lower_security = security.to_lowercase();
            for (key, value) in SEC_TYPES.iter() {
                if lower_security.contains(key) {
                    if let Some(sec_type) = SEC_TYPE_MAP.get(value) {
                        return *sec_type;
                    }
                }
            }
            SecurityType::Other
        }
    }

    impl SecurityIdentifier {
        pub fn decode(encoded_id: i64) -> Option<SecurityIdentifier> {
            let sectype = encoded_id.clone() >> SHIFT;
            let id = (encoded_id & MASK32) as u32;
            match sectype {
                EQUITY_M => Some(SecurityIdentifier { security_type: SecurityType::Equity, raw_id: id }),
                BOND_M => Some(SecurityIdentifier { security_type: SecurityType::Bond, raw_id: id }),
                OPT_M => Some(SecurityIdentifier { security_type: SecurityType::Option, raw_id: id }),
                FUT_M => Some(SecurityIdentifier { security_type: SecurityType::Future, raw_id: id }),
                ETF_M => Some(SecurityIdentifier { security_type: SecurityType::ETF, raw_id: id }),
                MUTF_M => Some(SecurityIdentifier { security_type: SecurityType::MutualF, raw_id: id }),
                CRYPT_M => Some(SecurityIdentifier { security_type: SecurityType::Crypto, raw_id: id }),
                FX_M => Some(SecurityIdentifier { security_type: SecurityType::FX, raw_id: id }),
                SWAP_M => Some(SecurityIdentifier { security_type: SecurityType::Swap, raw_id: id }),
                WRNT_M => Some(SecurityIdentifier { security_type: SecurityType::Wrnt, raw_id: id }),
                ADR_M => Some(SecurityIdentifier { security_type: SecurityType::Adr, raw_id: id }),
                PFD_M => Some(SecurityIdentifier { security_type: SecurityType::Pfd, raw_id: id }),
                OTHER_M => Some(SecurityIdentifier { security_type: SecurityType::Other, raw_id: id }),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::security_types::sec_types::SecurityType;

    //  START TESTING OF SecurityType::encode()

    #[test]
    fn test_sec_type_equity() {
        let sid = SecurityType::encode(SecurityType::Equity, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Equity);
    }

    #[test]
    fn test_sec_type_bond() {
        let sid = SecurityType::encode(SecurityType::Bond, 13234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Bond);
    }

    #[test]
    fn test_sec_type_option() {
        let sid = SecurityType::encode(SecurityType::Option, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Option);
    }

    #[test]
    fn test_sec_type_future() {
        let sid = SecurityType::encode(SecurityType::Future, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Future);
    }

    #[test]
    fn test_sec_type_etf() {
        let sid = SecurityType::encode(SecurityType::ETF, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::ETF);
    }

    #[test]
    fn test_sec_type_mutual_fund() {
        let sid = SecurityType::encode(SecurityType::MutualF, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::MutualF);
    }

    #[test]
    fn test_sec_type_crypto() {
        let sid = SecurityType::encode(SecurityType::Crypto, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Crypto);
    }

    #[test]
    fn test_sec_type_fx() {
        let sid = SecurityType::encode(SecurityType::FX, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::FX);
    }

    #[test]
    fn test_sec_type_swap() {
        let sid = SecurityType::encode(SecurityType::Swap, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Swap);
    }

    #[test]
    fn test_sec_type_wrnt() {
        let sid = SecurityType::encode(SecurityType::Wrnt, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Wrnt);
    }

    #[test]
    fn test_sec_type_adr() {
        let sid = SecurityType::encode(SecurityType::Adr, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Adr);
    }

    #[test]
    fn test_sec_type_pfd() {
        let sid = SecurityType::encode(SecurityType::Pfd, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Pfd);
    }

    #[test]
    fn test_sec_type_other() {
        let sid = SecurityType::encode(SecurityType::Other, 1234);
        let st = SecurityType::get_sec_type(sid);
        assert_eq!(st, SecurityType::Other);
    }


    //  START TESTING OF SecurityType::encode()


    #[test]
    fn test_sec_type_fs_equity() {
        let st = SecurityType::get_sec_type_fs("Equity");
        assert_eq!(st, SecurityType::Equity);
    }

    #[test]
    fn test_sec_type_fs_bond() {
        let st = SecurityType::get_sec_type_fs("Bond");
        assert_eq!(st, SecurityType::Bond);
    }

    #[test]
    fn test_sec_type_fs_option() {
        let st = SecurityType::get_sec_type_fs("Option");
        assert_eq!(st, SecurityType::Option);
    }

    #[test]
    fn test_sec_type_fs_future() {
        let st = SecurityType::get_sec_type_fs("Future");
        assert_eq!(st, SecurityType::Future);
    }
    #[test]
    fn test_sec_type_fs_etf() {
        let st = SecurityType::get_sec_type_fs("ETF");
        assert_eq!(st, SecurityType::ETF);
    }


    #[test]
    fn test_sec_type_fs_mutual_fund() {
        let st = SecurityType::get_sec_type_fs("Mutual FunD");
        assert_eq!(st, SecurityType::MutualF);
    }

    #[test]
    fn test_sec_type_fs_crypto() {
        let st = SecurityType::get_sec_type_fs("Crypto");
        assert_eq!(st, SecurityType::Crypto);
    }

    #[test]
    fn test_sec_type_fs_fx() {
        let st = SecurityType::get_sec_type_fs("FX");
        assert_eq!(st, SecurityType::FX);
    }

    #[test]
    fn test_sec_type_fs_swap() {
        let st = SecurityType::get_sec_type_fs("Swap");
        assert_eq!(st, SecurityType::Swap);
    }

    #[test]
    fn test_sec_type_fs_wrnt() {
        let st = SecurityType::get_sec_type_fs("Warrant");
        assert_eq!(st, SecurityType::Wrnt);
    }
    #[test]
    fn test_sec_type_fs_wrnt1() {
        let st = SecurityType::get_sec_type_fs("wrnt");
        assert_eq!(st, SecurityType::Wrnt);
    }

    #[test]
    fn test_sec_type_fs_wrnt2() {
        let st = SecurityType::get_sec_type_fs("wt ");
        assert_eq!(st, SecurityType::Wrnt);
    }

    #[test]
    fn test_get_detailed_sec_type_00() {
        let st = SecurityType::get_detailed_sec_type("Equity", "Agilent Technologies Inc");

        assert_eq!(st.0, SecurityType::Equity);
        assert_eq!(st.1, "Eqty");
    }

    #[test]
    fn test_get_detailed_sec_type_01() {
        let st = SecurityType::get_detailed_sec_type("Equity", "Agilent Technologies Inc ADR");

        assert_eq!(st.0, SecurityType::Adr);
        assert_eq!(st.1, "Adr");
    }

    #[test]
    fn test_get_detailed_sec_type_02() {
        let st = SecurityType::get_detailed_sec_type("Equity", "Agilent Technologies Inc ADR");

        assert_eq!(st.0, SecurityType::Adr);
        assert_eq!(st.1, "Adr");
    }

    #[test]
    fn test_get_detailed_sec_type_03() {
        let st = SecurityType::get_detailed_sec_type("Equity", "Agilent Technologies Inc Pfd");

        assert_eq!(st.0, SecurityType::Pfd);
        assert_eq!(st.1, "Pfd");
    }

    #[test]
    fn test_get_detailed_sec_type_04() {
        let st = SecurityType::get_detailed_sec_type("Equity", "Agilent Technologies Inc Warrant");

        assert_eq!(st.0, SecurityType::Wrnt);
        assert_eq!(st.1, "Wrnt");
    }

    #[test]
    fn test_get_detailed_sec_type_05() {
        let st = SecurityType::get_detailed_sec_type("Equity", "Agilent Technologies Inc Wrnt");

        assert_eq!(st.0, SecurityType::Wrnt);
        assert_eq!(st.1, "Wrnt");

    }

    #[test]
    fn test_get_detailed_sec_type_06() {
        let st = SecurityType::get_detailed_sec_type("Etf", "Vanguard S&P 500 ETF (VOO)");

        assert_eq!(st.0, SecurityType::ETF);
        assert_eq!(st.1, "ETF");

    }

    #[test]
    fn test_get_detailed_sec_type_07() {
        let st = SecurityType::get_detailed_sec_type("Mutual Fund", "Catalyst Systematic Alpha Fund");

        assert_eq!(st.0, SecurityType::MutualF);
        assert_eq!(st.1, "MutF");

    }


    #[test]
    fn test_get_detailed_sec_type_08() {
        let st = SecurityType::get_detailed_sec_type("XXDF", "Something that isn't yet defined");

        assert_eq!(st.0, SecurityType::Other);
        assert_eq!(st.1, "Other");

    }
}
