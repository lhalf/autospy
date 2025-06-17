use autospy::autospy;

#[autospy]
trait MyTrait<T> where T: Copy {
    fn function(&self) -> T;
}

fn use_trait<T: MyTrait<u32>>(trait_object: T) -> u32 {
    trait_object.function()
}

#[test]
fn spy_object_is_generic_with_where_clause() {
    let spy = MyTraitSpy::<u32>::default();
    spy.function.returns.push_back(10u32);

    assert_eq!(10u32, use_trait(spy));
}
