#[autospy::autospy(Item = String)]
trait TestTrait {
    type Item;
    fn function(&self, argument: Self::Item);
}

fn use_trait<T: TestTrait<Item = String>>(trait_object: T) {
    trait_object.function("hello".to_string())
}

#[test]
fn trait_with_associated_type_has_attribute_type_captured() {
    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(());

    use_trait(spy.clone());

    assert_eq!(vec!["hello"], spy.function.arguments.take_all());
}
