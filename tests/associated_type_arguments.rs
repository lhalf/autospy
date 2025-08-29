#[autospy::autospy]
trait TestTrait {
    #[autospy(String)]
    type Argument;
    #[autospy(String)]
    type Return;
    fn function(&self, argument: Self::Argument) -> Self::Return;
}

fn use_trait<T: TestTrait<Argument = String, Return = String>>(trait_object: T) -> String {
    trait_object.function("hello".to_string())
}

#[test]
fn trait_with_associated_type_has_attribute_type_captured() {
    let spy = TestTraitSpy::default();
    spy.function.returns.push_back("world!".to_string());

    assert_eq!("world!", use_trait(spy.clone()));

    assert_eq!(vec!["hello"], spy.function.arguments.take_all());
}
