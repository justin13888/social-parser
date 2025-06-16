#[cfg(feature = "typescript")]
pub use typescript::*;

pub mod meta;

#[cfg(feature = "typescript")]
mod typescript {
    use neon::prelude::*;

    /// Initialize context for different platforms
    pub(super) fn init_context(cx: &mut ModuleContext) -> NeonResult<()> {
        meta::init_context(cx)?;

        Ok(())
    }
}
