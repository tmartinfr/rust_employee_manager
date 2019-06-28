use std::collections::HashMap;

pub struct EmployeeBook {
    data: HashMap<String, Vec<String>>,
}

impl EmployeeBook {
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data: data }
    }

    pub fn add_employee_to_department(&mut self, new_employee: &str, department: &str) -> String {
        let employee_list = self
            .data
            .entry(String::from(department))
            .or_insert(Vec::new());
        match employee_list
            .iter()
            .find(|&employee| employee == new_employee)
        {
            Some(_) => {
                return format!(
                    "Employee {} already in department {}",
                    new_employee, department
                );
            }
            None => {
                employee_list.push(String::from(new_employee));
                return format!("Employee {} added to {}", new_employee, department);
            }
        }
    }

    pub fn list_all(&self) -> String {
        let mut ret = String::new();
        for (department, employee_list) in &self.data {
            ret.push_str(&format!("{} :\n", department));
            ret.push_str(&Self::list_employees(employee_list));
        }
        ret
    }

    pub fn list_employee_from_department(&self, department: &str) -> String {
        if let Some(employee_list) = self.data.get(department) {
            return Self::list_employees(employee_list);
        } else {
            return String::from(format!("Department {} does not exist", department));
        }
    }

    fn list_employees(employee_list: &Vec<String>) -> String {
        let mut ret = String::new();
        let mut employee_list = employee_list.clone();
        employee_list.sort_unstable();
        for employee in employee_list.iter() {
            ret.push_str(&format!("- {}\n", employee));
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::EmployeeBook;

    #[test]
    fn list_employee_from_department() {
        let mut book = EmployeeBook::new();
        book.add_employee_to_department("Gaston Lagaffe", "R&D");
        let employee_list = book.list_employee_from_department("R&D");
        assert_eq!(employee_list, "- Gaston Lagaffe\n");
    }
}
