use std::thread::JoinHandle;

pub enum Message {
    Run(String),
    Error(String),
    Output(String),
    Exit
}

pub struct JsRunnerComm {
    tx: std::sync::mpsc::Sender<Message>,
    handle: JoinHandle<()>,
    results: std::sync::mpsc::Receiver<Message>,
}

impl JsRunnerComm {
    pub fn enqueue_script(&self, code: &str) {
        self.tx.send(Message::Run(code.to_owned())).unwrap();
    }

    pub fn peek_result(&mut self) -> Option<String> {
        match self.results.try_recv() {
            Ok(Message::Output(s)) => Some(s),
            _ => None
        }
    }

    pub fn stop(&mut self) {
        self.tx.send(Message::Exit).unwrap();
    }
}

impl Drop for JsRunnerComm {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Compiles and runs a javascript code, returning the result converted to String
/// Returns None if code compiles with errors or in case of runtime errors
#[inline]
pub fn run_js(scope: &mut v8::ContextScope<v8::HandleScope>, code: &str) -> Option<String> {
    let code = v8::String::new(scope, &code)?;
    let script = v8::Script::compile(scope, code, None)?;
    let result = script.run(scope)?;
    let result = result.to_string(scope)?;
    Some(result.to_rust_string_lossy(scope))
}

pub fn spawn() -> JsRunnerComm {
    let mut exit = false;
    let (tx, rx) = std::sync::mpsc::channel();
    let (results_tx, results_rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move|| {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);
        // Create a new context.
        let context = v8::Context::new(handle_scope);
        let mut scope = &mut v8::ContextScope::new(handle_scope, context);
        while !exit {
            match rx.recv() {
                Ok(msg) => {
                    match msg {
                        Message::Run(code) => {
                            if let Some(s) = run_js(&mut scope, &code) {
                                results_tx.send(Message::Output(s)).unwrap();
                            } else {
                                // The error is printed to the stdout by v8,
                                results_tx.send(Message::Error("".to_owned())).unwrap();
                            }
                        }
                        Message::Exit => exit = true,
                        _ => {}
                    }
                }
                Err(_err) => {}
            }
          }
    });
  
    JsRunnerComm { tx, handle, results: results_rx }    
}