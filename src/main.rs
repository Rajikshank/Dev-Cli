use clap::{Parser,Subcommand};


#[derive(Debug)]
struct Bookmark{
    url:String,
    tags:Vec<String>,
}


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

        #[arg(short,long="tag")]
          tags:Vec<String>,  

    },

    List,
    Search{
        query :String
    },

}


fn main() {
  
  let cli =Cli::parse();

  match cli.command{
    Commands::Add {url,tags}=>{
        let bookmark= Bookmark{url,tags};

        println!("New bookmark created:");
       println!("{:#?}",bookmark);
        
        if bookmark.tags.is_empty(){
          println!("No bookmar added");
        }

        else{
            println!("Tags:");

            for tag in &bookmark.tags{
                println!("-{}",tag);
            }
        }
    }
    Commands::List=>{
        println!("Listing all bookmarks...");
    }

    Commands::Search {query}=>{
        println!("Searching for bookmarks matching: {}",query);
    }

  }

}
