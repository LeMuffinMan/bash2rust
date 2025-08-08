use clap::Parser;

//# is an attribute that change the comportement of code, here Parser and Debug 
//derive is an attribute that asks rust to generate certain trait implementation
#[derive(Parser, Debug)]
//This attribute enable to use this programme with --version --help, following what is in the .toml
#[command(version, about, long_about = None)]
struct Args
{
    //arg could be -g or --gen
    #[arg(short, long)]
    name: String,

    //same as before, and we set a default value in case the program is run without -c / --count x 
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main()
{
    //#[derive(Debug)] allow us to use the :? to indicate rust to use the the trait Debug to
    //display and format args
    // println!("{:?}", args);

    //call methode parse defined in the crate Parser, and set with #[derive(Parser)]
    //Parse extract the argument line sent executing this program
    let args = Args::parse();
    //it fills the struct args :
    //it looks for a short or long name (-n --name) and count (-c --count) to fill fields as a
    //string and an unsigned int on 8 bits

    //_ is a throwable variable, we could use i if we want to use it later
    // then we have a range : from the argument[0] until argument[args.count]
    for _ in 0..args.count
    {
        println!("Hello {}!", args.name);
    }
}
//this program can be run using : 
//cargo run -- -n Franck -c 3
//cargo run -- --name Marvin --count 42
