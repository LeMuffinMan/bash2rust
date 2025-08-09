mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() 
{
    let cli = Cli::parse();

    match cli.command 
    {
        Commands::Gen { cpp_mode } => commands::run_gen(cpp_mode),
        Commands::Test { minishell, pushswap, cub3d } =>
            commands::run_test(minishell, pushswap, cub3d),
        Commands::Metrics { lines, fcts, comments } =>
            commands::run_metrics(lines, fcts, comments),
        Commands::Checkmake {} => commands::run_checkmake(),
        Commands::Renamebonus {} => commands::run_renamebonus(),
        Commands::Readmegen {} => commands::run_readmegen(),
    }
}

