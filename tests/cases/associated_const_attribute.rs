use autospy::autospy;

#[autospy]
trait TestTrait {
    #[autospy(20)]
    const VALUE: u64;
}

#[test]
fn trait_with_associated_const_uses_attribute_value() {
    assert_eq!(20, TestTraitSpy::VALUE)
}
