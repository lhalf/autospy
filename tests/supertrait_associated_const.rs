#[autospy::autospy]
trait MyTrait: Supertrait {
    fn function(&self);
    autospy::supertrait! {
        trait Supertrait {
            #[autospy(1)]
            const READ_SIZE: usize;
            const DEFAULT_SIZE: usize;
            fn super_function(&self);
        }
    }
}

trait Supertrait {
    const READ_SIZE: usize;
    const DEFAULT_SIZE: usize;
    fn super_function(&self);
}

fn use_trait<T: MyTrait>(trait_object: &T) {
    trait_object.function();
    trait_object.super_function();
}

#[test]
fn supertrait_associated_consts_use_attribute_value_or_default() {
    assert_eq!(1, <MyTraitSpy as Supertrait>::READ_SIZE);
    assert_eq!(0, <MyTraitSpy as Supertrait>::DEFAULT_SIZE);

    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);
    spy.super_function.returns.set([()]);

    use_trait(&spy);
}
