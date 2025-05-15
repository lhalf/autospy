mod cases;

use autospy::autospy;

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
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(vec!["hello".to_string()], spy.function.arguments.take_all())
}

#[test]
fn single_static_impl_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument: impl ToString + 'static);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function("hello");
    }

    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!("hello", spy.function.arguments.take_all()[0].to_string())
}

#[test]
fn mutliple_owned_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument1: String, argument2: String);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function("hello1".to_string(), "hello2".to_string());
    }

    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(
        vec![("hello1".to_string(), "hello2".to_string())],
        spy.function.arguments.take_all()
    )
}

#[test]
fn multiple_borrowed_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument1: &str, argument2: &str);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function("hello1", "hello2");
    }

    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(
        vec![("hello1".to_string(), "hello2".to_string())],
        spy.function.arguments.take_all()
    )
}

#[test]
fn multiple_multiple_reference_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument1: &&&str, argument2: &&&&str);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function(&&"hello1", &&&"hello2");
    }

    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(
        vec![("hello1".to_string(), "hello2".to_string())],
        spy.function.arguments.take_all()
    )
}

#[test]
fn mutliple_owned_and_static_impl_argument_sync_trait() {
    #[autospy]
    trait TestTrait {
        fn function(&self, argument1: String, argument2: impl ToString + 'static);
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) {
        trait_object.function("hello1".to_string(), "hello2");
    }

    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    let arguments = &spy.function.arguments.take_all()[0];

    assert_eq!("hello1", arguments.0);
    assert_eq!("hello2", arguments.1.to_string())
}

#[test]
fn no_arguments_sync_trait_returning_u8() {
    #[autospy]
    trait TestTrait {
        fn function(&self) -> u8;
    }

    fn use_test_trait<T: TestTrait>(trait_object: T) -> u8 {
        trait_object.function()
    }

    let spy = TestTraitSpy::default();
    spy.function.returns.push_back(0);
    spy.function.returns.push_back(1);

    assert_eq!(use_test_trait(spy.clone()), 0);
    assert_eq!(use_test_trait(spy.clone()), 1);

    assert_eq!(vec![(), ()], spy.function.arguments.take_all())
}
