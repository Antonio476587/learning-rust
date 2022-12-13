fn main() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{s}");

    // String's work with UTF8 (Unicode Transformation Format)
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // push_str doesn't take ownership of the value
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push just take a character
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    //  + (add) Operator
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    // format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // Slicing Bytes and Scalar Values and Grapheme Clusterrs or Strings

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s should be Зд and is {}", s)
}
