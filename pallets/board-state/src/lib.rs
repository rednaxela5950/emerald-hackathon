//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::H256;

	// --- Constants ---

	// --- Type Definitions ---

	/// A struct to store a single block-number. Has all the right derives to store it in storage.
	/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_storage_derives/index.html>
	#[derive(
		Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct CompositeStruct<T: Config> {
		/// A block number.
		pub(crate) block_number: BlockNumberFor<T>,
	}

	/// Index for identifying boards.
	pub type BoardIndex = u16;
	/// Index for identifying threads within a board.
	pub type ThreadIndex = u16;
	/// Index for identifying posts within a thread.
	pub type PostIndex = u16;
	/// Index for identifiying shards of a board.
	pub type ShardIndex = u8;
	/// Index for identifying posts in a post buffer.
	pub type BufferIndex = u16;

	/// Content Identifier: A fixed-size byte array (e.g., 256-bit hash).
	pub type Cid = H256;

	/// Shard attester set: A dynamic-size array of AccountIds.
	pub type Attesters<T: Config> = BoundedVec<T::AccountId, T::AttesterSetSize>;

	/// Use Config associated type for flexibility on description length.
	type MaxDescLength<T> = <T as Config>::MaxDescLength;
	/// Use Config associated type for flexibility on rules length.
	type MaxRulesLength<T> = <T as Config>::MaxRulesLength;

	/// Metadata associated with a board.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct BoardMetadata<T: Config> {
		/// Name of the board.
		pub name: BoundedVec<u8, MaxNameLength<T>>,
		/// Short description of the board's topic.
		pub description: BoundedVec<u8, MaxDescLength<T>>,
		/// Rules specific to the board.
		pub rules: BoundedVec<u8, MaxRulesLength<T>>,
		/// The number of threads the board has.
		pub number_of_threads: ThreadIndex,
		/// The maximum number of posts each thread can have.
		pub posts_per_thread: PostIndex,
	}

	/// Metadata associated with a thread.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct ThreadMetadata<T: Config> {
		/// The block number when the thread was last bumped (created or last post added).
		pub bump_time: BlockNumberFor<T>,
		/// The number of active posts in this thread slot. Used to find the next PostIndex.
		pub post_count: PostIndex,
	}

	/// Data associated with a post.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct PostData<T: Config> {
		/// Content Identifier (fixed-size hash).
		pub cid: Cid, // Now uses the fixed-size array type
		/// The account ID of the author who created the post.
		pub author: T::AccountId,
		/// Block number when the post was created.
		pub created_at: BlockNumberFor<T>,
	}

	/// Data associated with a buffered post.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct BufferedPost<T: Config> {
		/// The core content and metadata of the post.
		pub data: PostData<T>,
		/// The index of the board this post belongs to.
		pub board_index: BoardIndex,
		/// The index of the thread this post belongs to within its board.
		pub thread_index: ThreadIndex,
	}

	/// A vote in the commit phase.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Vote {
		/// The attester votes that the data is available.
		True,
		/// The attester votes that the data is not available.
		False,
	}

	/// A commit of a vote.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub enum AttestationState<T: Config> {
		Pending,
		FirstCommit(H256),
		SecondCommit(H256, H256),
		Revealed(RevealedVote),
	}

	/// Data associated with an attestation.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AttestationData<T: Config> {
		/// The block number when the post was created.
		pub created_at: BlockNumberFor<T>,
		/// The votes for the attestation.
		pub votes: BoundedVec<AttestationState<T>, T::AttesterSetSize>,
	}

	/// A revealed vote.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum RevealedVote {
		/// The attester voted that the data is available.
		Aye,
		/// The attester voted that the data is not available.
		Nay,
		/// The revealed vote was invalid.
		Invalid,
	}

	// --- Pallet Definition ---
	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// --- Pallet Configuration ---
	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;

		// --- Configurable Constants ---

		/// Maximum length for a board name.
		#[pallet::constant]
		type MaxNameLength: Get<u32>;

		/// Maximum length for a board description.
		#[pallet::constant]
		type MaxDescLength: Get<u32>;

		/// Maximum length for board rules.
		#[pallet::constant]
		type MaxRulesLength: Get<u32>;

		/// Maximum number of attesters per shard.
		#[pallet::constant]
		type AttesterSetSize: Get<u32>;
	}

	// --- Pallet Storage ---

	#[pallet::storage]
	#[pallet::getter(fn board)]
	/// Stores metadata for each board.
	/// Key: BoardIndex
	/// Value: BoardMetadata
	pub type Board<T: Config> = StorageMap<
		_,
		Twox64Concat,
		BoardIndex,
		BoardMetadata<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn thread)]
	/// Stores metadata for each thread within a board.
	/// Key1: BoardIndex
	/// Key2: ThreadIndex
	/// Value: ThreadMetadata
	pub type Thread<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		BoardIndex,
		Twox64Concat,
		ThreadIndex,
		ThreadMetadata<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn post)]
	/// Stores the data for each post slot within a thread.
	/// Key1: BoardIndex
	/// Key2: ThreadIndex
	/// Key3: PostIndex
	/// Value: PostData
	pub type Post<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, BoardIndex>,
			NMapKey<Twox64Concat, ThreadIndex>,
			NMapKey<Twox64Concat, PostIndex>,
		),
		PostData<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn shard_attesters)]
	/// Stores the attester set for a shard.
	/// Key1: BoardIndex
	/// Key2: ShardIndex
	/// Value: Attesters<T>
	pub type ShardAttesters<T: Config> =
		StorageDoubleMap<_, Twox64Concat, BoardIndex, Twox64Concat, ShardIndex, Attesters<T>>;

	#[pallet::storage]
	#[pallet::getter(fn buffer_head)]
	/// Stores an index for the head of a board post buffer.
	/// Key: BoardIndex
	/// Value: BufferIndex
	pub type BufferHead<T: Config> = StorageMap<
		_,
		Twox64Concat,
		BoardIndex,
		BufferIndex,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn buffered_posts)]
	/// Stores posts pending availability attestation.
	/// Key1: BoardIndex
	/// Key2: BufferIndex
	/// Value: BufferedPost<T>
	pub type BufferedPosts<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		BoardIndex,
		Twox64Concat,
		BufferIndex,
		BufferedPost<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn attestations)]
	/// Stores a shard's attestations for a buffered post.
	/// Key1: BoardIndex
	/// Key2: BufferIndex
	/// Key3: ShardIndex
	/// Value: A bounded vector of attestations.
	pub type Attestations<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, BoardIndex>,
			NMapKey<Twox64Concat, BufferIndex>,
			NMapKey<Twox64Concat, ShardIndex>,
		),
		BoundedVec<AttestationState<T>, T::AttesterSetSize>,
	>;

	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		SomethingStored {
			/// The new value set.
			something: u32,
			/// The account who set the new value.
			who: T::AccountId,
		},
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a single u32 value as a parameter, writes the value
		/// to storage and emits an event.
		///
		/// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
		/// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::<T>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });

			// Return a successful `DispatchResult`
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		///
		/// It checks that the caller is a signed origin and reads the current value from the
		/// `Something` storage item. If a current value exists, it is incremented by 1 and then
		/// written back to storage.
		///
		/// ## Errors
		///
		/// The function will return an error under the following conditions:
		///
		/// - If no value has been set ([`Error::NoneValue`])
		/// - If incrementing the value in storage causes an arithmetic overflow
		///   ([`Error::StorageOverflow`])
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::<T>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage. This will cause an error in the event
					// of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::<T>::put(new);
					Ok(())
				},
			}
		}
	}
}
