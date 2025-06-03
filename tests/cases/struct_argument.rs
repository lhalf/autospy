use autospy::autospy;

#[derive(PartialEq, Debug)]
struct MyStruct {
    value: String,
}

#[autospy]
trait MyTrait {
    fn function(&self, argument: MyStruct);
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function(MyStruct {
        value: "hello".to_string(),
    });
}

#[test]
fn non_clone_struct_function_argument_captured() {
    let spy = MyTraitSpy::default();
    spy.function.returns.push_back(());

    use_trait(spy.clone());

    assert_eq!(
        vec![MyStruct {
            value: "hello".to_string()
        }],
        spy.function.arguments.take_all()
    )
}
