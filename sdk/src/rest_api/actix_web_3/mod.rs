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

mod backend_state;
mod endpoint;
mod key_state;
mod paging;
pub mod routes;
#[cfg(feature = "rest-api-actix-web-3-run")]
mod run;
mod service;
mod store_state;

pub use backend_state::BackendState;
pub use endpoint::{Backend, Endpoint};
pub use key_state::KeyState;
pub use paging::QueryPaging;
#[cfg(feature = "rest-api-actix-web-3-run")]
pub use run::run;
pub use service::{AcceptServiceIdParam, QueryServiceId};
pub use store_state::StoreState;
