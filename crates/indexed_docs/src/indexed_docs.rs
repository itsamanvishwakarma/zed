mod extension_indexed_docs_provider;
mod providers;
mod registry;
mod store;

use gpui::App;

pub use crate::extension_indexed_docs_provider::ExtensionIndexedDocsProvider;
pub use crate::providers::rustdoc::*;
pub use crate::registry::*;
pub use crate::store::*;

pub fn init(cx: &mut App) {
    IndexedDocsRegistry::init_global(cx);
    extension_indexed_docs_provider::init(cx);
}
