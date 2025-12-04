#[autospy::autospy]
trait MyTrait: Supertrait {
    fn function(&self);
    autospy::supertrait! {
        trait Supertrait {
            fn super_function(&self, captured: String, #[autospy(ignore)] ignored: String);
        }
    }
}

trait Supertrait {
    fn super_function(&self, captured: String, ignored: String);
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function();
    trait_object.super_function("captured".to_string(), "ignored".to_string());
}

#[test]
fn supertraits_support_ignore_attribute() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);
    spy.super_function.returns.set([()]);

    use_trait(&spy);
    assert_eq!(["captured".to_string()], spy.super_function.arguments);
}
