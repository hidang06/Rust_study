use std::{io, clone, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Student{
    name: String, 
    age: u8, 
}

#[derive(Debug, Clone)]
pub struct Class {
    inner: HashMap<String, Student>, 
}

impl Class {
    fn new() -> Self{
        Self { inner: HashMap::new(),
        }
    }
}

fn get_input() -> Option<String>{
    let mut buffer = String::new(); 
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    }
    else{
        Some(input)
    }
}

enum MainMenu{
    AddStudent, 
    ViewStudent, 
    RemoveStudent, 
    UpdateStudent,
}

impl MainMenu {
    fn choice(input: &str) -> Option<MainMenu>{
        match input {
            "1" => Some(MainMenu::AddStudent), 
            "2" => Some(MainMenu::ViewStudent), 
            "3" => Some(MainMenu::RemoveStudent), 
            "4" => Some(MainMenu::UpdateStudent),
            _ => None,
        }
    }
    fn show_choice(){
        println!("");
        println!("== Class Manager ==");
        println!("1. Add Student");
        println!("2. View Class");
        println!("3. Remove Student");
        println!("4. Update Student");
        println!("");
        println!("Enter selection: ");  
    }
}

fn main(){
    // let input = get_input();
    MainMenu::show_choice();
}