use async_trait::async_trait;
use autospy::autospy;

#[autospy]
#[async_trait]
trait MyTrait: Send + 'static {
    async fn function(&self, argument: String);
}

async fn use_trait<T: MyTrait>(trait_object: T) {
    tokio::task::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        trait_object.function("argument".to_string()).await;
    });
}

#[tokio::test]
async fn async_function_argument_captured_and_can_be_taken_with_timeout() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_trait(spy.clone()).await;

    assert_eq!(
        vec!["argument".to_string()],
        spy.function
            .arguments
            .take_all_with_timeout(std::time::Duration::from_millis(200))
            .await
            .unwrap()
    )
}
