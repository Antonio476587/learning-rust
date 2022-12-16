const LIST_OF_CONSONANTS: [char; 21] = [
    'B', 'C', 'D', 'F', 'G', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'S', 'T', 'V', 'X', 'Z', 'H', 'R',
    'W', 'Y',
];

fn main() {
    let famous_qoute = String::from("Success is the ability to go from one mistake to another without losing enthusiasm by Winston Churchill");

    let mut famous_quote_in_pig_latin: String = String::new(); 

    for string in famous_qoute.split_whitespace() {
        if let Some(bool) = is_first_letter_consonant(&string) {
            famous_quote_in_pig_latin.push_str(convert_to_pig_latin(&string, bool).as_str());
            famous_quote_in_pig_latin.push(' ');
        }
    }

    println!("{}{}", famous_quote_in_pig_latin.get(0..1).unwrap().to_uppercase(), famous_quote_in_pig_latin.get(1..).unwrap());
}

fn is_first_letter_consonant(s: &str) -> Option<bool> {
    let mut s_iterator = s.trim().chars();

    let mut list_of_consonants_min: Vec<char> = Vec::new();
    for c in LIST_OF_CONSONANTS {
        list_of_consonants_min.push(c.to_lowercase().next().unwrap());
    }

    if let Some(char) =  s_iterator.next() {
        for c in LIST_OF_CONSONANTS {
            if c == char {
                return Some(true);
            }
        }
        for c in list_of_consonants_min {
            if c == char {
                return Some(true);
            }
        }
        return Some(false);
    }  else {
        return None
    }
}

fn convert_to_pig_latin(s: &str, b: bool) -> String {
    let mut string: String = String::new();

    if b {
        string.push_str(s.get(1..).unwrap());
        string.push_str("-");
        string.push_str(&s.get(0..1).unwrap().to_lowercase());
        string.push_str("ay");
    } else {
        string.push_str(s);
        string.push_str("-hay");
    }

    string
}