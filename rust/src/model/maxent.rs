fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

use std::collections::HashMap;

fn dot_product(dict1: &HashMap<String, f64>, dict2: &HashMap<String, f64>) -> f64 {
    let mut result = 0.0;
    for (key, &v1) in dict1 {
        if let Some(&v2) = dict2.get(key) {
            result += v1 * v2;
        }
        // In case of null (None), we skip the key as per the original JavaScript logic.
    }
    result
}

