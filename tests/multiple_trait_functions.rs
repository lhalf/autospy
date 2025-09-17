#[autospy::autospy]
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
    spy.function_one.returns.set([()]);
    spy.function_two.returns.set([(), ()]);

    use_trait(spy.clone());

    assert_eq!(spy.function_one.arguments.take().len(), 1);
    assert_eq!(spy.function_two.arguments.take().len(), 2);
}
