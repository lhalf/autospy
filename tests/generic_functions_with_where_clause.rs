#[autospy::autospy]
trait MyTrait {
    fn function<T>(&self, value: T)
    where
        T: ToString + 'static;
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function(10u32);
}

#[test]
fn trait_functions_can_be_generic_with_where_clause() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(&spy);

    assert_eq!("10", spy.function.arguments.take()[0].to_string());
}
