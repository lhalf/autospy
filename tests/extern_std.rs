#![no_std]
extern crate std;

#[autospy::autospy]
trait MyTrait {
    fn function(&self) -> &str;
}

fn use_trait<T: MyTrait>(trait_object: &T) -> &str {
    trait_object.function()
}

#[test]
fn supports_when_std_is_an_extern_crate() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set(["hello"]);

    assert_eq!("hello", use_trait(&spy));
}
