// Define the struct outside any function
#[derive(Debug)] // Allows printing of struct with {:?}
pub struct User {
    name: String,
    age: u8,
    username: String,
    email: String,
}

impl User {
    // Constructor method
    pub fn new(name: String, age: u8, username: String, email: String) -> User {
        User {
            name,
            age,
            username,
            email,
        }
    }

    // Method to create a default person
    pub fn create_default_person() -> User {
        User {
            name: String::from("Alice"),
            age: 30,
            username: String::from("alice123"),
            email: String::from("alice@example.com"),
        }
    }

    // Method to print person details
    pub fn print_details(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Username: {}", self.username);
        println!("Email: {}", self.email);
    }
}
