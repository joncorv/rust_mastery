use std::collections::HashMap;
// return the most frequent first 3 digits of the octet ip addresses provided

// create result string
// create HashMap<String, usize> where String is 3 chars, and usize is frequency it's found
// create max_count

// iterate over all items
// find first three chars of data
// if chars are hashmap key, incriment data, or if it doesn't exist, set value
// also make sure to check if incrimented data i set is largest. if it is, sit max_count to it,
// and set result as the key

pub fn most_frequent_octet(data: Vec<String>) -> String {
    let mut result: String = String::new();
    let mut octet_frequenchy_hash: HashMap<String, usize> = HashMap::new();
    let mut most_appearances: usize = 0;

    for d in data {
        let first_3 = &d[0..=2];
        octet_frequenchy_hash
            .entry(first_3.to_string())
            .and_modify(|e| {
                *e += 1;
                if *e > most_appearances {
                    most_appearances = *e;
                    result = first_3.to_string();
                }
            })
            .or_insert(1);
    }

    println!("hash: {octet_frequenchy_hash:#?}");
    return result;
}

pub fn test_most_frequent_octet() {
    let data = vec![
        "192.168.1.1".to_string(),
        "192.168.1.2".to_string(),
        "192.168.1.3".to_string(),
        "192.168.1.4".to_string(),
        "192.168.1.5".to_string(),
        "192.168.1.6".to_string(),
        "500.168.1.1".to_string(),
        "500.168.1.2".to_string(),
        "500.168.1.3".to_string(),
        "500.168.1.4".to_string(),
        "500.168.1.5".to_string(),
        "500.168.1.6".to_string(),
        "500.168.1.7".to_string(),
        "101.168.1.1".to_string(),
        "101.168.1.2".to_string(),
        "101.168.1.3".to_string(),
        "101.168.1.4".to_string(),
        "101.168.1.5".to_string(),
    ];
    let test_result = most_frequent_octet(data.clone());
    println!("test data: {data:#?}");
    println!("results: {test_result:?}");
}
