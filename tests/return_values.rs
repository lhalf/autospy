#[autospy::autospy]
trait MyTrait {
    fn function(&self, input: u8) -> u8;
}

fn use_test_trait<T: MyTrait>(trait_object: T, input: u8) -> u8 {
    trait_object.function(input)
}

#[test]
fn returns_items_from_set_command_in_order() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([0, 1, 2]);

    assert_eq!(use_test_trait(spy.clone(), 0), 0);
    assert_eq!(use_test_trait(spy.clone(), 0), 1);
    assert_eq!(use_test_trait(spy.clone(), 0), 2);
}

#[test]
#[should_panic]
fn if_no_return_set_then_panics_when_called() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([]);
    use_test_trait(spy.clone(), 0);
}

#[test]
fn if_set_fn_called_with_fn_uses_that_function_to_create_return_values() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set_fn(|input| 2 * input);
    assert_eq!(use_test_trait(spy.clone(), 1), 2);
    assert_eq!(use_test_trait(spy.clone(), 2), 4);
    assert_eq!(use_test_trait(spy.clone(), 3), 6);
}

#[test]
fn if_set_fn_called_with_fn_mut_uses_that_function_to_create_return_values() {
    let spy = MyTraitSpy::default();
    let mut next = 0;
    spy.function.returns.set_fn(move |_| {
        next += 1;
        next
    });
    assert_eq!(use_test_trait(spy.clone(), 0), 1);
    assert_eq!(use_test_trait(spy.clone(), 0), 2);
    assert_eq!(use_test_trait(spy.clone(), 0), 3);
}
