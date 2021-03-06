#![allow(dead_code)]
mod title;
mod black_jack_tools;

mod player_strategies;
mod tests;
mod menus;


use std;
use std::io::Write;


fn main() {
    type RunCommand = fn(command_string: &str) -> Result<(), String>;
    println!("{}",title::TITLE);


    let mut curr_menu = "game_menu".to_string();
    let mut run_command_funct: RunCommand = menus::game_menu::run_command;
    print!("{} >",curr_menu);
    loop{
        let mut str_buff = String::new();

        std::io::stdout().flush().unwrap();
        let _comm_len = std::io::stdin().read_line(&mut str_buff);
        let command = str_buff.trim();


        let mut command_chunks = str_buff.split_whitespace();
        match command_chunks.next(){

            Some("quit") => break,
            Some("set_menu") => {
                match command_chunks.next(){
                    Some(menu_name) => {
                        run_command_funct = match menu_name {
                            "main_menu" => {
                                curr_menu = menu_name.to_string();
                                menus::main_menu::run_command
                            },
                            "game_menu" => {
                                curr_menu = menu_name.to_string();
                                menus::game_menu::run_command
                            },

                            _ => {
                                println!("Menu {} not found",menu_name);
                                run_command_funct
                            }

                        };

                    },
                    None => break
                }
            }
            Some(_)=> {
                match run_command_funct(command){
                    Ok(_) => (),
                    Err(err_str) => println!("{}",err_str)
                }
                ()
            },
            None => {
                print!("{} >",curr_menu);
                continue
            }
        }
        print!("{} >",curr_menu);

    }

    println!("Exiting");

}



