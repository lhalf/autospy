use autospy::autospy;

#[autospy]
trait TestTrait {
    #[autospy(String)]
    type Item: Clone;
    fn function(&self) -> Self::Item;
}

fn use_trait<T: TestTrait<Item = String>>(trait_object: T) -> String {
    trait_object.function()
}

#[test]
fn trait_with_associated_type_with_bounds_has_attribute_type_returned() {
    let spy = TestTraitSpy::default();
    spy.function.returns.push_back("hello".to_string());

    assert_eq!("hello", use_trait(spy));
}
