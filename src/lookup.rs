use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub text: String,
    pub url: String,
}

pub fn find_md_links(markdown: &String) -> Vec<Hyperlink> {
    let inline_link_pattern = r"\[([^\]]+)\]\(([^)]+)\)";
    let anchored_link_pattern = r"\[([^\]]+)\]:\s+(\S+)";
    let footnote_link_text_pattern = r"\[([^\]]+)\]\[(\d+)\]";
    let footnote_link_url_pattern = r"\[(\d+)\]:\s+(\S+)";

    let inline_link_re = Regex::new(inline_link_pattern).expect("Failed to compile regex");
    let anchored_link_re = Regex::new(anchored_link_pattern).expect("Failed to compile regex");
    let footnote_link_text_re = Regex::new(footnote_link_text_pattern).expect("Failed to compile regex");
    let footnote_link_url_re = Regex::new(footnote_link_url_pattern).expect("Failed to compile regex");

    let mut links = Vec::new();
    for caps in inline_link_re.captures_iter(markdown) {
        let link_text = caps.get(1).unwrap().as_str().to_string();
        let link_url = caps.get(2).unwrap().as_str().to_string();
        links.push(Hyperlink{ text: link_text, url: link_url });
    }

    for caps in anchored_link_re.captures_iter(markdown) {
        let link_text = caps.get(1).unwrap().as_str().to_string();
        let link_url = caps.get(2).unwrap().as_str().to_string();
        links.push(Hyperlink{ text: link_text, url: link_url });
    }

    let mut footnote_links = HashMap::new();
    let mut footnote_urls = HashMap::new();

    for caps in footnote_link_text_re.captures_iter(markdown) {
        let link_text = caps.get(1).unwrap().as_str().to_string();
        let link_number: usize = caps.get(2).unwrap().as_str().parse().expect("Failed to parse link number");
        footnote_links.insert(link_number, link_text);
    }

    for caps in footnote_link_url_re.captures_iter(markdown) {
        let link_number: usize = caps.get(1).unwrap().as_str().parse().expect("Failed to parse link number");
        let link_url = caps.get(2).unwrap().as_str().to_string();
        footnote_urls.insert(link_number, link_url);
    }

    for (key, value) in footnote_links.iter() {
        if let Some(url) = footnote_urls.get(key) {
            links.push(Hyperlink{ text: value.to_string(), url: url.to_string() });
        }
    }

    links
}
