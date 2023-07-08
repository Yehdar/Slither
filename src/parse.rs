use std::string::String;

use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::interface::Attribute;

pub fn parse_html(source: &str) -> RcDom {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source.as_bytes())
        .unwrap()
}

pub fn get_urls(handle: Handle) -> Vec<String> {
    let mut urls = vec![];
    let mut anchor_tags = vec![];

    for node in anchor_tags { 
        if let NodeData::Element { ref attrs, ..} = node {
            for attr in attrs.borrow().iter(){
                let Attribute {
                    ref name,
                    ref value,
                } = *attr;
                if &*(name.local) == "href" {
                    urls.push(value.to_string());
                }
            }
        }
    }
}
