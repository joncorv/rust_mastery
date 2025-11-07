// check that users are not connecting to the same exact ip addresses provided

use std::collections::{
    HashMap,
    HashSet, //
};

pub fn multi_account_cheating(data: Vec<(String, Vec<String>)>) -> HashSet<String> {
    if data.is_empty() {
        let empty_result: HashSet<String> = HashSet::new();
        return empty_result;
    }

    let mut ip_history: HashMap<Vec<String>, String> = HashMap::new();
    let mut result: HashSet<String> = HashSet::new();

    for (name, ip) in data {
        let mut ip = ip.clone();
        ip.sort();

        if let Some(existing_name) = ip_history.get(&ip) {
            result.insert(name.to_string());
            result.insert(existing_name.to_string());
        } else {
            ip_history.insert(ip, name);
        }
    }

    println!("ip history: {ip_history:#?}");
    return result;
}

pub fn test_multi_account_cheating() {
    let data1 = (
        "Doug".to_string(),
        vec![
            "192.168.1.1".to_string(),
            "192.168.1.2".to_string(),
            "192.168.1.3".to_string(),
            "192.168.1.4".to_string(),
        ],
    );
    let data2 = (
        "Sally".to_string(),
        vec![
            "192.168.1.3".to_string(),
            "192.168.1.4".to_string(),
            "192.168.1.5".to_string(),
            "192.168.1.6".to_string(),
        ],
    );
    let data3 = (
        "Rebecca".to_string(),
        vec![
            "192.168.1.6".to_string(),
            "192.168.1.7".to_string(),
            "192.168.1.8".to_string(),
            "192.168.1.9".to_string(),
        ],
    );
    let data4 = (
        "Douglas".to_string(),
        vec![
            "192.168.1.4".to_string(),
            "192.168.1.2".to_string(),
            "192.168.1.3".to_string(),
            "192.168.1.1".to_string(),
        ],
    );

    let data = vec![data1, data2, data3, data4];
    let test_result = multi_account_cheating(data.clone());

    println!("test data: {data:?}");
    println!("results: {test_result:?}");
}
