use regex_specificity::get;

fn main() {
    let string = "alice@myprovider.com";

    // You must ensure that the pattern fully matches the string before calling
    let patterns = vec![
        "alice@myprovider.com",
        "^alice@myprovider.com$",
        "alice@myprovider.co.",
        ".lice@myprovider.com",
        r"alice@myprovider\.[a-z]+",
        r"alice@myprovider\..+",
        ".*",
        r"alice@(myprovider|other)\.com",
        "",
        "^$",
    ];

    let mut results: Vec<(&str, u64)> = Vec::new();

    for pattern in patterns {
        if let Ok(score) = get(string, pattern) {
            results.push((pattern, score));
        }
    }

    results.sort_by(|(_, a), (_, b)| b.cmp(a));

    print(string, results);
}

fn print(string: &str, results: Vec<(&str, u64)>) {
    const RW: usize = 12;
    const PW: usize = 40;

    println!("Target String: '{}'", string);
    println!("┌{:─^rw$}┬{:─^pw$}┐", "─", "─", rw = RW, pw = PW);
    println!(
        "│ {:<rw$} │ {:<pw$} │",
        "Result",
        "Pattern",
        rw = RW - 2,
        pw = PW - 2
    );
    println!("├{:─^rw$}┼{:─^pw$}┤", "─", "─", rw = RW, pw = PW);
    for (pattern, result) in results {
        println!(
            "│ {:<rw$} │ {:<pw$} │",
            result,
            pattern,
            rw = RW - 2,
            pw = PW - 2
        );
    }
    println!("└{:─^rw$}┴{:─^pw$}┘", "─", "─", rw = RW, pw = PW);
}
