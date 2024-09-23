// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::{Error, ErrorKind, ResultExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::TestContext;

pub struct MockTransactionRequest {
    pub request_path: PathBuf,
    pub response_path: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) struct MockTransaction {
    pub(crate) current_request_number: Arc<AtomicUsize>,
    transaction_path: PathBuf,
}

impl MockTransaction {
    pub(crate) fn new(context: TestContext) -> Self {
        // Find the transaction root path
        let transaction_path = {
            let mut path = PathBuf::from(context.package_path);
            path.push("tests");
            path.push("data");
            path.push("transactions");
            path.push(format!(
                "{}.{}",
                &context.module_under_test, &context.transaction_name
            ));
            path
        };

        Self {
            current_request_number: Arc::new(AtomicUsize::new(0)),
            transaction_path,
        }
    }

    pub(crate) fn new_request(
        &self,
        create_when_not_exist: bool,
    ) -> azure_core::Result<MockTransactionRequest> {
        // First, ensure the transaction path exists
        if !self.transaction_path.exists() {
            if create_when_not_exist {
                std::fs::create_dir_all(&self.transaction_path).with_context(
                    ErrorKind::MockFramework,
                    || {
                        format!(
                            "cannot create transaction folder: {}",
                            self.transaction_path.display()
                        )
                    },
                )?;
            } else {
                return Err(Error::with_message(ErrorKind::MockFramework, || {
                    format!(
                        "the transaction location '{}' does not exist",
                        self.transaction_path
                            .canonicalize()
                            .unwrap_or(self.transaction_path.clone())
                            .display()
                    )
                }));
            }
        }

        let number = self.current_request_number.fetch_add(1, Ordering::Relaxed);
        let request_path = {
            let mut p = self.transaction_path.clone();
            p.push(format!("{number}.request.json"));
            p
        };
        let response_path = {
            let mut p = self.transaction_path.clone();
            p.push(format!("{number}.response.json"));
            p
        };

        Ok(MockTransactionRequest {
            request_path,
            response_path,
        })
    }
}
