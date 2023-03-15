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

//! Autogenerated weights for pallet_election_provider_multi_phase
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --chain=prod-test
// --steps=2
// --repeat=2
// --execution=wasm
// --template=./scripts/../devops/frame-weight-template.hbs
// --output=./scripts/../runtime/src/weights/pallet_election_provider_multi_phase.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub use pallet_election_provider_multi_phase::weights::WeightInfo;

/// Weights for pallet_election_provider_multi_phase using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Staking CurrentEra (r:1 w:0)
	// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Staking CurrentPlannedSession (r:1 w:0)
	// Proof: Staking CurrentPlannedSession (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Proof: Babe EpochIndex (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Babe GenesisSlot (r:1 w:0)
	// Proof: Babe GenesisSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Babe CurrentSlot (r:1 w:0)
	// Proof: Babe CurrentSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Staking ForceEra (r:1 w:0)
	// Proof: Staking ForceEra (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_nothing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `845`
		//  Estimated: `6834`
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(20_000_000)
			.saturating_add(Weight::from_proof_size(6834))
			.saturating_add(T::DbWeight::get().reads(8_u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_open_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1074`
		// Minimum execution time: 11_000 nanoseconds.
		Weight::from_ref_time(12_000_000)
			.saturating_add(Weight::from_proof_size(1074))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_open_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1074`
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(13_000_000)
			.saturating_add(Weight::from_proof_size(1074))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	fn finalize_signed_phase_accept_solution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `2809`
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(24_000_000)
			.saturating_add(Weight::from_proof_size(2809))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn finalize_signed_phase_reject_solution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `2603`
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_000_000)
			.saturating_add(Weight::from_proof_size(2603))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 222_000 nanoseconds.
		Weight::from_ref_time(18_500_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 28_310
			.saturating_add(Weight::from_ref_time(173_500).saturating_mul(v.into()))
			// Standard Error: 56_621
			.saturating_add(Weight::from_ref_time(61_000).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionIndices (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionNextIndex (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionsMap (max_values: None, max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `448 + a * (768 ±0) + d * (48 ±0)`
		//  Estimated: `9477 + a * (6912 ±0) + d * (441 ±0)`
		// Minimum execution time: 275_000 nanoseconds.
		Weight::from_ref_time(76_999_999)
			.saturating_add(Weight::from_proof_size(9477))
			// Standard Error: 53_834
			.saturating_add(Weight::from_ref_time(165_000).saturating_mul(a.into()))
			// Standard Error: 80_751
			.saturating_add(Weight::from_ref_time(340_000).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
			.saturating_add(Weight::from_proof_size(6912).saturating_mul(a.into()))
			.saturating_add(Weight::from_proof_size(441).saturating_mul(d.into()))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionIndices (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionNextIndex (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionsMap (max_values: None, max_size: None, mode: Measured)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1189`
		//  Estimated: `8436`
		// Minimum execution time: 37_000 nanoseconds.
		Weight::from_ref_time(37_000_000)
			.saturating_add(Weight::from_proof_size(8436))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase MinimumUntrustedScore (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `179 + v * (553 ±0) + t * (32 ±0)`
		//  Estimated: `4718 + v * (3871 ±0) + t * (224 ±0)`
		// Minimum execution time: 4_486_000 nanoseconds.
		Weight::from_ref_time(4_486_000_000)
			.saturating_add(Weight::from_proof_size(4718))
			// Standard Error: 705_898
			.saturating_add(Weight::from_ref_time(4_067_187).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_proof_size(3871).saturating_mul(v.into()))
			.saturating_add(Weight::from_proof_size(224).saturating_mul(t.into()))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase MinimumUntrustedScore (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `154 + v * (553 ±0) + t * (32 ±0)`
		//  Estimated: `2596 + v * (2212 ±0) + t * (128 ±0)`
		// Minimum execution time: 3_662_000 nanoseconds.
		Weight::from_ref_time(3_662_000_000)
			.saturating_add(Weight::from_proof_size(2596))
			// Standard Error: 202_002
			.saturating_add(Weight::from_ref_time(46_624).saturating_mul(v.into()))
			// Standard Error: 597_591
			.saturating_add(Weight::from_ref_time(3_149_558).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(Weight::from_proof_size(2212).saturating_mul(v.into()))
			.saturating_add(Weight::from_proof_size(128).saturating_mul(t.into()))
	}
}
