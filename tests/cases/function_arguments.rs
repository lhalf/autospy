#[cfg_attr(test, autospy::autospy)]
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
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(
        eight as *const (),
        spy.function.arguments.take_all()[0] as *const ()
    )
}
