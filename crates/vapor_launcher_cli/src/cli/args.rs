//! Shared launcher CLI argument helpers.

use vapor_launcher_core as core;
pub(super) use vapor_launcher_core::{ContentSource, ContentType};

pub(super) fn child(content_type: ContentType, content_id: String) -> core::ChildContentRef {
    core::ChildContentRef { content_type, content_id }
}
