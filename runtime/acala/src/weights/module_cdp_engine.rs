// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

//! Autogenerated weights for module_cdp_engine
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_cdp_engine.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_cdp_engine::WeightInfo for WeightInfo<T> {
	// Storage: CdpEngine LastAccumulationSecs (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Timestamp Now (r:0 w:1)
	fn on_initialize(c: u32, ) -> Weight {
		(9_426_000 as Weight)
			// Standard Error: 84_000
			.saturating_add((7_613_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: CdpEngine CollateralParams (r:1 w:1)
	fn set_collateral_params() -> Weight {
		(30_981_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:3 w:3)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: CdpEngine LiquidationContracts (r:1 w:0)
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:1 w:0)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	// Storage: Auction AuctionsIndex (r:1 w:1)
	// Storage: AuctionManager CollateralAuctions (r:0 w:1)
	// Storage: Auction AuctionEndTime (r:0 w:1)
	// Storage: Auction Auctions (r:0 w:1)
	fn liquidate_by_auction(b: u32, ) -> Weight {
		(203_462_000 as Weight)
			// Standard Error: 26_000
			.saturating_add((13_215_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(30 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:4 w:3)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:4 w:0)
	// Storage: Dex LiquidityPool (r:2 w:2)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	fn liquidate_by_dex() -> Weight {
		(313_409_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(38 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	fn settle() -> Weight {
		(109_920_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: CdpEngine LiquidationContracts (r:1 w:1)
	fn register_liquidation_contract() -> Weight {
		(16_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: CdpEngine LiquidationContracts (r:1 w:1)
	fn deregister_liquidation_contract() -> Weight {
		(17_350_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
