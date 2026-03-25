//! Manifest: the first entry in every .spdf ZIP container.
//! Contains format version, layer checksums, asset index, and integrity hash.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::types::{AssetRef, DocumentId, GeneratorInfo, SpdfVersion};

/// Layer paths inside the container and their SHA-256 checksums.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerManifests {
    pub semantic: String,
    pub layout: String,
    pub styles: String,
    pub render: String,
    pub metadata: String,
    pub audit: String,
}

/// Top-level manifest.json written as the first ZIP entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub version: SpdfVersion,
    pub document_id: DocumentId,
    pub created_at: DateTime<Utc>,
    pub generator: GeneratorInfo,
    pub layers: LayerManifests,
    pub assets: Vec<AssetRef>,
    pub manifest_hash: String,
}

impl Manifest {
    /// Build a new manifest. Call `finalize` after all layer checksums are set
    /// to compute the `manifest_hash`.
    pub fn new(document_id: DocumentId, generator: GeneratorInfo) -> Self {
        Self {
            format: "SPDF".to_string(),
            version: SpdfVersion::V1_0,
            document_id,
            created_at: Utc::now(),
            generator,
            layers: LayerManifests {
                semantic: String::new(),
                layout: String::new(),
                styles: String::new(),
                render: String::new(),
                metadata: String::new(),
                audit: String::new(),
            },
            assets: Vec::new(),
            manifest_hash: String::new(),
        }
    }

    /// Compute the manifest integrity hash over all layer checksums.
    pub fn finalize(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.layers.semantic.as_bytes());
        hasher.update(self.layers.layout.as_bytes());
        hasher.update(self.layers.styles.as_bytes());
        hasher.update(self.layers.render.as_bytes());
        hasher.update(self.layers.metadata.as_bytes());
        hasher.update(self.layers.audit.as_bytes());
        for asset in &self.assets {
            hasher.update(asset.checksum.as_bytes());
        }
        self.manifest_hash = format!("{:x}", hasher.finalize());
    }
}

/// Compute SHA-256 hex digest for a byte slice.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
