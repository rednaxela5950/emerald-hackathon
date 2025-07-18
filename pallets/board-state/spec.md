# Pallet `board-state` Refactoring Spec

This document outlines the necessary changes to the `board-state` pallet to implement a commit-reveal scheme for data availability attestations.

**Process:**
1.  Before starting a step, we will review and confirm the plan.
2.  Upon completion of a step, it will be marked as "Complete."
3.  A summary of the changes will be added to the "Changelog" section.

---

## 1. Define Core Data Structures

**Status:** Complete

**Plan:**
The following enums will be added to `lib.rs` to represent the core data of the attestation process.

-   **`Vote` enum:** Represents the pre-image of a vote commitment (`True`/`False`).
-   **`RevealedVote` enum:** Represents a revealed vote (`Aye`/`Nay`/`Invalid`).

**Rationale:**
These simple enums form the basis of the commit-reveal scheme. `Vote` is used off-chain by attesters and verified by the pallet during the reveal phase. `RevealedVote` represents the final, on-chain outcome of an attestation.

---

## 2. Refactor `Cid` Type

**Status:** Complete

**Plan:**
The `Cid` type alias will be refactored to use the idiomatic `H256` hash type.

-   Change `pub type Cid = [u8; CID_LENGTH];` to `pub type Cid = H256;`
-   Remove the `pub const CID_LENGTH: usize = 32;` constant.

**Rationale:**
Using `sp_core::H256` is the idiomatic way to represent 256-bit hashes in Substrate. This change makes the `Cid` type (representing a Merkle root) consistent with other hashes in the pallet and the broader ecosystem.

---

## 3. Add Imports

**Status:** Complete

**Plan:**
The following import will be added to `lib.rs`.

```rust
use sp_core::H256;
```

**Rationale:**
`H256` is a 256-bit hash type provided by `sp_core` and is required for our data structures.

---

## 4. Update Configuration (`Config` trait)

**Status:** Complete

**Plan:**
The `Config` trait will be updated to change the type of `AttesterSetSize`.

-   Change `type AttesterSetSize: Get<u8>;` to `type AttesterSetSize: Get<u32>;`.

**Rationale:**
The `BoundedVec` type requires its bound to be a `u32`. The `AttesterSetSize` constant defines this bound for collections of attesters.

---

## 5. Refine Type Aliases

**Status:** Complete

**Plan:**
The `Attesters<T>` type alias will be corrected to use the proper generic constraints.

-   Change `pub type Attesters<T> = BoundedVec<AccountId, AttesterSetSize>;` to `pub type Attesters<T: Config> = BoundedVec<T::AccountId, T::AttesterSetSize>;`

**Rationale:**
The original type alias was missing the `T: Config` trait bound, which is required to access associated types from the `Config` trait.

---

## 6. Implement Attestation Lifecycle and Storage

**Status:** Complete

**Plan:**
A new `AttestationState` enum will be created to correctly and efficiently model the two-commit-then-reveal lifecycle. The pallet's storage items will be renamed and reorganized for clarity and to resolve naming conflicts.

*   **6a. Define `AttestationState` Enum:**
    *   **Action:** Add a new enum `AttestationState<T: Config>` to `lib.rs`.
    *   **Definition:**
        ```rust
        pub enum AttestationState<T: Config> {
            Pending,
            FirstCommit(H256),
            SecondCommit(H256, H256),
            Revealed(RevealedVote),
        }
        ```

*   **6b. Fix `ShardAttesters` Storage:**
    *   **Action:** Rename getter to `shard_attesters` and type to `ShardAttesters`.

*   **6c. Fix `BufferHead` Storage:**
    *   **Action:** Rename getter to `buffer_head` and type to `BufferHead`.

*   **6d. Fix `BufferedPosts` Storage:**
    *   **Action:** Rename getter to `buffered_posts` and type to `BufferedPosts`.

*   **6e. Create `Attestations` Storage:**
    *   **Action:** The old, incorrect `buffered_post_shard` item will be completely replaced with a new `StorageNMap`.
    *   **Getter:** `attestations`
    *   **Type:** `Attestations`
    *   **Keys:** `(BoardIndex, BufferIndex, ShardIndex)`
    *   **Value:** `BoundedVec<AttestationState<T>, T::AttesterSetSize>`

**Rationale:**
This design is highly efficient and correctly models the two-commit-then-reveal process. The `AttestationState` enum ensures that only the data relevant to the current stage of the process is stored on-chain, making invalid states impossible to represent and discarding commit hashes after they are used. This minimizes storage footprint and I/O. The storage items are renamed to be clear and descriptive, resolving collisions from the template code.

---

## Changelog

*   **2025-07-17:**
    *   **Completed Step 6:** Implemented the `AttestationState` enum and reorganized all storage items for clarity and efficiency.
    *   **Finalized plan for Step 6:** Decided on a stateful `AttestationState` enum to manage the two-commit lifecycle efficiently.
    *   Refined `Attesters<T>` type alias with correct generic constraints.
    *   Updated `Config` trait to change `AttesterSetSize` to `Get<u32>`.
    *   Added `H256` import.
    *   Refactored `Cid` type to use `H256` and removed `CID_LENGTH` constant.
    *   Added `Vote` and `RevealedVote` data structures to `lib.rs`.
    *   Initialized `spec.md`.