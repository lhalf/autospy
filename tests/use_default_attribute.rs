#[autospy::autospy]
trait TestTrait {
    #[autospy(20)]
    const VALUE: u64;
    #[autospy(use_default)]
    fn function(&self) -> u64 {
        Self::VALUE + 100
    }
}

fn use_test_trait<T: TestTrait>(trait_object: T) -> u64 {
    trait_object.function()
}

#[test]
fn function_marked_with_use_default_is_not_spied() {
    assert_eq!(120, use_test_trait(TestTraitSpy::default()));
}
