use std::str::FromStr;

use solana_rpc_client::nonblocking::rpc_client;
use solana_rpc_client_api::config::RpcTransactionConfig;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status_client_types::{
    UiTransactionEncoding, option_serializer::OptionSerializer,
};

const LOG_PREFIX: &str = "Program log:";

#[derive(Debug, Clone)]
pub struct TxLogParser {
    pub rpc_url: String,
    pub tx_sig: String,
    pub log_filter: Option<String>,
    pub tx_logs: Option<Vec<String>>,
}

impl TxLogParser {
    pub fn new(tx_sig: String, log_filter: Option<&str>, rpc_url: String) -> Self {
        TxLogParser {
            tx_sig,
            log_filter: log_filter.map(|s| s.to_string()),
            rpc_url,
            tx_logs: None,
        }
    }

    pub async fn parse(&mut self) -> Result<(), String> {
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

        if let Some(meta) = tx.transaction.meta {
            if let OptionSerializer::Some(logs) = meta.log_messages {
                for log in logs {
                    if log.contains(LOG_PREFIX) {
                        let mut log = log.replace(&LOG_PREFIX, "");
                        log = log.trim().to_string();

                        if log.is_empty() {
                            continue;
                        }
                        tx_logs.push(log);
                    }
                }
            }
        }

        if let Some(ref log_filter) = self.log_filter {
            tx_logs.retain(|log| log.to_lowercase().contains(&log_filter.to_lowercase()));
        }

        self.tx_logs = Some(tx_logs);

        Ok(())
    }

    pub fn print_logs(&self) {
        if let Some(ref logs) = self.tx_logs {
            println!("Transaction Logs:");
            for (idx, log) in logs.iter().enumerate() {
                println!("[{}] {}", idx + 1, log);
            }
        } else {
            println!("No logs found.");
        }
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
        let tx_sig = "4qKcAD1PaSoEfCrHzs9UQr5LZ9A1vBCLSMysyE3erfV4PUh4kaWvUS1nWZrYx6vaHsACkBpVJBpjatu5K7yZXZi9";
        let mut parser = TxLogParser::new(tx_sig.to_string(), None, rpc_url);
        let logs = parser.parse().await;
        assert!(logs.is_ok());

        parser.print_logs();
    }

    #[tokio::test]
    async fn test_tx_log_parser_with_filter() {
        let rpc_url = env::var("RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let tx_sig = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY";
        let mut parser = TxLogParser::new(tx_sig.to_string(), Some("Instruction"), rpc_url);
        let logs = parser.parse().await;
        assert!(logs.is_ok());

        parser.print_logs();
    }
}
