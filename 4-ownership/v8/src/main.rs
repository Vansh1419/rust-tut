fn main() {
    // Basic slices
    let arr = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];
    println!("{:?}", slice);

    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let vec_slice: &[i32] = &vec[3..4];
    println!("{:?}", vec_slice);

    let mut s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{:?}", hello);
    println!("{:?}", world);

    // Range shortcut for slices
    // initial [..n]
    // final [n..]
    // total [..]

    // First word of string
    let word = first_word_no_slice(&s);
    println!("{}", s);
    println!("{}", word);
    println!("{}", &s[..word]);

    s.clear();
    /*
    error occur when string goes out of bound
    thread 'main' (2841492) panicked at src/main.rs:31:21:
    end byte index 5 is out of bounds for string of length 0
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    println!("{}", s);
    println!("{}", word);
    // println!("{}", &s[..word]);

    // With using slice
    let mut ns = String::from("Hello world");
    let wrd = first_word_slice(&ns);
    println!("{}", ns);
    println!("{}", wrd);
    // ns.clear(); // here we get mutable reference error
    let s2 = "second word";
    let word2 = first_word_slice(s2);
    
    println!("{}", s2);
    println!("{}", word2);
}

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
