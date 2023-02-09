use std::collections::HashMap;
use regex::Regex;
use scraper::Html;
use crate::lodestone::internal::common::{build_selector, get_from_lodestone, get_value, url_builder};
use crate::lodestone::internal::selectors;
use crate::lodestone::structs::linkshell::{LodestoneLinkShell, LodestoneLinkShellBuilder};

pub(crate) async fn get_linkshell(id: u64) -> LodestoneLinkShell {
    let url = url_builder("linkshell", id.to_string().as_str());
    let page_text_fut = get_from_lodestone(url.as_str());
    let page_text = page_text_fut.await;

    let name_selector = build_selector(selectors::LS_NAME_SELECTOR);
    let member_selector = build_selector(selectors::LS_MEMBER_SELECTOR);
    let member_name_selector = build_selector(selectors::LS_MEMBER_NAME_SELECTOR);

    let name = get_value(&name_selector, page_text.clone());
    let member_id_regex = Regex::new(selectors::LS_MEMBER_ID_REGEX).unwrap();

    let htm = Html::parse_document(page_text.as_str());
    let members_raw = htm.select(&member_selector);

    let mut members_vec: HashMap<u64, String> = HashMap::new();
    members_vec.clear();

    for member_raw in members_raw {
        let inter = member_raw.html();
        let member_str = inter.as_str();

        let name = get_value(&member_name_selector, inter.clone()).await;
        let id_raw = member_id_regex.captures(member_str).unwrap();
        let id = id_raw.get(1).unwrap().as_str();

        members_vec.insert(id.parse().unwrap(), name);
    }

    LodestoneLinkShellBuilder::default()
        .id(id)
        .name(name.await)
        .members(members_vec)
        .build()
        .unwrap()
}