mod bindings;
pub mod common;
pub mod platforms;

use bindings::typescript;

use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    typescript::init_context(&mut cx)
}
