use neon::prelude::*;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

fn parse_sql_to_json_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let binding = cx.argument::<JsString>(0)?.value(&mut cx);
    let sql = binding.as_str();

    let dialect = MySqlDialect {};
    let parse_result = Parser::parse_sql(&dialect, sql);

    match parse_result {
        Ok(statements) => {
            let serialized = serde_json::to_string_pretty(&statements).unwrap();
            Ok(cx.string(serialized))
        }
        Err(e) => {
            Ok(cx.string(format!("{:?}", e)))
        }
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse_sql_to_json_string", parse_sql_to_json_string)?;
    Ok(())
}
