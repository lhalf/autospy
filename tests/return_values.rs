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
    assert_eq!(use_test_trait(spy, 0), 2);
}

#[test]
fn if_no_return_set_then_panics_when_called() {
    let spy = MyTraitSpy::default();
    assert_eq!(
        panic_message(|| use_test_trait(spy, 0)),
        Some("function 'function' had 0 return values set, but was called 1 time(s)".to_string())
    );
}

#[test]
#[should_panic(expected = "function 'function' had unused return values when dropped")]
fn if_a_return_set_and_not_used_then_panics_when_dropped() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([0]);
}

#[test]
fn if_set_fn_and_not_used_then_does_not_panic_when_dropped() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set_fn(|_| 0);
}

#[test]
fn if_take_used_then_panics_message_is_correct() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([0]);
    use_test_trait(spy.clone(), 0);
    spy.function.arguments.take();
    assert_eq!(
        panic_message(|| use_test_trait(spy, 0)),
        Some("function 'function' had 1 return values set, but was called 2 time(s)".to_string())
    );
}

#[test]
fn if_take_used_and_then_more_returns_set_panic_message_is_correct() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([0]);
    use_test_trait(spy.clone(), 0);
    spy.function.returns.set([0]);
    use_test_trait(spy.clone(), 0);
    spy.function.arguments.take();
    assert_eq!(
        panic_message(|| use_test_trait(spy, 0)),
        Some("function 'function' had 2 return values set, but was called 3 time(s)".to_string())
    );
}

#[test]
fn if_set_fn_called_with_fn_uses_that_function_to_create_return_values() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set_fn(|input| 2 * input);
    assert_eq!(use_test_trait(spy.clone(), 1), 2);
    assert_eq!(use_test_trait(spy.clone(), 2), 4);
    assert_eq!(use_test_trait(spy, 3), 6);
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
    assert_eq!(use_test_trait(spy, 0), 3);
}

#[test]
fn calling_set_overrides_set_fn_and_vice_versa() {
    let spy = MyTraitSpy::default();

    spy.function.returns.set([0]);
    spy.function.returns.set_fn(|_| 1);
    assert_eq!(use_test_trait(spy.clone(), 0), 1);

    spy.function.returns.set([0]);
    assert_eq!(use_test_trait(spy, 0), 0);
}

fn panic_message<F, R>(function: F) -> Option<String>
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(function)
        .err()
        .and_then(|boxed_any| boxed_any.downcast_ref::<String>().map(|s| s.to_string()))
}
