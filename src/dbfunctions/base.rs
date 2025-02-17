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

use std::{env, process};
use diesel::{pg::PgConnection, prelude::*};
use dotenvy::dotenv;

/// Handles database connections for different environments.
///
///  provides a function to establish a connection to the database based
/// on the current `DATABASE_ENV` variable. Supported environments include:
/// - `development` (default)
/// - `test`
/// - `production`


  /// Enum representing the available database environments.
  ///
  /// The environment is determined from the `DATABASE_ENV` environment variable.
  #[derive(Debug)]
  enum DatabaseEnv {
    Development,
    Test,
    Production,
  }

impl DatabaseEnv {
  /// Retrieves the database environment from the `DATABASE_ENV` variable.
  ///
  /// This function reads the `DATABASE_ENV` environment variable and returns the corresponding
  /// `DatabaseEnv` variant. If the variable is missing or contains an invalid value,
  /// the function prints an error message and **terminates the process** with `process::exit(1)`.
  ///
  /// ## Expected Values:
  /// - `"development"` → Returns `DatabaseEnv::Development`
  /// - `"test"` → Returns `DatabaseEnv::Test`
  /// - `"production"` → Returns `DatabaseEnv::Production`
  ///
  /// ## Behavior on Errors:
  /// - If `DATABASE_ENV` is **unset or contains an invalid value**, the program exits with an error.
  ///
  /// # Examples
  /// ```ignore
  ///  /// std::env::set_var("DATABASE_ENV", "test");
  /// assert_eq!(DatabaseEnv::from_env(), DatabaseEnv::Test);
  ///
  /// std::env::set_var("DATABASE_ENV", "production");
  /// assert_eq!(DatabaseEnv::from_env(), DatabaseEnv::Production);
  ///
  /// std::env::set_var("DATABASE_ENV", "invalid_value"); // This will terminate the process.
  /// DatabaseEnv::from_env(); // Exits with an error.
  /// ```
  fn from_env() -> Self {
    match env::var("DATABASE_ENV").as_deref() {
      Ok("test") => Self::Test,
      Ok("production") => Self::Production,
      Ok("development") => Self::Development,
      _ => {
        eprintln!(
          "Invalid database environment: {:?}. Expected 'development', 'test', or 'production'. for DATABASE_ENV variable" ,
          env::var("DATABASE_ENV").unwrap_or_else(|_| "None".to_string())
        );
        process::exit(1);
      }
    }
  }



    /// Retrieves the database URL corresponding to the current environment.
    ///
    /// # Returns
    /// - `Some(String)` containing the database URL if found.
    /// - `None` if no matching database URL is set in the environment.
    ///
    /// # Examples
    /// ```ignore
    /// use database::DatabaseEnv;
    /// std::env::set_var("DEV_DATABASE_URL", "postgres://dev_user@localhost/dev_db");
    /// assert_eq!(DatabaseEnv::Development.database_url(), Some("postgres://dev_user@localhost/dev_db".to_string()));
    /// ```
    fn database_url(&self) -> Option<String> {
      match self {
        Self::Development => env::var("DEV_DATABASE_URL").ok(),
        Self::Test => env::var("TEST_DATABASE_URL").ok(),
        Self::Production => env::var("PROD_DATABASE_URL").ok(),
      }
    }
  }

  /// Establishes a connection to the database or exits on failure.
  ///
  /// This function:
  /// 1. Loads the `.env` file (if present).
  /// 2. Determines the database environment from `DATABASE_ENV`.
  /// 3. Retrieves the appropriate database URL (`DEV_DATABASE_URL`, `TEST_DATABASE_URL`, or `PROD_DATABASE_URL`).
  /// 4. Attempts to establish a connection to the database.
  /// 5. Exits with an error message if the connection fails.
  ///
  /// # Panics
  /// This function does **not** panic, but it **exits the process** with a non-zero
  /// status code if:
  /// - `DATABASE_ENV` is unknown.
  /// - The corresponding database URL is not set.
  /// - The database connection fails.
  ///
  /// # Examples
  /// ```ignore
  /// use establish_connection_or_exit;
  /// let conn = establish_connection_or_exit(); // Connects to the appropriate DB
  /// ```
  pub fn establish_connection_or_exit() -> PgConnection {
    dotenv().ok(); // Load .env file if available

    let env = DatabaseEnv::from_env();
    let database_url = env.database_url().unwrap_or_else(|| {
      eprintln!("No database URL found for {:?} environment", env);
      process::exit(1);
    });

    PgConnection::establish(&database_url).unwrap_or_else(|err| {
      eprintln!("Failed to connect to database: {}", err);
      process::exit(1);
    })
  }
