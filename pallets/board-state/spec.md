# Pallet `board-state` Refactoring Spec

This document outlines the necessary changes to the `board-state` pallet to implement a commit-reveal scheme for data availability attestations.

**Process:**
1.  Before starting a step, we will review and confirm the plan.
2.  Upon completion of a step, it will be marked as "Complete."
3.  A summary of the changes will be added to the "Changelog" section.

---

## 1. Define Data Structures

**Status:** Not Started

**Plan:**
The following enums and structs will be added to `lib.rs` to represent the different stages of the attestation process.

-   **`Vote` enum:** Represents the pre-image of a vote commitment.
    -   `True`
    -   `False`
-   **`Commit<T: Config>` struct:** Represents a committed vote stored on-chain.
    -   `hash: H256`
    -   `created_at: BlockNumberFor<T>`
-   **`RevealedVote` enum:** Represents a revealed vote, stored on-chain after the reveal phase.
    -   `Aye`
    -   `Nay`
    -   `Invalid`

**Rationale:**
The commit-reveal scheme requires attesters to first commit to a vote by submitting a hash, and later reveal the original vote.
-   The `Vote` enum represents the choice an attester makes off-chain (`True` or `False`). It is combined with a secret salt, hashed, and then submitted as a `Commit`. The pallet uses the `Vote` enum structure during the reveal phase to verify the attester's submission against the stored hash. It is **never stored directly** on-chain during the commit phase.
-   The `Commit` struct stores the hash on-chain, preventing other attesters from seeing the vote before the reveal phase.
-   The `RevealedVote` enum represents the final, public state of a vote after the reveal phase is complete. `Invalid` is included to handle cases where a revealed vote does not match the committed hash.

---

## 2. Add Imports

**Status:** Not Started

**Plan:**
The following import will be added to `lib.rs`.

```rust
use sp_core::H256;
```

**Rationale:**
`H256` is a 256-bit hash type provided by `sp_core`, a core Substrate library. It is the standard type for representing cryptographic hashes within the Substrate ecosystem and is the appropriate type for the `hash` field in our `Commit` struct.

---

## 3. Update Configuration (`Config` trait)

**Status:** Not Started

**Plan:**
The `Config` trait will be updated to change the type of `AttesterSetSize`.

-   Change `type AttesterSetSize: Get<u8>;` to `type AttesterSetSize: Get<u32>;`.

**Rationale:**
The `BoundedVec` type, which is used for storing collections of a dynamic length up to a fixed bound, requires its bound to be a `u32`. The `AttesterSetSize` constant defines this bound for collections of attesters. Changing its type from `u8` to `u32` is necessary to comply with the trait constraints of `BoundedVec`.

---

## 4. Refine Type Aliases

**Status:** Not Started

**Plan:**
The `Attesters<T>` type alias will be corrected to use the proper generic constraints.

-   Change `pub type Attesters<T> = BoundedVec<AccountId, AttesterSetSize>;` to `pub type Attesters<T: Config> = BoundedVec<T::AccountId, T::AttesterSetSize>;`

**Rationale:**
The original type alias was missing the `T: Config` trait bound. This bound is required to access the associated types `T::AccountId` and `T::AttesterSetSize` which are defined within the `Config` trait. The corrected version ensures that `Attesters<T>` is correctly defined for any type `T` that implements our pallet's `Config`.

---

## 5. Reorganize Storage

**Status:** Not Started

**Plan:**
The pallet's storage items will be renamed and redefined for clarity, correctness, and to prevent naming conflicts.

-   Rename `StorageDoubleMap` for shards from `Thread` to `ShardAttesters`.
-   Rename `StorageMap` for the post buffer head from `Post` to `BufferHead`.
-   Rename `StorageDoubleMap` for buffered posts from `Thread` to `BufferedPosts`.
-   Rename `StorageNMap` for buffered post attestations from `Post` to `BufferedPostCommits`.
-   Correct the keys for `BufferedPostCommits` to be `(BoardIndex, BufferIndex, ShardIndex)`.
-   Define the value for `BufferedPostCommits` as `BoundedVec<Commit<T>, T::AttesterSetSize>`.
-   Add a new `StorageNMap` named `RevealedVotes` to store revealed votes with keys `(BoardIndex, BufferIndex, ShardIndex)` and value `BoundedVec<RevealedVote, T::AttesterSetSize>`.

**Rationale:**
Several storage items in the template code were named `Thread` or `Post`, leading to ambiguity and naming collisions.
-   **Renaming:** The renames (`ShardAttesters`, `BufferHead`, `BufferedPosts`, `BufferedPostCommits`) provide clear, descriptive names for each storage item, reflecting its actual purpose.
-   **Key Correction:** The keys for `BufferedPostCommits` must correctly map a specific shard's attestations to a specific buffered post. The `(BoardIndex, BufferIndex, ShardIndex)` structure achieves this.
-   **Value Definition:** The values for `BufferedPostCommits` and the new `RevealedVotes` map must be `BoundedVec`s to hold a collection of commits or revealed votes from all attesters in a shard, bounded by `AttesterSetSize`.

---

## Changelog

*   **2025-07-17:**
    *   Initialized `spec.md`.
    *   Added detailed rationales for each step.
    *   Added status tracking and a changelog.
    *   Clarified the role of the `Vote` enum in the commit-reveal process.