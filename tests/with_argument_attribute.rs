use std::string::FromUtf8Error;

#[autospy::autospy]
trait MyTrait {
    fn function(
        &self,
        #[autospy(into = "Result<String, FromUtf8Error>", with = "String::from_utf8")] bytes: Vec<
            u8,
        >,
    );
}

fn use_trait<T: MyTrait>(trait_object: T) {
    trait_object.function(b"hello world!".to_vec());
}

#[test]
fn functions_with_into_with_attribute_return_that_type() {
    let spy = MyTraitSpy::default();
    spy.function.returns.set([()]);

    use_trait(spy.clone());

    assert_eq!(
        vec![Ok(String::from("hello world!"))],
        spy.function.arguments.get()
    );
}
