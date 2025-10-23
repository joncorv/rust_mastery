pub fn shift_word_to_back(arr: Vec<char>, word: String) -> Vec<char> {
    if arr.is_empty() {
        let blank_vec: Vec<char> = Vec::new();
        return blank_vec;
    }

    let (mut seeker, mut writer): (usize, usize) = (arr.len() - 1, arr.len() - 1);
    let mut word_search_index: usize = word.len() - 1;
    let mut arr = arr.clone();
    let word: Vec<char> = word.chars().collect();

    'main_loop: loop {
        if arr[seeker] == word[word_search_index] {
            (arr[seeker], arr[writer]) = (arr[writer], arr[seeker]);
            if word_search_index > 0 {
                seeker -= 1;
                writer -= 1;
                word_search_index -= 1;
            } else {
                break 'main_loop;
            }
        } else {
            seeker -= 1
        }
    }

    arr

    // create seeker and writer as len of arr -1
    // shadow copy arr as mutable
    // word_search_index is it's len -1

    // while word_search_index > 0
    // if arr[seeker] == word[word_search_index]
    // swap seeker and searcher vals in tuple
    // decrement seeker, searcher, and word_search_index
    // else decrement the seeker
    // return arr
}

pub fn test_shift_word_to_back() {
    println!("move word to the back of the array. keep everything else in order");
    let arr = vec!['a', 'z', 'b', 'f', 'c', 'z'];
    let word = "abc".to_string();
    let test_result = shift_word_to_back(arr.clone(), word.clone());
    println!("chars: {arr:?}, word: {word:?}");
    println!("result: {test_result:?}");
}
