use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "flowstack")]
#[command(version = "1.0")]
#[command(about = "bash automation and testing scripts orchestrator")]

//# is an attribute that change the comportement of code, here Parser and Debug 
//derive is an attribute that asks rust to generate certain trait implementation
//This attribute enable to use this programme with --version --help, following what is in the .toml
#[command(version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands 
{
    Gen
    {
        //short for -c or -cpp
        #[arg(long = "cpp")]
        c_mode: bool,
    },
    Test
    {
        //long for --push_swap / --minishell / -cub3d
        #[arg(long)]
        minishell: bool,
        #[arg(long)]
        pushswap: bool,
        #[arg(long)]
        cub3d: bool,
    },
    Metrics 
    {
        //long for lines / comments / fcts
        #[arg(long)]
        lines: bool,
        fcts: bool,
        comments: bool,
    },
    Checkmake {},
    Renamebonus {},
    Readmegen {},
}

fn main()
{
    let cli = Cli::parse();

    match cli.command
    {
        Commands::Gen { c_mode } => 
        {
            if c_mode
            {
                println!("About to generate C++ project basis");
            }
            else
            {
                println!("About to generate C project basis");
            }
        }
        Commands::Test { minishell, pushswap, cub3d } =>
        {
            if minishell
            {
                println!("About to test minishell");
            }
            if pushswap
            {
                println!("About to test pushswap");
            }
            if cub3d
            {
                println!("About to test cub3d");
            }
            if !minishell && !pushswap && !cub3d
            {
                println!("tests available");
            }
        }
        Commands::Metrics { lines, fcts, comments } =>
        {
            if lines 
            {
                println!("About to count lines in project");
            }
            if fcts 
            {
                println!("About to count functions in project");
            }
            if comments 
            {
                println!("About to count comments in project");
            }
        }
        Commands::Checkmake {} =>
        {
            println!("About to check the Makefile");
        }
        Commands::Renamebonus {} =>
        {
            println!("About to setup the bonus folder");
        }
        Commands::Readmegen {} =>
        {
            println!("About to generate Readme");
        }
    }
}
//this program can be run using : 
//cargo run -- -n Franck -c 3
//cargo run -- --name Marvin --count 42
