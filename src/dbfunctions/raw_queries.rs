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

#[macro_export]
macro_rules! m_get_news_stories {
  ($string1:expr) => {
    format!(
      r#"SELECT a.title, a.url
               FROM articles a
               INNER JOIN feeds f ON a.hashid = f.articleid
               INNER JOIN symbols s ON f.sid = s.sid
               WHERE s.symbol = '{}'"#,
      $string1
    )
  };
}

#[macro_export]
macro_rules! m_get_news_count {
  ($string1:expr) => {
    format!(
      r#"SELECT COUNT(a.title)
               FROM articles a
               INNER JOIN feeds f ON a.hashid = f.articleid
               INNER JOIN symbols s ON f.sid = s.sid
               WHERE s.symbol = '{}'"#,
      $string1
    )
  };
}

// const PROC_STATS: &str = r#"WITH durations AS (
// SELECT
// proc_id,
// EXTRACT(EPOCH FROM (end_time - start_time)) / 60 AS duration_minutes
// FROM
// procstates
// WHERE
// end_time IS NOT NULL
// )
// SELECT
// proc_id,
// ROUND(AVG(duration_minutes)::numeric, 2) AS average_duration,
// ROUND(STDDEV(duration_minutes)::numeric, 2) AS stddev_duration,
// ROUND(VARIANCE(duration_minutes)::numeric, 2) AS variance_duration,
// ROUND(PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY duration_minutes)::numeric, 2) AS median_duration,
// ROUND(PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY duration_minutes)::numeric, 2) AS first_quartile,
// ROUND(PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY duration_minutes)::numeric, 2) AS third_quartile,
// ROUND(MIN(NULLIF(duration_minutes, 0))::numeric, 2) AS min_non_zero_duration,  -- Minimum non-zero duration
// ROUND(MAX(duration_minutes)::numeric, 2) AS max_duration,
// COUNT(*) AS process_count,
// CASE
// WHEN (MAX(duration_minutes) - MIN(duration_minutes)) > 0 THEN 'Increasing'
// WHEN (MAX(duration_minutes) - MIN(duration_minutes)) < 0 THEN 'Decreasing'
// ELSE 'Stable'
// END AS trend
// FROM
// durations
// GROUP BY
// proc_id;"#;

#[cfg(test)]
mod test {
  #[test]
  fn t_01() {
    assert_eq!(
      m_get_news_stories!("AAPL"),
      r#"SELECT a.title, a.url
               FROM articles a
               INNER JOIN feeds f ON a.hashid = f.articleid
               INNER JOIN symbols s ON f.sid = s.sid
               WHERE s.symbol = 'AAPL'"#
    );
  }

  #[test]
  fn t_02() {
    assert_eq!(
      m_get_news_count!("AAPL"),
      r#"SELECT COUNT(a.title)
               FROM articles a
               INNER JOIN feeds f ON a.hashid = f.articleid
               INNER JOIN symbols s ON f.sid = s.sid
               WHERE s.symbol = 'AAPL'"#
    );
  }
}
