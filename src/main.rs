use std::fmt;

fn main() {
    println!("# Hello, World! 🌍");

    let i_am = Person {
        name: "Ruud",
        profession: "software engineer",
        from: "Nĳmegen, the Netherlands",
        home_page: "https://www.ruud.online",
    };

    println!();
    println!("{}", i_am);
}

struct Person<'a> {
    name: &'a str,
    profession: &'a str,
    from: &'a str,
    home_page: &'a str,
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "My name is {} and I am a {} from {}. To learn more, please visit [my homepage]({}).",
            self.name, self.profession, self.from, self.home_page,
        )
    }
}
