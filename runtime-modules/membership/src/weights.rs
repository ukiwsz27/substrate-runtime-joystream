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
//! DATE: 2022-10-19, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=membership
// --extrinsic=*
// --chain=dev
// --steps=20
// --repeat=10
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=./scripts/../runtime-modules/membership/src/weights.rs

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
	fn member_remark() -> Weight;
	fn create_member(_i: u32, _j: u32, ) -> Weight;
}

/// Weights for membership using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Membership MembershipPrice (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Storage: Membership InitialInvitationCount (r:1 w:0)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Storage: Membership MembershipById (r:0 w:1)
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight {
		(73_455_000 as Weight)
			// Standard Error: 63_000
			.saturating_add((3_324_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 63_000
			.saturating_add((1_627_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipPrice (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:1)
	// Storage: Membership InitialInvitationCount (r:1 w:0)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Storage: Membership ReferralCut (r:1 w:0)
	fn buy_membership_with_referrer(_i: u32, j: u32, ) -> Weight {
		(181_305_000 as Weight)
			// Standard Error: 84_000
			.saturating_add((1_002_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	// Storage: Membership MemberIdByHandleHash (r:1 w:2)
	fn update_profile(i: u32, j: u32, ) -> Weight {
		(147_745_000 as Weight)
			// Standard Error: 149_000
			.saturating_add((1_231_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 149_000
			.saturating_add((859_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn update_accounts_none() -> Weight {
		(1_621_000 as Weight)
	}
	// Storage: Membership MembershipById (r:1 w:1)
	fn update_accounts_root() -> Weight {
		(23_867_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	fn update_accounts_controller() -> Weight {
		(23_628_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:1)
	fn update_accounts_both() -> Weight {
		(23_692_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership ReferralCut (r:0 w:1)
	fn set_referral_cut() -> Weight {
		(14_301_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:2 w:2)
	fn transfer_invites() -> Weight {
		(31_861_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:2)
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Storage: Instance6WorkingGroup Budget (r:1 w:1)
	// Storage: Membership InitialInvitationBalance (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn invite_member(i: u32, j: u32, ) -> Weight {
		(63_596_000 as Weight)
			// Standard Error: 181_000
			.saturating_add((1_613_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 186_000
			.saturating_add((2_358_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Storage: Membership MembershipPrice (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: Balances Locks (r:2 w:2)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Storage: Membership MembershipById (r:0 w:1)
	fn gift_membership(i: u32, j: u32, ) -> Weight {
		(176_076_000 as Weight)
			// Standard Error: 234_000
			.saturating_add((3_286_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 241_000
			.saturating_add((594_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Membership MembershipPrice (r:0 w:1)
	fn set_membership_price() -> Weight {
		(14_535_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance6WorkingGroup WorkerById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:1)
	fn update_profile_verification() -> Weight {
		(31_488_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance6WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance6WorkingGroup WorkerById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:1)
	fn set_leader_invitation_quota() -> Weight {
		(33_604_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership InitialInvitationBalance (r:0 w:1)
	fn set_initial_invitation_balance() -> Weight {
		(15_123_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership InitialInvitationCount (r:0 w:1)
	fn set_initial_invitation_count() -> Weight {
		(15_296_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_staking_account_candidate() -> Weight {
		(77_769_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:1)
	fn confirm_staking_account() -> Weight {
		(33_482_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_staking_account() -> Weight {
		(48_439_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	fn member_remark() -> Weight {
		(21_024_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Membership MemberIdByHandleHash (r:1 w:1)
	// Storage: Membership InitialInvitationCount (r:1 w:0)
	// Storage: Membership NextMemberId (r:1 w:1)
	// Storage: Membership MembershipById (r:0 w:1)
	fn create_member(i: u32, j: u32, ) -> Weight {
		(76_715_000 as Weight)
			// Standard Error: 85_000
			.saturating_add((2_202_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 87_000
			.saturating_add((655_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn buy_membership_without_referrer(i: u32, j: u32, ) -> Weight {
		0
	}
	fn buy_membership_with_referrer(_i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_profile(i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_accounts_none() -> Weight {
		0
	}
	fn update_accounts_root() -> Weight {
		0
	}
	fn update_accounts_controller() -> Weight {
		0
	}
	fn update_accounts_both() -> Weight {
		0
	}
	fn set_referral_cut() -> Weight {
		0
	}
	fn transfer_invites() -> Weight {
		0
	}
	fn invite_member(i: u32, j: u32, ) -> Weight {
		0
	}
	fn gift_membership(i: u32, j: u32, ) -> Weight {
		0
	}
	fn set_membership_price() -> Weight {
		0
	}
	fn update_profile_verification() -> Weight {
		0
	}
	fn set_leader_invitation_quota() -> Weight {
		0
	}
	fn set_initial_invitation_balance() -> Weight {
		0
	}
	fn set_initial_invitation_count() -> Weight {
		0
	}
	fn add_staking_account_candidate() -> Weight {
		0
	}
	fn confirm_staking_account() -> Weight {
		0
	}
	fn remove_staking_account() -> Weight {
		0
	}
	fn member_remark() -> Weight {
		0
	}
	fn create_member(i: u32, j: u32, ) -> Weight {
		0
	}
}
