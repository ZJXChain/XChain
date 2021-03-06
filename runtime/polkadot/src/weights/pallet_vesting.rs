// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-01, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	fn vest_locked(l: u32, s: u32) -> Weight {
		(93_789_000 as Weight)
			// Standard Error: 70_000
			.saturating_add((41_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 182_000
			.saturating_add((211_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vest_unlocked(_l: u32, s: u32) -> Weight {
		(90_737_000 as Weight)
			// Standard Error: 0
			.saturating_add((263_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vest_other_locked(l: u32, s: u32) -> Weight {
		(85_211_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((153_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 45_000
			.saturating_add((289_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn vest_other_unlocked(l: u32, s: u32) -> Weight {
		(90_368_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((31_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 45_000
			.saturating_add((132_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn vested_transfer(l: u32, _s: u32) -> Weight {
		(167_500_000 as Weight)
			// Standard Error: 194_000
			.saturating_add((255_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn force_vested_transfer(l: u32, _s: u32) -> Weight {
		(174_000_000 as Weight)
			// Standard Error: 70_000
			.saturating_add((143_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn not_unlocking_merge_schedules(l: u32, s: u32) -> Weight {
		(101_778_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((194_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 48_000
			.saturating_add((361_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlocking_merge_schedules(l: u32, s: u32) -> Weight {
		(104_111_000 as Weight)
			// Standard Error: 88_000
			.saturating_add((276_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 240_000
			.saturating_add((194_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
