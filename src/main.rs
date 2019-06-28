use std::io::{self, Write};
mod employee_book;
use employee_book::EmployeeBook;

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
