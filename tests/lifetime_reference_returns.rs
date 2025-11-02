#[autospy::autospy]
trait MyTrait<'a> {
    fn function(&self) -> &'a u32;
}

fn use_trait<'a, T: MyTrait<'a>>(trait_object: T) -> &'a u32 {
    trait_object.function()
}

#[test]
fn supports_returning_reference_values_with_trait_lifetime() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([&10]);

    assert_eq!(&10, use_trait(spy));
}
