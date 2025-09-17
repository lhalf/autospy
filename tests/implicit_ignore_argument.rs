#[autospy::autospy]
trait MyTrait {
    fn function(&self, _: &str, captured: &str);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("ignored", "captured");
}

#[test]
fn arguments_with_no_name_are_not_captured() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_test_trait(spy.clone());

    assert_eq!(vec!["captured".to_string()], spy.function.arguments.take());
}
