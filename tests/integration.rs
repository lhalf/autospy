use autospy::autospy;

<<<<<<< HEAD
#[autospy]
trait SimpleTrait {
    fn function(&self, argument: String);
}

// expected generated code
// trait SimpleTrait {
//     fn function(&self, argument: String);
// }

// struct SimpleTraitSpy {
//     pub function: SpyFunction<String>
// }

// impl SimpleTrait for SimpleTraitSpy {
//     fn function(&self, argument: String) {
//         self.function.spy(argument)
//     }
// }


#[cfg(test)]
mod tests {
    use super::{SimpleTrait, SimpleTraitSpy};

    #[test]
    fn single_argument_sync_trait() {
        fn use_simple_trait<S: SimpleTrait>(trait_object: S) {
            trait_object.function("hello".to_string());
        }

        let spy = SimpleTraitSpy::default();

        use_simple_trait(spy);

        assert_eq!(vec!["hello".to_string()], spy.function.arguments.take_all())
    }
}
=======
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
>>>>>>> 18a4225 (refactor into autospy_macro and autospy)
