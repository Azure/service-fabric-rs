// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Hyper-compatible `Service<http::Uri>` connector that delegates
//! the "what to dial" decision to a [`TargetResolver`].

mod service;

pub use self::service::{TargetConnector, TargetConnectorBuilder};
