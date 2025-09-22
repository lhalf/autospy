#[autospy::autospy]
trait MyTrait {
    fn function(&self, argument: String);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("hello".to_string());
}

#[test]
fn arguments_are_partial_eq_to_standard_types() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(spy.clone());

    assert_eq!(["hello"], spy.function.arguments);
    assert_eq!(&["hello"], spy.function.arguments);
    assert_eq!(vec!["hello"], spy.function.arguments);
    assert_eq!(["hello"].as_slice(), spy.function.arguments);

    assert_eq!(spy.function.arguments, ["hello"]);
    assert_eq!(spy.function.arguments, &["hello"]);
    assert_eq!(spy.function.arguments, vec!["hello"]);
    assert_eq!(spy.function.arguments, ["hello"].as_slice());
}
