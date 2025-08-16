use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "bash2rust")]
#[command(about = "bash automation and testing scripts wrapper in rust")]
#[command(version, about, long_about = None)]
pub struct Cli 
{
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands 
{
    Gen 
    {
        #[arg(long = "cpp")]
        cpp_mode: bool,
        #[arg(long = "clean")]
        clean: bool,
    },
    Test 
    {
        #[arg(long)]
        minishell: bool,
        #[arg(long)]
        pushswap: bool,
        #[arg(long)]
        cub3d: bool,
    },
    Checkmake {
        #[arg(long = "cpp")]
        cpp_mode: bool,
    },
    Renamebonus {},
    Srcupdate {},
}
