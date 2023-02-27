#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, http::{self, header::ContentType}, test, web};
    use crate::api::endpoints::profile::profile::get_profile_by_id;

    #[actix_web::test]
    async fn test_index_ok() {
        let mut app = test::init_service(App::new())
            
            .route("/profile/{id}", web::get().to(get_profile_by_id)
        ).await;

        let req = test::TestRequest::get()
            .uri("/profile/21568996")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let app = test::init_service(App::new()
            .route("/profile/0",
                   web::get()
                       .to(get_profile_by_id))
        ).await;

        let req = test::TestRequest::get()
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}