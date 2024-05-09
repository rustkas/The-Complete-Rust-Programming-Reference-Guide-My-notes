use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn count_words(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let text = cx.argument::<JsString>(0)?.value(&mut cx);
    let word = cx.argument::<JsString>(1)?.value(&mut cx);
    Ok(cx.number(text.split(" ").filter(|s| s == &word).count() as f64))
}

fn make_an_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    // Create some values:
    let n = cx.number(9000);
    let s = cx.string("hello");
    let b = cx.boolean(true);

    // Create a new array:
    let array = cx.empty_array();

    // Push the values into the array:
    array.set(&mut cx, 0, n)?;
    array.set(&mut cx, 1, s)?;
    array.set(&mut cx, 2, b)?;

    // Return the array:
    Ok(array)
}
#[allow(unused_imports)]
use num_cpus::get;
fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("count_words", count_words)?;
    cx.export_function("make_an_array", make_an_array)?;
    cx.export_function("get", get_num_cpus)?;
    Ok(())
}
