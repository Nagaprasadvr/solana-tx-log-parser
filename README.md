# Solana Transaction Log Parser

# Usage

## From Tx Signature

```rust
let filter = "Instruction"; // Example filter
let tx_sig = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY";
let mut tx_log_parser = TxLogParser::new(tx_sig.to_string(), Some(filter), rpc_url);
tx_log_parser.parse().await?;
let tx_logs = tx_log_parser.get_logs();
tx_log_parser.print_logs();
```

## From UiTransactionWithStatusMeta

```rust
let tx_logs = TxLogParser::parse_from_tx(tx, None);
TxLogParser::print_logs_from_vec(tx_logs);
```

```bash
[
  "Program ComputeBudget111111111111111111111111111111 invoke [1]",
  "Program ComputeBudget111111111111111111111111111111 success",
  "Program ComputeBudget111111111111111111111111111111 invoke [1]",
  "Program ComputeBudget111111111111111111111111111111 success",
  "Program 11111111111111111111111111111111 invoke [1]",
  "Program 11111111111111111111111111111111 success",
  "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]",
  "Program log: Create",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program log: Instruction: GetAccountDataSize",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1569 of 119536 compute units",
  "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program 11111111111111111111111111111111 invoke [2]",
  "Program 11111111111111111111111111111111 success",
  "Program log: Initialize the associated token account",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program log: Instruction: InitializeImmutableOwner",
  "Program log: Please upgrade to SPL Token 2022 for immutable owner support",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 112949 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program log: Instruction: InitializeAccount3",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4188 of 109067 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 24988 of 129550 compute units",
  "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
  "Program DCJ9x4W4XhvWs6JbFzdZCVeJXacpFaYDCgaVh2ozW7ak invoke [1]",
  "Program log: Instruction: Execute",
  "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]",
  "Program log: Instruction: Buy",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
  "Program log: Instruction: Transfer",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 46450 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program 11111111111111111111111111111111 invoke [3]",
  "Program 11111111111111111111111111111111 success",
  "Program 11111111111111111111111111111111 invoke [3]",
  "Program 11111111111111111111111111111111 success",
  "Program data: vdt/007mYe5BydZn7ifb74pFuUCn7gJvKmpCVSqFGE9GOxo/PZhCj9uL4REAAAAAuV0aR9sHAAABtTEHVVVozSL2ufMAunSA11PSFlGqEl/V6US5W/D1DpLOsA1oAAAAAO371tEHAAAAxJvitrVnAwDtT7PVAAAAAMQD0GokaQIA",
  "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [3]",
  "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2003 of 34113 compute units",
  "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
  "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 36246 of 67510 compute units",
  "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
  "Program DCJ9x4W4XhvWs6JbFzdZCVeJXacpFaYDCgaVh2ozW7ak consumed 74130 of 104562 compute units",
  "Program DCJ9x4W4XhvWs6JbFzdZCVeJXacpFaYDCgaVh2ozW7ak success"
]
```

```bash
Transaction Logs:
[1] Instruction: GetAccountDataSize
[2] Instruction: InitializeImmutableOwner
[3] Instruction: InitializeAccount3
[4] Instruction: Execute
[5] Instruction: Buy
[6] Instruction: Transfer
```
