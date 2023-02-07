extern crate core;

use std::collections::HashMap;
use std::future::Future;
use std::vec;

use regex::Regex;
use scraper::{Html, Selector};
use unix_ts::ts;

use crate::lodestone::internal::selectors;
use crate::lodestone::structs::freecompany::{LodestoneFreeCompany, LodestoneFreeCompanyBuilder};
use crate::lodestone::structs::linkshell::{LodestoneLinkShell, LodestoneLinkShellBuilder};
use crate::lodestone::structs::profile::{LodestoneProfile, LodestoneProfileBuilder};

mod internal;
pub(crate) mod structs;
mod tests;

#[derive(Default, Clone, Debug)]
pub struct Lodestone {}

impl Lodestone {
    pub fn new() -> Self {
        Lodestone {}
    }

    pub async fn get_from_lodestone(&self, item_type: &str, id: u64) -> String {
        let mut base_url = String::from("https://eu.finalfantasyxiv.com/lodestone/");
        base_url.push_str(item_type);
        base_url.push_str("/");
        base_url.push_str(id.to_string().as_str());
        Lodestone::get_page(base_url.as_str()).await
    }

    pub async fn get_profile(&self, id: u64) -> LodestoneProfile {
        let page_text_fut = self.get_from_lodestone("character", id);

        let n_selector = self.build_selector(selectors::PROFILE_NAME_SELECTOR);
        let nd_selector = self.build_selector(selectors::PROFILE_NAMEDAY_SELECTOR);
        let fc_n_selector = self.build_selector(selectors::PROFILE_FC_NAME_SELECTOR);
        let deity_selector = self.build_selector(selectors::PROFILE_DEITY_SELECTOR);
        let title_selector = self.build_selector(selectors::PROFILE_TITLE_SELECTOR);

        let page_text = page_text_fut.await;

        let name = self.get_value(&n_selector, page_text.clone());
        let name_day = self.get_value(&nd_selector, page_text.clone());
        let fc_name = self.get_value(&fc_n_selector, page_text.clone());

        let deity = self.get_value(&deity_selector, page_text.clone());
        let title = self.get_value(&title_selector, page_text.clone());

        let (race, clan, gender) = self.process_race_clan_gender(page_text.clone()).await;

        LodestoneProfileBuilder::default()
            .id(id.clone())
            .name(name.await.clone())
            .nameday(name_day.await.clone())
            .race(race.clone())
            .clan(clan.clone())
            .gender(gender.clone())
            .title(title.await.clone())
            .free_company(fc_name.await.clone())
            .grand_company(
                self.process_grand_company_profile(page_text.clone().to_string())
                    .await,
            )
            .bio(self.process_bio(page_text.clone().to_string()).await)
            .deity(deity.await.clone())
            .build()
            .unwrap()
    }

    pub async fn get_linkshell(&self, id: u64) -> LodestoneLinkShell {
        let page_text_fut = self.get_from_lodestone("linkshell", id);
        let page_text = page_text_fut.await;

        let name_selector = self.build_selector(selectors::LS_NAME_SELECTOR);
        let member_selector = self.build_selector(selectors::LS_MEMBER_SELECTOR);
        let member_name_selector = self.build_selector(selectors::LS_MEMBER_NAME_SELECTOR);

        let name = self.get_value(&name_selector, page_text.clone());
        let member_id_regex = Regex::new(selectors::LS_MEMBER_ID_REGEX).unwrap();


        let htm = Html::parse_document(page_text.as_str());
        let members_raw = htm.select(&member_selector);

        let mut members_vec: HashMap<u64, String> = HashMap::new();
        members_vec.clear();

        for member_raw in members_raw {
            let inter = member_raw.html();
            let member_str = inter.as_str();

            let name = self.get_value(&member_name_selector, inter.clone()).await;
            let id_raw = member_id_regex.captures(member_str).unwrap();
            let id = id_raw.get(1).unwrap().as_str();

            members_vec.insert(id.parse().unwrap(), name);
        };

        LodestoneLinkShellBuilder::default()
            .id(id)
            .name(name.await)
            .members(members_vec)
            .build()
            .unwrap()
    }

    pub async fn get_free_company(&self, id: u64) -> LodestoneFreeCompany {
        let page_text_fut = self.get_from_lodestone("freecompany", id);

        let n_selector = self.build_selector(selectors::FC_NAME_SELECTOR);
        let m_c_selector = self.build_selector(selectors::FC_MEMBER_COUNT_SELECTOR);
        let n_est_selector = self.build_selector(selectors::FC_NO_ESTATE_SELECTOR);
        let formed_selector = self.build_selector(selectors::FC_FORMED_SELECTOR);
        let gc_selector = self.build_selector(selectors::FC_GRAND_COMPANY_SELECTOR);
        let rank_selector = self.build_selector(selectors::FC_RANK_SELECTOR);
        let server_selector = self.build_selector(selectors::FC_SERVER_SELECTOR);
        let slogan_selector = self.build_selector(selectors::FC_SLOGAN_SELECTOR);
        let tag_selector = self.build_selector(selectors::FC_TAG_SELECTOR);

        let page_text = page_text_fut.await;

        let name = self.get_value(&n_selector, page_text.clone());
        let member_count = self.get_value(&m_c_selector, page_text.clone());
        let no_estate = self.get_value(&n_est_selector, page_text.clone());
        let gc = self.get_value(&gc_selector, page_text.clone());
        let rank = self.get_value(&rank_selector, page_text.clone());
        let serve = self.get_value(&server_selector, page_text.clone());
        let slogan = self.get_value(&slogan_selector, page_text.clone());
        let tag = self.get_value(&tag_selector, page_text.clone());
        let form = self.get_value(&formed_selector, page_text.clone());

        let grand_company = self.process_grand_company(gc.await);
        let formed = self.process_formed(form.await);
        let estate = self.process_estate(page_text.clone(), no_estate);
        let server = self.process_server(serve.await);

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

    async fn process_bio(&self, raw_text: String) -> Vec<String> {
        let bio_selector = self.build_selector(selectors::PROFILE_BIO_SELECTOR);
        let bio = self.get_value(&bio_selector, raw_text);
        let bio_int = bio.await;

        let bio_split = bio_int.split("<br>");
        let mut post_bio_split: Vec<String> = vec![];
        for s in bio_split {
            post_bio_split.push(String::from(s));
        }
        post_bio_split
    }

    async fn process_race_clan_gender(&self, raw_text: String) -> (String, String, String) {
        let rcg_selector = self.build_selector(selectors::PROFILE_RACE_CLAN_GENDER_SELECTOR);
        let rcg_regex = Regex::new(selectors::PROFILE_RACE_CLAN_GENDER_REGEX);

        let race_clan_gender = self.get_value(&rcg_selector, raw_text.clone());
        let mut race: String = String::new();
        let mut clan: String = String::new();
        let mut gender: String = String::new();

        let rcg_int = race_clan_gender.await;
        let inter = rcg_regex.unwrap();
        let result = inter.captures_iter(rcg_int.as_str());

        for r in result {
            race = String::from(&r[1]);
            clan = String::from(&r[2]);
            gender = String::from(&r[3]);
        }

        (race, clan, gender)
    }

    async fn process_grand_company_profile(&self, raw_string: String) -> HashMap<String, String> {
        let gc_n_selector = self.build_selector(selectors::PROFILE_GC_NAME_SELECTOR);
        let gc_raw = self.get_value(&gc_n_selector, raw_string).await;
        let gc_regex = Regex::new(selectors::PROFILE_GC_REGEX)
            .unwrap()
            .captures(gc_raw.as_str());
        let gc_result = gc_regex.unwrap();
        let gc_name = gc_result.get(1).map_or("", |m| m.as_str());
        let gc_rank = gc_result.get(2).map_or("", |m| m.as_str());

        [
            ("gc_name".to_string(), gc_name.to_string()),
            ("gc_rank".to_string(), gc_rank.to_string()),
        ]
        .iter()
        .cloned()
        .collect()
    }

    async fn process_grand_company(&self, raw_string: String) -> String {
        let gc_regex = Regex::new(selectors::FC_GRAND_COMPANY_REGEX).unwrap();
        let gc_result = gc_regex.captures(raw_string.as_str());
        let gc_name = gc_result.unwrap().get(1).unwrap().as_str();
        gc_name.to_string()
    }

    async fn process_formed(&self, raw_string: String) -> String {
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
        &self,
        raw_string: String,
        no_estate: impl Future<Output = String>,
    ) -> HashMap<String, String> {
        let est_greet_selector = self.build_selector(selectors::FC_ESTATE_GREETING_SELECTOR);
        let est_name_selector = self.build_selector(selectors::FC_ESTATE_NAME_SELECTOR);
        let est_plot_selector = self.build_selector(selectors::FC_ESTATE_PLOT_SELECTOR);

        let mut est_val: HashMap<String, String> = [
            ("estate_name".to_string(), "".to_string()),
            ("estate_greeting".to_string(), "".to_string()),
            ("estate_plot".to_string(), "".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let no_est = no_estate.await;
        if no_est.is_empty() {
            let est_greeting = self
                .get_value(&est_greet_selector, raw_string.clone())
                .await;
            let est_name = self.get_value(&est_name_selector, raw_string.clone()).await;
            let est_plot = self.get_value(&est_plot_selector, raw_string.clone()).await;
            est_val
                .entry("estate_name".to_string())
                .insert_entry(est_name);
            est_val
                .entry("estate_greeting".to_string())
                .insert_entry(est_greeting.replace("<br>", " "));
            est_val
                .entry("estate_plot".to_string())
                .insert_entry(est_plot);
        }
        return est_val;
    }

    async fn process_server(&self, raw_string: String) -> HashMap<String, String> {
        let se_regex = Regex::new(selectors::FC_SERVER_REGEX)
            .unwrap()
            .captures(raw_string.as_str());
        let se_result = se_regex.unwrap();

        let se_name = se_result.get(1).map_or("", |m| m.as_str());
        let se_dc = se_result.get(2).map_or("", |m| m.as_str());

        let mut se_map: HashMap<String, String> = HashMap::with_capacity(2);
        se_map.insert("World".to_string(), se_name.to_string());
        se_map.insert("DataCentre".to_string(), se_dc.to_string());
        return se_map;
    }

    fn build_selector(&self, css_selector: &str) -> Selector {
        Selector::parse(css_selector).unwrap()
    }

    async fn get_page(url: &str) -> String {
        let resp_fut = reqwest::get(url);
        let response = resp_fut.await.unwrap();
        if response.status().is_success() {
            response.text().await.unwrap()
        } else {
            String::new()
        }
    }

    async fn get_value(&self, selector: &Selector, body: String) -> String {
        let htm = body;
        let mut element: String = String::new();
        for elem in Html::parse_document(htm.as_str()).select(&selector) {
            element = elem.inner_html();
            break;
        }
        element
    }
}
