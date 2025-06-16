pub mod instagram;

#[cfg(feature = "typescript")]
pub use typescript::*;

#[cfg(feature = "typescript")]
mod typescript {
    use neon::prelude::*;

    /// Initialize context for meta module
    pub(super) fn init_context(_cx: &mut ModuleContext) -> NeonResult<()> {
        // TODO: Implement

        Ok(())
    }
}
