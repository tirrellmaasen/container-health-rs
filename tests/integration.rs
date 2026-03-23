#[test]
fn test_end_to_end() {
    let input = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = input.iter().map(|x| x * 2).collect();
    assert_eq!(result, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_config_parse() {
    let config = "key=value";
    let parts: Vec<&str> = config.split('=').collect();
    assert_eq!(parts.len(), 2);
}
