use std::collections::HashMap;

// given a vector of tuples containing ip addresses and account names,
// return the account names that appear in multiple ip addresses

pub fn account_sharing(data: Vec<(String, String)>) -> Vec<String> {
    let mut test_hash: HashMap<String, String> = HashMap::new();
    let mut result: Vec<String> = Vec::new();

    for (ip, name) in data {
        match test_hash.get(&name) {
            Some(hash_ip) => {
                if hash_ip.clone() != ip {
                    result.push(name);
                } else {
                    test_hash.insert(name, ip);
                };
            }
            None => {
                test_hash.insert(name, ip);
            }
        }
    }

    return result;
}

pub fn test_account_sharing() {
    let data: Vec<(String, String)> = vec![
        ("192.168.1.1".to_string(), "Doug".to_string()),
        ("192.168.1.2".to_string(), "Frank".to_string()),
        ("192.168.1.3".to_string(), "Steve".to_string()),
        ("192.168.1.4".to_string(), "Tim".to_string()),
        ("192.168.1.5".to_string(), "Doug".to_string()),
    ];
    let test_result = account_sharing(data.clone());
    println!("test data: {data:?}");
    println!("results: {test_result:?}");
}
