#[allow(clippy::needless_lifetimes)]
#[autospy::autospy]
trait MyTrait {
    fn function<'a>(&self, value: &'a str) -> u8;
}

fn use_trait<T: MyTrait>(trait_object: &T) -> u8 {
    trait_object.function("hello")
}

#[test]
fn trait_functions_can_have_lifetimes() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([10]);

    assert_eq!(10, use_trait(&spy));

    assert_eq!(["hello"], spy.function.arguments);
}
