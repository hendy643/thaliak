pub(crate) mod profile {
    use actix_web::{get, web};
    use crate::api::endpoints::err_no_id::ErrNoId;
    use crate::lodestone::structs::profile::LodestoneProfile;
    use crate::lodestone::Lodestone;

    #[get("/profile/{lodestone_id}")]
    pub async fn get_profile_by_id(
        lodestone_id: web::Path<u64>,
    ) -> Result<LodestoneProfile, ErrNoId> {
        let lodestone = Lodestone::new();
        let profile = lodestone
            .get_profile(lodestone_id.to_string().parse().unwrap())
            .await;

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
