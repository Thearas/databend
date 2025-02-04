//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use std::env;

use tempfile::TempDir;

use crate::configs::AzureStorageBlobConfig;
use crate::configs::DiskStorageConfig;
use crate::configs::S3StorageConfig;
use crate::configs::StorageConfig;
use crate::datasources::DataSourceContext;

#[test]
fn test_context() -> common_exception::Result<()> {
    let tmp_data = TempDir::new().unwrap();
    let tmp_path = tmp_data.path().to_str().unwrap().to_string();
    let mut storage_config = StorageConfig {
        storage_type: "disk".to_string(),
        disk: DiskStorageConfig {
            data_path: tmp_path,
            /// temporary directory for testing, default to current directory
            temp_data_path: env::current_dir()?.display().to_string(),
        },
        s3: S3StorageConfig::default(),
        azure_storage_blob: AzureStorageBlobConfig::default(),
    };

    let dal = DataSourceContext::create("test_tenant", "test_cluster", storage_config.clone())
        .get_data_accessor();
    assert!(dal.is_ok());

    storage_config.storage_type = "not exists".to_string();
    let dal = DataSourceContext::create("test_tenant", "test_cluster", storage_config)
        .get_data_accessor();
    assert!(dal.is_err());

    Ok(())
}
