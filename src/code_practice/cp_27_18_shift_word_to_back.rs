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

    'main_loop: loop {
        if arr[seeker] != word[word_search_index] {
            (arr[seeker], arr[writer]) = (arr[writer], arr[seeker]);
            if word_search_index < word.len() - 1 {
                seeker += 1;
                writer += 1;
                word_search_index += 1;
            } else {
                break 'main_loop;
            }
        } else {
            seeker += 1
        }
    }

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

    let arr = vec!['a', 'f', 'b', 'u', 'c', 'c', 'd', 'k'];
    let word = "abc".to_string();
    let test_result = shift_word_to_back(arr.clone(), word.clone());
    println!("chars: {arr:?}, word: {word:?}");
    println!("result: {test_result:?}");

    let arr = vec![];
    let word = "abc".to_string();
    let test_result = shift_word_to_back(arr.clone(), word.clone());
    println!("chars: {arr:?}, word: {word:?}");
    println!("result: {test_result:?}");
}
