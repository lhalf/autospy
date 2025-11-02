#[autospy::autospy]
trait MyTrait {
    fn function(&self) -> &'static u32;
}

fn use_trait<T: MyTrait>(trait_object: T) -> &'static u32 {
    trait_object.function()
}

#[test]
fn supports_static_reference_return_values() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([&10]);

    assert_eq!(&10, use_trait(spy));
}
