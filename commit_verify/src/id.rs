// Client-side-validation foundation libraries.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bitcoin_hashes::sha256::Midstate;
use bitcoin_hashes::{sha256, Hash};

use crate::CommitEncode;

/// High-level API used in client-side validation for producing a single
/// commitment to the data, which includes running all necessary procedures like
/// concealment with [`CommitConceal`], merklization, strict encoding,
/// wrapped into [`CommitEncode`], followed by the actual commitment to its
/// output.
pub trait CommitmentId: CommitEncode {
    const TAG: [u8; 32];

    /// Type of the resulting commitment.
    type Id: From<[u8; 32]>;

    /// Performs commitment to client-side-validated data
    #[inline]
    fn commitment_id(&self) -> Self::Id {
        let midstate = Midstate::from_inner(Self::TAG);
        let mut engine = sha256::HashEngine::from_midstate(midstate, 64);
        self.commit_encode(&mut engine);
        sha256::Hash::from_engine(engine).into_inner().into()
    }
}