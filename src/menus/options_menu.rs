

pub fn get_name() -> &'static str{
    return "options-menu"
}

pub fn run_command(command_string: &str) -> Result<(), String>{
    let args: Vec<&str> = command_string.split_whitespace().collect();
    println!("COMMAND: {}",command_string);
    match args[0]{
        "set_val" => Ok(test_funct()),
        "" => Ok(test_funct_args(&args)),
        _ => Err(format!("Function {} Not Found", args[0]))
    }
}


pub fn load() -> Result<(),String>{
    println!("LOADING OPTIONS_MENU");

    Ok(())
}
pub fn unload() -> Result<(),String>{
    println!("UNLOADING OPTIONS_MENU");
    Ok(())
}
pub fn help() -> String{
    let mut help_str = String::new();
    help_str.push_str("I'll Fill this in eventually");
    return help_str
}


pub fn test_funct() -> (){
    println!("TEST MENU Testing!!!")
}

pub fn test_funct_args(args: &Vec<&str>) -> () {
    println!("test_funct: {} {} {} ", args[0],args[1],args[2])
}






