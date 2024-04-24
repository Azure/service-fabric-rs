// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MySettings {
    pub my_config_section: MyConfigSection,
}

#[derive(Debug, Deserialize)]
pub struct MyConfigSection {
    pub my_string: String,
    pub my_bool: bool,
    pub my_int: u32,
}
