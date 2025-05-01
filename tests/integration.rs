use autospy::autospy;

#[autospy]
trait SimpleTrait {
    fn function(&self, argument: String);
}

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
