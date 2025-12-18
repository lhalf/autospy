#[derive(PartialEq, Debug)]
struct MyStruct {
    value: String,
}

#[autospy::autospy]
trait MyTrait<'a> {
    fn one(&self) -> &MyStruct;
    fn two(&self) -> &'a str;
    fn three(&self) -> &str;
}

fn use_trait<'a, T: MyTrait<'a>>(trait_object: &T) -> (&MyStruct, &'a str, &str) {
    (trait_object.one(), trait_object.two(), trait_object.three())
}

#[test]
fn supports_multiple_functions_returning_references_with_different_lifetimes() {
    let return_struct = MyStruct {
        value: "hello".to_string(),
    };

    let spy = MyTraitSpy::default();
    spy.one.returns.set([&return_struct]);
    spy.two.returns.set(["hello"]);
    spy.three.returns.set(["there"]);

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
