// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
// TODO: implement wrapper around FABRIC_CLAIMS_CREDENTIALS

use windows_core::WString;

use super::FabricProtectionLevel;

#[allow(non_snake_case, reason = "Consistency with underlying API")]
struct FabricClaimsCredentials {
    pub ServerCommonNames: Vec<WString>,
    pub IssuerThumbprints: Vec<WString>,
    pub LocalClaims: WString,
    pub ProtectionLevel: FabricProtectionLevel,
    /// FABRIC_CLAIMS_CREDENTIALS_EX1
    pub ServerThumbprints: Option<Vec<WString>>,
}

// TODO: finish this
