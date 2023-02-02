
//! Autogenerated weights for `pallet_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-10-175`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ice-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_grandpa
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/frost/src/weights/pallet_grandpa_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_grandpa::WeightInfo for WeightInfo<T> {
	/// The range of component `x` is `[0, 1]`.
	fn report_equivocation(x: u32, ) -> Weight {
		// Minimum execution time: 125_808 nanoseconds.
		Weight::from_ref_time(126_609_908)
			// Standard Error: 65_671
			.saturating_add(Weight::from_ref_time(223_291).saturating_mul(x.into()))
	}
	// Storage: Grandpa Stalled (r:0 w:1)
	fn note_stalled() -> Weight {
		// Minimum execution time: 7_553 nanoseconds.
		Weight::from_ref_time(7_823_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
