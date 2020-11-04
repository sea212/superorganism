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
use frame_support::{decl_module, dispatch::{
	Codec, Decode, DispatchError, Encode, fmt::{
		Debug, Formatter, Result as FMTResult},
	},
};
use frame_system::ensure_signed;
#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
/// Public interface to PhysicalIdentity
pub mod traits;


type IdentityLevel = u8;
type ProofType<T> = PhysicalProof<T, [u8; 32]>;
type Ticket<T> = <T as frame_system::Trait>::AccountId;
type IdentityId<T> = <T as frame_system::Trait>::AccountId;

/// Structure that contains the proof
#[derive(Clone, Decode, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PhysicalProof<T, ProofData> where
	ProofData: Codec + Clone + Debug + Decode + Encode + Eq + PartialEq,
	T: Trait,
{
	proof: ProofData,
	date: T::BlockNumber,
}

// impl Debug because Debug cannot be derived for T: Trait
impl<T, ProofData> Debug for PhysicalProof<T, ProofData> where 
	ProofData: Codec + Clone + Debug + Decode + Encode + Eq + PartialEq,
	T: Trait {

    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        f.debug_struct("PhysicalProof")
         .field("proof", &self.proof)
         .field("date", &self.date)
         .finish()
    }
}

/// Structure that contains the identity ID, level and proof
#[derive(Clone, Decode, Debug, Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PhysicalIdentityData<T, ProofData> where
	ProofData: Codec + Clone + Decode + Debug + Encode + Eq + PartialEq,
	T: Trait,
{
	identity: T::AccountId,
	level: IdentityLevel,
	proof: PhysicalProof<T, ProofData>,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	// type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Request a peer review to gain a specific IdentityLev
		#[weight = 10_000]
		fn request_peer_review(origin, identity_level: IdentityLevel) {
			let caller = ensure_signed(origin)?;
			Self::do_request_peer_review(caller, identity_level)?;
			// What happens here is that it either returns the Err(e) or Ok(()), DispatchResult is implicit
		}

		/// As a reviewer, approve a reviewed PhysicalIdentity by supplying a proof
		#[weight = 10_000]
		pub fn approve_identity(origin, review_process: Ticket<T>, proof_data: ProofType<T>) {
			let _ = ensure_signed(origin)?;
			Self::do_approve_identity(review_process, proof_data)?;
		}
		
		/// As a reviewer, reject a reviewed PhysicalIdentity
		#[weight = 10_000]
		pub fn reject_identity(origin, review_process: Ticket<T>) {
			let _ = ensure_signed(origin)?;
			Self::do_reject_identity(review_process)?;
		}
	}
}

impl<T: Trait> Module<T> {
	fn do_request_peer_review(user: Ticket<T>, identity_level: IdentityLevel)
		-> Result<T::AccountId, DispatchError>
	{
		// TODO implement
		Ok(user)
	}

	fn do_approve_identity(review_process: Ticket<T>, proof_data: ProofType<T>)
		-> Result<(), DispatchError>
	{
		// TODO implement
		Ok(())
	}

	fn do_reject_identity(review_process: Ticket<T>) -> Result<(), DispatchError> {
		// TODO implement
		Ok(())
	}
}

impl<T: Trait> traits::PeerReviewedPhysicalIdentity<ProofType<T>> for Module<T> {
	type Address = T::AccountId;
	type Ticket = Ticket<T>;
	type IdentityLevel = IdentityLevel;
	type IdentityId = IdentityId<T>;

	/// Request a peer review to gain a specific IdentityLevel
	fn request_peer_review(user: Self::Address, identity_level: Self::IdentityLevel)
		-> Result<Self::Ticket, DispatchError>
	{
		Self::do_request_peer_review(user, identity_level)
	}

	/// As a reviewer, approve a reviewed PhysicalIdentity by supplying a proof
	fn approve_identity(review_process: Self::Ticket, proof_data: ProofType<T>)
		-> Result<(), DispatchError>
	{
		Self::do_approve_identity(review_process, proof_data)
	}

	/// As a reviewer, reject a reviewed PhysicalIdentity
	fn reject_identity(review_process: Self::Ticket) -> Result<(), DispatchError> {
		Self::do_reject_identity(review_process)
	}

	/// Receive the identity level of a specific PhysicalIdentity.
	fn get_identity_level(identity: Self::IdentityId) -> Option<Self::IdentityLevel> {
		// TODO: implement
		Some(5)
	}
}
