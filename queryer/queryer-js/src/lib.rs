use neon::prelude::*;

fn example_sql(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(queryer::example_sql()))
}

fn query(mut cx: FunctionContext) -> JsResult<JsString> {
    let sql = cx.argument::<JsString>(0)?;
    let output = cx.argument_opt(1);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(async { queryer::query(sql.value(&mut cx)).await.unwrap() });
    if let Some(v) = output {
        let csv: Handle<JsString> = v.downcast(&mut cx).unwrap();
        let csv = csv.value(&mut cx);
        if csv.eq("csv") {
            return Ok(cx.string(data.to_csv().unwrap()));
        }
        return cx.throw_error(format!(
            "Output type {} not supported",
            csv
        ));
    } else {
        return Ok(cx.string(data.to_csv().unwrap()));
    }

}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("example_sql", example_sql)?;
    cx.export_function("query", query)?;
    Ok(())
}
