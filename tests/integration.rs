use autospy::autospy;

#[test]
fn single_owned_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument: String);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function("hello".to_string());
    }

    let spy = TestTraitSpy::default();

    use_test_trait(spy.clone());

    assert_eq!(vec!["hello".to_string()], spy.function.arguments.take_all())
}

#[test]
fn single_borrowed_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument: &str);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function("hello");
    }

    let spy = TestTraitSpy::default();

    use_test_trait(spy.clone());

    assert_eq!(vec!["hello".to_string()], spy.function.arguments.take_all())
}

#[test]
fn single_multiple_reference_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument: &&&str);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function(&&"hello");
    }

    let spy = TestTraitSpy::default();

    use_test_trait(spy.clone());

    assert_eq!(vec!["hello".to_string()], spy.function.arguments.take_all())
}
