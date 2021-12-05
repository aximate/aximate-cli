mod accounts;
mod cli;
mod slp;

#[tokio::main]
async fn main() {
  cli::match_app(cli::app().get_matches()).await;
}
