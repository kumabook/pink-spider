#![feature(plugin, old_io)]
#![plugin(string_cache_plugin)]
extern crate html5ever;
extern crate regex;

#[macro_use]
extern crate string_cache;

use std::old_io as io;
use std::default::Default;
use html5ever::sink::common::{Document, Doctype, Text, Comment, Element};
use html5ever::sink::rcdom::{RcDom, Handle};
use html5ever::{parse, one_input};

extern crate pink_spider;

fn main() {
    let input = io::stdin().read_to_string().unwrap();
    println!("playlist extractor\n");
//    println!("{}", input);
    pink_spider::scraper::extract_playlist(input);
}
