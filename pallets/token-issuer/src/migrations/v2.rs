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
use frame_support::traits::OnRuntimeUpgrade;

const LOG_TARGET: &str = "token-issuer::migration";

pub struct TokenIssuerMigration<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for TokenIssuerMigration<T> {
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

			log::info!(target: LOG_TARGET, "Start to migrate TransferWhiteList storage...");
			TransferWhiteList::<T>::translate::<Vec<AccountIdOf<T>>, _>(
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
			let count =
				IssueWhiteList::<T>::iter().count() + TransferWhiteList::<T>::iter().count();
			Weight::from(T::DbWeight::get().reads_writes(count as u64 + 1, count as u64 + 1))
		} else {
			// We don't do anything here.
			Weight::zero()
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let issue_white_list_cnt = IssueWhiteList::<T>::iter().count();
		// print out the pre-migrate storage count
		log::info!(target: LOG_TARGET, "IssueWhiteList storage count: {:?}", issue_white_list_cnt);

		let transfer_white_list_cnt = TransferWhiteList::<T>::iter().count();
		log::info!(
			target: LOG_TARGET,
			"TransferWhiteList storage count: {:?}",
			transfer_white_list_cnt
		);

		let cnt = (issue_white_list_cnt as u32, transfer_white_list_cnt as u32);
		Ok(cnt.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(cnt: Vec<u8>) -> Result<(), &'static str> {
		let (issue_white_list_cnt_old, transfer_white_list_cnt_old) =
			<(u32, u32)>::decode(&mut &cnt[..])
				.map_err(|_| "Failed to decode the storage count")?;

		let issue_white_list_cnt_new = IssueWhiteList::<T>::iter().count();
		// print out the post-migrate storage count
		log::info!(
			target: LOG_TARGET,
			"IssueWhiteList post-migrate storage count: {:?}",
			issue_white_list_cnt_new
		);
		ensure!(
			issue_white_list_cnt_new as u32 == issue_white_list_cnt_old,
			"Failed to migrate IssueWhiteList storage"
		);

		let transfer_white_list_cnt_new = TransferWhiteList::<T>::iter().count();
		log::info!(
			target: LOG_TARGET,
			"TransferWhiteList post-migrate storage count: {:?}",
			transfer_white_list_cnt_new
		);
		ensure!(
			transfer_white_list_cnt_new as u32 == transfer_white_list_cnt_old,
			"Failed to migrate TransferWhiteList storage"
		);

		Ok(())
	}
}