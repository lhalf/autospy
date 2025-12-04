#[autospy::autospy]
trait MyTrait {
    fn function(&self, function: &[u8]);
}

fn use_test_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function(&[1]);
}

#[test]
fn borrowed_slice_converted_to_owned_vec() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_test_trait(&spy);

    assert_eq!([vec![1]], spy.function.arguments);
}
