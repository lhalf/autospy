#[autospy::autospy]
trait MyTrait {
    fn function(&self, #[nospy] ignored: &str, captured: &str);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("ignored", "captured");
}

#[test]
fn borrowed_argument_coverted_to_owned() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(
        vec!["captured".to_string()],
        spy.function.arguments.take_all()
    );
}
