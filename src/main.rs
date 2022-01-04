use std::io;


fn get_user_input() -> (String, String, String, String) {
    let mut first_name: String = String::new();
    let mut last_name: String = String::new();
    let mut male: String = String::new();
    let mut age: String = String::new();
    
    println!("Enter a FIRST NAME:");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Wrong Input");
    
    println!("Enter a LAST NAME");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Wrong input");

    println!("Enter a MALE");
    io::stdin()
        .read_line(&mut male)
        .expect("Wrong Input");

    println!("Enter a AGE");
    io::stdin()
        .read_line(&mut age)
        .expect("Wrong input");

    return (first_name, last_name, male, age);
}


struct UserDefinition {
    first_name: String,
    last_name: String,
    male: String,
    age: u8,
    is_blank: bool
}


impl UserDefinition { 
    fn create_user(&mut self) {
        
        if self.is_blank == false {
            println!("User already created");
            return;
        }

        let (first_name, last_name, male, age) = get_user_input();

        let age: u8 = age.trim().parse().expect("Wrong number");

        println!("User was created");

        self.first_name = first_name;
        self.last_name = last_name;
        self.male = male;
        self.age = age;
        self.is_blank = false;

    }
}


impl UserDefinition {
    fn update_user (&mut self) {

        if self.is_blank {
            println!("User was not created");
            return;
        }

        println!("Change INFO or press ENTER to use current \n");
        let (first_name, last_name, male, age) = self::get_user_input();

        self.first_name = if first_name == "\n" { self.first_name.clone() } else { first_name };
        self.last_name = if last_name == "\n" { self.last_name.clone() } else { last_name } ;
        self.male = if male == "\n" { self.male.clone() } else { male } ;
        self.age = if age == "\n" { self.age.clone() } else { age.trim().parse().expect("Wrong number") };
    }
}
    

impl UserDefinition {
    fn get_user_info(& self) {
        if self.is_blank {
            println!("User was not created, please called CREATE command");
        } else {
            println!(
                "User Info:\n \nFIRST NAME:{}\nLAST NAME:{}\nMALE:{}\nAGE:{}\n"
                , self.first_name, self.last_name, self.male, self.age
                );
            }
        }
}


impl UserDefinition {
    fn create_blank_user () -> UserDefinition {
        UserDefinition {
            first_name: String::from(""),
            last_name: String::from(""),
            male:  String::from("UNKOWN"),
            age: 0,
            is_blank: true,
        }
    }
}


struct Commands {
    user_create: String,
    user_update: String,
    user_info: String,
    coomands: String,
    exit: String 
}


impl Commands {
    fn get_list_commands(&self) {
        println!("Create User command: {}", self.user_create);
        println!("Update User command: {}", self.user_update);
        println!("Ger User info command: {}", self.user_info);
        println!("Get List Commands command: {}", self.coomands);
        println!("Shut down Application command {}", self.exit);
    }
}


impl Commands {
    fn set_application_commands() -> Commands {
        Commands {
            user_create: String::from("CREATE"),
            user_update: String::from("UPDATE"),
            user_info: String::from("INFO"),
            coomands: String::from("COMMANDS"),
            exit: String::from("EXIT"),
        }
    }
}


fn main() {

    let commands: Commands = Commands::set_application_commands();

    let mut user: UserDefinition = UserDefinition::create_blank_user(); 

    println!("Application was started");

    loop {
        println!("Enter a command:");

        let mut command: String = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        
        command = command.trim().parse().expect("Invalid Entering");
        
        println!("Command: {}", command);

        if command == commands.user_create {

            if user.is_blank == false {
                println!("User already created");
                continue;

            }

            user.create_user();

        } else if command == commands.user_update {
            user.update_user();
        } else if command == commands.user_info {
            user.get_user_info();            
        } else if command == commands.coomands {
            commands.get_list_commands();
        } else if command == commands.exit {
            break;
        }
        else {
            println!("Invalid command was entered {}", command)
        }
    }

    println!("Programm Shutdown")
}