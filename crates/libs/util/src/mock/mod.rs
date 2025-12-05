// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// Experimental. APIs may change without notice.
// Mock utilities for testing.

mod runtime;
pub use runtime::{CreateServiceArg, StatelessServiceInstanceDriver};

mod stateless;
pub use stateless::StatelessServicePartitionMock;
