
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}



fn main() {
    let x = 5;

    let s: &'static str = "s is static lifetime";

    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string: {}", result);

    let novel = String::from("starwars, a long long time ago..");
    let first_sentence = novel.split(',')
        .next()
        .expect("can not find the symbol ','");
    let i = ImportantExcerpt { part: first_sentence };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
