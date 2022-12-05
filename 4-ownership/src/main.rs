fn main() {
    memory();
    ownership();
    reference();
    reference_to_nothing();
    get_first_word();
}

fn memory() {
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");

    // let s2 = s1; // Move (s1 is no longer valid)
    let s2 = s1.clone(); // Deep copy (s1 is still valid)

    print!("{}, world!", s1);
}

fn ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    print!("{}", s); // Error: s is no longer valid"
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2); // Error: cannot borrow `s` as mutable more than once at a time
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

fn change(s: &mut String) -> usize { // s is a mutable reference to a String
    s.push_str(", world!");
    s.len()
}

fn reference_to_nothing() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
}

fn get_first_word() {
    let s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s);
    // let word = first_word(&s2);
    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
