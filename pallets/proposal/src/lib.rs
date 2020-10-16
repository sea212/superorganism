// Copyright 2020 Harald Heckmann

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

//! # pallet-proposal
//! Manages proposal and concern rounds as well as the correspondant voting rounds


use frame_support::{decl_module, decl_storage, decl_event, Parameter, print, debug,
	dispatch::{DispatchResult, Dispatchable, DispatchError},
	traits::{Get, Currency, ReservableCurrency,
		schedule::{Anon, DispatchTime, LOWEST_PRIORITY},
	},
	weights::Weight,
};
use frame_system::{ensure_root, RawOrigin::Root};
// use frame_system;
use codec::{Codec, Decode, Encode};
// Fixed point arithmetic
use sp_arithmetic::Permill;
#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// Contains the five different states the pallet can be in
#[derive(Copy, Clone, Debug, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum States {
	Uninitialized,
	Propose,
	VotePropose,
	Concern,
	VoteConcern,
	VoteCouncil,
}

impl Default for States {
    fn default() -> Self {
        States::Uninitialized
    }
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	// Type trait constraints
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;

	type Currency: ReservableCurrency<Self::AccountId>;

	/// Define the Scheduler type. Must implement (unamed) scheduling trait Anon
	type Scheduler: Anon<Self::BlockNumber, Self::Proposal, Self::PalletsOrigin>;
	type Proposal: Parameter + Dispatchable<Origin=Self::Origin> + From<Call<Self>>;
	type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>> + Codec + Clone + Eq;

	// Parameters
	/// How long is an identified user locked out from submitting proposals / concerns
	/// for bad behaviour. Value in seconds.
	type IdentifiedUserPenality: Get<u32>;

	/// Part 1.1: Proposal state configuration
	// How many (slashable) funds must a simple User (no identity) lock to be able to propose?
	// type UserProposeFee: Get<BalanceOf<Self>>;
	
	/// How many proposals can an identified user submit per proposal round?
	type ProposeIdentifiedUserCap: Get<u8>;

	/// How high is the reward (%) for the proposer if the proposal is converted into a project?
	type ProposeReward: Get<Permill>;

	/// How long can proposals be submitted? Value in seconds.
	type ProposeRoundDuration: Get<Self::BlockNumber>;

	/// Part 1.2: Proposal voting state configuration
	/// How many votes (%) does a proposal require to be accepted for the next round?
	type ProposeVoteAcceptanceMin: Get<Permill>;

	/// How long can votes for proposals be submitted?
	type ProposeVoteDuration: Get<Self::BlockNumber>;

	/// Which identity level (number of random verifications) is required to vote?
	type ProposeVoteIdentityLevel: Get<u16>;

	/// How many votes can each identified user (with an appropriate identity level) submit?
	type ProposeVoteMaxPerIdentifiedUser: Get<u16>;

	/// How high is the reward if a proposal that the user voted for passes into next round?
	type ProposeVoteCorrectReward: Get<BalanceOf<Self>>;

	/// Part 2.1: Concern state configuration
	/// How many concerns can an identified user submit per concern round?
	type ConcernIdentifiedUserCap: Get<u8>;

	/// How high is the reward if the concern receives enough votes to be passed to the next state?
	type ConcernReward: Get<BalanceOf<Self>>;

	/// How long can concerns be submitted? Value in seconds.
	type ConcernRoundDuration: Get<Self::BlockNumber>;

	// How many (slashable) funds must a simple User (no identity) lock to be able to submit a concern?
	// type UserConcernFee: Get<BalanceOf<Self>>;

	/// Part 2.2: Concern voting state configuration
	/// How many votes (%) does a concern require to be accepted for the next round?
	type ConcernVoteAcceptanceMin: Get<Permill>;

	/// How long can votes for concerns be submitted?
	type ConcernVoteDuration: Get<Self::BlockNumber>;

	/// Which identity level (number of random verifications) is required to vote?
	type ConcernVoteIdentityLevel: Get<u16>;

	/// How many votes can each identified user (with an appropriate identity level) submit?
	type ConcernVoteMaxPerIdentifiedUser: Get<u16>;

	/// How high is the reward if a concern that the user voted for passes into next round?
	type ConcernVoteCorrectReward: Get<BalanceOf<Self>>;

	/// Part 3: Final evaluation of the winning proposals and associated concern by the council
	/// How much time is reserved for the council to vote? Value in seconds
	type CouncilVoteRoundDuration: Get<Self::BlockNumber>;

	/// How many percent of the council must agree that a concern is too serious to launch a
	/// project from the associated proposal?
	type CouncilAcceptConcernMinVotes: Get<Permill>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Proposal {
		/// The current proposal state
		// Note: We must specify config() for at least one storage item, otherwise
		// the state machine cannot be initialized during genesis, because
		// add_extra_genesis won't be called at all (1. Nov 2020)
		pub State get(fn state) config(): States = States::Uninitialized;

		pub NextTransit get(fn next_transit): T::BlockNumber;
	}
	add_extra_genesis {
		build(|_| {
			let _ = <Module<T>>::do_state_transit();
		}); 
	}
}

decl_event! {
	pub enum Event {
		/// Rotated to the next state. \[NewState\]
		StateRotated(States),
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		// Fetch configuration
		/// How long is an identified user locked out from submitting proposals / concerns
		/// for bad behaviour. Value in seconds.
		const IdentifiedUserPenality: u32 = T::IdentifiedUserPenality::get() as u32;

		// Part 1.1: Proposal state configuration
		// How many (slashable) funds must a simple User (no identity) lock to be able to propose?
		// const UserProposeFee: BalanceOf<T> = T::UserProposeFee::get();
		
		/// How many proposals can an identified user submit per proposal round?
		const ProposeIdentifiedUserCap: u8 = T::ProposeIdentifiedUserCap::get() as u8;

		/// How high is the reward (%) for the proposer if the proposal is converted into a project?
		const ProposeReward: Permill = T::ProposeReward::get();

		/// How long can proposals be submitted? Value in seconds.
		const ProposeRoundDuration: T::BlockNumber = T::ProposeRoundDuration::get();

		// Part 1.2: Proposal voting state configuration
		/// How many votes (%) does a proposal require to be accepted for the next round?
		// const ProposeVoteAcceptanceMin: Permill = T::ProposeVoteAcceptanceMin::get() as Permill;

		/// How long can votes for proposals be submitted?
		const ProposeVoteDuration: T::BlockNumber = T::ProposeVoteDuration::get();

		/// Which identity level (number of random verifications) is required to vote?
		const ProposeVoteIdentityLevel: u16 = T::ProposeVoteIdentityLevel::get() as u16;

		/// How many votes can each identified user (with an appropriate identity level) submit?
		const ProposeVoteMaxPerIdentifiedUser: u16 = T::ProposeVoteMaxPerIdentifiedUser::get() as u16;

		/// How high is the reward if a proposal that the user voted for passes into next round?
		const ProposeVoteCorrectReward: BalanceOf<T> = T::ProposeVoteCorrectReward::get();

		// Part 2.1: Concern state configuration
		/// How many concerns can an identified user submit per concern round?
		const ConcernIdentifiedUserCap: u8 = T::ConcernIdentifiedUserCap::get() as u8;

		/// How high is the reward if the concern receives enough votes to be passed to the next state?
		const ConcernReward: BalanceOf<T> = T::ConcernReward::get();

		/// How long can concerns be submitted? Value in seconds.
		const ConcernRoundDuration: T::BlockNumber = T::ConcernRoundDuration::get();

		// How many (slashable) funds must a simple User (no identity) lock to be able to submit a concern?
		// const UserConcernFee: BalanceOf<T> = T::UserConcernFee::get();

		// Part 2.2: Concern voting state configuration
		/// How many votes (%) does a concern require to be accepted for the next round?
		// const ConcernVoteAcceptanceMin: Permill = T::ConcernVoteAcceptanceMin::get() as Permill;

		/// How long can votes for concerns be submitted?
		const ConcernVoteDuration: T::BlockNumber = T::ConcernVoteDuration::get();

		/// Which identity level (number of random verifications) is required to vote?
		const ConcernVoteIdentityLevel: u16 = T::ConcernVoteIdentityLevel::get() as u16;

		/// How many votes can each identified user (with an appropriate identity level) submit?
		const ConcernVoteMaxPerIdentifiedUser: u16 = T::ConcernVoteMaxPerIdentifiedUser::get() as u16;

		/// How high is the reward if a concern that the user voted for passes into next round?
		const ConcernVoteCorrectReward: BalanceOf<T> = T::ConcernVoteCorrectReward::get();

		/// Part 3: Final evaluation of the winning proposals and associated concern by the council
		/// How much time is reserved for the council to vote? Value in seconds
		const CouncilVoteRoundDuration: T::BlockNumber = T::CouncilVoteRoundDuration::get();

		/// How many percent of the council must agree that a concern is too serious to launch a
		/// project from the associated proposal?
		const CouncilAcceptConcernMinVotes: Permill = T::CouncilAcceptConcernMinVotes::get() as Permill;
		

		/// If this module was added during a runtime upgrade, start the state machine
		// If you want to implement this feature, consider:
		// 1. This function is called before the runtime state is initialized, therefore
		// 	  we don't have access to the current block number. This means that we we cannot
		//    figure out when the scheduler should transit into the next state in do_state_transit() (31. Oct 2020)
		// 2. This function might be called multiple times during the upgrade process. When using
		//	  an anonymous scheduler (like currently - 31. Oct 2020), multiple calls are scheduled.
		//	  It might be necessary to switch to a named scheduler.
		/*
		fn on_runtime_upgrade() -> Weight {
			if let States::Uninitialized = <State>::get() {
				let _ = Self::do_state_transit();
			}

			0
		}*/

		
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		/// Enforce state transit
		// Only for test purposes. Will be deleted in the future.
		fn state_transit(origin) -> DispatchResult {
			// check and change the current state
			ensure_root(origin)?;
			Self::do_state_transit()
		}
	}
}

impl<T: Trait> Module<T> {
	fn do_state_transit() -> DispatchResult {
		let mut transit_time: T::BlockNumber = T::BlockNumber::from(0);

		// TODO: Check if there are proposals.
		// TODO: Early state transition when every member of the council has voted.
		// TODO: Make Scheduler named and cancel any scheduled state transits before adding new.
		let newstate: States = <State>::mutate(|state| {
			match state {
				States::Uninitialized => {
					*state = States::Propose;
					transit_time = T::ProposeRoundDuration::get();
				},
				States::Propose => {
					*state = States::VotePropose;
					transit_time = T::ProposeVoteDuration::get();
				},
				States::VotePropose => {
					*state = States::Concern;
					transit_time = T::ConcernRoundDuration::get();
				},
				States::Concern => {
					*state = States::VoteConcern;
					transit_time = T::ConcernVoteDuration::get();
				},
				States::VoteConcern => {
					*state = States::VoteCouncil;
					transit_time = T::CouncilVoteRoundDuration::get();
				},
				States::VoteCouncil => {
					*state = States::Propose;
					transit_time = T::ProposeRoundDuration::get();
				}
			}
		*state
		});

		let current_block: T::BlockNumber = frame_system::Module::<T>::block_number();
		let next_state_transit: T::BlockNumber = current_block + transit_time;

		if T::Scheduler::schedule(
			DispatchTime::At(next_state_transit),
			None,
			LOWEST_PRIORITY,
			Root.into(),
			Call::state_transit().into(),
		).is_err() {
			// Todo: Appropriate Error or handling.
			return Err(DispatchError::Other("Setting anonymous scheduler for \"state_transit\" failed"));
		};

		NextTransit::<T>::put(next_state_transit);
		Self::deposit_event(Event::StateRotated(newstate));
		Ok(())
	}
}
