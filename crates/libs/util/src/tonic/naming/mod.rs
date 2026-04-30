// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Naming layer: turn an SF (or any other) name into a `DialTarget`.
//!
//! The trait is transport-agnostic: it returns a `DialTarget`
//! (host + port) and pulls in nothing from tonic / hyper / tower.

mod default;
mod resolver;
mod selector;

pub use self::default::{FabricTargetResolver, FabricTargetResolverBuilder};
pub use self::resolver::{BoxError, TargetResolver};
pub use self::selector::{DialTarget, SelectError, TargetSelector};
