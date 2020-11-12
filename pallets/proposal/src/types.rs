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

use frame_support::dispatch::{Decode, Encode, Vec};
use pallet_community_identity::IdentityId;
use frame_system::Trait;
#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};
use crate::Permill;


// Important: Change Vec<u8> to a fixed length type (otherwise attackable)
pub type ProposalCID = Vec<u8>;
pub type ConcernCID = ProposalCID;

/// Contains proposal and vote count
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proposal {
	pub proposal: ProposalCID,
	pub votes: u32,
}

impl Proposal {
	pub fn new(proposal: ProposalCID) -> Self {
		Proposal{proposal, votes: 0}
	}
}

impl Default for Proposal {
	fn default() -> Self {
		Proposal::new(ProposalCID::default())
	}
}

/// Contains concern and vote count
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Concern {
	pub concern: ConcernCID,
	pub associated_proposal: ProposalCID,
	pub votes: u32,
}

impl Concern {
	pub fn new(concern: ConcernCID, associated_proposal: ProposalCID) -> Self {
		Concern{concern, associated_proposal, votes: 0}
	}
}

impl Default for Concern {
	fn default() -> Self {
		Concern::new(ConcernCID::default(), ProposalCID::default())
	}
}

// TODO: Remove pub fields and write getters
/// Contains one winning proposal
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalWinner<T: Trait> {
	pub proposer: IdentityId<T>, // For later rewards
	pub proposal: ProposalCID,
	pub vote_ratio: Permill
}

impl<T: Trait> ProposalWinner<T> {
	pub fn new(proposer: IdentityId<T>, proposal: ProposalCID, vote_ratio: Permill) -> Self {
		ProposalWinner{proposer, proposal, vote_ratio}
	}
}

impl<T: Trait> Default for ProposalWinner<T> {
	fn default() -> Self {
		ProposalWinner::new(Default::default(), Default::default(), Default::default())
	}
}

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
