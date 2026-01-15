#[autospy::autospy]
trait MyIteratorOwned {
    #[autospy(String)]
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

fn use_trait_one<T>(trait_object: &mut T) -> Option<String>
where
    T: for<'a> MyIteratorOwned<Item<'a> = String>,
{
    trait_object.next()
}

#[test]
fn trait_with_generic_owned_associated_type_is_supported() {
    let mut spy = MyIteratorOwnedSpy::default();
    spy.next.returns.set([Some("hello".to_string())]);

    assert_eq!(Some("hello".to_string()), use_trait_one(&mut spy));
}

#[autospy::autospy]
trait MyIteratorReference {
    #[autospy(&'a str)]
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

fn use_trait_two<T>(trait_object: &mut T) -> Option<&str>
where
    T: for<'a> MyIteratorReference<Item<'a> = &'a str>,
{
    trait_object.next()
}

#[test]
fn trait_with_generic_reference_associated_type_is_supported() {
    let mut spy = MyIteratorReferenceSpy::default();
    spy.next.returns.set([Some("hello")]);

    assert_eq!(Some("hello"), use_trait_two(&mut spy));
}

#[autospy::autospy]
trait LendingIterator {
    #[autospy(&'a str)]
    type Item<'a>
    where
        Self: 'a;

    #[allow(clippy::elidable_lifetime_names)]
    fn next<'a>(&'a mut self) -> Self::Item<'a>;
}

fn use_trait<T>(lending_iterator: &mut T) -> &str
where
    T: for<'a> LendingIterator<Item<'a> = &'a str>,
{
    lending_iterator.next()
}

#[test]
fn trait_with_lending_generic_reference_associated_type_is_supported() {
    let mut spy = LendingIteratorSpy::default();
    spy.next.returns.set(["hello!"]);

    assert_eq!("hello!", use_trait(&mut spy));
}
