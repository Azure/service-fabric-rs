// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use windows_bindgen::{bindgen, Result};

fn main() -> Result<()> {
    let log = bindgen(["--etc", "bindings.txt"])?;
    println!("{}", log);
    Ok(())
}
