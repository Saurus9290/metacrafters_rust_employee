enum EmployeeType {
    JuniorEngineer,
    SeniorEngineer,
}

struct Employee {
    name: String,
    salary: u32,
    id: u32,
    employee_type: EmployeeType,
}

impl Employee {
    fn new(name: String, id: u32, employee_type: EmployeeType) -> Employee {
        let salary = match employee_type {
            EmployeeType::JuniorEngineer => 50_000,
            EmployeeType::SeniorEngineer => 60_000,
        };

        Employee {
            name,
            salary,
            id,
            employee_type,
        }
    }

    fn display_info(&self) {
        println!("Employee Name: {}", self.name);
        println!("Employee ID: {}", self.id);
        println!(
            "Employee Type: {}",
            match self.employee_type {
                EmployeeType::JuniorEngineer => "Junior Engineer",
                EmployeeType::SeniorEngineer => "Senior Engineer",
            }
        );
        println!("Employee Salary: ${}", self.salary);
    }
}

fn main() {
    let employee1 = Employee::new("Alice".to_string(), 1001, EmployeeType::JuniorEngineer);
    let employee2 = Employee::new("Bob".to_string(), 1002, EmployeeType::SeniorEngineer);

    employee1.display_info();
    println!(); // Just to add a blank line between the two employee displays
    employee2.display_info();
}
