# Solana Execution Cost Example

A small, offline Rust example for exploring how a hypothetical transaction's network fee and tip affect its net result. It uses integer lamports rather than floating-point arithmetic and has no network access, wallet, or signing code.

## Run

Install a Rust toolchain, then run:

```sh
cargo run -- 1000000000 1002000000 5000 200000
```

The four arguments are **input**, **output**, **network fee**, and **tip**, all as non-negative integer lamports. This fictional example returns a gross change of 2,000,000 lamports and a net estimate of 1,795,000 lamports after the two costs.

```text
Gross change: 2000000 lamports (0.002000000 SOL)
Network fee:  5000 lamports
Tip:          200000 lamports
Net estimate: 1795000 lamports (0.001795000 SOL)
```

## Assumptions and limits

- Input and output represent the same asset, valued in lamports, over the same hypothetical round trip.
- The supplied network fee and tip are separate costs and are not already included in the input or output amounts. Avoid counting either twice.
- Other costs, transfers, rent, token balances, failed transactions, slippage, and changing market state are not modeled. Include only costs applicable to the transaction being analyzed.
- This is an illustrative calculation, not a quote, simulation, submission, finalized transaction, or proof of realized profit. Actual outcomes require finalized on-chain reconciliation.

## Verify

```sh
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```
