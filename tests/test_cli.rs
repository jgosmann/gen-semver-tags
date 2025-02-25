#[test]
fn test_cli() {
    trycmd::TestCases::new()
        .case("tests/cli/*.md")
        .case("tests/cli/*.toml")
        .run();
}
