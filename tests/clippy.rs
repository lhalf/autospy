#![deny(missing_docs)]

//! example documentation
#[autospy::autospy]
trait MyTrait {
    fn function(&self);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function();
}

#[test]
fn clippy_does_not_warn_for_missing_docs_on_expanded_spy() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(spy.clone());

    assert_eq!([()], spy.function.arguments);
}
