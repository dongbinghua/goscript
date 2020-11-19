extern crate goscript_codegen as cg;
extern crate goscript_parser as fe;
extern crate goscript_types as types;
extern crate goscript_vm as vm;

pub struct Config {
    // working directory
    pub work_dir: Option<String>,
    // base path for non-local imports
    pub base_path: Option<String>,
    // print debug info in parser
    pub trace_parser: bool,
    // print debug info in checker
    pub trace_checker: bool,
    // proint debug info for vm
    pub trace_vm: bool,
}

pub struct Engine {
    config: Config,
    ffi: vm::ffi::FfiFactory,
}

impl Engine {
    pub fn new(config: Config) -> Engine {
        Engine {
            config: config,
            ffi: vm::ffi::FfiFactory::new(),
        }
    }

    pub fn run(&self, path: &str) -> usize {
        let config = types::Config {
            work_dir: self.config.work_dir.clone(),
            base_path: self.config.base_path.clone(),
            trace_parser: self.config.trace_parser,
            trace_checker: self.config.trace_checker,
        };
        let fs = &mut fe::FileSet::new();
        let el = &mut fe::errors::ErrorList::new();
        let code = cg::entry::parse_check_gen(path, &config, fs, el);
        if let Ok(bc) = code {
            let mut vm = vm::vm::GosVM::new(bc);
            vm.run(&self.ffi);
            0
        } else {
            if self.config.trace_vm {
                el.sort();
                print!("{}", el);
            }
            code.unwrap_err()
        }
    }

    pub fn register_extension(&mut self, name: &'static str, ctor: Box<vm::ffi::Ctor>) {
        self.ffi.register(name, ctor);
    }
}
