use autospy::autospy;

#[autospy]
trait MyTrait {
    fn function(&self, function: &[u8]);
}

fn use_test_trait<T: MyTrait>(trait_object: T) {
    trait_object.function(&[1]);
}

#[test]
fn borrowed_slice_converted_to_owned_vec() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_test_trait(spy.clone());

    assert_eq!(vec![vec![1]], spy.function.arguments.take_all())
}
