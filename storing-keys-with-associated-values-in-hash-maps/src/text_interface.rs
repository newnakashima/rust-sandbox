use std::collections::HashMap;

pub enum Operation {
    Add,
    ListDepartment,
    ListCompany,
    Invalid,
}

pub type MyMap<'a> = HashMap<String, &'a mut Vec<String>>;

pub fn list_company() -> String {
    String::from("company")
}

pub fn list_department() -> String {
    String::from("department")
}

pub fn add_employee<'a>(map: &'a mut HashMap<String, &'a mut Vec<String>>, words: &mut Vec<&str>, default: &'a mut Vec<String>) -> String {
    let name = match words.get(1) {
        Some(&n) => n,
        None => return String::from("name is invalid")
    };
    let department = match words.get(3) {
        Some(&d) => d,
        None => return String::from("department is invalid")
    };

    let name = name.clone().to_string();
    let department = department.clone();

    let v = map.get_mut(department);
    if let Some(vector) = v {
        vector.push(name.to_string())
    } else {
        default.push(name.to_string());
        map.insert(department.to_string(), default);
    }

    format!("{} is added to {} department", name, department)
}

pub fn parse_list_argument(arg: Option<&&str>) -> Operation {
    return match arg {
        Some(&"Company") => Operation::ListCompany,
        Some(&"Department") => Operation::ListDepartment,
        _ => Operation::Invalid,
    }
}

pub fn parse_words(input: &str) -> Vec<&str> {
    let mut words = vec![];
    for word in input.split_whitespace() {
        words.push(word)
    }

    words
}

pub fn parse_operation(first: Option<&&str>, second: Option<&&str>) -> Operation {
    return match first {
        Some(&"Add") => Operation::Add,
        Some(&"List") => {
            return parse_list_argument(second);
        }
        _ => Operation::Invalid,
    }
}

mod tests {
    use super::parse_words;


    #[test]
    fn test_parse_words() {
        let input = String::from("Add Sally to Engineering");
    }
}
