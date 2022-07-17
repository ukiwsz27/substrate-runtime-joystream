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

//! Autogenerated weights for content
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=content
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for content.
pub trait WeightInfo {
	fn create_channel(_a: u32, _b: u32, _c: u32, _d: u32, ) -> Weight;
	fn update_channel(_a: u32, _b: u32, _c: u32, ) -> Weight;
}

/// Weights for content using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87aa6eccf0cc6941ba2e31cdb5870e3229] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b872d56750ffbaedbf3dd8dd3900c756381] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad3323e092df90358439e7c6649f66d93f] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:0 w:100)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:0 w:1)
	fn create_channel(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		(418_831_000 as Weight)
			// Standard Error: 101_000
			.saturating_add((8_752_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_232_000
			.saturating_add((14_913_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 120_000
			.saturating_add((16_543_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 101_000
			.saturating_add((7_587_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:0 w:99)
	fn update_channel(a: u32, b: u32, c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 94_000
			.saturating_add((10_374_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 89_000
			.saturating_add((8_027_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 521_664_627_231_369_472_000
			.saturating_add((279_810_196_904_114_560_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((5042865410931 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((2521432705489 as Weight).saturating_mul(c as Weight)))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_channel(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		0
	}
	fn update_channel(a: u32, b: u32, c: u32, ) -> Weight {
		0
	}
}
