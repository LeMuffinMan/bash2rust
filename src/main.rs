mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() 
{
    let cli = Cli::parse();

    match cli.command 
    {
        Commands::Gen { cpp_mode, clean } => commands::run_gen(cpp_mode, clean),
        Commands::Test { minishell, pushswap, cub3d } =>
            commands::run_test(minishell, pushswap, cub3d),
        Commands::Checkmake { cpp_mode } => commands::run_checkmake(cpp_mode),
        Commands::Renamebonus {} => commands::run_renamebonus(),
        Commands::Srcupdate {} => commands::run_srcupdate(),
    }
}

