#[autospy::autospy]
trait MyTrait<'a> {
    fn function(&self) -> &'a u32;
}

fn use_trait<'a, T: MyTrait<'a>>(trait_object: &T) -> &'a u32 {
    trait_object.function()
}

#[test]
fn supports_returning_reference_values_with_trait_lifetime() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([&10]);

    assert_eq!(&10, use_trait(&spy));
}

#[autospy::autospy]
trait MyTrait2<'a> {
    fn function(&self) -> Result<&'a u32, ()>;
}

fn use_trait2<'a, T: MyTrait2<'a>>(trait_object: &T) -> Result<&'a u32, ()> {
    trait_object.function()
}

#[test]
fn supports_returning_type_containing_lifetime_reference_values() {
    let spy = MyTrait2Spy::default();
    spy.function.returns.set([Ok(&10)]);

    assert_eq!(Ok(&10), use_trait2(&spy));
}
