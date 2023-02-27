pub mod linkshell {
    use actix_web::{get, web};
    use crate::api::endpoints::err_no_id::ErrNoId;
    use crate::lodestone::Lodestone;
    use crate::lodestone::structs::linkshell::LodestoneLinkShell;

    #[get("/linkshell/{lodestone_id}")]
    pub async fn get_linkshell_by_id(
        lodestone_id: web::Path<u64>,
    ) -> Result<LodestoneLinkShell, ErrNoId> {
        let lodestone = Lodestone::new();
        let linkshell = lodestone
            .get_linkshell(lodestone_id.to_string().parse().unwrap())
            .await;

        if linkshell.id > 0 {
            Ok(linkshell)
        } else {
            let response = ErrNoId {
                id: lodestone_id.to_string().parse().unwrap(),
                err: String::from("linkshell not found"),
            };
            Err(response)
        }
    }
}