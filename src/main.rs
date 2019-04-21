use std::collections::HashMap;
use std::io::{self, Write};

struct EmployeeBook {
    data: HashMap<String, Vec<String>>,
}

impl EmployeeBook {
    fn new() -> EmployeeBook {
        let data = HashMap::new();
        EmployeeBook { data: data }
    }

    fn add_employee_to_department(&mut self, new_employee: &str, department: &str) -> String {
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

    fn list_all(&self) -> String {
        let mut ret = String::new();
        for (department, employee_list) in &self.data {
            ret.push_str(&format!("{} :\n", department));
            ret.push_str(&EmployeeBook::list_employees(employee_list));
        }
        ret
    }

    fn list_employee_from_department(&self, department: &str) -> String {
        if let Some(employee_list) = self.data.get(department) {
            return EmployeeBook::list_employees(employee_list);
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

fn main() {
    let mut book = EmployeeBook::new();

    println!(
        "\
         Welcome, allowed commands are:\n\
         > Add <employeename> to <department>\n\
         > List\n\
         > List <department>"
    );
    println!("");

    loop {
        print!("> ");
        io::stdout().flush().expect("Flush output error");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.pop().expect("Failed to pop newline");
        if input.len() > 0 {
            let args: Vec<&str> = input.split(' ').collect();
            let output = exec_cmd(args, &mut book);
            println!("{}", output);
        }
    }
}

fn exec_cmd(mut args: Vec<&str>, book: &mut EmployeeBook) -> String {
    match args.remove(0) {
        "Add" => {
            return cmd_add(args, book);
        }
        "List" => {
            return cmd_list(args, book);
        }
        _ => {
            return String::from("Unknown command");
        }
    }
}

fn cmd_add(args: Vec<&str>, book: &mut EmployeeBook) -> String {
    if args.len() != 3 {
        return String::from("Invalid number of arguments to Add command");
    }
    if args.get(1) != Some(&"to") {
        return String::from("Missing \"to\" keyword in Add command");
    }
    let employee = args[0];
    let department = args[2];
    book.add_employee_to_department(employee, department)
}

fn cmd_list(args: Vec<&str>, book: &EmployeeBook) -> String {
    match args.len() {
        0 => return book.list_all(),
        1 => return book.list_employee_from_department(args[0]),
        _ => return String::from("Invalid number of arguments to List command"),
    }
}
