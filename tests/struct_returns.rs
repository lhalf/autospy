#[derive(PartialEq, Debug)]
struct MyStruct {
    value: String,
}

#[autospy::autospy]
trait MyTrait {
    fn function(&self) -> MyStruct;
}

fn use_trait<T: MyTrait>(trait_object: T) -> MyStruct {
    trait_object.function()
}

#[test]
fn non_clone_struct_can_be_returned_by_spy() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([MyStruct {
        value: "hello".to_string(),
    }]);

    assert_eq!(
        MyStruct {
            value: "hello".to_string()
        },
        use_trait(spy)
    )
}
