// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

pub struct FabricX509Credentials{
    AllowedCommonNames: Vec<WString> ,
    FABRIC_X509_FIND_TYPE FindType;
    void *FindValue;
    FABRIC_X509_STORE_LOCATION StoreLocation;
    LPCWSTR StoreName;
    FABRIC_PROTECTION_LEVEL ProtectionLevel;
    void *Reserved;

}