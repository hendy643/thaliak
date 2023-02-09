use std::collections::HashMap;
use std::future::Future;
use regex::Regex;
use unix_ts::ts;
use crate::lodestone::internal::common::{build_selector, get_from_lodestone, get_value, url_builder};
use crate::lodestone::internal::selectors;
use crate::lodestone::structs::freecompany::{LodestoneFreeCompany, LodestoneFreeCompanyBuilder};

pub async fn get_free_company(id: u64) -> LodestoneFreeCompany {
    let url = url_builder("freecompany", id.to_string().as_str());
    let page_text_fut = get_from_lodestone(url.as_str());

    let n_selector = build_selector(selectors::FC_NAME_SELECTOR);
    let m_c_selector = build_selector(selectors::FC_MEMBER_COUNT_SELECTOR);
    let n_est_selector = build_selector(selectors::FC_NO_ESTATE_SELECTOR);
    let formed_selector = build_selector(selectors::FC_FORMED_SELECTOR);
    let gc_selector = build_selector(selectors::FC_GRAND_COMPANY_SELECTOR);
    let rank_selector = build_selector(selectors::FC_RANK_SELECTOR);
    let server_selector = build_selector(selectors::FC_SERVER_SELECTOR);
    let slogan_selector = build_selector(selectors::FC_SLOGAN_SELECTOR);
    let tag_selector = build_selector(selectors::FC_TAG_SELECTOR);

    let page_text = page_text_fut.await;

    let name = get_value(&n_selector, page_text.clone());
    let member_count = get_value(&m_c_selector, page_text.clone());
    let no_estate = get_value(&n_est_selector, page_text.clone());
    let gc = get_value(&gc_selector, page_text.clone());
    let rank = get_value(&rank_selector, page_text.clone());
    let serve = get_value(&server_selector, page_text.clone());
    let slogan = get_value(&slogan_selector, page_text.clone());
    let tag = get_value(&tag_selector, page_text.clone());
    let form = get_value(&formed_selector, page_text.clone());

    let grand_company = process_grand_company(gc.await);
    let formed = process_formed(form.await);
    let estate = process_estate(page_text.clone(), no_estate);
    let server = process_server(serve.await);

    LodestoneFreeCompanyBuilder::default()
        .id(id)
        .name(name.await)
        .member_count(member_count.await.parse().unwrap())
        .estate(estate.await)
        .formed(formed.await)
        .grand_company(grand_company.await)
        .rank(rank.await.parse().unwrap())
        .server(server.await)
        .slogan(slogan.await)
        .tag(tag.await)
        .build()
        .unwrap()
}

async fn process_grand_company(raw_string: String) -> String {
    let gc_regex = Regex::new(selectors::FC_GRAND_COMPANY_REGEX).unwrap();
    let gc_result = gc_regex.captures(raw_string.as_str());
    let gc_name = gc_result.unwrap().get(1).unwrap().as_str();
    gc_name.to_string()
}

async fn process_formed(raw_string: String) -> String {
    let formed_regex = Regex::new(selectors::FC_FORMED_REGEX).unwrap();
    let formed_string = {
        let formed_result = formed_regex.captures(raw_string.as_str()).unwrap();
        let mut formed_date: u64 = 0;
        for formed_match in formed_result.iter() {
            let mat = formed_match.unwrap().as_str();
            let ret_str: u64 = match mat.parse::<u64>() {
                Ok(n) => n,
                Err(_) => 0,
            };
            formed_date = ret_str;
        }

        let t = ts!(formed_date as i64);
        t.to_utc_datetime().date_naive().to_string()
    };
    return formed_string;
}

async fn process_estate(
    raw_string: String,
    no_estate: impl Future<Output=String>,
) -> HashMap<String, String> {
    let est_greet_selector = build_selector(selectors::FC_ESTATE_GREETING_SELECTOR);
    let est_name_selector = build_selector(selectors::FC_ESTATE_NAME_SELECTOR);
    let est_plot_selector = build_selector(selectors::FC_ESTATE_PLOT_SELECTOR);

    let mut est_val: HashMap<String, String> = [
        ("name".to_string(), "".to_string()),
        ("greeting".to_string(), "".to_string()),
        ("plot".to_string(), "".to_string()),
    ]
        .iter()
        .cloned()
        .collect();

    let no_est = no_estate.await;
    if no_est.is_empty() {
        let est_greeting = get_value(&est_greet_selector, raw_string.clone())
            .await;
        let est_name = get_value(&est_name_selector, raw_string.clone()).await;
        let est_plot = get_value(&est_plot_selector, raw_string.clone()).await;
        est_val.insert("name".to_string(), est_name);
        est_val.insert("greeting".to_string(), est_greeting.replace("<br>", " "));
        est_val.insert("plot".to_string(), est_plot);
    }
    return est_val;
}

async fn process_server(raw_string: String) -> HashMap<String, String> {
    let se_regex = Regex::new(selectors::FC_SERVER_REGEX)
        .unwrap()
        .captures(raw_string.as_str());
    let se_result = se_regex.unwrap();

    let se_name = se_result.get(1).map_or("", |m| m.as_str());
    let se_dc = se_result.get(2).map_or("", |m| m.as_str());

    let mut se_map: HashMap<String, String> = HashMap::with_capacity(2);
    se_map.insert("world".to_string(), se_name.to_string());
    se_map.insert("data_centre".to_string(), se_dc.to_string());
    return se_map;
}