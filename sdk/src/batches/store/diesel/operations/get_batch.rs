// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::BatchStoreOperations;
use crate::batches::store::{
    diesel::{
        schema::{batches, transactions},
        BatchModel,
    },
    Batch, BatchStoreError,
};
use crate::error::InternalError;
use diesel::{prelude::*, result::Error::NotFound};

pub(in crate::batches::store::diesel) trait GetBatchOperation {
    fn get_batch(&self, id: &str) -> Result<Option<Batch>, BatchStoreError>;
}

#[cfg(feature = "postgres")]
impl<'a> GetBatchOperation for BatchStoreOperations<'a, diesel::pg::PgConnection> {
    fn get_batch(&self, id: &str) -> Result<Option<Batch>, BatchStoreError> {
        self.conn.transaction::<_, BatchStoreError, _>(|| {
            let batch_model = batches::table
                .select(batches::all_columns)
                .filter(batches::header_signature.eq(id))
                .first::<BatchModel>(self.conn)
                .map(Some)
                .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
                .map_err(|err| {
                    BatchStoreError::InternalError(InternalError::from_source(Box::new(err)))
                })?;

            if let Some(batch_model) = batch_model {
                let transaction_models = transactions::table
                    .select(transactions::all_columns)
                    .filter(transactions::batch_id.eq(id))
                    .load(self.conn)
                    .map_err(|err| {
                        BatchStoreError::InternalError(InternalError::from_source(Box::new(err)))
                    })?;

                Ok(Some(Batch::from((batch_model, transaction_models))))
            } else {
                Ok(None)
            }
        })
    }
}

#[cfg(feature = "sqlite")]
impl<'a> GetBatchOperation for BatchStoreOperations<'a, diesel::sqlite::SqliteConnection> {
    fn get_batch(&self, id: &str) -> Result<Option<Batch>, BatchStoreError> {
        self.conn.transaction::<_, BatchStoreError, _>(|| {
            let batch_model = batches::table
                .select(batches::all_columns)
                .filter(batches::header_signature.eq(id))
                .first::<BatchModel>(self.conn)
                .map(Some)
                .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
                .map_err(|err| {
                    BatchStoreError::InternalError(InternalError::from_source(Box::new(err)))
                })?;

            if let Some(batch_model) = batch_model {
                let transaction_models = transactions::table
                    .select(transactions::all_columns)
                    .filter(transactions::batch_id.eq(id))
                    .load(self.conn)
                    .map_err(|err| {
                        BatchStoreError::InternalError(InternalError::from_source(Box::new(err)))
                    })?;

                Ok(Some(Batch::from((batch_model, transaction_models))))
            } else {
                Ok(None)
            }
        })
    }
}
