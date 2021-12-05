use crate::accounts;
use aximate::Account;
use reqwest::Error;
use serde::Deserialize;
use tabular::{Row, Table};

const URL: &str = "https://game-api.axie.technology";

#[derive(Deserialize, Debug)]
pub struct SlpItem {
  client_id: String,
  total: u32,
  claimable_total: u32,
  last_claimed_item_at: u64,
  // update_time: u128,
}

pub async fn list_account_slp(
  accounts: Vec<Account>,
) -> Result<Vec<SlpItem>, Error> {
  let ronins: Vec<String> = accounts.into_iter().map(|a| a.ronin).collect();
  let url = format!("{}/slp/{}", URL, ronins.join(","));

  let response: Vec<SlpItem> =
    reqwest::Client::new().get(url).send().await?.json().await?;

  Ok(response)
}

pub async fn print_table(accounts: Vec<Account>, items: Vec<SlpItem>) {
  let mut table = Table::new("{:<} {:>} {:>} {:<} {:<}");
  table.add_row(
    Row::new()
      .with_cell("NAME")
      .with_cell("TOTAL")
      .with_cell("CLAIMABLE")
      .with_cell("NEXT CLAIM")
      .with_cell("LAST CLAIM"),
  );

  for slp in items {
    let eth_addr: &str = &slp.client_id;
    let ronin: &str = &aximate::addr::eth_to_ron(eth_addr);
    let account = accounts::find_account(accounts.clone(), ronin);

    table.add_row(
      Row::new()
        .with_cell(account.name)
        .with_cell(slp.total)
        .with_cell(slp.claimable_total)
        .with_cell("-")
        .with_cell(slp.last_claimed_item_at),
    );
  }

  print!("{}", table)
}
