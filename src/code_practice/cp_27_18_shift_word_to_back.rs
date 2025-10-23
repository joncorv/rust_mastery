pub fn shift_word_to_back(arr: Vec<char>, word: String) -> Vec<char> {
    if arr.is_empty() {
        let blank_vec: Vec<char> = Vec::new();
        return blank_vec;
    }

    let (mut seeker, mut writer): (usize, usize) = (0, 0);
    let mut word_search_index: usize = 0;
    let mut arr = arr.clone();
    let word: Vec<char> = word.chars().collect();
    let len_arr = arr.len();
    let len_word = word.len();
    let default_char: char = ' ';

    while seeker < len_arr {
        // get test_char safely because you'll run out of bounds
        let test_char: char;
        if let Some(ch) = word.get(word_search_index) {
            test_char = *ch;
        } else {
            test_char = default_char;
        }

        // if seeker finds test char, move seeker and word_search index up
        if arr[seeker] == test_char {
            word_search_index += 1;
            seeker += 1;
        } else {
            // or else swap and iteratate seeker + writer
            arr.swap(seeker, writer);
            seeker += 1;
            writer += 1
        }
    }

    // clean up and write the given word to the end of the array
    for (index, letter) in word.iter().enumerate() {
        arr[len_arr - len_word + index] = *letter;
    }

    arr
}

pub fn test_shift_word_to_back() {
    println!("move word to the back of the array. keep everything else in order");

    let arr = vec!['a', 'x', 'b', 'y', 'c', 'z'];
    let word = "abc".to_string();
    let test_result = shift_word_to_back(arr.clone(), word.clone());
    println!("chars: {arr:?}, word: {word:?}");
    println!("result: {test_result:?}");

    let arr = vec!['a', 'w', 'b', 'x', 'c', 'y', 'd', 'z'];
    let word = "abcd".to_string();
    let test_result = shift_word_to_back(arr.clone(), word.clone());
    println!("chars: {arr:?}, word: {word:?}");
    println!("result: {test_result:?}");

    let arr = vec![];
    let word = "abc".to_string();
    let test_result = shift_word_to_back(arr.clone(), word.clone());
    println!("chars: {arr:?}, word: {word:?}");
    println!("result: {test_result:?}");
}
