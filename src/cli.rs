use crate::{accounts, slp};
use clap::{App, Arg, ArgMatches};

pub fn app() -> App<'static> {
  let arg_accounts_json = Arg::new("accounts-json")
    .long("accounts-json")
    .takes_value(true)
    .required(true)
    .env("AXIMATE_ACCOUNTS_JSON");

  return App::new("Aximate")
    .about("Utilities for interacting with the Axie Infinity ecosystem")
    .subcommand(
      App::new("get")
        .about("get one or more of a resources")
        .subcommand(
          App::new("account")
            .about("get one or more accounts")
            .arg(arg_accounts_json.clone()),
        )
        .subcommand(
          App::new("slp")
            .about("get the SLP of one or more accounts")
            .arg(arg_accounts_json),
        ),
    );
}

pub async fn match_app(matches: ArgMatches) {
  match matches.subcommand() {
    Some(("get", get_m)) => match get_m.subcommand() {
      Some(("account", account_m)) => cmd_get_account(account_m),
      Some(("slp", slp_m)) => cmd_get_slp(slp_m).await,
      _ => eprintln!("get subcommand is required, use help for details"),
    },
    _ => eprintln!("subcommand is required, use help for details"),
  }
}

fn cmd_get_account(m: &ArgMatches) {
  let path = m.value_of("accounts-json").unwrap();
  accounts::print_table(accounts::read_json(path));
}

async fn cmd_get_slp(m: &ArgMatches) {
  let path = m.value_of("accounts-json").unwrap();
  let accounts = accounts::read_json(path);
  let slp_res = slp::list_account_slp(accounts.clone()).await.unwrap();
  slp::print_table(accounts, slp_res).await;
}
