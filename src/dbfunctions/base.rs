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

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::{env, process};
use dotenvy::dotenv;


/// Establishes a connection to a Postgres database using Diesel.
///
/// This function will attempt to read the `DATABASE_URL` environment variable,
/// which is expected to contain the database connection string.
/// Ensure that this environment variable is set prior to
/// invoking this function.
///
/// If the `DATABASE_URL` environment variable is not set, the function will
/// print an error message and terminate the program with a status code of `1`.
///
/// If a connection to the database cannot be established,
/// the function will print an error message and also exit the program with a status code of `1`.
/// A database connection is crucial for program functionality; without it, the program should not proceed.
/// The choice to exit instead of panicking is made because this function is ideally
/// one of the first operations executed in the program.
///
/// # Returns
///
/// On successful connection to the database, this function will return a `PgConnection`.
/// In the event of any errors, the program will terminate.
///
/// # Example
///```ignore
/// use dbfunctions::base::establish_connection_or_exit;
///
/// fn main() {
///     let conn = establish_connection_or_exit();
///     // Operations continue with `conn` or the program will have already exited.
/// }
/// ```
///
pub fn establish_connection_or_exit() -> PgConnection {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(db) => db,
        Err(_) => {
            eprintln!("No Database url set");
            process::exit(1);
        }
    };

    let conn = PgConnection::establish(&database_url).unwrap_or_else(|_| {
        eprintln!("Can't establish db connection");
        process::exit(1);
    }
    );


    conn
}