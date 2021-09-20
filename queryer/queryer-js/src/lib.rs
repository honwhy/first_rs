use neon::prelude::*;

fn example_sql(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(queryer::example_sql()))
}

fn query(mut cx: FunctionContext) -> JsResult<JsString> {
    let sql = cx.argument::<JsString>(0)?;
    let sql = sql.value(&mut cx);
    let output = cx.argument_opt(1);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(async { queryer::query(sql).await.unwrap() });

    // match output {
    //     None => Ok(cx.string(data.to_csv().unwrap())),
    //     Some(ref v) => {
    //         let v = v.to_string(&mut cx);
    //         match v {
    //             todo!(),
    //         }
    //     }
    // }
    Ok(cx.string("not yet"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("example_sql", example_sql)?;
    cx.export_function("query", query)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
