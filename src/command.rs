use std::fs;
use std::io::Write;

// Command 
pub trait  Command {
    fn handle(&self) -> i32;
}

// Add Command
pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new (args: Vec<String>) -> Self{
        return  AddCommand {
            args
        }
    }

}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        // dbg!(&self.args);

        let description_option = &self.args.get(2);
        

        if let Some(description) = description_option {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("Storage.txt")
                .expect("File not found");

            writeln!(file, "{description}").expect("File not Writable");

            println!("Todo added");

            return 0;
        } else {
            print!("Description is required \n");

            return 1;
        }
    }
}














// List Command

pub struct ListCommand {
    // ...
}

impl ListCommand {
    pub fn new () -> Self{
        return  ListCommand {
            // .......
        }
    }

    
    
}
impl Command for ListCommand {
    
    fn handle(&self) -> i32 {
        let contents = fs::read_to_string("Storage.txt").expect("File not found.");

        println!("{contents}");
        0
    }
}