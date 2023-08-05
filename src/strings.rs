fn str() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('!');
    println!("s1 = {}, s2 = {}", s1, s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    println!("s1 = {}, s2 = {}, s3 = {s3}", s1, s2);

    let s1 = "tic";
    let s2 = "tac";
    let s3 = String::from("toe");
    let s = format!("{}-{s2}-{s3}", s1);
    println!("s = {s}");

    let hello = "Здравствуйте";
    let s = &hello[..6];
    println!("s = {s}");

    let hello = String::from(hello);
    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }

    let char_array = [104, 101, 108, 108, 111];
    let hello = String::from_utf8(char_array.to_vec()).unwrap();
    println!("char array to string = {hello}");

    let is_c = "asd".contains("c");
    println!("is_c = {is_c}");
    let is_c = String::from("zxc").contains("xc");
    println!("is_c = {is_c}");

    let is_c = String::from("zxc").replace("c", "qc");
    println!("is_c = {is_c}");

    let zxc = String::from("zxc");
    let is_c = zxc.replace("c", "qc");
    println!("is_c = {is_c}, zxc = {zxc}");
}
