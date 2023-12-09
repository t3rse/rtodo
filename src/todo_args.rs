use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TodoArgs {

    #[arg(short='a', long)]
    pub add: Option<String>,

    #[arg(short='l', long)]
    pub list: Option<String>,

    #[arg(short='r', long)]
    pub remove: Option<String>,

}

