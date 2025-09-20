#[autospy::autospy]
trait MyTrait: Unpin {
    fn function(&self);
    autospy::supertrait! {
        trait Unpin {}
    }
}

// trait MarkerTrait {}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function();
}

#[test]
fn marker_supertraits_are_supported() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(spy);
}
