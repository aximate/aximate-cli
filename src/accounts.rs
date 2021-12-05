use aximate::Account;
use std::fs::File;
use tabular::{Row, Table};

pub fn read_json(path: &str) -> Vec<Account> {
  let contents = File::open(path).unwrap();
  let accounts: Vec<Account> = serde_json::from_reader(contents).unwrap();
  return accounts;
}

pub fn find_account(accounts: Vec<Account>, ronin: &str) -> Account {
  return match accounts.iter().find(|&a| a.ronin == ronin) {
    Some(v) => v.clone(),
    None => Account {
      name: "MISSING".to_string(),
      ronin: "".to_string(),
      discord_id: "".to_string(),
    },
  };
}

pub fn print_table(accounts: Vec<Account>) {
  let mut table = Table::new("{:>} {:<} {:<}");
  table.add_row(
    Row::new()
      .with_cell("DISCORD ID")
      .with_cell("NAME")
      .with_cell("RONIN"),
  );

  for account in accounts {
    table.add_row(
      Row::new()
        .with_cell(account.discord_id)
        .with_cell(account.name)
        .with_cell(account.ronin),
    );
  }

  print!("{}", table);
}
