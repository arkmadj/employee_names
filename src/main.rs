use std::{collections::HashMap, io};

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print_departments(&employees);

        println!("Please enter an action (e.g Add Ahmad to Engineering or Remove Ahmad from Engineering or Get all from Engineering): ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match extract_command(&input) {
            Ok((action, name, department)) => match action.as_str() {
                "Add" => add_employee(&mut employees, &department, &name),
                "Remove" => remove_employee(&mut employees, &department, &name),
                "Get" => print_department_employees(&employees, &department),
                _ => break,
            },
            Err(error) => {
                eprintln!("Error: {}", error);
                break;
            }
        }
    }
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    let employees = departments
        .entry(department.to_string())
        .or_insert_with(Vec::new);

    match employees.binary_search(&employee.to_string()) {
        Ok(_) => {
            println!("Employee already exists.");
        }
        Err(pos) => {
            employees.insert(pos, employee.to_string());
            println!("Added {} to {} department", employee, department);
        }
    }
}

fn remove_employee(
    departments: &mut HashMap<String, Vec<String>>,
    department: &str,
    employee: &str,
) {
    if let Some(employees) = departments.get_mut(department) {
        if let Ok(pos) = employees.binary_search(&employee.to_string()) {
            employees.remove(pos);
        } else {
            println!("Employee does not exist.")
        }

        if employees.is_empty() {
            departments.remove(department);
            println!("Removed {} from {} department", employee, department);
        }
    } else {
        println!("Department does not exist.")
    }
}

fn print_departments(departments: &HashMap<String, Vec<String>>) {
    for (department, employees) in departments {
        println!("Department: {}, Employees: {:?}", department, employees);
    }
}

fn print_department_employees(departments: &HashMap<String, Vec<String>>, department: &str) {
    match departments.get(department) {
        Some(employees) => println!("{:?}", employees),
        None => println!("No employees found in the {} department", department),
    }
}

fn extract_command(command: &str) -> Result<(String, String, String), String> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() > 3 {
        let action = parts[0].to_string();
        let name = parts[1].to_string();
        let department = parts[3].to_string();
        Ok((action, name, department))
    } else {
        Err("Input form is invalid. Expected format: '<action> <name> to  <department>".to_string())
    }
}
