fn main() {

    let s = String::from("hello, world");

    //first_word(&s);

    // these are string slices
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // this is the same as above
    let _hello = &s[..5]; 
    let _world = &s[6..];

    // this gets the whole string
    let _all = &s[..];

    let _first = even_better_first_word(&s);

    let mut t = String::from("hello, kitty");

    let word = even_better_first_word(&t);

    //t.clear(); // this will throw an error when combined with the 
                 // print statement below because clear() actually
                 // takes a mutable reference and print is an 
                 // immutable reference

    println!("the first word is: {}", word);

    // a string literal is also a slice;
    // the type of this is &str;
    // &str types are immutable references
    let string = "Hello, world";

    let string_on_heap = String::from(string);

    // this passes our String as a slice
    let _fword1 = even_better_first_word(&string_on_heap[..]);

    // this passes the string literal as a slice
    let _fword2 = even_better_first_word(&string[..]);

    // this also works because string literals are already slices!
    let _fword3 = even_better_first_word(string);

    // there are also other types of slices
    let array = [1, 2, 3, 4, 5];
    let a_slice = &array[1..3];
}

// this one returns a string slice
//fn better_first_word(s: &String) -> &str {
fn even_better_first_word(s: &str) -> &str { // this can be used on string literals now also!
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // return whole word if no space found
}

// returns a byte index value into the String parameter;
// but this is tedious and error-prone because the index
// has no meaning outside the separate context of s
fn first_word(s: &String) -> usize {

    // because we need to go through element by element
    let bytes = s.as_bytes();

    // create an iterator;
    // with enumerate, first element is the index, second 
    // is a reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
 
    // otherwise, return length of whole string
    s.len()
}
