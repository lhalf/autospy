#[autospy::autospy]
trait MyTrait: Supertrait {
    fn function(&self);
    autospy::supertrait! {
        trait Supertrait {
            #[autospy(String)]
            type Item;
            fn super_function(&self) -> Self::Item;
        }
    }
}

trait Supertrait {
    type Item;
    fn super_function(&self) -> Self::Item;
}

fn use_trait<T: MyTrait>(trait_object: &T) -> <T as Supertrait>::Item {
    trait_object.function();
    trait_object.super_function()
}

#[test]
fn supertrait_associated_type_uses_attribute_type() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);
    spy.super_function.returns.set(["hello".to_string()]);

    assert_eq!("hello", use_trait(&spy));
}
