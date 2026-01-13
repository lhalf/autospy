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

#[autospy::autospy]
trait MyTrait2 {
    fn function(&self) -> Result<&str, ()>;
}

fn use_trait2<T: MyTrait2>(trait_object: &T) -> Result<&str, ()> {
    trait_object.function()
}

#[test]
fn supports_return_value_containing_reference() {
    let spy = MyTrait2Spy::default();
    spy.function.returns.set([Ok("hello")]);

    assert_eq!(Ok("hello"), use_trait2(&spy));
}
