#[autospy::autospy]
trait MyTrait<T> {
    fn function(&self) -> T;
}

fn use_trait<T: MyTrait<String>>(trait_object: &T) -> String {
    trait_object.function()
}

#[test]
fn spy_object_is_generic() {
    let spy = MyTraitSpy::<String>::default();
    spy.function.returns.set(["hello".to_string()]);

    assert_eq!("hello", use_trait(&spy));
}
