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

use crate::*;
use alloc::vec::Vec;
use frame_support::traits::OnRuntimeUpgrade;

const LOG_TARGET: &str = "cross-in-out::migration";

pub struct CrossInOutMigration<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for CrossInOutMigration<T> {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// Check the storage version
		let onchain_version = Pallet::<T>::on_chain_storage_version();
		if onchain_version < 2 {
			// Transform storage values
			// We transform the storage values from the old into the new format.
			log::info!(target: LOG_TARGET, "Start to migrate IssueWhiteList storage...");
			IssueWhiteList::<T>::translate::<Vec<AccountIdOf<T>>, _>(
				|k: CurrencyId, value: Vec<AccountIdOf<T>>| {
					log::info!(target: LOG_TARGET, "Migrated to boundedvec for {:?}...", k);

					let target_bounded_vec: BoundedVec<AccountIdOf<T>, T::MaxLengthLimit>;

					if value.len() != 0 {
						target_bounded_vec = BoundedVec::try_from(value).unwrap();
					} else {
						target_bounded_vec =
							BoundedVec::<AccountIdOf<T>, T::MaxLengthLimit>::default();
					}

					Some(target_bounded_vec)
				},
			);

			// Update the storage version
			StorageVersion::new(2).put::<Pallet<T>>();

			// Return the consumed weight
			let count = IssueWhiteList::<T>::iter().count();
			Weight::from(T::DbWeight::get().reads_writes(count as u64 + 1, count as u64 + 1))
		} else {
			// We don't do anything here.
			Weight::zero()
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let cnt = IssueWhiteList::<T>::iter().count();
		// print out the pre-migrate storage count
		log::info!(target: LOG_TARGET, "IssueWhiteList pre-migrate storage count: {:?}", cnt);
		Ok((cnt as u64).encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(cnt: Vec<u8>) -> Result<(), &'static str> {
		let new_count = IssueWhiteList::<T>::iter().count();
		// decode cnt to u64
		let old_count = u64::decode(&mut &cnt[..]).unwrap();

		// print out the post-migrate storage count
		log::info!(
			target: LOG_TARGET,
			"IssueWhiteList post-migrate storage count: {:?}",
			new_count
		);

		ensure!(
			new_count as u64 == old_count,
			"Post-migration storage count does not match pre-migration count"
		);

		Ok(())
	}
}
