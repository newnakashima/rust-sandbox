use std::collections::HashMap;
use std::io::Error;

pub type DepartmentList = HashMap<String, Vec<String>>;

pub fn validate_words(item: &str) -> Result<Vec<&str>, Error> {
    let mut words: Vec<&str> = vec![];
    item.split_whitespace().for_each(|i| words.push(i));

    if words.len() < 2 {
        let error = Error::new(std::io::ErrorKind::InvalidInput, "oh no");
        return Err(error);
    }

    Ok(words)
}

pub fn add_employee(
    store: &mut DepartmentList,
    department_name: &str,
    employee_name: &str,
) {
    let existing = store.get_mut(department_name);
    if let Some(list) = existing {
        list.push(employee_name.to_string());
    } else {
        store.insert(department_name.to_string(), vec![employee_name.to_string()]);
    }
}
