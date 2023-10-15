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

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::{env, process};
use std::error::Error;
use chrono::{DateTime, Local};
use crate::alpha_lib::news_type::RawFeed;
use crate::db_models::{Feed, NewNewsOverview};
use crate::schema::newsoverviews::{items, sid, relevance, sentiment,creation};

pub fn insert_and_get_news_root(conn: PgConnection, s_id: i64, item_count: i32, s_entiment: String,
                                r_elevance: String) -> Result<Feed, Box<dyn Error>> {

    let localt: DateTime<Local> = Local::now();
    let now = localt.naive_local();

    let feed = NewNewsOverview{
        items: &item_count,
        sid: s_id,
        sentiment: &s_entiment,
        relevance: &r_elevance,
        creation: &now,
    };



    todo!()


}