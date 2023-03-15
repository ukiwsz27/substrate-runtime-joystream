// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=frame_system
// --extrinsic=*
// --chain=prod-test
// --steps=2
// --repeat=2
// --execution=wasm
// --template=./scripts/../devops/frame-weight-template.hbs
// --output=./scripts/../runtime/src/weights/frame_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub use frame_system::weights::WeightInfo;

/// Weights for frame_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_ref_time(2_500_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(209).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_500_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 11
			.saturating_add(Weight::from_ref_time(1_255).saturating_mul(b.into()))
	}
	// Storage: System Digest (r:1 w:1)
	// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: unknown `0x3a686561707061676573` (r:0 w:1)
	// Proof Skipped: unknown `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `495`
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_ref_time(3_000_000)
			.saturating_add(Weight::from_proof_size(495))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_ref_time(2_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(725_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_ref_time(2_000_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(542_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39 + p * (69 ±0)`
		//  Estimated: `39 + p * (70 ±0)`
		// Minimum execution time: 4_000 nanoseconds.
		Weight::from_ref_time(4_000_000)
			.saturating_add(Weight::from_proof_size(39))
			// Standard Error: 4_500
			.saturating_add(Weight::from_ref_time(1_027_500).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_proof_size(70).saturating_mul(p.into()))
	}
}
