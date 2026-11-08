use solana_execution_costs::{estimate, format_sol};
use std::{env, process::ExitCode};

const USAGE: &str = "Usage: solana-execution-costs <input> <output> <network-fee> <tip>\nAll amounts are non-negative integer lamports.";

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        return Err(USAGE.to_owned());
    }
    let mut amounts = [0_u64; 4];
    for (i, arg) in args.iter().enumerate() {
        amounts[i] = arg
            .parse()
            .map_err(|_| format!("Invalid lamport amount: {arg}\n{USAGE}"))?;
    }
    let result = estimate(amounts[0], amounts[1], amounts[2], amounts[3]);
    println!(
        "Gross change: {} lamports ({} SOL)",
        result.gross_lamports,
        format_sol(result.gross_lamports)
    );
    println!("Network fee:  {} lamports", amounts[2]);
    println!("Tip:          {} lamports", amounts[3]);
    println!(
        "Net estimate: {} lamports ({} SOL)",
        result.net_lamports,
        format_sol(result.net_lamports)
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
