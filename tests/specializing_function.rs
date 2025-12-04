#[autospy::autospy]
trait MyTrait {
    fn function(&self, argument: u32)
    where
        Self: Sized;
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function(10);
}

#[test]
fn supports_specializing_function_in_trait() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(&spy);

    assert_eq!([10], spy.function.arguments);
}
