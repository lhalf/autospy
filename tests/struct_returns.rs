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
    );
}

struct NonDebugStruct {
    value: String,
}

#[autospy::autospy]
trait MyTrait2 {
    fn function2(&self) -> NonDebugStruct;
}

fn use_trait2<T: MyTrait2>(trait_object: T) -> NonDebugStruct {
    trait_object.function2()
}

#[test]
fn non_debug_struct_can_be_returned_by_spy() {
    let spy = MyTrait2Spy::default();
    spy.function2.returns.set([NonDebugStruct {
        value: "hello".to_string(),
    }]);

    assert_eq!("hello", use_trait2(spy).value);
}
