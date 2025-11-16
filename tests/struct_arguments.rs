#[derive(PartialEq, Debug)]
struct MyStruct {
    value: String,
}

#[autospy::autospy]
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
    spy.function.returns.set([()]);

    use_trait(spy.clone());

    assert_eq!(
        [MyStruct {
            value: "hello".to_string()
        }],
        spy.function.arguments
    )
}

struct NonDebugStruct {
    value: String,
}

#[autospy::autospy]
trait MyTrait2 {
    fn function(&self, argument: NonDebugStruct);
}

fn use_trait2<T: MyTrait2>(trait_object: T) {
    trait_object.function(NonDebugStruct {
        value: "hello".to_string(),
    });
}

#[test]
fn non_debug_struct_function_argument_captured() {
    let spy = MyTrait2Spy::default();
    spy.function.returns.set([()]);

    use_trait2(spy.clone());

    assert_eq!("hello", spy.function.arguments.get()[0].value)
}
