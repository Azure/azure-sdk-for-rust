// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Provides an implementation of the Azure Data Cosmos SDK query engine API.

use core::str;

use serde::Deserialize;

use crate::query::{PartitionKeyRange, QueryPipeline};

pub struct QueryEngine;

impl azure_data_cosmos::query::QueryEngine for QueryEngine {
    fn create_pipeline(
        &self,
        query: &str,
        plan: &[u8],
        pkranges: &[u8],
    ) -> azure_core::Result<Box<dyn azure_data_cosmos::query::QueryPipeline + Send>> {
        #[derive(Deserialize)]
        struct PartitionKeyRangeResult {
            #[serde(rename = "PartitionKeyRanges")]
            pub ranges: Vec<PartitionKeyRange>,
        }

        let plan = serde_json::from_slice(plan)?;
        let pkranges: PartitionKeyRangeResult = serde_json::from_slice(pkranges)?;
        let pipeline = QueryPipeline::new(query, plan, pkranges.ranges)?;
        Ok(Box::new(QueryPipelineAdapter(pipeline)))
    }

    fn supported_features(&self) -> azure_core::Result<&str> {
        Ok(crate::query::SUPPORTED_FEATURES.as_str())
    }
}

impl From<crate::Error> for azure_core::Error {
    fn from(err: crate::Error) -> Self {
        let kind = match err.kind() {
            crate::ErrorKind::DeserializationError => azure_core::error::ErrorKind::DataConversion,
            crate::ErrorKind::UnknownPartitionKeyRange => {
                azure_core::error::ErrorKind::DataConversion
            }
            crate::ErrorKind::UnsupportedQueryPlan => azure_core::error::ErrorKind::DataConversion,
            crate::ErrorKind::InvalidUtf8String => azure_core::error::ErrorKind::DataConversion,
            _ => azure_core::error::ErrorKind::Other,
        };
        let message = format!("{}", &err);
        azure_core::Error::with_error(kind, err, message)
    }
}

pub struct QueryPipelineAdapter(crate::query::QueryPipeline);

impl azure_data_cosmos::query::QueryPipeline for QueryPipelineAdapter {
    fn query(&self) -> Option<&str> {
        self.0.query()
    }

    fn complete(&self) -> bool {
        self.0.complete()
    }

    fn run(&mut self) -> azure_core::Result<azure_data_cosmos::query::PipelineResult> {
        let result = self.0.run()?;
        Ok(azure_data_cosmos::query::PipelineResult {
            is_completed: result.terminated,
            items: result.items,
            requests: result
                .requests
                .into_iter()
                .map(|request| azure_data_cosmos::query::QueryRequest {
                    id: request.id,
                    partition_key_range_id: request.pkrange_id.into_owned(),
                    continuation: request.continuation,
                    query: request.query,
                    include_parameters: request.include_parameters,
                    drain: false,
                })
                .collect(),
        })
    }

    fn provide_data(
        &mut self,
        data: Vec<azure_data_cosmos::query::QueryResult>,
    ) -> azure_core::Result<()> {
        tracing::debug!("providing data to pipeline");
        for data in data {
            self.0.provide_data(
                data.partition_key_range_id,
                data.request_id,
                data.result,
                data.next_continuation,
            )?;
        }
        Ok(())
    }
}
