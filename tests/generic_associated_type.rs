#[autospy::autospy]
trait MyIterator {
    #[autospy(String)]
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

fn use_trait<T>(trait_object: &mut T) -> Option<String>
where
    T: for<'a> MyIterator<Item<'a> = String>,
{
    trait_object.next()
}

#[test]
fn trait_with_generic_associated_type_is_supported() {
    let mut spy = MyIteratorSpy::default();
    spy.next.returns.set([Some("hello".to_string())]);

    assert_eq!(Some("hello".to_string()), use_trait(&mut spy));
}
