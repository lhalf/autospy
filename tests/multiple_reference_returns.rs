#[derive(PartialEq, Debug)]
struct MyStruct {
    value: String,
}

#[autospy::autospy]
trait MyTrait<'a> {
    fn function_one(&self) -> &MyStruct;
    fn function_two(&self) -> &'a str;
    fn function_three(&self) -> &str;
}

fn use_trait<'a, T: MyTrait<'a>>(trait_object: &T) -> (&MyStruct, &'a str, &str) {
    (
        trait_object.function_one(),
        trait_object.function_two(),
        trait_object.function_three(),
    )
}

#[test]
fn supports_multiple_functions_returning_references_with_different_lifetimes() {
    let return_struct = MyStruct {
        value: "hello".to_string(),
    };

    let spy = MyTraitSpy::default();
    spy.function_one.returns.set([&return_struct]);
    spy.function_two.returns.set(["hello"]);
    spy.function_three.returns.set(["there"]);

    assert_eq!(
        (
            &MyStruct {
                value: "hello".to_string()
            },
            "hello",
            "there"
        ),
        use_trait(&spy)
    );
}
