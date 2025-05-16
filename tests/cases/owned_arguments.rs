#[autospy::autospy]
trait MyTrait {
    fn function(&self, argument: String);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("hello".to_string());
}

#[test]
fn owned_function_argument_captured() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_trait(spy.clone());

    assert_eq!(vec!["hello".to_string()], spy.function.arguments.take_all())
}
