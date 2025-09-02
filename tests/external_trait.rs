use std::io::Read;

#[autospy::autospy(external)]
trait Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}

fn use_trait<R: Read>(mut trait_object: R) -> std::io::Result<usize> {
    let mut buffer = [];
    trait_object.read(&mut buffer)
}

#[test]
fn external_traits_can_be_mocked() {
    let spy = ReadSpy::default();
    spy.read
        .returns
        .set([Err(std::io::Error::other("deliberate test error"))]);

    assert!(use_trait(spy).is_err());
}
