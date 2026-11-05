// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use crate::{
    diagnostics::DiagnosticsContext,
    error::{CosmosError, CosmosErrorBuilder},
    models::CosmosResponse,
};

/// Request diagnostics awaiting exactly one surfaced page or error.
#[derive(Default)]
pub(super) struct RecoveryDiagnostics(Option<Arc<DiagnosticsContext>>);

impl RecoveryDiagnostics {
    pub(super) fn absorb(&mut self, diagnostics: Option<Arc<DiagnosticsContext>>) {
        let Some(next) = diagnostics else { return };
        self.0 = Some(match self.0.take() {
            Some(prior) => Arc::new(
                DiagnosticsContext::aggregate_sub_operations(&[prior, next])
                    .expect("nonempty diagnostics sources"),
            ),
            None => next,
        });
    }

    pub(super) fn take(&mut self) -> Option<Arc<DiagnosticsContext>> {
        self.0.take()
    }

    pub(super) fn attach_response(&mut self, response: CosmosResponse) -> CosmosResponse {
        match self.take() {
            Some(prior) => response.with_aggregated_prior_diagnostics(&[prior]),
            None => response,
        }
    }

    pub(super) fn attach_error(&mut self, error: CosmosError) -> CosmosError {
        if self.0.is_none() {
            return error;
        }
        self.absorb(error.diagnostics());
        let diagnostics = self.take().expect("prior diagnostics present");
        let diagnostics = Arc::new(diagnostics.clone_with_status(error.status()));
        CosmosErrorBuilder::from_error(error)
            .with_diagnostics(diagnostics)
            .build()
    }
}
