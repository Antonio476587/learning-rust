const TWELVE_ORDINAL_NUMBERS: [&str; 12] = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"];

const TWELVE_VERSES: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    println!("It's time to sing 🎶");

    let mut ordinal_number_count = 0;

    for ordinal_number in TWELVE_ORDINAL_NUMBERS {

        ordinal_number_count = ordinal_number_count + 1;

        let mut verse_count = ordinal_number_count;
        
        // I realized that I had to write the ordinal numbers in lowercase, but there is a function for it.
        println!("\nOn the {} day of Christmas my true love sent to me", ordinal_number.to_lowercase());
        
        while verse_count > 0 {

            if ordinal_number_count > 1 &&  verse_count == 1 {
                println!("And {} 🎵", TWELVE_VERSES[verse_count - 1].to_lowercase());
            } else if ordinal_number_count == 1 {
                println!("{} 🎵", TWELVE_VERSES[verse_count - 1]);
            } else {
                println!("{} 🎵,", TWELVE_VERSES[verse_count - 1]);
            }

            verse_count = verse_count - 1;
        }

    }
}


