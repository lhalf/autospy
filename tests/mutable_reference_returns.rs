#[autospy::autospy]
trait MyTrait {
    fn function(&mut self) -> &mut u8;
}

fn use_trait<T: MyTrait>(trait_object: &mut T) -> &mut u8 {
    trait_object.function()
}

#[test]
fn supports_mutable_reference_return_values() {
    let mut return_value = 10;
    let mut spy = MyTraitSpy::default();
    spy.function.returns.set([&mut return_value]);

    assert_eq!(11, *use_trait(&mut spy) + 1);
}
