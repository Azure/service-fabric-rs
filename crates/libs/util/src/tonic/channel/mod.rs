// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! Channel composition: [`SwapChannel`] (atomic Channel hot-swap)
//! plus the [`TargetChannel`] convenience alias / builder.

mod builder;
mod swap;

pub use self::builder::{TargetChannel, TargetChannelBuilder};
pub use self::swap::SwapChannel;
