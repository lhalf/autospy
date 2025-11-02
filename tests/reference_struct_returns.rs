#[derive(PartialEq, Debug)]
struct MyStruct {
    value: String,
}

#[autospy::autospy]
trait MyTrait {
    fn function(&self) -> &MyStruct;
}

fn use_trait<T: MyTrait>(trait_object: &T) -> &MyStruct {
    trait_object.function()
}

#[test]
fn supports_returning_referenced_structs() {
    let return_struct = MyStruct {
        value: "hello".to_string(),
    };
    let spy = MyTraitSpy::default();
    spy.function.returns.set([&return_struct]);

    assert_eq!(
        &MyStruct {
            value: "hello".to_string()
        },
        use_trait(&spy)
    )
}
