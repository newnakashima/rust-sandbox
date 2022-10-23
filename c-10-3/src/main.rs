fn longest<'a, 'b>(x: &'a str, y: &'a str, z: &'b str) -> &'a str {
    println!("{}", z);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn elision(x: &str) -> &str {
    &x[0..1]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("{}", r);

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let string3 = "bar";

        let result = longest(string1.as_str(), string2, string3);
        println!("The longest string is {}", result);
    }
}

