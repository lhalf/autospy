use std::net::IpAddr;

#[autospy::autospy]
trait MyTrait {
    fn function(&self, #[autospy(into=IpAddr)] ip: [u8; 4]);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function([0, 0, 0, 0]);
}

#[test]
fn functions_with_return_attribute_return_that_type() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_trait(spy.clone());

    assert_eq!(
        vec![IpAddr::from([0, 0, 0, 0])],
        spy.function.arguments.take_all()
    );
}
