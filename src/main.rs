#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

struct Record {
    name: String,
    phone: String,
}

impl From<String> for Record {
    fn from(value: String) -> Self {
        let splitted: Vec<&str> = value.split(";").take(2).collect();

        Record {
            name: splitted[0].into(),
            phone: splitted[1].into(),
        }
    }
}

fn main() {}

fn add_to_list(name: String, phone: String) -> Result<(), Box<dyn Error>> {
    let mut list = get_phones()?;

    Ok(())
}

fn get_index_to_add(list: &Vec<Record>, name: &String) -> usize {
    if list.len() == 0 {
        return 0;
    }

    let a: Vec<bool> = list.iter().map(|r| &r.name < name).collect();
    println!("{:?}", a);

    return match list.iter().position(|r| (&r.name < name) == false) {
        None => list.len(),
        Some(i) => i,
    };
}

fn get_phones() -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open("~/phones.txt")?;

    let lines = io::BufReader::new(file).lines();

    Ok(lines.map(|x| Record::from(x.unwrap())).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Record {
        fn new(name: &str) -> Self {
            Record {
                name: name.to_string(),
                phone: "".to_string(),
            }
        }
    }

    #[test]
    fn emptylist_should_return0() {
        let list = vec![];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 0);
    }

    #[test]
    fn oneelement_should_return0() {
        let list = vec![Record::new("joao")];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 0);
    }

    #[test]
    fn oneelement_should_return1() {
        let list = vec![Record::new("alberto")];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 1);
    }

    #[test]
    fn twoelements_should_return0() {
        let list = vec![Record::new("gabriel"), Record::new("wesley")];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 0);
    }

    #[test]
    fn twoelements_should_return1() {
        let list = vec![Record::new("alberto"), Record::new("gabriel")];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 1);
    }

    #[test]
    fn twoelements_should_return2() {
        let list = vec![Record::new("alberto"), Record::new("bia")];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 2);
    }

    #[test]
    fn treeelements_should_return0() {
        let list = vec![
            Record::new("gabriel"),
            Record::new("julia"),
            Record::new("wesley"),
        ];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 0);
    }

    #[test]
    fn treeelements_should_return1() {
        let list = vec![
            Record::new("alberto"),
            Record::new("julia"),
            Record::new("wesley"),
        ];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 1);
    }

    #[test]
    fn treeelements_should_return2() {
        let list = vec![
            Record::new("alberto"),
            Record::new("bia"),
            Record::new("julia"),
        ];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 2);
    }

    #[test]
    fn treeelements_should_return3() {
        let list = vec![
            Record::new("alberto"),
            Record::new("bia"),
            Record::new("clare"),
        ];
        let name = "filipe".to_string();

        let index = get_index_to_add(&list, &name);

        assert_eq!(index, 3);
    }
}
