#[autospy::autospy]
trait MyTrait<A, R> {
    fn function(&self, argument: A) -> R;
}

fn use_trait<T: MyTrait<u32, String>>(trait_object: T) -> String {
    trait_object.function(100u32)
}

#[test]
fn spy_object_is_generic_over_all_generics() {
    let spy = MyTraitSpy::<u32, String>::default();
    spy.function.returns.set(["hello".to_string()]);

    assert_eq!("hello", use_trait(spy.clone()));
    assert_eq!(vec![100u32], spy.function.arguments.get())
}
