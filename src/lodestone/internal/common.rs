use scraper::{Html, Selector};


pub(crate) fn url_builder(item_type: &str, id: &str) -> String {
    let mut base_url = String::from("https://eu.finalfantasyxiv.com/lodestone/");
    base_url.push_str(item_type);
    base_url.push_str("/");
    base_url.push_str(id.to_string().as_str());
    return base_url;
}
pub(crate) async fn get_from_lodestone(url: &str) -> String {
    get_page(url.clone()).await
}

pub(crate) async fn get_page(url: &str) -> String {
    let resp_fut = reqwest::get(url);
    let response = resp_fut.await.unwrap();
    if response.status().is_success() {
        response.text().await.unwrap()
    } else {
        String::new()
    }
}

pub(crate) async fn get_value(selector: &Selector, body: String) -> String {
    let htm = body;
    let mut element: String = String::new();
    for elem in Html::parse_document(htm.as_str()).select(&selector) {
        element = elem.inner_html();
        break;
    }
    element
}

pub(crate) fn build_selector(css_selector: &str) -> Selector {
    Selector::parse(css_selector).unwrap()
}