// Command 
pub trait  Command {
    fn handle(&self) -> i32;
}

// Add Command
pub struct AddCommand {
    // ...
}

impl AddCommand {
    pub fn new () -> Self{
        return  AddCommand {
            // .......
        }
    }

}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        println!("Adding the todo.....");
    
        0
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
        println!("Displaying all todo.....");

        0
    }
}