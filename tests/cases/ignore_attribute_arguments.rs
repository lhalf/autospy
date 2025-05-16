#[autospy::autospy]
trait MyTrait {
    fn function(&self, #[ignore] ignored: &str, captured: &str);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function("ignored", "captured");
}

#[test]
fn arguments_marked_with_ignore_attribute_are_no_captured() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(
        vec!["captured".to_string()],
        spy.function.arguments.take_all()
    );
}
