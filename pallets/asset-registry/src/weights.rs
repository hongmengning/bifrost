// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_asset_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `git-actions`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_asset_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./weight.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_asset_registry.
pub trait WeightInfo {
	fn register_foreign_asset() -> Weight;
	fn update_foreign_asset() -> Weight;
	fn register_native_asset() -> Weight;
	fn update_native_asset() -> Weight;
	fn register_token_metadata() -> Weight;
	fn register_vtoken_metadata() -> Weight;
	fn register_vstoken_metadata() -> Weight;
	fn register_vsbond_metadata() -> Weight;
	fn register_multilocation() -> Weight;
}

/// Weights for bifrost_asset_registry using the Bifrost node and recommended hardware.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BifrostWeight<T> {
	// Storage: AssetRegistry NextForeignAssetId (r:1 w:1)
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn register_foreign_asset() -> Weight {
		(116_430_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn update_foreign_asset() -> Weight {
		(99_783_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn register_native_asset() -> Weight {
		(112_008_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn update_native_asset() -> Weight {
		(120_119_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AssetRegistry NextTokenId (r:1 w:1)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:1)
	fn register_token_metadata() -> Weight {
		(92_672_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	fn register_vtoken_metadata() -> Weight {
		(95_643_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	fn register_vstoken_metadata() -> Weight {
		(96_277_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	fn register_vsbond_metadata() -> Weight {
		(98_679_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToWeights (r:0 w:1)
	fn register_multilocation() -> Weight {
		(61_509_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetRegistry NextForeignAssetId (r:1 w:1)
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn register_foreign_asset() -> Weight {
		(116_430_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn update_foreign_asset() -> Weight {
		(99_783_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn register_native_asset() -> Weight {
		(112_008_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:1)
	fn update_native_asset() -> Weight {
		(120_119_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: AssetRegistry NextTokenId (r:1 w:1)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:1)
	fn register_token_metadata() -> Weight {
		(92_672_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	fn register_vtoken_metadata() -> Weight {
		(95_643_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	fn register_vstoken_metadata() -> Weight {
		(96_277_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	fn register_vsbond_metadata() -> Weight {
		(98_679_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Storage: AssetRegistry LocationToCurrencyIds (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:1)
	// Storage: AssetRegistry CurrencyIdToWeights (r:0 w:1)
	fn register_multilocation() -> Weight {
		(61_509_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}