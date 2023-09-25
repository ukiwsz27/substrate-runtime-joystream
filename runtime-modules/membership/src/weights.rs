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

//! Autogenerated weights for membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --pallet=membership
// --extrinsic=*
// --chain=prod-test
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/membership/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for membership.
pub trait WeightInfo {
	fn buy_membership_without_referrer(_i: u32, _j: u32, ) -> Weight;
	fn buy_membership_with_referrer(_i: u32, _j: u32, ) -> Weight;
	fn update_profile(_i: u32, _j: u32, ) -> Weight;
	fn update_accounts_none() -> Weight;
	fn update_accounts_root() -> Weight;
	fn update_accounts_controller() -> Weight;
	fn update_accounts_both() -> Weight;
	fn set_referral_cut() -> Weight;
	fn transfer_invites() -> Weight;
	fn invite_member(_i: u32, _j: u32, ) -> Weight;
	fn gift_membership(_i: u32, _j: u32, ) -> Weight;
	fn set_membership_price() -> Weight;
	fn update_profile_verification() -> Weight;
	fn set_leader_invitation_quota() -> Weight;
	fn set_initial_invitation_balance() -> Weight;
	fn set_initial_invitation_count() -> Weight;
	fn add_staking_account_candidate() -> Weight;
	fn confirm_staking_account() -> Weight;
	fn remove_staking_account() -> Weight;
	fn member_remark_without_payment() -> Weight;
	fn member_remark_with_payment() -> Weight;
	fn create_member(_i: u32, _j: u32, ) -> Weight;
}

/// Weights for membership using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Membership MembershipPrice (r:1 w:0)
	// Proof: Membership MembershipPrice (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Proof: Membership MemberIdByHandleHash (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Membership InitialInvitationCount (r:1 w:0)
	// Proof: Membership InitialInvitationCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Proof: Membership NextMemberId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:0 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 100]`.
	/// The range of component `j` is `[0, 100]`.
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `212`
		//  Estimated: `6647`
		// Minimum execution time: 75_000 nanoseconds.
		Weight::from_parts(37_199_354, 0u64)
			.saturating_add(Weight::from_proof_size(6647))
			// Standard Error: 5_110
			.saturating_add(Weight::from_parts(1_342_445, 0u64).saturating_mul(i.into()))
			// Standard Error: 5_110
			.saturating_add(Weight::from_parts(409_582, 0u64).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Membership MembershipPrice (r:1 w:0)
	// Proof: Membership MembershipPrice (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Proof: Membership MemberIdByHandleHash (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Membership InitialInvitationCount (r:1 w:0)
	// Proof: Membership InitialInvitationCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Proof: Membership NextMemberId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Membership ReferralCut (r:1 w:0)
	// Proof: Membership ReferralCut (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 100]`.
	/// The range of component `j` is `[0, 100]`.
	fn buy_membership_with_referrer(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `523`
		//  Estimated: `9743`
		// Minimum execution time: 91_000 nanoseconds.
		Weight::from_parts(45_763_092, 0u64)
			.saturating_add(Weight::from_proof_size(9743))
			// Standard Error: 5_894
			.saturating_add(Weight::from_parts(1_389_512, 0u64).saturating_mul(i.into()))
			// Standard Error: 5_894
			.saturating_add(Weight::from_parts(497_070, 0u64).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Membership MemberIdByHandleHash (r:1 w:2)
	// Proof: Membership MemberIdByHandleHash (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 100]`.
	/// The range of component `j` is `[0, 100]`.
	fn update_profile(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297`
		//  Estimated: `5131`
		// Minimum execution time: 61_000 nanoseconds.
		Weight::from_parts(25_265_985, 0u64)
			.saturating_add(Weight::from_proof_size(5131))
			// Standard Error: 4_989
			.saturating_add(Weight::from_parts(1_438_161, 0u64).saturating_mul(i.into()))
			// Standard Error: 4_989
			.saturating_add(Weight::from_parts(423_609, 0u64).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn update_accounts_none() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn update_accounts_root() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252`
		//  Estimated: `2600`
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_parts(13_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(2600))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn update_accounts_controller() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252`
		//  Estimated: `2600`
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_parts(13_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(2600))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn update_accounts_both() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252`
		//  Estimated: `2600`
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_parts(13_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(2600))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership ReferralCut (r:0 w:1)
	// Proof: Membership ReferralCut (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn set_referral_cut() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership MembershipById (r:2 w:2)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn transfer_invites() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420`
		//  Estimated: `5200`
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_parts(18_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(5200))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Membership MembershipById (r:1 w:2)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Proof: Membership MemberIdByHandleHash (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Instance6WorkingGroup Budget (r:1 w:1)
	// Proof: Instance6WorkingGroup Budget (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Membership InitialInvitationBalance (r:1 w:0)
	// Proof: Membership InitialInvitationBalance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Proof: Membership NextMemberId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `j` is `[0, 100]`.
	fn invite_member(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `582`
		//  Estimated: `13033`
		// Minimum execution time: 86_000 nanoseconds.
		Weight::from_parts(48_496_951, 0u64)
			.saturating_add(Weight::from_proof_size(13033))
			// Standard Error: 5_774
			.saturating_add(Weight::from_parts(1_358_933, 0u64).saturating_mul(i.into()))
			// Standard Error: 5_706
			.saturating_add(Weight::from_parts(437_981, 0u64).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Proof: Membership MemberIdByHandleHash (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Membership MembershipPrice (r:1 w:0)
	// Proof: Membership MembershipPrice (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:2 w:2)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Proof: Membership NextMemberId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:0 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `j` is `[0, 100]`.
	fn gift_membership(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `18902`
		// Minimum execution time: 133_000 nanoseconds.
		Weight::from_parts(94_080_490, 0u64)
			.saturating_add(Weight::from_proof_size(18902))
			// Standard Error: 5_728
			.saturating_add(Weight::from_parts(1_369_491, 0u64).saturating_mul(i.into()))
			// Standard Error: 5_660
			.saturating_add(Weight::from_parts(438_067, 0u64).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: Membership MembershipPrice (r:0 w:1)
	// Proof: Membership MembershipPrice (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_membership_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance6WorkingGroup WorkerById (r:1 w:0)
	// Proof: Instance6WorkingGroup WorkerById (max_values: None, max_size: Some(175), added: 2650, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn update_profile_verification() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `665`
		//  Estimated: `5250`
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_parts(18_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance6WorkingGroup CurrentLead (r:1 w:0)
	// Proof: Instance6WorkingGroup CurrentLead (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Instance6WorkingGroup WorkerById (r:1 w:0)
	// Proof: Instance6WorkingGroup WorkerById (max_values: None, max_size: Some(175), added: 2650, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn set_leader_invitation_quota() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `665`
		//  Estimated: `5753`
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_parts(19_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(5753))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership InitialInvitationBalance (r:0 w:1)
	// Proof: Membership InitialInvitationBalance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_initial_invitation_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership InitialInvitationCount (r:0 w:1)
	// Proof: Membership InitialInvitationCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_initial_invitation_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:1)
	// Proof: Membership StakingAccountIdMemberStatus (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_staking_account_candidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `520`
		//  Estimated: `11509`
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_parts(29_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(11509))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:1)
	// Proof: Membership StakingAccountIdMemberStatus (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	fn confirm_staking_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `331`
		//  Estimated: `5132`
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_parts(19_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(5132))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:1)
	// Proof: Membership StakingAccountIdMemberStatus (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_staking_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `637`
		//  Estimated: `11509`
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_parts(28_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(11509))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	fn member_remark_without_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252`
		//  Estimated: `2600`
		// Minimum execution time: 11_000 nanoseconds.
		Weight::from_parts(11_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(2600))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn member_remark_with_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510`
		//  Estimated: `7806`
		// Minimum execution time: 36_000 nanoseconds.
		Weight::from_parts(37_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(7806))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Proof: Membership MemberIdByHandleHash (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Membership InitialInvitationCount (r:1 w:0)
	// Proof: Membership InitialInvitationCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Proof: Membership NextMemberId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:0 w:1)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `j` is `[0, 100]`.
	fn create_member(i: u32, j: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3533`
		// Minimum execution time: 63_000 nanoseconds.
		Weight::from_parts(17_319_363, 0u64)
			.saturating_add(Weight::from_proof_size(3533))
			// Standard Error: 5_674
			.saturating_add(Weight::from_parts(1_511_269, 0u64).saturating_mul(i.into()))
			// Standard Error: 5_607
			.saturating_add(Weight::from_parts(491_831, 0u64).saturating_mul(j.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn buy_membership_with_referrer(i: u32, j: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_profile(i: u32, j: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_accounts_none() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_accounts_root() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_accounts_controller() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_accounts_both() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_referral_cut() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn transfer_invites() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn invite_member(i: u32, j: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn gift_membership(i: u32, j: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_membership_price() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_profile_verification() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_leader_invitation_quota() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_initial_invitation_balance() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_initial_invitation_count() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn add_staking_account_candidate() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn confirm_staking_account() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn remove_staking_account() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn member_remark_without_payment() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn member_remark_with_payment() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn create_member(i: u32, j: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
}
