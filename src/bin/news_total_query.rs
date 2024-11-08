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

#[cfg(not(tarpaulin_include))]
extern crate diesel;
extern crate serde;
use std::convert::TryFrom;

use alpha_vantage_rust::{
  alpha_lib::misc_functions::get_exe_name,
  db_funcs::{get_proc_id_or_insert, log_proc_end, log_proc_start},
  dbfunctions::base::establish_connection_or_exit,
};
use bigdecimal::BigDecimal;
use diesel::{pg::data_types::PgNumeric, prelude::*, sql_query};
use dotenvy::dotenv;
use serde::{Serialize, Serializer};
fn pg_numeric_to_i32(numeric: &PgNumeric) -> Option<i32> {
  match numeric {
    PgNumeric::Positive { digits, .. } | PgNumeric::Negative { digits, .. } => {
      // Extract the digits and combine them into a number
      let mut result = 0_i32;
      for &digit in digits {
        result = result * 10_000 + digit as i32; // PgNumeric stores digits in base 10,000
      }
      Some(result)
    }
    _ => None,
  }
}

fn serialize_pg_numeric_as_i32<S>(value: &PgNumeric, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let int_value = pg_numeric_to_i32(value).unwrap_or(0); // Convert PgNumeric to i32 or default to 0
  serializer.serialize_i32(int_value)
}
#[derive(QueryableByName, Debug, Serialize)]
pub struct ArticleCount {
  #[diesel(sql_type = diesel::sql_types::Varchar)]
  pub source_name: String,
  #[diesel(sql_type = diesel::sql_types::Numeric)]
  #[serde(serialize_with = "serialize_pg_numeric_as_i32")]
  pub article_count: PgNumeric,
}

#[allow(dead_code)]
fn pg_numeric_to_decimal(weight: i16, scale: u16, digits: Vec<i16>) -> f64 {
  let mut value = 0.0;
  let base: f64 = 10000.0;
  let mut base_multiplier: f64 = base.powi(weight.into());

  for &digit in &digits {
    value += (digit as f64) * base_multiplier;
    base_multiplier /= base;
  }
  value / base.powi(scale.into())
}
fn main() {
  let conn = &mut establish_connection_or_exit();

  dotenv().ok();
  let id_val = get_proc_id_or_insert(conn, &get_exe_name()).unwrap();
  let pid = log_proc_start(conn, id_val).unwrap();

  match get_article_counts(conn) {
    Ok(results) => {
      println!("Source Name | Article Count");
      let serialized = serde_json::to_string(&results).unwrap();
      println!("{:?}", serialized);
    }
    Err(error) => {
      _ = log_proc_end(conn, pid, 3).unwrap();
      eprintln!("Error executing query: {}", error);
    }
  }
  _ = log_proc_end(conn, pid, 2).unwrap();
}

fn get_article_counts(conn: &mut PgConnection) -> QueryResult<Vec<ArticleCount>> {
  let query = r#"
        WITH article_counts AS (
            SELECT s.source_name, COUNT(a.hashid) AS article_count
            FROM sources s
            JOIN articles a ON s.id = a.sourceid
            GROUP BY s.source_name
        ),
        total_count AS (
            SELECT 'Total' AS source_name, SUM(article_count) AS article_count
            FROM article_counts
        )
        SELECT source_name, article_count
        FROM (
            SELECT source_name, article_count
            FROM article_counts
            UNION ALL
            SELECT source_name, article_count
            FROM total_count
        ) AS combined_results
        ORDER BY
            CASE WHEN source_name = 'Total' THEN 1 ELSE 0 END,
            source_name;
    "#;
  sql_query(query).load::<ArticleCount>(conn)
}
