// Since this is an integration test, it's a seperate crate. We need to import the
// crate we're testing.
extern crate bucketize;
use bucketize::Bucketizer;

// This isn't going to test anything more than we already do; it's just an end-to-end
// test. In a more complex crate, unit tests would be smaller in scope.

#[test]
fn complex_bucketizer() {
    let b = Bucketizer::new()
        .bucket(None, Some(-100.0), -100.0)
        .bucket(Some(-100.0), Some(0.0), -50.0)
        .bucket(Some(0.0), Some(100.0), 50.0)
        .bucket(Some(100.0), None, 100.0);

    assert_eq!(b.bucketize(37.4), Some(50.0));
}

