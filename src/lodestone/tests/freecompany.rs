#[cfg(test)]
mod free_company_tests {
    use crate::lodestone::Lodestone;

    #[tokio::main]
    #[test]
    async fn get_free_company_test() {
        let lodestone = Lodestone::new();
        let lodestone_fc = lodestone.get_free_company(9228860798900688657).await;

        assert_eq!(
            lodestone_fc.id, 9228860798900688657,
            "Member count does not match"
        );
        assert_eq!(
            lodestone_fc.name, "The British Empire",
            "FC name does not match"
        );
        assert_eq!(lodestone_fc.id, 9228860798900688657, "FC id does not match");
        assert_eq!(
            lodestone_fc.formed, "2022-04-13",
            "FC formed date does not match"
        );
        assert_eq!(
            lodestone_fc.grand_company, "Immortal Flames",
            "FC grand company does not match"
        );
        assert_eq!(
            lodestone_fc.estate.get("greeting").unwrap(),
            "Downstairs Tea Room for all Refreshments. Upstairs Adventurer and Merchants Guild.",
            "FC greeting does not match"
        );
        assert_eq!(
            lodestone_fc.estate.get("name").unwrap(),
            "Guild Manor",
            "FC estate name does not match"
        );
        assert_eq!(
            lodestone_fc.estate.get("plot").unwrap(),
            "Plot 35, 8 Ward, The Lavender Beds (Medium)",
            "FC estate plot does not match"
        );
        assert_eq!(lodestone_fc.tag, "«TEA»", "FC tag does not match");
        assert_eq!(lodestone_fc.rank, 30, "FC Rank does not match");
        assert_eq!(
            lodestone_fc.slogan, "The Empire on which the sun never sets",
            "FC slogan does not match"
        );

        assert_eq!(
            lodestone_fc.server.get("world").unwrap(),
            "Omega",
            "Server World does not match"
        );
        assert_eq!(
            lodestone_fc.server.get("data_centre").unwrap(),
            "Chaos",
            "Server DC does not match"
        );

        println!("lodestone_fc: {}", lodestone_fc.to_string())
    }
}
