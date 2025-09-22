#[autospy::autospy]
trait MyTrait {
    fn function(&self, argument: &str);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("hello");
}

#[test]
fn borrowed_argument_coverted_to_owned() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_test_trait(spy.clone());

    assert_eq!(["hello".to_string()], spy.function.arguments)
}
