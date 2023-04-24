// exploring some of rust's collections, unique and helpful
// data structures that COME FREE WITH YOUR XBOX (std library)

use std::collections::HashMap;

fn main() {
    // vectors store more than one value of the same type in
    // a linear fashion on the memory heap
    //
    // The Vec<T> structure can be instantiated as an empty vector:
    let v: Vec<i32> = Vec::new();
    // or can be given some values as a macro and initialized:
    let v = vec![1, 2, 3];
    println!("A vector can be instantiated with the vec![] macro: {:?}", v);

    // vectors can be mutable and their values updated:
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("A vector can be pushed some new values: {:?}", v);

    // values inside a vector can be referenced using indexing
    // or the get() method:
    let v = vec![1, 2, 3, 4, 5];

    // reference the vector element directly using &
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // reference the vector element using get() and explicit matching
    let third: Option<&i32> = v.get(2); // get() returns an Option enum
    // in this case, v.get(2) returns Some(&v[2])
    // implement a match to extract data from that Option result:
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // a vector can be iterated over using for loops:
    let v = vec![25, 50, 75, 100];
    for i in &v {
        println!("loop over: {i}");
    }

    // mutable vectors can also be iterated over and changed:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * dereference operator is used to refer to i and not &i
    }

    // although vectors cannot store multiple data types, creating a vector
    // that stores an enum type of different variants allows for de facto
    // multi-type vectors:
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // And now, for the String type that comes with the standard library.
    //
    // In essence, strings are a list of characters, represented in UTF-8
    // encoding using 4-bit or 8-bit character variables.
    // In rust, strings are implemented as a collection of bytes, and the
    // String data structure gives some helpful methods to construct and
    // modify strings while preserving ownership and code safety on the heap.
    //
    // Rust only has one string type in the core language, which is the
    // "str" string slice. A slice is just a pre-defined list of values.
    // For example, a string literal that is known at compile is a string slice
    // of type str.
    //
    // By comparison, the String type is provided by the standard library, and
    // acts more like a data structure than a compile time variable.
    // String is a growable, mutable, owned, UTF-8 encoded string type.
    // Similar to Vec<T>, String is instantiated and manipulated using
    // dot notation and method construction:
    let mut s = String::new(); // instantiates an empty String
    
    // String methods can be used on string literals
    let data = "initial contents"; // data is a string literal str

    let s = data.to_string(); // to_string() is a method of String

    // this method also works on literals directly:
    let s = "initial contents".to_string();

    println!("data: {data}");
    println!("s: {s}");

    // String also has a method from() that can encode string literals
    let s = String::from("from contents");
    println!("from string: {s}");

    // The push_str() method can append string literals to mutable Strings:
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    // lastly, a less common but vital collection data structure that rust
    // provides is the Hash Map.
    //
    // In rust, hash maps take the form of a data structure like HashMap<K, V>.
    // This type stores a mapping of keys of type K to values of type V using
    // a hashing function.
    //
    // hash maps are useful when you want to look up data not by index,
    // but by some known key. Accessing data in a hash map is more complicated
    // than an array, but can be much faster.
    //
    // To start, instantiate an empty hash map:
    let mut scores = HashMap::new();

    // Then, the insert() method can input key value pairs into the map:
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // using the get() method and passing the key, we can extract the value
    // associated with the key:
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // get(&K) will return Option<&V>. If there's no value for the key, get()
    // will return None.
    // In the above code, the Option is handled by using copied() to get an
    // Option<T> rather than an Option<&T>, then an unwrap_or(0) to unwrap
    // the data from the Option and set the value to 0 if the Option is None.
    //
    // Iterating over hash maps are also possible:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
