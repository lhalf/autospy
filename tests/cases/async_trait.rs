use async_trait::async_trait;
use autospy::autospy;

#[autospy]
#[async_trait]
trait MyTrait {
    async fn function(&self, argument: String) -> String;
}

async fn use_trait<T: MyTrait>(trait_object: T) -> String {
    trait_object.function("argument".to_string()).await
}

#[tokio::test]
async fn async_function_argument_captured_and_return_value_returned() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back("return value".to_string());

    assert_eq!("return value", use_trait(spy.clone()).await);
    assert_eq!(
        vec!["argument".to_string()],
        spy.function.arguments.take_all()
    )
}
