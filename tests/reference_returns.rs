#[autospy::autospy]
trait MyTrait {
    fn function(&self) -> &str;
}

fn use_trait<T: MyTrait>(trait_object: &T) -> &str {
    trait_object.function()
}

#[test]
fn supports_reference_return_values() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set(["hello"]);

    assert_eq!("hello", use_trait(&spy));
}
