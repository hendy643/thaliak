pub mod freecompany {
    use actix_web::{get, web};
    use crate::api::endpoints::err_no_id::ErrNoId;
    use crate::lodestone::Lodestone;
    use crate::lodestone::structs::freecompany::LodestoneFreeCompany;

    #[get("/freecompany/{lodestone_id}")]
    pub async fn get_free_company_by_id(
        lodestone_id: web::Path<u64>,
    ) -> Result<LodestoneFreeCompany, ErrNoId> {
        let lodestone = Lodestone::new();
        let profile = lodestone.get_free_company(lodestone_id.to_owned()).await;

        if profile.id > 0 {
            Ok(profile)
        } else {
            let response = ErrNoId {
                id: lodestone_id.to_string().parse().unwrap(),
                err: String::from("profile not found"),
            };
            Err(response)
        }
    }
}