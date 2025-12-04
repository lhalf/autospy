#[autospy::autospy]
trait MyTrait {
    fn function<A: ToString + 'static, B: std::io::Read + 'static>(&self, arg1: A, arg2: B);
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function(10u32, b"hello".as_ref());
}

#[test]
fn trait_functions_can_have_multiple_generics() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(&spy);

    let mut arguments = spy.function.arguments.take();

    assert_eq!("10", arguments[0].0.to_string());

    let mut buffer = String::new();
    arguments[0].1.read_to_string(&mut buffer).unwrap();
    assert_eq!("hello", buffer);
}
