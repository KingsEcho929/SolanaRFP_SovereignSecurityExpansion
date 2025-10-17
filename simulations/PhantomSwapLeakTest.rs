use modules::SlippagePathVerifier;

#[test]
fn test_phantom_swap_path_verification() {
    let mut verifier = SlippagePathVerifier::new();
    let legit_path = "path_legit";
    let ghost_path = "path_ghost";

    verifier.register_valid_path(legit_path);
    assert!(verifier.verify_path(legit_path)); // legit path allowed
    assert!(!verifier.verify_path(ghost_path)); // phantom path blocked
    assert_eq!(verifier.last_event(), "PhantomPathDetected(path_ghost)");
}
