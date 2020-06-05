use rusty_v8 as v8;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};
use std::convert::TryFrom;

fn get_source_file(file: &str) -> Result<String, Error> {
  let mut file = File::open(file)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn execute_js(code: &str) {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  
  let mut isolate = v8::Isolate::new(Default::default());
  
  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();
  
  let context = v8::Context::new(scope);
  let mut context_scope = v8::ContextScope::new(scope, context);
  let scope = context_scope.enter();
  
  let code = v8::String::new(scope, code).unwrap();
  println!("javascript code: {}", code.to_rust_string_lossy(scope));
  
  let mut script = v8::Script::compile(scope, context, code, None).unwrap();
  let result = script.run(scope, context).unwrap();
  let result = result.to_string(scope).unwrap();
  println!("result: {}", result.to_rust_string_lossy(scope));
}

pub fn module_origin<'a>(
  s: &mut impl v8::ToLocal<'a>,
  resource_name: v8::Local<'a, v8::String>,
) -> v8::ScriptOrigin<'a> {
  let resource_line_offset = v8::Integer::new(s, 0);
  let resource_column_offset = v8::Integer::new(s, 0);
  let resource_is_shared_cross_origin = v8::Boolean::new(s, false);
  let script_id = v8::Integer::new(s, 123);
  let source_map_url = v8::String::new(s, "").unwrap();
  let resource_is_opaque = v8::Boolean::new(s, true);
  let is_wasm = v8::Boolean::new(s, false);
  let is_module = v8::Boolean::new(s, true);
  v8::ScriptOrigin::new(
    resource_name.into(),
    resource_line_offset,
    resource_column_offset,
    resource_is_shared_cross_origin,
    script_id,
    source_map_url.into(),
    resource_is_opaque,
    is_wasm,
    is_module,
  )
}

fn execute_module(name: &str) {
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  
  let mut isolate = v8::Isolate::new(Default::default());

  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();

  let context = v8::Context::new(scope);
  let mut context_scope = v8::ContextScope::new(scope, context);
  let scope = context_scope.enter();

  let code = v8::String::new(scope, "import 'some thing'; 1 + 1").unwrap();
  let origin = module_origin(scope, resource_name: v8::Local<'a, v8::String>)
}

fn main() {
  match get_source_file("test.js") {
    Ok(source) => execute_js(&source),
    Err(err) =>  eprintln!("error: {}", err),
  };
}
