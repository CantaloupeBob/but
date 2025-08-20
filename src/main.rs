use clap::{Parser, Subcommand};
use std::fmt::Write;
use types::TransactionType;

mod types;

#[derive(Parser, Debug)]
#[command(name = "but", about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ToMd {
        #[arg(short, long)]
        note: Option<String>,
        #[arg(required = true)]
        path: String,
    },
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::ToMd { path, note } => {
            let json = std::fs::read_to_string(path).unwrap();
            let json: types::BroadcastJson = serde_json::from_str(&json).unwrap();

            let create_txs = json
                .transactions
                .iter()
                .filter(|tx| {
                    matches!(
                        tx.transaction_type,
                        TransactionType::Create | TransactionType::Create2
                    )
                })
                .collect::<Vec<_>>();

            let call_txs = json
                .transactions
                .iter()
                .filter(|tx| tx.transaction_type == TransactionType::Call)
                .collect::<Vec<_>>();

            let mut markdown = String::new();

            let earliest_receipt = json.receipts.iter().reduce(|a, b| {
                if a.block_number < b.block_number {
                    a
                } else {
                    b
                }
            });
            if let Some(earliest_receipt) = earliest_receipt {
                writeln!(
                    &mut markdown,
                    "## `{}` @ `{}`",
                    if earliest_receipt.block_number.starts_with("0x") {
                        usize::from_str_radix(&earliest_receipt.block_number[2..], 16)
                            .map(|x| x.to_string())
                            .unwrap_or_else(|_| earliest_receipt.block_number.clone())
                    } else {
                        earliest_receipt.block_number.clone()
                    },
                    json.commit
                )?;
            }
            if let Some(note) = note {
                writeln!(&mut markdown)?;
                writeln!(&mut markdown, "**Note:** {}", note)?;
            }

            if !create_txs.is_empty() {
                writeln!(&mut markdown, "### Deployed contracts")?;
                writeln!(&mut markdown)?;
                writeln!(&mut markdown, "| Name | Address |  Tx  |")?;
                writeln!(&mut markdown, "| ---- | ------- | ---- |")?;
                for create_tx in create_txs {
                    writeln!(
                        &mut markdown,
                        "| `{}` | [`{}`]({}) | [`{}`]({}) |",
                        create_tx.contract_name.clone().unwrap_or_default(),
                        create_tx.contract_address,
                        contract_explorer_link(create_tx),
                        create_tx.hash,
                        tx_explorer_link(create_tx)
                    )?;
                }
            }

            if !call_txs.is_empty() {
                writeln!(&mut markdown)?;
                writeln!(&mut markdown, "### Calls")?;
                writeln!(&mut markdown)?;
                writeln!(&mut markdown, "| Name | Address | Function | Args |  Tx  |")?;
                writeln!(&mut markdown, "| ---- | ------- | -------- | ---- | ---- |")?;
                for call_tx in call_txs {
                    writeln!(
                        &mut markdown,
                        "| `{}` | [`{}`]({}) | `{}` | <ol>{}</ol> | [`{}`]({}) |",
                        call_tx.contract_name.clone().unwrap_or_default(),
                        call_tx.contract_address,
                        contract_explorer_link(call_tx),
                        call_tx.function.clone().unwrap_or_default(),
                        call_tx
                            .arguments
                            .clone()
                            .unwrap_or_default()
                            .iter()
                            .map(|arg| format!("<li><code>{}</code></li>", arg))
                            .collect::<Vec<_>>()
                            .join(""),
                        call_tx.hash,
                        tx_explorer_link(call_tx)
                    )?;
                }
            }

            print!("{}", markdown);
        }
    }

    Ok(())
}

fn chain_id_to_explorer(chain_id: &str) -> String {
    let chain_id = if let Some(stripped) = chain_id.strip_prefix("0x") {
        usize::from_str_radix(stripped, 16)
    } else {
        chain_id.parse::<usize>()
    }
    .unwrap_or_else(|_| panic!("cannot parse chain id: {}", chain_id));

    match chain_id {
        0xa4b1 => "https://arbiscan.io",
        8453 => "https://basescan.org",
        999 => "https://hyperevmscan.io",
        1 => "https://etherscan.io",
        _ => panic!("Unknown chain id: {}", chain_id),
    }
    .to_string()
}

fn tx_explorer_link(tx: &types::Transaction) -> String {
    format!(
        "{}/tx/{}",
        chain_id_to_explorer(&tx.transaction.chain_id),
        tx.hash
    )
}

fn contract_explorer_link(tx: &types::Transaction) -> String {
    format!(
        "{}/address/{}",
        chain_id_to_explorer(&tx.transaction.chain_id),
        tx.contract_address
    )
}
