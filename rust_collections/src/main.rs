// exploring some of rust's collections, unique and helpful
// data structures that COME FREE WITH YOUR XBOX (std library)


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
}
