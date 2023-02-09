use std::collections::HashMap;
use regex::Regex;
use crate::lodestone::internal::common::{build_selector, get_from_lodestone, get_value, url_builder};
use crate::lodestone::internal::selectors;
use crate::lodestone::structs::profile::{LodestoneProfile, LodestoneProfileBuilder};

pub(crate) async fn get_profile(id: u64) -> LodestoneProfile {
    let url = url_builder("character", id.to_string().as_str());
    let page_text_fut = get_from_lodestone(url.as_str());

    let n_selector = build_selector(selectors::PROFILE_NAME_SELECTOR);
    let nd_selector = build_selector(selectors::PROFILE_NAMEDAY_SELECTOR);
    let fc_n_selector = build_selector(selectors::PROFILE_FC_NAME_SELECTOR);
    let deity_selector = build_selector(selectors::PROFILE_DEITY_SELECTOR);
    let title_selector = build_selector(selectors::PROFILE_TITLE_SELECTOR);

    let page_text = page_text_fut.await;

    let name = get_value(&n_selector, page_text.clone());
    let name_day = get_value(&nd_selector, page_text.clone());
    let fc_name = get_value(&fc_n_selector, page_text.clone());

    let deity = get_value(&deity_selector, page_text.clone());
    let title = get_value(&title_selector, page_text.clone());

    let (race, clan, gender) = process_race_clan_gender(page_text.clone()).await;

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
            process_grand_company_profile(page_text.clone().to_string())
                .await,
        )
        .bio(process_bio(page_text.clone().to_string()).await)
        .deity(deity.await.clone())
        .url(url.to_string())
        .build()
        .unwrap()
}

pub(crate) async fn process_bio(raw_text: String) -> Vec<String> {
    let bio_selector = build_selector(selectors::PROFILE_BIO_SELECTOR);
    let bio = get_value(&bio_selector, raw_text);
    let bio_int = bio.await;

    let bio_split = bio_int.split("<br>");
    let mut post_bio_split: Vec<String> = vec![];
    for s in bio_split {
        post_bio_split.push(String::from(s));
    }
    post_bio_split
}

pub(crate) async fn process_race_clan_gender(raw_text: String) -> (String, String, String) {
    let rcg_selector = build_selector(selectors::PROFILE_RACE_CLAN_GENDER_SELECTOR);
    let rcg_regex = Regex::new(selectors::PROFILE_RACE_CLAN_GENDER_REGEX);

    let race_clan_gender = get_value(&rcg_selector, raw_text.clone());
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

pub(crate) async fn process_grand_company_profile(raw_string: String) -> HashMap<String, String> {
    let gc_n_selector = build_selector(selectors::PROFILE_GC_NAME_SELECTOR);
    let gc_raw = get_value(&gc_n_selector, raw_string).await;
    let gc_regex = Regex::new(selectors::PROFILE_GC_REGEX)
        .unwrap()
        .captures(gc_raw.as_str());
    let gc_result = gc_regex.unwrap();
    let gc_name = gc_result.get(1).map_or("", |m| m.as_str());
    let gc_rank = gc_result.get(2).map_or("", |m| m.as_str());

    [
        ("name".to_string(), gc_name.to_string()),
        ("rank".to_string(), gc_rank.to_string()),
    ]
    .iter()
    .cloned()
    .collect()
}