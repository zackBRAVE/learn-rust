use std::collections::HashMap;

fn run() {
    // - 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
    let mut arr = vec![
        5, 3, 9, 1, 8, 10, 9, 2, 2, 11, 1, 6, 4, 4, 21, 21, 21, 21, 21,
    ];
    let median = median(&mut arr);
    println!("median is {median}");
    let median = med(&mut arr);
    println!("median is {median}");
    let mode = mode(&mut arr);
    println!("mode is {mode}");
    let mode = simple_mode(&mut arr);
    println!("mode is {mode}");

    // - 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
    // 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
    let str = String::from("first lady is a beauty");
    let pig_latinized = pig_latin(str);
    println!("{pig_latinized}");

    // - 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
    // 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
    // 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
}

fn median(v: &mut Vec<i32>) -> i32 {
    println!("original vector: {:?}", v);

    v.sort();
    println!("sorted vector: {:?}", v);

    let len = v.len();
    let median_index = len / 2;
    println!("total len: {len}, median index: {median_index}");

    return v.get(median_index).copied().unwrap_or(0);
}

// find media of an i32 array
fn med(v: &mut Vec<i32>) -> i32 {
    println!("original vector: {:?}", v);

    v.sort();
    println!("sorted vector: {:?}", v);

    let len = v.len();
    let median_index = len / 2;
    println!("median index is {}", median_index);
    v[median_index]
}

fn mode(v: &mut Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for number in v {
        *counts.entry(number).or_insert(0) += 1;
    }
    **counts.iter().max_by_key(|&(_, &count)| count).unwrap().0
}

// find mode from a vector
fn simple_mode(v: &mut Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for number in v {
        *counts.entry(number).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut max_number = 0;
    for (number, count) in counts {
        if count > max_count {
            max_count = count;
            max_number = *number;
        }
    }
    max_number
}

fn pig_latin(s: String) -> String {
    let words = s.split_whitespace();
    let mut new_sentence = Vec::new();

    for word in words {
        let first_char_of_word = word.chars().next().unwrap_or(' ');
        if is_consonant(first_char_of_word) {
            // let new_word = (&word[1..]).to_string() + &first_char_of_word.to_string()[..] + "ay";
            let new_word = format!(
                "{}-{}{}",
                (&word[1..]).to_string(),
                &first_char_of_word.to_string()[..],
                "ay"
            );
            new_sentence.push(new_word);
            continue;
        }

        let new_word = format!("{}-{}", word.to_string(), "hay");
        new_sentence.push(new_word);
    }

    println!("{:?}", new_sentence);

    new_sentence.join(" ")
}

fn is_vowel(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
}

fn is_consonant(ch: char) -> bool {
    !is_vowel(ch)
}
