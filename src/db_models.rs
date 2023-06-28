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


use  chrono::prelude::*;
use  diesel::prelude::*;
use crate::schema::symbols;
use chrono::NaiveDateTime;
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
    pub overview:bool,
    pub intraday:bool,
    pub summary:bool,
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
    pub overview:&'a bool,
    pub intraday:&'a bool,
    pub summary:&'a bool,
    pub c_time: &'a NaiveDateTime,
    pub m_time: &'a NaiveDateTime,

}