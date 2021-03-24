-- Copyright 2021 Cargill Incorporated
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--     http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.
-- -----------------------------------------------------------------------------

CREATE TABLE purchase_order (
    id BIGSERIAL PRIMARY KEY,
    uid VARCHAR NOT NULL,
    workflow_status VARCHAR NOT NULL,
    is_closed BOOLEAN NOT NULL,
    accepted_version_id VARCHAR NOT NULL,
    service_id TEXT
) INHERITS (chain_record);

CREATE TABLE purchase_order_version (
    id BIGSERIAL PRIMARY KEY,
    purchase_order_uid VARCHAR NOT NULL,
    version_id VARCHAR NOT NULL,
    is_draft BOOLEAN NOT NULL,
    current_revision_number BIGINT NOT NULL,
    service_id TEXT,
) INHERITS (chain_record);

CREATE TABLE purchase_order_version_revision (
    id BIGSERIAL PRIMARY KEY,
    version_id VARCHAR NOT NULL,
    revision_number BIGINT NOT NULL,
    order_xml_3_4 VARCHAR NOT NULL,
    submitter VARCHAR NOT NULL,
    created_at BIGINT NOT NULL,
    service_id TEXT
) INHERITS (chain_record);

CREATE TABLE purchase_order_alternate_id (
    id BIGSERIAL PRIMARY KEY,
    purchase_order_uid VARCHAR NOT NULL,
    alternate_id_type VARCHAR NOT NULL,
    alternate_id VARCHAR NOT NULL,
    service_id TEXT
) INHERITS (chain_record);
