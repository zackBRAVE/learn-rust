fn run() {
    vector();
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector() {
    // let vec: Vec<i32> = Vec::new();

    let mut vec2 = Vec::new();
    vec2.push(3);

    let mut v = vec![1, 2, 3];
    let third = &v[2];
    println!("the third element of {:?} is {third}", v);

    let third = v.get(2);
    match third {
        Some(third) => println!("the third element of {:?} is {third}", v),
        None => println!("there is no third element in {:?}", v),
    };

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let spreadsheet_vec = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("red")),
    ];
}
