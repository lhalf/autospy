#[autospy::autospy]
trait MyTrait {
    fn function<T: ToString + 'static>(&self, value: T);
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function(10u32);
}

#[test]
fn trait_functions_can_be_generic() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(&spy);

    assert_eq!("10", spy.function.arguments.take()[0].to_string());
}
