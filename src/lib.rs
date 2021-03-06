#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::integer_arithmetic)]
pub mod accounts;
pub mod accounts_background_service;
pub mod accounts_cache;
pub mod accounts_db;
pub mod accounts_hash;
pub mod accounts_index;
pub mod accounts_update_notifier_interface;
pub mod ancestors;
pub mod append_vec;
pub mod bank;
pub mod bank_client;
pub mod bank_forks;
pub mod bank_utils;
pub mod block_cost_limits;
pub mod blockhash_queue;
pub mod builtins;
pub mod commitment;
pub mod contains;
pub mod cost_model;
pub mod cost_tracker;
pub mod epoch_stakes;
pub mod execute_cost_table;
pub mod genesis_utils;
pub mod hardened_unpack;
pub mod hashed_transaction;
pub mod inline_spl_token;
pub mod instruction_recorder;
pub mod loader_utils;
pub mod log_collector;
pub mod message_processor;
mod native_loader;
pub mod neon_evm_program;
pub mod non_circulating_supply;
mod pubkey_bins;
mod read_only_accounts_cache;
pub mod rent_collector;
pub mod secondary_index;
pub mod serde_snapshot;
pub mod snapshot_package;
pub mod snapshot_utils;
pub mod sorted_storages;
pub mod stake_weighted_timestamp;
pub mod stakes;
pub mod status_cache;
mod system_instruction_processor;
pub mod transaction_batch;
pub mod vote_account;
pub mod vote_sender_types;

#[macro_use]
extern crate solana_metrics;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate solana_frozen_abi_macro;
