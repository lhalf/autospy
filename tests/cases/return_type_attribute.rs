#[autospy::autospy]
trait MyTrait {
    #[autospy(returns = String)]
    fn function(&self) -> impl ToString;
}

fn use_trait<T: MyTrait>(trait_object: T) -> String {
    trait_object.function().to_string()
}

#[test]
fn functions_with_return_attribute_return_that_type() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back("hello".to_string());

    assert_eq!("hello", use_trait(spy));
}
