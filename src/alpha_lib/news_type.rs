/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2023. Dwight J. Browne
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

// NEWS based on https://www.alphavantage.co/query?function=NEWS_SENTIMENT&tickers=AAPL&apikey=demo


use serde::Deserialize;
use serde_with::{serde_as,DefaultOnNull};

#[derive(Default, Debug, Clone, PartialEq,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsRoot {
    pub items: String,
    #[serde(rename = "sentiment_score_definition")]
    pub sentiment_score_definition: String,
    #[serde(rename = "relevance_score_definition")]
    pub relevance_score_definition: String,
    pub feed: Vec<Feed>,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub title: String,
    pub url: String,
    #[serde(rename = "time_published")]
    pub time_published: String,
    pub authors: Vec<String>,
    pub summary: String,
    #[serde(rename = "banner_image")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub banner_image: String,
    pub source: String,
    #[serde(rename = "category_within_source")]
    pub category_within_source: String,
    #[serde(rename = "source_domain")]
    pub source_domain: String,
    pub topics: Vec<Topic>,
    #[serde(rename = "overall_sentiment_score")]
    pub overall_sentiment_score: f64,
    #[serde(rename = "overall_sentiment_label")]
    pub overall_sentiment_label: String,
    #[serde(rename = "ticker_sentiment")]
    pub ticker_sentiment: Vec<TickerSentiment>,
}

#[derive(Default, Debug, Clone, PartialEq,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub topic: String,
    #[serde(rename = "relevance_score")]
    pub relevance_score: String,
}

#[derive(Default, Debug, Clone, PartialEq,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerSentiment {
    pub ticker: String,
    #[serde(rename = "relevance_score")]
    pub relevance_score: String,
    #[serde(rename = "ticker_sentiment_score")]
    pub ticker_sentiment_score: String,
    #[serde(rename = "ticker_sentiment_label")]
    pub ticker_sentiment_label: String,
}