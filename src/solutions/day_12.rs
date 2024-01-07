use std::str::FromStr;

use serde_json::Value;

pub fn solve_1(json: &str) -> i64 {
    fn sum(json: &Value) -> i64 {
        match json {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(number) => number.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(values) => values.iter().map(sum).sum(),
            Value::Object(map) => map.values().map(sum).sum(),
        }
    }

    let json = Value::from_str(json).unwrap();
    sum(&json)
}

pub fn solve_2(json: &str) -> i64 {
    fn sum(json: &Value) -> i64 {
        match json {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(number) => number.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(values) => values.iter().map(sum).sum(),
            Value::Object(map) => {
                let red = map
                    .values()
                    .flat_map(|v| match v {
                        Value::String(string) => Some(string),
                        _ => None,
                    })
                    .any(|s| s == "red");

                if red {
                    0
                } else {
                    map.values().map(sum).sum()
                }
            }
        }
    }

    let json = Value::from_str(json).unwrap();
    sum(&json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        assert_eq!(6, solve_1(r#"[1,2,3]"#));
        assert_eq!(6, solve_1(r#"{"a":2,"b":4}"#));
        assert_eq!(3, solve_1(r#"[[[3]]]"#));
        assert_eq!(3, solve_1(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(0, solve_1(r#"{"a":[-1,1]}"#));
        assert_eq!(0, solve_1(r#"[-1,{"a":1}]"#));
        assert_eq!(0, solve_1(r#"[]"#));
        assert_eq!(0, solve_1(r#"{}"#));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt").trim();

        assert_eq!(119_433, solve_1(input));
    }

    #[test]
    fn day_12_part_02_sample() {
        assert_eq!(6, solve_2(r#"[1,2,3]"#));
        assert_eq!(4, solve_2(r#"[1,{"c":"red","b":2},3]"#));
        assert_eq!(0, solve_2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
        assert_eq!(6, solve_2(r#"[1,"red",5]"#));
    }

    #[test]
    fn day_12_part_02_solution() {
        let input = include_str!("../../inputs/day_12.txt").trim();

        assert_eq!(68_466, solve_2(input));
    }
}
