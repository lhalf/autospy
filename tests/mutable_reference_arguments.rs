#[autospy::autospy]
trait MyTrait {
    fn function(&self, argument: &mut u8);
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function(&mut 10);
}

#[test]
fn supports_mutable_reference_arguments() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(&spy);

    assert_eq!([10], spy.function.arguments);
}
