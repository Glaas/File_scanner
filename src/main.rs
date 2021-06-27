use glob::glob_with;
use glob::MatchOptions;

fn main() {
    println!("Hello, world!");
    println!("Hello, world!");

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with("C:/users/glaas/[!.]*/*.pdf", options).unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }
}

