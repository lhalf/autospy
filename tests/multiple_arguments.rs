#[autospy::autospy]
trait MyTrait {
    fn function(&self, arg1: &str, arg2: u32) -> usize;
}

fn use_trait<T: MyTrait>(trait_object: T) -> bool {
    for _ in 0..100 {
        trait_object.function("hello", 10);
    }
    true
}

#[test]
fn multiple_arguments_are_captured_in_spy() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set_fn(|(arg1, _)| arg1.len());

    use_trait(spy.clone());

    assert_eq!(100, spy.function.arguments.get().len())
}
