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

use diesel::serialize::ToSql;
use AlphaVantage_Rust::m_get_news_stories;
extern crate diesel;
extern crate serde;
use diesel::prelude::*;
use diesel::sql_query;
use AlphaVantage_Rust::dbfunctions::base::establish_connection_or_exit;

#[derive(QueryableByName, Debug)]
pub struct NewsStories {
    #[sql_type = "diesel::sql_types::Varchar"]
    pub title: String,
    #[sql_type = "diesel::sql_types::Varchar"]
    pub url: String,
}

fn main() {
    let symbol = "TSLA";
    let query = m_get_news_stories!(symbol);
    let connection = &mut establish_connection_or_exit();
    let news_stories: Vec<NewsStories> = sql_query(query)
        .get_results(connection)
        .expect("Error loading news stories");

    println!("{} News Stories for {}", news_stories.len(), symbol);
    for news_story in news_stories {
        println!("Title: {}", news_story.title);
        println!("URL: {}", news_story.url);
        println!("");
    }
}
