#![feature(collections)]

use std::io::prelude::*;

fn display_choices<T:std::fmt::Display>(choices: &[T]) -> i64 {
    let mut ret;
    
    loop
    {
        for (index, choice) in choices.iter().enumerate() {
            println!("{}. {}", index + 1, choice);
        }

        let stdin = std::io::stdin();
        let mut choice_text = String::new();
        stdin.lock().read_line(&mut choice_text).ok().expect("failed to read choice");
        let choice_opt: Option<i64> = choice_text.trim().parse::<i64>().ok();
        
        let choice = match choice_opt {
            Some(choice)             => choice,
            None                     => -1
        };

        if choice < 1 || choice > choices.len() as i64 {
            println!("Please input a logical number as choice!");
            continue;
        }

        ret = choice;
        break;
    }

    ret
}

use std::convert::AsRef;

fn does_user_answer_yes() -> bool {
    let mut answer = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut answer).ok().expect("failed to read answer");

    let mut does_he_answer_yes;

    match answer.to_lowercase().as_ref() {
        "yes" | "yeah" | "yea" | "sure" => {
            does_he_answer_yes = true;
        },
        _ => {
            does_he_answer_yes = false;
        }
    }

    does_he_answer_yes
}

fn read_line<T:std::str::FromStr>() -> T {
    let mut t_text = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut t_text).ok().expect("failed to read something");

    let t_opt: Option<T> = t_text.trim().parse::<T>().ok();

    let _t = match t_opt {
        Some(_t) => _t,
        None     => unimplemented!()
    };

    _t
}

fn add_nginx_proxy_service() {
    println!("Does your service run inside a Docker container ?");
    
    let is_in_docker = does_user_answer_yes();
    
    if is_in_docker {
        println!("Can you give me the ID or the name of the container?");
        unimplemented!();
    } else {
        println!("Oooh... Does it runs on localhost ?");
        let is_in_localhost = does_user_answer_yes();

        if is_in_localhost {
            println!("Then, it is possible ! On which port is it running?");
            let port = read_line::<u32>();
            unimplemented!(); 
        } else {
            println!("Hmm, it will be complicated. As now, I can't do it.");
        }
    }
        
}

fn display_help() {
}

fn exit() {
}

fn main() {
    println!("Welcome into the Rust Manager, select what you want to do:");

    let actions = ["Add a nginx proxy service", "Help", "Exit"];
    let action_choice = display_choices(&actions);

    match action_choice {
        1 => add_nginx_proxy_service(),
        2 => display_help(),
        3 => exit(),
        _ => unreachable!()
    }
}
