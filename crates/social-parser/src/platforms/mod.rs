use neon::prelude::*;

pub mod meta;

/// Initialize context for different platforms
pub(super) fn init_context(cx: &mut ModuleContext) -> NeonResult<()> {
    meta::init_context(cx)?;

    Ok(())
}
