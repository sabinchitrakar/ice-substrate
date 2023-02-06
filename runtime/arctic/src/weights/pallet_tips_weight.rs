
//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-10-175`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("arctic"), DB CACHE: 1024

// Executed Command:
// ./target/release/ice-node
// benchmark
// pallet
// --chain
// arctic
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_tips
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/arctic/src/weights/pallet_tips_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Minimum execution time: 55_219 nanoseconds.
		Weight::from_ref_time(56_670_732)
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(2_351).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		// Minimum execution time: 52_615 nanoseconds.
		Weight::from_ref_time(53_556_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 7]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Minimum execution time: 40_540 nanoseconds.
		Weight::from_ref_time(38_711_717)
			// Standard Error: 10
			.saturating_add(Weight::from_ref_time(2_195).saturating_mul(r.into()))
			// Standard Error: 26_791
			.saturating_add(Weight::from_ref_time(653_637).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn tip(t: u32, ) -> Weight {
		// Minimum execution time: 27_116 nanoseconds.
		Weight::from_ref_time(27_617_291)
			// Standard Error: 4_743
			.saturating_add(Weight::from_ref_time(294_976).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn close_tip(t: u32, ) -> Weight {
		// Minimum execution time: 85_363 nanoseconds.
		Weight::from_ref_time(86_684_694)
			// Standard Error: 15_624
			.saturating_add(Weight::from_ref_time(259_857).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Minimum execution time: 34_199 nanoseconds.
		Weight::from_ref_time(35_120_162)
			// Standard Error: 6_366
			.saturating_add(Weight::from_ref_time(73_077).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}