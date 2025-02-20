
//! Autogenerated weights for `pallet_trust`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-16, STEPS: `20`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-fennel-protocol
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_trust
// --extrinsic
// *
// --steps
// 20
// --repeat
// 100
// --raw
// --output
// ./


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn issue_trust() -> Weight;
    fn revoke_trust() -> Weight;
    fn remove_trust() -> Weight;
    fn remove_revoked_trust() -> Weight;
    fn request_trust() -> Weight;
    fn cancel_trust_request() -> Weight;
}

/// Weight functions for pallet_trust.
pub struct SubstrateWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeights<T> {
	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn issue_trust() -> Weight {
		(37_637_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TrustModule TrustRevocation (r:1 w:1)
	// Storage: TrustModule CurrentRevoked (r:1 w:1)
	fn revoke_trust() -> Weight {
		(90_811_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TrustModule TrustIssuance (r:1 w:0)
	fn remove_trust() -> Weight {
		(8_490_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: TrustModule TrustRevocation (r:1 w:0)
	fn remove_revoked_trust() -> Weight {
		(8_510_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: TrustModule TrustRequestList (r:1 w:1)
	// Storage: TrustModule CurrentRequests (r:1 w:1)
	fn request_trust() -> Weight {
		(36_766_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TrustModule TrustRequestList (r:1 w:0)
	fn cancel_trust_request() -> Weight {
		(10_089_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

impl WeightInfo for () {
	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn issue_trust() -> Weight {
		(37_637_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: TrustModule TrustRevocation (r:1 w:1)
	// Storage: TrustModule CurrentRevoked (r:1 w:1)
	fn revoke_trust() -> Weight {
		(90_811_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: TrustModule TrustIssuance (r:1 w:0)
	fn remove_trust() -> Weight {
		(8_490_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: TrustModule TrustRevocation (r:1 w:0)
	fn remove_revoked_trust() -> Weight {
		(8_510_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: TrustModule TrustRequestList (r:1 w:1)
	// Storage: TrustModule CurrentRequests (r:1 w:1)
	fn request_trust() -> Weight {
		(36_766_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: TrustModule TrustRequestList (r:1 w:0)
	fn cancel_trust_request() -> Weight {
		(10_089_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
}
