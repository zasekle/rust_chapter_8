use std::collections::HashMap;

fn main() {
    // vector_stuff();
    // string_stuff();
    hash_map_stuff();
}

fn vector_stuff() {

    //Vector (which I assume comes from the c++ naming convention) works just like a c++ vector.
    let mut v = Vec::<isize>::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("i: {:#?}", v);

    let first = &v[0];

    //Can access by index and return an immutable element.
    println!("first: {first}");

    //Can access by index and return the element as mutable.
    //Can panic because it does not have error checking.
    let first = &mut v[0];

    *first += 5;

    //The first element is now 6.
    println!("i: {:#?}", v);

    //This will get the index, but it will also use error checking.
    let error_first = v.get(0);

    match error_first {
        Some(val) => println!("Index exists, value is {val}"),
        None => println!("Index does not exist."),
    }

    let error_first = v.get(100);

    match error_first {
        Some(val) => println!("Index exists, value is {val}"),
        None => println!("Index does not exist."),
    }

    //The entire vector is a single object. This means it follows the same rules as other variables
    // do for the borrow checker. If a single mutable element is outstanding, no other elements
    // inside the vector can be accessed, even immutably. For example, the below variable cannot be
    // accessed because a mutable version is accessed in terms of `v` above.
    // println!("Failure here: {first}");

    //There are several ways to iterate over vectors.

    for i in &v {
        println!("i: {i}");
    }

    for i in v.iter() {
        println!("i: {i}");
    }

    for i in &mut v {
        *i += 1;
        println!("i: {i}");
    }

    for i in v.iter_mut() {
        *i += 1;
        println!("i: {i}");
    }

    //A really cool idea that I didn't consider is using enums to store multiple types in the same
    // vector.
    #[derive(Debug)]
    enum HelloWorld {
        Hello(i32),
        World(String),
    }

    let hello_world_vec = vec![
        HelloWorld::Hello(123),
        HelloWorld::World(String::from("Da world")),
    ];

    println!("hello_world_vec: {:#?}", hello_world_vec);
}

fn string_stuff() {

    //The first thing is that there are `String` types and `&str` (or string slices) types. The
    // compiler can coerce an &String to &str.
    let mut string = String::new();

    string.push('a');
    string.push('b');
    string.push('c');

    println!("string: {string}");

    //Using a string literal.
    let string_literal = "def";

    //Converting a string literal (or string slice) to a string.
    let string_literal_to_string = string_literal.to_string();

    println!("string_literal: {string_literal} string_literal_to_string: {string_literal_to_string}");

    //Extracting a part of a string as a string slice using a range. This should be used with
    // caution. It can crash the program and it is accessing the string in bytes, not chars (more
    // on this below).
    let string_literal = &string[0..2];

    println!("string_literal: {string_literal}");

    //Can push a string. This does NOT take ownership of the value (if it was a variable), it seems
    // to make a copy.
    string.push_str("def");

    //Can push a single char.
    string.push('g');

    println!("string: {string}");

    //If I make two strings I can concatenate them.
    let first = String::from("first ");
    let second = String::from("second");

    //An important note here is that the first string is invalid after this occurs. This is b/c the
    // add function takes the first string in and appends a copy of the second to it.
    let both = first + &second;

    println!("both: {both}");

    //Can also do something like below. This will break the immutable part of the variable, but
    // if there is no need for it to be immutable it would minimize the number of variables and
    // the amount of code inside the function. And internally it is essentially the same thing as
    // the above add.
    let mut first = String::from("first ");

    first += &second;

    println!("first: {first}");

    //The plus operator can be unwieldy. `format!` macro helps take care of this.
    let s1 = String::from("abra");
    let s2 = String::from("kadabra");
    let s3 = String::from("alakazam");

    let combined = format!("{s1} {s2} {s3}");

    println!("combined: {combined}");

    //Strings inside use UTF-8 and after working with string inside Kotlin, I quite like the way
    // they do it. They seem to have designed it avoid problems of simply accessing an element
    // inside the string and assuming it is a char. The reality is that UTF-8 can take up multiple
    // bytes, not just one. So when using Rust strings, you need to manually convert them to the
    // type that you want. There are three types.
    // 1) bytes: This is essentially a Vec<u8> and works just like a string in c++ does. `.bytes`
    // 2) scalars(chars): This will group the values into single unicode characters. `.chars`
    // 3) grapheme clusters: This is a bit more involved in chars, but there are languages that
    //  require `connecting` characters. There is no built in function to list by grapheme chars.

    for c in "Зд".chars() {
        println!("c: {c}");
    }

    for b in "Зд".bytes() {
        println!("b: {b}");
    }

    //Internally strings are string as a Vec<u8>. If string manipulations need to be made
    // (especially if the string is guaranteed to be in ASCII) a nice little trick is just to
    // covert the String->Vec<u8>. It does require iterating through the string once for this.
    let bytes = string.as_bytes().to_vec();

    println!("bytes: {:#?}", bytes);
}

fn hash_map_stuff() {

    //As an initial note Rust provides two types of maps out of the box. The HashMap which is
    // sorted based on hashing the values (an unordered_map in c++). Then the other type is a
    // BTreeMap which is a map stored using a BTree (a map in c++).

    use std::collections::HashMap;

    //Hash maps do not have a macro to construct them like Strings and Vectors do.
    let mut unordered_map = HashMap::<isize, String>::new();

    unordered_map.insert(1, String::from("one"));
    unordered_map.insert(2, String::from("two"));
    unordered_map.insert(3, String::from("three"));

    println!("unordered_map: {:#?}", unordered_map);

    //Can initialize a map through from and pass an array of Tuples.
    let from_map = HashMap::from(
        [
            (4, String::from("four")),
            (5, String::from("five")),
        ]
    );

    println!("from_map: {:#?}", from_map);

    //Can use get and it will error check the return value. Note that these return an
    // Option<&String>.
    let successful_get = unordered_map.get(&1);
    let failed_get = unordered_map.get(&1000);

    match successful_get {
        None => println!("successful_get returned None"),
        Some(val) => println!("successful_get: {val}"),
    }

    match failed_get {
        None => println!("failed_get returned None"),
        Some(val) => println!("failed_get: {val}"),
    }

    //One thing that is different in Rust over c++ is that when a value is inserted into a map in
    // Rust, the old value is replaced. This is not true in c++ where the old value is retained.
    unordered_map.insert(1, String::from("new_value"));

    println!("unordered_map: {:#?}", unordered_map);

    //If the desired functionality is to add a value only if it does NOT exist then .entry() can be
    // used.
    unordered_map.entry(1).or_insert(String::from("String did not exist"));
    unordered_map.entry(4).or_insert(String::from("four"));

    println!("unordered_map: {:#?}", unordered_map);

    //Because entry returns a mutable reference it can be used to change a value based on how many
    // times it has been seen. This example was copied verbatim from the Rust book b/c its such
    // a good example.
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    //By default the HashMap uses a hashing function called SipHash. This provides more security,
    // but it is not the most performant. If more performance is needed, the hasher can be changed.
}
