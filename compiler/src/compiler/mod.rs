mod helper;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    values::PointerValue,
    OptimizationLevel,
};

use crate::parse::{Ast, AstNode, KeywordConfig};

use self::helper::ParseExpr;

pub enum VariableMetaType {
    String,
    Number,
}

pub struct Compiler<'a> {
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    ast: &'a Ast,
    config: &'a KeywordConfig,
    variable: Arc<Mutex<BTreeMap<String, PointerValue<'a>>>>,
    variable_metadata: Arc<Mutex<BTreeMap<String, VariableMetaType>>>,
    function_name_dist: Arc<Mutex<BTreeMap<String, String>>>,
    line: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(
        context: &'a Context,
        module: Module<'a>,
        builder: Builder<'a>,
        ast: &'a Ast,
        config: &'a KeywordConfig,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            ast,
            config,
            variable: Default::default(),
            line: 0,
            variable_metadata: Default::default(),
            function_name_dist: Default::default(),
        }
    }
    pub fn parse_ast_to_module(&mut self) {
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function(
            "main",
            main_fn_type,
            Some(inkwell::module::Linkage::External),
        );
        let basic_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(basic_block);

        for c in self.ast {
            self.line = self.line + 1;
            self.parse_command(c, &self.builder);
        }

        let i32_zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&i32_zero));
    }

    fn parse_command(&self, command: &AstNode, builder: &Builder<'a>) {
        match command.op {
            crate::parse::Operation::None => todo!(),
            crate::parse::Operation::Ident(_) => todo!(),
            crate::parse::Operation::Value(_) => todo!(),
            crate::parse::Operation::Call => {
                ParseExpr::parse_call_function_syntax(
                    self.context,
                    &builder,
                    &self.module,
                    &Arc::clone(&self.variable).lock().unwrap(),
                    &Arc::clone(&self.variable_metadata).lock().unwrap(),
                    &self.function_name_dist.lock().unwrap(),
                    self.line,
                    self.config,
                    command,
                );
            }
            crate::parse::Operation::NewVariable => ParseExpr::parse_new_variable_syntax(
                self.context,
                &builder,
                &self.module,
                &mut Arc::clone(&self.variable).lock().unwrap(),
                &mut Arc::clone(&self.variable_metadata).lock().unwrap(),
                self.line,
                self.config,
                command,
            ),
            crate::parse::Operation::SetVariable => ParseExpr::parse_set_variable_syntax(
                self.context,
                &builder,
                &self.module,
                &mut Arc::clone(&self.variable).lock().unwrap(),
                &mut Arc::clone(&self.variable_metadata).lock().unwrap(),
                self.line,
                self.config,
                command,
            ),
        }
    }

    pub fn build_to_file(&self, path: &str, llvm_ir_code_path: Option<&str>) -> Result<(), String> {
        pre_save_file(path);
        if let Some(path) = llvm_ir_code_path {
            pre_save_file(path);
            self.module.print_to_file(path).unwrap();
        }
        Target::initialize_all(&InitializationConfig::default());
        // use the host machine as the compilation target
        self.module.verify().unwrap();
        let target_triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        // make a target from the triple
        let target = Target::from_triple(&target_triple).map_err(|e| format!("{:?}", e))?;

        // make a machine from the target
        let target_machine = target
            .create_target_machine(
                &target_triple,
                &cpu,
                &features,
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| "Unable to create target machine!".to_string())?;

        target_machine
            .write_to_file(&self.module, FileType::Object, path.as_ref())
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}

fn pre_save_file(path: &str) {
    let path = PathBuf::from_str(path).unwrap();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
}
