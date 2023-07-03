use std::string::String;

use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::interface::Attribute;

fn parse_html(source: &str) -> RcDom {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut source.as_bytes())
}
