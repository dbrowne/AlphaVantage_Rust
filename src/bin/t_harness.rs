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

use dotenvy::dotenv;
use AlphaVantage_Rust::alpha_lib::misc_functions::get_exe_name;
use AlphaVantage_Rust::db_funcs::{get_proc_id_or_insert,log_proc_start, log_proc_end};
use AlphaVantage_Rust::dbfunctions::base:: establish_connection_or_exit;

fn main() {
    let conn = &mut establish_connection_or_exit();

    dotenv().ok();
    let proc_name = get_exe_name();

    let pid =get_proc_id_or_insert(conn,&proc_name).unwrap();

    let p_pid = log_proc_start(conn,pid).unwrap();

    let xx = log_proc_end(conn,p_pid).unwrap();

}