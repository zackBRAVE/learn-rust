fn run() {
    let sentence = String::from("This is a sentence");
    let word_sentence = String::from("sentence");

    println!("first word index {}", first_word(&sentence));
    println!("first word index {}", first_word(&word_sentence));

    println!("first word {}", first_word_with_slice(&sentence));
    println!("first word {}", first_word_with_slice(&word_sentence));

    println!("second word {}", second_word(&sentence));
    println!("second word {}", second_word(&word_sentence));

    println!("1st word {}", nth_word(&sentence, 1));
    println!("2nd word {}", nth_word(&sentence, 2));
    println!("3rd word {}", nth_word(&sentence, 3));
    println!("4th word {}", nth_word(&sentence, 4));
    println!("5th word {}", nth_word(&sentence, 5));

    println!("0th word {}", nth_word(&word_sentence, 0));
    println!("1st word {}", nth_word(&word_sentence, 1));
    println!("1st word {}", nth_word(&sentence[..4], 1));
    println!("2nd word {}", nth_word(&word_sentence, 2));

    let arr = [1, 2, 3, 4];
    let arr_slice = &arr[1..3];

    assert_eq!(arr_slice, &[2, 3]);
}

fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (index, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return index;
        }
    }

    str.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[..i];
        }
    }

    &s
}

fn second_word(s: &String) -> &str {
    let bytes = s.trim().as_bytes();
    let mut first_index = 0;

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            if first_index == 0 {
                first_index = i;
                continue;
            }
            return &s[first_index + 1..i];
        }
    }

    &s[0..0]
}

fn nth_word(s: &str, nth: usize) -> &str {
    let bytes = s.trim().as_bytes();
    let mut count = nth;
    let mut first_index = 0;

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            count -= 1;
            if first_index == 0 && count <= 0 {
                return &s[..i];
            } else if count == 0 {
                return &s[first_index + 1..i];
            }
            first_index = i;
        }
    }

    if first_index > 0 && count == 1 {
        &s[first_index + 1..]
    } else if first_index == 0 && count == 1 {
        &s
    } else {
        &s[0..0]
    }
}
