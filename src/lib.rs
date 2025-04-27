use std::{collections::HashMap, str::FromStr};

use regex::Regex;
use solana_rpc_client::nonblocking::rpc_client;
use solana_rpc_client_api::config::RpcTransactionConfig;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status_client_types::{
    UiTransactionEncoding, UiTransactionStatusMeta, option_serializer::OptionSerializer,
};

pub const PROGRAM_LOG_PREFIX: &str = "Program log:";
pub const COMPUTE_UNIT_LOG_DISC: &str = "compute units";

pub type ComputeUnitLog = HashMap<Pubkey, u64>;

#[derive(Debug, Clone)]
pub struct TxLogParser {
    pub rpc_url: String,
    pub tx_sig: String,
    pub log_filter: Option<String>,
    pub include_cu_logs: bool,
    pub tx_logs: Option<Vec<String>>,
    pub compute_unit_logs: Option<ComputeUnitLog>,
    pub compute_units_consumed: Option<u64>,
}

impl TxLogParser {
    pub fn new(
        rpc_url: String,
        tx_sig: String,
        log_filter: Option<&str>,
        include_cu_logs: bool,
    ) -> Self {
        TxLogParser {
            tx_sig,
            log_filter: log_filter.map(|s| s.to_string()),
            rpc_url,
            include_cu_logs,
            tx_logs: None,
            compute_units_consumed: None,
            compute_unit_logs: None,
        }
    }

    pub async fn parse(&mut self) -> Result<(), String> {
        let cu_regex = Regex::new(r"Program (\w+) consumed (\d+) of (\d+) compute units")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        let rpc = rpc_client::RpcClient::new_with_commitment(
            self.rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );

        let tx_sig = Signature::from_str(&self.tx_sig)
            .map_err(|_| format!("Invalid transaction signature: {}", self.tx_sig))?;

        let tx = rpc
            .get_transaction_with_config(
                &tx_sig,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::JsonParsed),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            )
            .await
            .map_err(|e| format!("Failed to get transaction: {}", e))?;

        let mut tx_logs: Vec<String> = Vec::new();
        let mut compute_unit_logs: ComputeUnitLog = ComputeUnitLog::new();

        if let Some(meta) = tx.transaction.meta {
            if let OptionSerializer::Some(logs) = meta.log_messages {
                for log in logs {
                    if log.contains(PROGRAM_LOG_PREFIX) {
                        let mut log = log.replace(&PROGRAM_LOG_PREFIX, "");
                        log = log.trim().to_string();

                        if log.is_empty() {
                            continue;
                        }
                        tx_logs.push(log);
                    }
                    if self.include_cu_logs {
                        if log.contains(COMPUTE_UNIT_LOG_DISC) {
                            if let Some(captures) = cu_regex.captures(&log) {
                                let program_id = Pubkey::from_str(&captures[1])
                                    .map_err(|_| format!("Invalid program ID: {}", &captures[1]))?;
                                let consumed: u64 = captures[2].parse().unwrap();
                                compute_unit_logs.insert(program_id, consumed);
                            } else {
                                println!("No match found!");
                            }
                        }
                    }
                }
            }

            if let OptionSerializer::Some(compute_units) = meta.compute_units_consumed {
                self.compute_units_consumed = Some(compute_units);
            }
        }

        if let Some(ref log_filter) = self.log_filter {
            tx_logs.retain(|log| log.to_lowercase().contains(&log_filter.to_lowercase()));
        }

        self.tx_logs = Some(tx_logs);

        if self.include_cu_logs {
            self.compute_unit_logs = Some(compute_unit_logs);
        }

        Ok(())
    }

    pub fn get_tx_logs(&self) -> Vec<String> {
        self.tx_logs
            .as_ref()
            .map_or(Vec::new(), |logs| logs.clone())
    }

    pub fn get_cu_logs(&self) -> ComputeUnitLog {
        self.compute_unit_logs
            .as_ref()
            .map_or(ComputeUnitLog::new(), |logs| logs.clone())
    }

    pub fn get_compute_units_consumed(&self) -> Option<u64> {
        self.compute_units_consumed
    }

    pub fn print_tx_logs(&self) {
        if let Some(ref logs) = self.tx_logs {
            println!("Transaction Logs:");
            for (idx, log) in logs.iter().enumerate() {
                println!("[{}] {}", idx + 1, log);
            }
        } else {
            println!("No logs found.");
        }
    }

    pub fn print_cu_logs(&self) {
        if let Some(ref logs) = self.compute_unit_logs {
            println!("Compute Unit Logs:");
            for (program_id, consumed) in logs.iter() {
                println!("Program ID: {}, Consumed: {}", program_id, consumed);
            }
        } else {
            println!("No compute unit logs found.");
        }
    }

    pub fn print_logs_from_vec(logs: &Vec<String>) {
        println!("Transaction Logs:");
        for (idx, log) in logs.iter().enumerate() {
            println!("[{}] {}", idx + 1, log);
        }
    }

    pub fn parse_from_tx(tx: &UiTransactionStatusMeta, log_filter: Option<String>) -> Vec<String> {
        let mut tx_logs: Vec<String> = Vec::new();

        if let OptionSerializer::Some(ref logs) = tx.log_messages {
            for log in logs {
                if log.contains(PROGRAM_LOG_PREFIX) {
                    let mut log = log.replace(&PROGRAM_LOG_PREFIX, "");
                    log = log.trim().to_string();

                    if log.is_empty() {
                        continue;
                    }
                    tx_logs.push(log);
                }
            }
        }

        if let Some(ref log_filter) = log_filter {
            tx_logs.retain(|log| log.to_lowercase().contains(&log_filter.to_lowercase()));
        }

        tx_logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_tx_log_parser() {
        let rpc_url = env::var("RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let tx_sig = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY";
        let mut parser = TxLogParser::new(rpc_url, tx_sig.to_string(), None, false);
        let logs = parser.parse().await;
        assert!(logs.is_ok());

        parser.print_tx_logs();
    }

    #[tokio::test]
    async fn test_tx_log_parser_with_filter() {
        let rpc_url = env::var("RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let tx_sig = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY";
        let mut parser = TxLogParser::new(rpc_url, tx_sig.to_string(), Some("Instruction"), false);
        let logs = parser.parse().await;
        assert!(logs.is_ok());

        parser.print_tx_logs();

        let logs = parser.get_tx_logs();

        let compute_units_consumed = parser.get_compute_units_consumed();

        assert!(compute_units_consumed.is_some());

        assert!(!logs.is_empty());
    }

    #[tokio::test]
    async fn test_tx_log_parser_with_cu_logs() {
        let rpc_url = env::var("RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let tx_sig = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY";
        let mut parser = TxLogParser::new(rpc_url, tx_sig.to_string(), None, true);
        let logs = parser.parse().await;
        assert!(logs.is_ok());

        parser.print_cu_logs();

        let logs = parser.get_cu_logs();

        assert!(!logs.is_empty());
    }
}
