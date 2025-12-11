use std::collections::HashMap;

pub fn parse(input: &str) -> HashMap<&str, Vec<&str>>{
    let mut hash_map = HashMap::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let split: Vec < _ > = line.split(": ").collect();
        assert_eq! (split.len(), 2);

        let key = split[0];
        let value: Vec < _ > = split[1].split(" ").collect();

        hash_map.insert(key, value);
    }

    hash_map
}


pub const TEST_DATA: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let hash_map = parse(TEST_DATA);

        assert_eq!(hash_map.get("you"), Some(&vec!["bbb", "ccc"]));
    }
}