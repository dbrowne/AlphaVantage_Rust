/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2024. Dwight J. Browne
 * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use chrono::{prelude::*, NaiveDateTime};
use diesel::prelude::*;

use crate::schema::{
  articles, authormaps, authors, feeds, intradayprices, newsoverviews, overviewexts, overviews,
  procstates, proctypes, sources, summaryprices, symbols, tickersentiments, topicmaps, topicrefs,
  topstats,
};

#[derive(Queryable, Debug)]
pub struct Symbol {
  pub sid: i64,
  pub symbol: String,
  pub name: String,
  pub sec_type: String,
  pub region: String,
  pub marketopen: NaiveTime,
  pub marketclose: NaiveTime,
  pub timezone: String,
  pub currency: String,
  pub overview: bool,
  pub intraday: bool,
  pub summary: bool,
  pub c_time: NaiveDateTime,
  pub m_time: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = symbols)]
#[diesel(primary_key(sid))]
pub struct NewSymbol<'a> {
  pub sid: &'a i64,
  pub symbol: &'a str,
  pub name: &'a str,
  pub sec_type: &'a str,
  pub region: &'a str,
  pub marketopen: &'a NaiveTime,
  pub marketclose: &'a NaiveTime,
  pub timezone: &'a str,
  pub currency: &'a str,
  pub overview: &'a bool,
  pub intraday: &'a bool,
  pub summary: &'a bool,
  pub c_time: &'a NaiveDateTime,
  pub m_time: &'a NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Overview {
  pub sid: i64,
  pub symbol: String,
  pub name: String,
  pub description: String,
  pub cik: String,
  pub exch: String,
  pub curr: String,
  pub country: String,
  pub sector: String,
  pub industry: String,
  pub address: String,
  pub fiscalyearend: String,
  pub latestquarter: NaiveDate,
  pub marketcapitalization: i64,
  pub ebitda: i64,
  pub peratio: f32,
  pub pegratio: f32,
  pub bookvalue: f64,
  pub dividendpershare: f32,
  pub dividendyield: f32,
  pub eps: f32,
  pub c_time: NaiveDateTime,
  pub mod_time: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = overviews)]
pub struct NewOverview<'a> {
  pub sid: &'a i64,
  pub symbol: &'a str,
  pub name: &'a str,
  pub description: &'a str,
  pub cik: &'a str,
  pub exch: &'a str,
  pub curr: &'a str,
  pub country: &'a str,
  pub sector: &'a str,
  pub industry: &'a str,
  pub address: &'a str,
  pub fiscalyearend: &'a str,
  pub latestquarter: &'a NaiveDate,
  pub marketcapitalization: &'a i64,
  pub ebitda: &'a i64,
  pub peratio: &'a f32,
  pub pegratio: &'a f32,
  pub bookvalue: &'a f64,
  pub dividendpershare: &'a f32,
  pub dividendyield: &'a f32,
  pub eps: &'a f32,
  pub c_time: &'a NaiveDateTime,
  pub mod_time: &'a NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Overviewext {
  pub sid: i64,
  pub revenuepersharettm: f32,
  pub profitmargin: f32,
  pub operatingmarginttm: f32,
  pub returnonassetsttm: f32,
  pub returnonequityttm: f32,
  pub revenuettm: i64,
  pub grossprofitttm: i64,
  pub dilutedepsttm: f32,
  pub quarterlyearningsgrowthyoy: f32,
  pub quarterlyrevenuegrowthyoy: f32,
  pub analysttargetprice: f32,
  pub trailingpe: f32,
  pub forwardpe: f32,
  pub pricetosalesratiottm: f32,
  pub pricetobookratio: f32,
  pub evtorevenue: f32,
  pub evtoebitda: f32,
  pub beta: f64,
  pub annweekhigh: f64,
  pub annweeklow: f64,
  pub fiftydaymovingaverage: f64,
  pub twohdaymovingaverage: f64,
  pub sharesoutstanding: f64,
  pub dividenddate: NaiveDate,
  pub exdividenddate: NaiveDate,
  pub c_time: NaiveDateTime,
  pub mod_time: NaiveDateTime,
}

/// Overviewexts table exists to minimize compile time Diesel 64 column feature is too slow.
#[derive(Insertable, Debug)]
#[diesel(table_name = overviewexts)]
pub struct NewOverviewext<'a> {
  pub sid: &'a i64,
  pub revenuepersharettm: &'a f32,
  pub profitmargin: &'a f32,
  pub operatingmarginttm: &'a f32,
  pub returnonassetsttm: &'a f32,
  pub returnonequityttm: &'a f32,
  pub revenuettm: &'a i64,
  pub grossprofitttm: &'a i64,
  pub dilutedepsttm: &'a f32,
  pub quarterlyearningsgrowthyoy: &'a f32,
  pub quarterlyrevenuegrowthyoy: &'a f32,
  pub analysttargetprice: &'a f32,
  pub trailingpe: &'a f32,
  pub forwardpe: &'a f32,
  pub pricetosalesratiottm: &'a f32,
  pub pricetobookratio: &'a f32,
  pub evtorevenue: &'a f32,
  pub evtoebitda: &'a f32,
  pub beta: &'a f64,
  pub annweekhigh: &'a f64,
  pub annweeklow: &'a f64,
  pub fiftydaymovingaverage: &'a f64,
  pub twohdaymovingaverage: &'a f64,
  pub sharesoutstanding: &'a f64,
  pub dividenddate: &'a NaiveDate,
  pub exdividenddate: &'a NaiveDate,
  pub c_time: &'a NaiveDateTime,
  pub mod_time: &'a NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct IntraDayPrice {
  pub eventid: i32,
  pub tstamp: NaiveDateTime,
  pub sid: i64,
  pub symbol: String,
  pub open: f32,
  pub high: f32,
  pub low: f32,
  pub close: f32,
  pub volume: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = intradayprices)]
pub struct NewIntraDayPrice<'a> {
  pub sid: &'a i64,
  pub tstamp: &'a NaiveDateTime,
  pub symbol: &'a str,
  pub open: &'a f32,
  pub high: &'a f32,
  pub low: &'a f32,
  pub close: &'a f32,
  pub volume: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct SummaryPrice {
  pub eventid: i32,
  pub date: NaiveDate,
  pub sid: i64,
  pub symbol: String,
  pub open: f32,
  pub high: f32,
  pub low: f32,
  pub close: f32,
  pub volume: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = summaryprices)]
pub struct NewSummaryPrice<'a> {
  pub date: &'a NaiveDate,
  pub sid: &'a i64,
  pub symbol: &'a str,
  pub open: &'a f32,
  pub high: &'a f32,
  pub low: &'a f32,
  pub close: &'a f32,
  pub volume: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct TopStat {
  pub eventid: i32,
  pub date: NaiveDateTime,
  pub event_type: String,
  pub sid: i64,
  pub symbol: String,
  pub price: f32,
  pub change_val: f32,
  pub change_pct: f32,
  pub volume: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = topstats)]
pub struct NewTopStat<'a> {
  pub date: &'a NaiveDateTime,
  pub event_type: &'a str,
  pub sid: &'a i64,
  pub symbol: &'a str,
  pub price: &'a f32,
  pub change_val: &'a f32,
  pub change_pct: &'a f32,
  pub volume: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct TopicRef {
  pub id: i32,
  pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = topicrefs)]
pub struct NewTopicRef<'a> {
  pub name: &'a String,
}

#[derive(Queryable, Debug)]
pub struct Author {
  pub id: i32,
  pub author_name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = authors)]
pub struct NewAuthor<'a> {
  pub author_name: &'a String,
}

#[derive(Queryable, Debug)]
pub struct NewsOverview {
  pub id: i32,
  pub sid: i64,
  pub items: i32,
  pub hashid: String,
  pub creation: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = newsoverviews)]
pub struct NewNewsOverview<'a> {
  pub sid: i64,
  pub items: &'a i32,
  pub hashid: &'a String,
  pub creation: &'a NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Feed {
  pub id: i32,
  pub sid: i64,
  pub newsoverviewid: i32,
  pub articleid: String,
  pub sourceid: i32,
  pub osentiment: f64,
  pub sentlabel: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = feeds)]
pub struct NewFeed<'a> {
  pub sid: &'a i64,
  pub newsoverviewid: &'a i32,
  pub articleid: &'a String,
  pub sourceid: &'a i32,
  pub osentiment: &'a f64,
  pub sentlabel: &'a String,
}

#[derive(Queryable, Debug)]
pub struct Article {
  pub hashid: String,
  pub sourceid: i32,
  pub category: String,
  pub title: String,
  pub url: String,
  pub summary: String,
  pub banner: String,
  pub author: i32,
  pub ct: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = articles)]
pub struct NewArticle<'a> {
  pub hashid: &'a String,
  pub sourceid: &'a i32,
  pub category: &'a String,
  pub title: &'a String,
  pub url: &'a String,
  pub summary: &'a String,
  pub banner: &'a String,
  pub author: &'a i32,
  pub ct: &'a NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct AuthorMap {
  pub id: i32,
  pub feedid: i32,
  pub authorid: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = authormaps)]
pub struct NewAuthorMap<'a> {
  pub feedid: &'a i32,
  pub authorid: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct Source {
  pub id: i32,
  pub source_name: String,
  pub domain: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = sources)]
pub struct NewSource<'a> {
  pub source_name: &'a String,
  pub domain: &'a String,
}

#[derive(Queryable, Debug)]
pub struct TickerSentiment {
  pub id: i32,
  pub feedid: i32,
  pub sid: i64,
  pub relevance: f64,
  pub tsentiment: f64,
  pub sentimentlable: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tickersentiments)]
pub struct NewTickerSentiment<'a> {
  pub feedid: &'a i32,
  pub sid: &'a i64,
  pub relevance: &'a f64,
  pub tsentiment: &'a f64,
  pub sentimentlable: &'a String,
}

#[derive(Queryable, Debug)]
pub struct TopicMap {
  pub id: i32,
  pub sid: i64,
  pub feedid: i32,
  pub topicid: i32,
  pub relscore: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = topicmaps)]
pub struct NewTopicMap<'a> {
  pub sid: &'a i64,
  pub feedid: &'a i32,
  pub topicid: &'a i32,
  pub relscore: &'a f64,
}

#[derive(Queryable, Debug)]
pub struct ProcType {
  pub id: i32,
  pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = proctypes)]
pub struct NewProcType<'a> {
  pub name: &'a String,
}

#[derive(Queryable, Debug)]
pub struct ProcState {
  pub spid: i32,
  pub proc_id: i32,
  pub start_time: NaiveDateTime,
  pub end_state: i32,
  pub end_time: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = procstates)]
pub struct NewProcState<'a> {
  pub proc_id: &'a i32,
  pub start_time: &'a NaiveDateTime,
  pub end_state: &'a i32,
  pub end_time: &'a NaiveDateTime,
}
