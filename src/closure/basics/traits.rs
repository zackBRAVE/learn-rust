#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

fn _sort_by_key() {
    let mut list = [
        Rectangle {
            width: 10,
            _height: 1,
        },
        Rectangle {
            width: 3,
            _height: 5,
        },
        Rectangle {
            width: 7,
            _height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn _fn_once() {
    // let mut list = [
    //     Rectangle {
    //         width: 10,
    //         height: 1,
    //     },
    //     Rectangle {
    //         width: 3,
    //         height: 5,
    //     },
    //     Rectangle {
    //         width: 7,
    //         height: 12,
    //     },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     // value moved out, can only be called once
    //     sort_operations.push(value);
    //     r.width
    // })
}

fn _fn_mut() {
    let mut list = [
        Rectangle {
            width: 10,
            _height: 1,
        },
        Rectangle {
            width: 3,
            _height: 5,
        },
        Rectangle {
            width: 7,
            _height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

pub fn run() {
    // _sort_by_key();
    _fn_once();
    _fn_mut();
}
