/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::media::MediaJobState;
use crate::common::{AppId, DomainId, MediaObjectId, TaskId};
use crate::AppMediaObjectId;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportMediaJobProgress {
    /// Reporting upload progress
    UploadFromDomain {
        app_id:   AppId,
        media_id: MediaObjectId,
        state:    MediaJobState,
    },
    /// Reporting download progress
    DownloadToDomain {
        app_id:   AppId,
        task_id:  Option<TaskId>,
        media_id: MediaObjectId,
        state:    MediaJobState,
    },
}

/// Confirming upload is created
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UploadCreated {
    Created { media_id: AppMediaObjectId, domain_id: DomainId },
}

/// Confirming download is created
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DownloadCreated {
    Created { media_id: AppMediaObjectId, domain_id: DomainId },
}

/// Confirming media object is scheduled for deletion
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaObjectDeleted {
    Deleted { media_id: AppMediaObjectId },
}
