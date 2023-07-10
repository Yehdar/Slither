extern crate hyper;
extern crate url;

use std::io::Read;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::fmt;

use self::hyper::Client;
use self::hyper::status::StatusCode;
use self::url::{ParseResult, Url, UrlParser};

use parsing;

#[derive(Debug, Clone)]
pub enum UrlState {
    Accessible(Url),
    BadStatus(Url, StatusCode),
    ConnectionFailed(Url),
    TimedOut(Url),
    Malformed(String),
}

impl fmt::Display for UrlStat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UrlState::Accessible(ref url) => format!("!! {}", url).fmt(f),
            UrlState::BadStatus(ref url, ref status) => format!("x {} ({})", url, status).fmt(f),
            UrlState::ConnectionFailed(ref url) => format!("x {} (connection failed)", url).fmt(f),
            UrlState::TimedOut(ref url) => format!("x {} (timed out)", url).fmt(f),
            UrlState::Malformed(ref url) => format!("x {} (malformed)", url).fmt(f),
        }
    }
}
