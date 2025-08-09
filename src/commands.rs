
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
    println!("About to check the Makefile");
}

pub fn run_renamebonus() {println!("About to setup the bonus folder");}

pub fn run_readmegen() {println!("About to generate Readme");}
