use clap::{Parser,Subcommand};


#[derive(Parser)]
#[command(name="devlink")]
#[command(version="0.1.0")]
#[command(about="Save and search developer bookmarks")]


struct Cli{

    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand)]
enum Commands{
    
    Add{
        url:String,
    },

    List,
    Search{
        query :String
    },

}


fn main() {
  
}
