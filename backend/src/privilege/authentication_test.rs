#[cfg(test)]
mod auth_test {
    use crate::privilege::authentication::add_policy;

    #[tokio::test]
    async fn auth_test() {
        let p = vec![
            "jse".to_string(),
            "admin2".to_string(),
            "data".to_string(),
            "write".to_string(),
        ];

        let rs = add_policy(p).await;

        println!("{:?}", rs);
    }
}
