#[test]
fn test_phantom_swap_path_verification() {
    let mut verifier = SlippagePathVerifier::new();
    let path_id = "path_ghost";

    verifier.register_valid_path("path_legit");
    assert!(!verifier.verify(path_id)); // phantom path
    assert_eq!(verifier.last_event(), "PhantomPathDetected");
}
