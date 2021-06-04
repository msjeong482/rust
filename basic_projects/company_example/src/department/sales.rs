use crate::department::Employee;

pub fn add_to_department(name: String) -> Employee {
    Employee {
        name: name.to_string(),
        department: String::from("sales"),
    }
}
