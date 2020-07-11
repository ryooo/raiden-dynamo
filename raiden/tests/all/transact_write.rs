#[cfg(test)]
mod tests {

    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use raiden::*;

    #[derive(Raiden)]
    #[raiden(table_name = "user")]
    #[derive(Debug, Clone)]
    pub struct User {
        #[raiden(partition_key)]
        id: String,
        name: String,
    }

    #[test]
    fn test_minimum_transact_write() {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        async fn example() {
            let tx = ::raiden::WriteTx::new(Region::Custom {
                endpoint: "http://localhost:8000".into(),
                name: "ap-northeast-1".into(),
            });
            let cond = User::condition().attr_not_exists(User::id());
            let input = User::put_item_builder()
                .id("testId".to_owned())
                .name("bokuweb".to_owned())
                .build()
                .unwrap();
            let input2 = User::put_item_builder()
                .id("testId2".to_owned())
                .name("bokuweb".to_owned())
                .build()
                .unwrap();
            assert_eq!(
                tx.put(User::put(input).condition(cond))
                    .put(User::put(input2))
                    .run()
                    .await
                    .is_ok(),
                true,
            )
        }
        rt.block_on(example());
    }

    #[test]
    fn test_transact_write_put_and_update() {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        async fn example() {
            let tx = ::raiden::WriteTx::new(Region::Custom {
                endpoint: "http://localhost:8000".into(),
                name: "ap-northeast-1".into(),
            });
            let input = User::put_item_builder()
                .id("testId".to_owned())
                .name("bokuweb".to_owned())
                .build()
                .unwrap();
            let set_expression = User::update_expression()
                .set(User::name())
                .value("updated!!");

            let res = tx
                .put(User::put(input))
                .update(User::update("testId2").set(set_expression))
                .run()
                .await;
            assert_eq!(res.is_ok(), true);
        }
        rt.block_on(example());
    }

    #[test]
    fn test_transact_write_with_prefix_suffix() {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        async fn example() {
            let tx = ::raiden::WriteTx::new(Region::Custom {
                endpoint: "http://localhost:8000".into(),
                name: "ap-northeast-1".into(),
            });
            let input = User::put_item_builder()
                .id("testId".to_owned())
                .name("bokuweb".to_owned())
                .build()
                .unwrap();
            assert_eq!(
                tx.put(
                    User::put(input)
                        .table_prefix("test-")
                        .table_suffix("-staging"),
                )
                .run()
                .await
                .is_ok(),
                true,
            )
        }
        rt.block_on(example());
    }
}
