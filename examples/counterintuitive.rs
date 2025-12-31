use regex_specificity::get;

fn main() {
    let string = "cat";

    let pattern1 = r".*";
    let pattern2 = r".*a.*";

    assert_eq!(
        get(string, pattern1).unwrap(),
        get(string, pattern2).unwrap()
    )
}
