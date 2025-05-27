use autospy::autospy;

#[cfg_attr(test, autospy)]
trait MyTrait {
    fn function_one(&self);
    fn function_two(&self);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function_one();

    trait_object.function_two();
    trait_object.function_two();
}

#[test]
fn can_spy_on_how_many_times_functions_called() {
    let spy = MyTraitSpy::default();
    spy.function_one.returns.push_back_n((), 1);
    spy.function_two.returns.push_back_n((), 2);

    use_trait(spy.clone());

    assert_eq!(spy.function_one.arguments.take_all().len(), 1);
    assert_eq!(spy.function_two.arguments.take_all().len(), 2);
}
