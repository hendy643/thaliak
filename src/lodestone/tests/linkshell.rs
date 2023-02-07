#[cfg(test)]
mod profile_tests {
    use crate::lodestone::Lodestone;

    #[tokio::main]
    #[test]
    async fn get_linkshell_test() {
        let lodestone = Lodestone::new();
        let lodestone_linkshell = lodestone.get_linkshell(10977524091770589).await;

        assert_eq!(
            lodestone_linkshell.name, "The Falling Snow",
            "linkshell name does not match"
        );
        println!("{:?}", lodestone_linkshell)
    }
}