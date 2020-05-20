use async_graphql::*;

#[async_std::test]
pub async fn test_defer() {
    #[derive(Clone)]
    struct MyObj;

    #[Object]
    impl MyObj {
        async fn value(&self) -> i32 {
            20
        }

        async fn obj(&self) -> Deferred<MyObj> {
            MyObj.into()
        }
    }

    struct Query;

    #[Object]
    impl Query {
        async fn value(&self) -> Deferred<i32> {
            10.into()
        }

        async fn obj(&self) -> Deferred<MyObj> {
            MyObj.into()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let query = r#"{
        value
    }"#;
    assert_eq!(
        schema.execute(&query).await.unwrap().data,
        serde_json::json!({
            "value": 10,
        })
    );

    let query = r#"{
        value
        obj {
            value
            obj {
                value
            }
        }
    }"#;
    assert_eq!(
        schema.execute(&query).await.unwrap().data,
        serde_json::json!({
            "value": 10,
            "obj": {
                "value": 20,
                "obj": {
                    "value": 20
                }
            }
        })
    );
}
