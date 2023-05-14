struct CensoredString {
    censored_string: String,
    censored_vowels: String,
}

impl CensoredString {
    fn uncensor(&self) -> String {
        let mut uncensored_string = String::new();
        let mut vowel_index = 0;

        for c in self.censored_string.chars() {
            if c == '*' {
                uncensored_string.push(self.censored_vowels.chars().nth(vowel_index).unwrap());
                vowel_index += 1;
            } else {
                uncensored_string.push(c);
            }
        }

        uncensored_string
    }
}

fn main() {
    let censored = CensoredString {
        censored_string: String::from("Wh*r* d*d my v*w*ls g*?"),
        censored_vowels: String::from("eeioeo"),
    };
    let uncensored = censored.uncensor();
    println!("{}", uncensored); // Output: "Where did my vowels go?"

    let censored2 = CensoredString {
        censored_string: String::from("abcd"),
        censored_vowels: String::from(""),
    };
    let uncensored2 = censored2.uncensor();
    println!("{}", uncensored2); // Output: "abcd"

    let censored3 = CensoredString {
        censored_string: String::from("*PP*RC*S*"),
        censored_vowels: String::from("UEAE"),
    };
    let uncensored3 = censored3.uncensor();
    println!("{}", uncensored3); // Output: "UPPERCASE"
}
