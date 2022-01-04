use std::io;

struct UserDefinition {
    first_name: String,
    last_name: String,
    male: String,
    age: u8,
    is_blank: bool
}


struct Commands {
    user_create: String,
    user_update: String,
    user_info: String,
    coomands: String,
    exit: String 
}


fn set_application_commands() -> Commands {
     Commands {
        user_create: String::from("CREATE"),
        user_update: String::from("UPDATE"),
        user_info: String::from("INFO"),
        coomands: String::from("COMMANDS"),
        exit: String::from("EXIT")
    }
}


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


fn get_list_commands(commands: &Commands) {
    println!("Create User command: {}", commands.user_create);
    println!("Update User command: {}", commands.user_update);
    println!("Ger User info command: {}", commands.user_info);
    println!("Get List Commands command: {}", commands.coomands);
    println!("Shut down Application command {}", commands.exit);
}

fn create_user() -> UserDefinition {
 
    let (first_name, last_name, male, age) = get_user_input();

    let age: u8 = age.trim().parse().expect("Wrong number");

    println!("User was created");

    return UserDefinition {
        first_name: first_name,
        last_name: last_name,
        male: male,
        age: age,
        is_blank: false
    }
}


fn update_user (user: &mut UserDefinition) {

    if user.is_blank {
        println!("User was not created");
        return;
    }

    println!("Change INFO or press ENTER to use current \n");
    let (first_name, last_name, male, age) = get_user_input();

    user.first_name = if first_name == "\n" { user.first_name.clone() } else { first_name };
    user.last_name = if last_name == "\n" { user.last_name.clone() } else { last_name } ;
    user.male = if male == "\n" { user.male.clone() } else { male } ;
    user.age = if age == "\n" { user.age.clone() } else { age.trim().parse().expect("Wrong number") };
}


fn get_user_info(user: &UserDefinition) {
    if user.is_blank {
        println!("User was not created, please called CREATE command");
    } else {
        println!(
            "User Info:\n \nFIRST NAME:{}\nLAST NAME:{}\nMALE:{}\nAGE:{}\n"
            , user.first_name, user.last_name, user.male, user.age
        );
    }
}


fn create_blank_user () -> UserDefinition {
    UserDefinition {
        first_name: String::from(""),
        last_name: String::from(""),
        male: String::from("UNKOWN"),
        age: 0,
        is_blank: true
    }
}


fn main() {
    
    let commands: Commands = set_application_commands();

    let mut user: UserDefinition = create_blank_user(); 

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

            user = create_user();

        } else if command == commands.user_update {
            update_user(&mut user);
        } else if command == commands.user_info {
            get_user_info(&user);            
        } else if command == commands.coomands {
            get_list_commands(&commands);
        } else if command == commands.exit {
            break;
        }
        else {
            println!("Invalid command was entered {}", command)
        }
    }

    println!("Programm Shutdown")
}