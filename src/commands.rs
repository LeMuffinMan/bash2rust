use std::process::Command;
use std::env;

pub fn execute_script(interpretor: &str, path: &str) {
    let status = Command::new(interpretor)
        .arg(&path)
        .status();

    match status
    {
        Ok(s) if s.success() => println!("Script executed successfully"),
        Ok(s) => eprintln!("Script executed but failed with status: {}", s),
        Err(e) => eprintln!("Failed to execute script: {}", e),
    }
}

pub fn run_srcupdate() {
    let home = env::var("HOME").expect("HOME variable not set");
    let script_path = format!("{}/.local/scripts/42cpp-project-starter/scripts/src_updater.sh", home);

    execute_script("bash", &script_path);
}

pub fn run_gen(cpp_mode: bool) {
    let home = env::var("HOME").expect("HOME variable not set");
    let script_path = if cpp_mode {
        format!("{}/.local/scripts/42cpp-project-starter/project-starter/run.py --cpp", home)
    } else {
        format!("{}/.local/scripts/42cpp-project-starter/project-starter/run.py", home)
    };

    execute_script("python", &script_path);
}

pub fn run_test(minishell: bool, pushswap: bool, cub3d: bool) {
    //args en plus ? : recipes ?
    if minishell {
        let home = env::var("HOME").expect("HOME variable not set");
        let script_path = format!("{}/.local/scripts/minishell_tester/muffinette.sh", home);

        execute_script("bash", &script_path);
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

pub fn run_checkmake(cpp_mode: bool) {
    let home = env::var("HOME").expect("HOME variable not set");
    let script_path = if cpp_mode {
        format!("{}/.local/scripts/CanIPush42/CanIPush42.sh --cpp", home)
    } else {
        format!("{}/.local/scripts/CanIPush42/CanIPush42.sh --cpp", home)
    };

    execute_script("bash", &script_path);
}

pub fn run_renamebonus() {
    let home = env::var("HOME").expect("HOME variable not set");
    let script_path = format!("{}/.local/scripts/rename_bonus.sh", home);

    execute_script("bash", &script_path);
}

