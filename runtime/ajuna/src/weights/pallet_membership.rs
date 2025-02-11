// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `DESKTOP-0F6V7QQ`, CPU: `Intel(R) Core(TM) i7-10875H CPU @ 2.30GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: 1024

// Executed Command:
// ./target/release/ajuna-node
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-AGPL
// --output=./runtime/ajuna/src/weights/pallet_membership.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: `CouncilMembership::Members` (r:1 w:1)
	/// Proof: `CouncilMembership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 16_300_000 picoseconds.
		Weight::from_parts(16_701_433, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 2_557
			.saturating_add(Weight::from_parts(40_119, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `CouncilMembership::Members` (r:1 w:1)
	/// Proof: `CouncilMembership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CouncilMembership::Prime` (r:1 w:0)
	/// Proof: `CouncilMembership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 19_400_000 picoseconds.
		Weight::from_parts(19_806_808, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 325
			.saturating_add(Weight::from_parts(23_348, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `CouncilMembership::Members` (r:1 w:1)
	/// Proof: `CouncilMembership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CouncilMembership::Prime` (r:1 w:0)
	/// Proof: `CouncilMembership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 19_600_000 picoseconds.
		Weight::from_parts(20_157_575, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 446
			.saturating_add(Weight::from_parts(35_835, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `CouncilMembership::Members` (r:1 w:1)
	/// Proof: `CouncilMembership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CouncilMembership::Prime` (r:1 w:0)
	/// Proof: `CouncilMembership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn reset_members(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_489_894, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 767
			.saturating_add(Weight::from_parts(145_291, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `CouncilMembership::Members` (r:1 w:1)
	/// Proof: `CouncilMembership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CouncilMembership::Prime` (r:1 w:1)
	/// Proof: `CouncilMembership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 20_100_000 picoseconds.
		Weight::from_parts(20_913_153, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 402
			.saturating_add(Weight::from_parts(34_223, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `CouncilMembership::Members` (r:1 w:0)
	/// Proof: `CouncilMembership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `CouncilMembership::Prime` (r:0 w:1)
	/// Proof: `CouncilMembership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70 + m * (32 ±0)`
		//  Estimated: `4687 + m * (32 ±0)`
		// Minimum execution time: 9_500_000 picoseconds.
		Weight::from_parts(10_014_768, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 360
			.saturating_add(Weight::from_parts(11_867, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `CouncilMembership::Prime` (r:0 w:1)
	/// Proof: `CouncilMembership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn clear_prime() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_700_000 picoseconds.
		Weight::from_parts(3_989_737, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
