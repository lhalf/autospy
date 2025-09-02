#[autospy::autospy]
trait MyTrait {
    fn function(&self) -> String;
}

fn use_trait<T: MyTrait>(trait_object: T) -> String {
    trait_object.function()
}

#[test]
fn returns_owned_values() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back("hello!".to_string());

    assert_eq!("hello!", use_trait(spy));
}
