#[cfg(test)]
mod profile_tests {
    use crate::lodestone::Lodestone;

    #[tokio::main]
    #[test]
    async fn get_character_test() {
        let lodestone = Lodestone::new();
        let lodestone_profile = lodestone.get_profile(21568996).await;
        assert_eq!(
            lodestone_profile.name,
            "Wompus Senekane",
            "profile name does not match"
        );
        assert_eq!(
            lodestone_profile.id,
            21568996,
            "profile id does not match"
        );
        assert_eq!(
            lodestone_profile.clan,
            "Seeker of the Sun",
            "profile clan does not match"
        );
        assert_eq!(
            lodestone_profile.free_company,
            "The British Empire",
            "free company does not match"
        );

        println!("{}", lodestone_profile);
    }
}