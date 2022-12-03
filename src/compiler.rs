use deno_ast::swc::ast::{ModuleItem, Program, Script, Stmt};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CompilerError {
    message: String,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred while compiling")
    }
}

impl Error for CompilerError {}

pub struct Compiler<'a: 'ctx, 'ctx> {
    context: &'a Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
    main_fn: Option<FunctionValue<'ctx>>,
}

impl<'a: 'ctx, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            module: context.create_module("main"),
            builder: context.create_builder(),
            variables: HashMap::new(),
            main_fn: None,
        }
    }

    pub fn compile_main_function(&mut self) -> Result<(), CompilerError> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", fn_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);
        self.main_fn = Some(main_fn);

        Ok(())
    }

    pub fn compile(&mut self, program: &Program) -> Result<(), CompilerError> {
        if self.main_fn.is_none() {
            return Err(CompilerError {
                message: "No main function found".to_string(),
            });
        }

        match program {
            Program::Module(module) => self.compile_module(module)?,
            Program::Script(script) => self.compile_script(script)?,
        }

        Ok(())
    }

    fn compile_script(&mut self, script: &Script) -> Result<(), CompilerError> {
        (&script.body)
            .into_iter()
            .try_for_each(|stmt| self.compile_statement(stmt))
    }

    fn compile_module(&mut self, module: &deno_ast::swc::ast::Module) -> Result<(), CompilerError> {
        (&module.body)
            .into_iter()
            .try_for_each(|item| self.compile_module_item(item))
    }

    fn compile_module_item(&mut self, item: &ModuleItem) -> Result<(), CompilerError> {
        match item {
            ModuleItem::ModuleDecl(_) => {
                return Err(CompilerError {
                    message: "Module declarations are not supported".to_string(),
                });
            }
            ModuleItem::Stmt(stmt) => self.compile_statement(stmt),
        }
    }

    fn compile_statement(&self, statement: &Stmt) -> Result<(), CompilerError> {
        Ok(())
    }

    #[inline]
    fn main_fn(&self) -> FunctionValue<'ctx> {
        self.main_fn.unwrap()
    }
}
