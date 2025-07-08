use autospy::autospy;

#[autospy]
trait MyTrait {
    unsafe fn function(&self) -> u32;
}

fn use_trait<T: MyTrait>(trait_object: T) -> u32 {
    unsafe { trait_object.function() }
}

#[test]
fn handles_unsafe_trait_functions() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(10);

    assert_eq!(10, use_trait(spy));
}
