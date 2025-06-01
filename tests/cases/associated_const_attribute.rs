use autospy::autospy;

#[autospy]
trait TestTrait {
    #[autospy(20)]
    const VALUE: u64;
    const DEFAULT_VALUE: bool;
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn trait_with_associated_const_uses_attribute_value_or_default_if_not_specified() {
    assert_eq!(20, TestTraitSpy::VALUE);
    assert!(!TestTraitSpy::DEFAULT_VALUE);
}
