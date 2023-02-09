#[cfg(test)]
mod profile_tests {
    use crate::lodestone::Lodestone;

    #[tokio::main]
    #[test]
    async fn get_character_test() {
        let lodestone = Lodestone::new();

        let lodestone_profile = lodestone.get_profile(21568996).await;
        /*
                pub id: u64,
        pub name: String,
        pub nameday: String,
        pub race: String,
        pub clan: String,
        pub gender: String,
        pub title: String,
        pub free_company: String,
        pub grand_company: HashMap<String, String>,
        pub bio: Vec<String>,
        pub deity: String,
             */
        assert_eq!(lodestone_profile.id, 21568996, "profile id does not match");
        assert_eq!(
            lodestone_profile.name, "Wompus Senekane",
            "profile name does not match"
        );
        assert_eq!(
            lodestone_profile.nameday, "10th Sun of the 5th Umbral Moon",
            "profile nameday does not match"
        );
        assert_eq!(
            lodestone_profile.race, "Miqo'te",
            "profile race does not match"
        );
        assert_eq!(
            lodestone_profile.clan, "Seeker of the Sun",
            "profile clan does not match"
        );
        assert_eq!(lodestone_profile.gender, "♀", "profile gender does not match");
        assert_eq!(
            lodestone_profile.free_company, "The British Empire",
            "free company does not match"
        );
        assert_eq!(
            lodestone_profile.grand_company.get("name").unwrap(),
            "Immortal Flames",
            "Grand Company name does not match"
        );
        assert_eq!(
            lodestone_profile.grand_company.get("rank").unwrap(),
            "Flame Captain",
            "Grand Company rank does not match"
        );
        assert_eq!(lodestone_profile.deity, "Menphina, the Lover", "deity does not match");
        println!("lodestone_profile: {}", lodestone_profile)
    }
}
