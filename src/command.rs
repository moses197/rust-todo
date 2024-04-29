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

        let description_option: &Option<&String> = &self.args.get(2);
        

        if let Some(description) = description_option {
            let mut file: fs::File = fs::OpenOptions::new()
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
        let contents: String = fs::read_to_string("Storage.txt").expect("File not found.");

        println!("{contents}");
        0
    }
}


// --- test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command() {

        let args: Vec<String> = vec![
            "todo".to_string(),
            "add".to_string(),
            "My Todo 4".to_string(),
        ];
        let command: AddCommand = AddCommand::new(args);


        // Act 
        let exit_code: i32 = command.handle();

        // Assert
        assert_eq!(0, exit_code);
    }
}


//// List Test



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_command() {

        let args = vec![
            "todo".to_string(),
            "add".to_string(),
            "My Todo 4".to_string(),
        ];
        let command = AddCommand::new(args);


        // Act 
        let exit_code = command.handle();

        // Assert
        assert_eq!(0, exit_code);
    }

    #[test]
    fn list_command() {

        // Prepare
        let command = ListCommand::new();


        // Act 
        let exit_code = command.handle();

        // Assert
        assert_eq!(0, exit_code);
    }
}