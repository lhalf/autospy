#[allow(clippy::missing_safety_doc)]
#[autospy::autospy]
#[allow(unsafe_code)]
unsafe trait MyTrait {
    fn function(&self) -> u32;
}

fn use_trait<T: MyTrait>(trait_object: T) -> u32 {
    trait_object.function()
}

#[test]
fn handles_unsafe_trait() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([10]);

    assert_eq!(10, use_trait(spy));
}
