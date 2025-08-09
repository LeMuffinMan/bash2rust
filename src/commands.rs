use std::process::Command;
use std::env;

pub fn run_gen(cpp_mode: bool)
{
    if cpp_mode { 
        println!("About to generate C++ project starter");
    } else {
        println!("About to generate C project starter");
    }
}

pub fn run_test(minishell: bool, pushswap: bool, cub3d: bool)
{
    if minishell {
        println!("About to test minishell");
    }
    if pushswap  {
        println!("About to test push_swap");
    }
    if cub3d {
        println!("About to test cub3d");
    } 
    if !minishell && !pushswap && !cub3d {
        println!("Available testers : \n- Minishell\n- Push_swap\n- cub3d") 
    }
}

pub fn run_metrics(lines: bool, fcts: bool, comments: bool) {
    if lines {
        println!("About to count lines in project");
    }
    if fcts {
        println!("About to count functions in project");
    }
    if comments {
        println!("About to count comments in project");
    }
    if !lines && !fcts && !comments {
        println!("Metrics available : lines, functions, comments")
    }
}

pub fn run_checkmake()
{
    //We want to execute the script CanIPush42.sh which depend of the utils.sh script at the same
    //path
    //BUT we want to execute CanIPush42.sh from anywhere on the system
    println!("About to check the Makefile");

    //mut because we want to modify it to add the path of our scripts 
    //env::var("PATH") : try to get the PATH variable from env of our program
    //unwrap_or_default : if it does not exist, we return an empty string ""
    let mut path = env::var("PATH").unwrap_or_default();
    let home = env::var("HOME").expect("HOME variable not set");
    let scripts_path = format!("{}/.local/scripts", home);

    // println!("{:?}", scripts_path);
    let status = Command::new("bash")
        .arg("-c")
        .arg("$HOME/.local/scripts/CanIPush42/CanIPush42.sh")
        .status();

    match status
    {
        Ok(s) if s.success() => println!("Script executed successfully"),
        Ok(s) => eprintln!("Script exited with status: {}", s),
        Err(e) => eprintln!("Failed to execute script: {}", e),
    }
}

pub fn run_renamebonus() {println!("About to setup the bonus folder");}

pub fn run_readmegen() {println!("About to generate Readme");}
