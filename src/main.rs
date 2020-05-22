use rusty_v8 as v8;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};



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

fn main() {
  match get_source_file("test.js") {
    Ok(source) => execute_js(&source),
    Err(err) =>  eprintln!("error: {}", err),
  };
  
  //let source = get_source_file("test.js");

  //println!("code: {}", source);
  
  /*
  let platform = v8::new_default_platform().unwrap();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
  
  let mut isolate = v8::Isolate::new(Default::default());
  
  let mut handle_scope = v8::HandleScope::new(&mut isolate);
  let scope = handle_scope.enter();
  
  let context = v8::Context::new(scope);
  let mut context_scope = v8::ContextScope::new(scope, context);
  let scope = context_scope.enter();
  
  let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
  println!("javascript code: {}", code.to_rust_string_lossy(scope));
  
  let mut script = v8::Script::compile(scope, context, code, None).unwrap();
  let result = script.run(scope, context).unwrap();
  let result = result.to_string(scope).unwrap();
  println!("result: {}", result.to_rust_string_lossy(scope));
  */
}
