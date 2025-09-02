#[autospy::autospy]
trait MyTrait {
    fn function(&self, function: fn() -> u8);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function(eight);
}

fn eight() -> u8 {
    8
}

#[test]
fn borrowed_argument_coverted_to_owned() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_test_trait(spy.clone());

    assert_eq!(
        eight as *const (),
        spy.function.arguments.get()[0] as *const ()
    )
}
