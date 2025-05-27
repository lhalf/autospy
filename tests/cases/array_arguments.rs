use autospy::autospy;

#[cfg_attr(test, autospy)]
trait MyTrait {
    fn function(&self, function: [u8; 1]);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function([0; 1]);
}

#[test]
fn borrowed_argument_coverted_to_owned() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(vec![[0; 1]], spy.function.arguments.take_all())
}
