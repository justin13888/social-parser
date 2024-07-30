use neon::prelude::*;

// use log::{info, Log, Metadata, Record};

// struct JsLogger {
//     cx: Arc<Mutex<ModuleContext>>,
// }

// unsafe impl<'a> Send for JsLogger<'a> {}
// unsafe impl<'a> Sync for JsLogger<'a> {}

// impl Log for JsLogger<'_> {
//     fn enabled(&self, _metadata: &Metadata) -> bool {
//         true
//     }

//     fn log(&self, record: &Record) {
//         // if self.enabled(record.metadata()) {
//         //     if let Ok(mut cx) = self.cx.lock() {
//         //         // let f = &mut **cx;
//         //         let js_log: Handle<JsFunction> = cx
//         //             .string("console.log")
//         //             .downcast_or_throw(&mut **cx)
//         //             .unwrap();
//         //         js_log.call(&mut **cx, cx.undefined(), vec![]).unwrap();
//         //     }
//         // }
//     }

//     fn flush(&self) {}
// }

fn init_js_bindings(cx: &mut ModuleContext) -> NeonResult<()> {
    // cx.export_function("hello", hello)?;

    Ok(())
}

pub fn init_context(cx: &mut ModuleContext) -> NeonResult<()> {
    // TODO: Implement logger
    init_js_bindings(cx)?;

    Ok(())
}
