fn _for_in() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // next line won't work cause v1_iter has been moved in 'for in' call
    // println!("Got {:?}", v1_iter);
}

fn _next() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    println!("Got: {:?}", v1);
    println!("Got: {}", v1_iter.next().unwrap());

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn _into_iter() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.into_iter();

    // next line won't work cause into_iter() took the ownership of v1
    // println!("Got: {:?}", v1);

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn _iter_mut() {
    let mut v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter_mut();

    // next line won't work cause into_iter() took the ownership of v1
    // println!("Got: {:?}", v1);
    println!("Got: {}", v1_iter.next().unwrap());

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn _iter_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    println!("sum of {:?} is {}", v1, total);

    // next line won't work cause v1_iter is moved when calling sum()
    // println!("iter: {:?}", v1_iter);
}

fn _iter_map() {
    let v1 = vec![1, 2, 3];

    let v1_plus_one: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    println!("Plus one of {:?} is {:?}", v1, v1_plus_one);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

fn _iter_filter() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneakers"),
        },
        Shoe {
            size: 13,
            style: String::from("sandals"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneakers"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );

    println!("shoes in my size: {:#?}", in_my_size);
}

pub fn run() {
    // _for_in();
    // _next();
    // _into_iter();
    // _iter_mut();
    // _iter_sum();
    // _iter_map();
    _iter_filter();
}
