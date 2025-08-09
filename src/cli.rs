use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "flowstack")]
#[command(about = "bash automation and testing scripts orchestrator")]
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
    Metrics 
    {
        #[arg(long)]
        lines: bool,
        #[arg(long)]
        fcts: bool,
        #[arg(long)]
        comments: bool,
    }, 
    Checkmake {},
    Renamebonus {},
    Readmegen {},
}
