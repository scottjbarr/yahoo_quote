// Get quotes from Yahoo.
//
// Author : Scott Barr
// Date   : 4 Feb 2015
//

extern crate curl;

use curl::http;
use std::str;
use std::env;

const URL: &'static str = "http://download.finance.yahoo.com/d/quotes.csv";

#[test]
fn test_strip_value() {
    assert_eq!("Hi", strip_value("\"Hi\" "));
}

#[test]
fn test_format_quote() {
    let line = "\"Hugs \",42.00,\"3/2/2015\",\"4:00pm\"";
    let symbol = "XOXO";
    let parts = line.split(",").collect();

    let expected = "Hugs  (XOXO) 42.00 @ 3/2/2015 4:00pm";

    assert_eq!(expected, format_quote(symbol, parts));
}

#[test]
fn test_get_quote() {
    assert!(false, "Work out how to stub HTTP requests");
}

// Clean up part from the quote data.
fn strip_value(val:&str) -> String {
    val.trim().replace("\"", "")
}

// Format the quote data.
fn format_quote(symbol:&str, parts:Vec<&str>) -> String {
    format!("{} ({}) {} @ {} {}",
        strip_value(parts[0]),
        symbol,
        strip_value(parts[parts.len() - 3]),
        strip_value(parts[parts.len() - 2]),
        strip_value(parts[parts.len() - 1]))
}

// Get a quote from Yahoo.
fn get_quote(symbol:&str) -> String {
    let url_args = format!("s={}&f=nl1d1t1", symbol);
    let url = format!("{}?{}", URL, url_args);
    let resp = http::handle().get(url).exec().unwrap();
    let body = str::from_utf8(resp.get_body());

    let line = match body {
        Ok(content) => { content }, //.trim() },
        Err(e) => { panic!("HTTP Status Code {:?}", e) }
    };

    // Take the parts from the end of the line and work backwards. It would
    // be better to try a proper csv parser.
    let parts: Vec<&str> = line.split(",").collect();

    format_quote(symbol, parts)
}

pub fn main() {
    if env::args().len() < 2 {
        panic!("Please supply one or more symbols");
    }

    let mut args: Vec<_> = env::args().collect();

    // remove program name
    args.remove(0);

    // Actually, this is dumb. I can request multiple symbols in one request
    // eg s=GE,MO,USDAUD=X,AAPL
    for symbol in args.iter() {
        println!("{}", get_quote(symbol));
    }
}
