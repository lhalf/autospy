#[cfg_attr(test, autospy::autospy)]
trait MyTrait {
    fn function(&self, argument: impl ToString + 'static);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("hello");
}

#[test]
fn trait_argument_captured_in_box() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_trait(spy.clone());

    assert_eq!("hello", spy.function.arguments.take_all()[0].to_string())
}
