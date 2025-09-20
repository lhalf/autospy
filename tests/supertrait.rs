#[autospy::autospy]
trait MyTrait: Supertrait {
    fn function(&self);
    autospy::supertrait! {
        trait Supertrait {
            fn super_function(&self);
        }
    }
}

trait Supertrait {
    fn super_function(&self);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function();
    trait_object.super_function();
}

#[test]
fn supertraits_are_supported() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);
    spy.super_function.returns.set([()]);

    use_trait(spy);
}
