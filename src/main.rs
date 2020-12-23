use clap::App;

mod authoring;
mod document;
mod publishing;

#[tokio::main]
async fn main() {
    let matches = App::new("HashtagWiki")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sander Dijkhuis <mail@sanderdijkhuis.nl>")
        .subcommand(App::new("author").about("serves dynamically locally"))
        .subcommand(App::new("publish").about("writes static output to out dir"))
        .get_matches();

    match matches.subcommand() {
        None => println!("Use the help subcommand for help"),
        Some(("author", _)) => authoring::serve(([127, 0, 0, 1], 3030)).await,
        Some(("publish", _)) => publishing::run(),
        Some((&_, _)) => println!("Unknown input"),
    }
}
