#[cfg_attr(test, autospy::autospy)]
trait MyTrait {
    fn function(&self) -> String;
}

fn use_trait<T: MyTrait>(trait_object: T) -> String {
    trait_object.function()
}

#[test]
fn returns_values_in_order() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back("first_return".to_string());
    spy.function.returns.push_back("second_return".to_string());

    assert_eq!("first_return", use_trait(spy.clone()));
    assert_eq!("second_return", use_trait(spy));
}
