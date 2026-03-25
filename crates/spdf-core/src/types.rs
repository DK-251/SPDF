use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// SPDF format version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpdfVersion {
    pub major: u32,
    pub minor: u32,
}

impl SpdfVersion {
    pub const V1_0: Self = Self { major: 1, minor: 0 };
}

impl std::fmt::Display for SpdfVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Document lifecycle states. Transitions are one-way from SIGNED onward.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentState {
    Draft,
    Review,
    Signed,
    Certified,
}

impl DocumentState {
    /// Returns `true` if transitioning from `self` to `target` is allowed.
    pub fn can_transition_to(&self, target: &DocumentState) -> bool {
        matches!(
            (self, target),
            (DocumentState::Draft, DocumentState::Review)
                | (DocumentState::Review, DocumentState::Draft)
                | (DocumentState::Review, DocumentState::Signed)
                | (DocumentState::Signed, DocumentState::Certified)
        )
    }
}

/// Text direction for the document.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextDirection {
    Ltr,
    Rtl,
}

/// Identifies the origin of a document.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SourceFormat {
    Native,
    ConvertedFromPdf,
    Template,
}

/// Generator metadata embedded in the manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorInfo {
    pub name: String,
    pub version: String,
}

/// Element ID: unique within a document.
/// Format: `el-{unix_ms}-{seq}-{rand_hex}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ElementId(pub String);

impl ElementId {
    pub fn new() -> Self {
        let now = Utc::now().timestamp_millis();
        let rand: u16 = (Uuid::new_v4().as_u128() & 0xFFFF) as u16;
        Self(format!("el-{now}-{rand:04x}"))
    }
}

impl Default for ElementId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Document ID: globally unique, prefixed with `spdf-`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new() -> Self {
        Self(format!("spdf-{}", Uuid::new_v4()))
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Asset reference within the container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRef {
    pub asset_id: String,
    pub mime_type: String,
    pub size: u64,
    pub checksum: String,
}

/// Timestamps common to most elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamps {
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Timestamps {
    pub fn now() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            modified_at: now,
        }
    }
}
