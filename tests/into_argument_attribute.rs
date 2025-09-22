use std::net::Ipv4Addr;

#[autospy::autospy]
trait MyTrait {
    fn function(&self, #[autospy(into = "Ipv4Addr")] ip: [u8; 4]);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function([192, 168, 1, 1]);
}

#[test]
fn functions_with_into_attribute_return_that_type() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(spy.clone());

    assert_eq!([Ipv4Addr::new(192, 168, 1, 1)], spy.function.arguments);
}
