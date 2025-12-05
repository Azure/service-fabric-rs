// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use mssf_core::WString;
use mssf_util::tokio::TokioExecutor;

/// Shared by the entire SF app.
pub struct AppContext {
    pub port: u32,
    pub hostname: WString,
    pub rt: TokioExecutor,
}

impl AppContext {
    pub fn new(port: u32, hostname: WString, rt: TokioExecutor) -> Self {
        Self { port, hostname, rt }
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}
