use std::collections::BTreeMap;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, PointerValue},
};

use crate::parse::{AstNode, AstNodeValue, KeywordConfig};

use super::VariableMetaType;

pub struct CompilerHelper;
impl CompilerHelper {
    pub fn create_sting_variable<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        string: String,
        name: &str,
    ) -> PointerValue<'a> {
        let i8_types = context.i8_type();
        let string_size = context
            .i64_type()
            .const_int(string.len().try_into().unwrap(), false);
        let string_value = builder
            .build_array_malloc(i8_types, string_size.clone(), name)
            .unwrap();

        // builder.build_store(string_value, context.const_string(string.as_bytes(), false));
        let global_string_value =
            builder.build_global_string_ptr(&string.replace("\\n", "\n"), "str");
        builder
            .build_memcpy(
                string_value,
                4,
                global_string_value.as_pointer_value(),
                4,
                string_size.clone(),
            )
            .unwrap();

        return string_value;
    }
    pub fn create_number_variable<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        value: f64,
        name: &str,
    ) -> PointerValue<'a> {
        if value.fract() == 0.0 {
            Self::create_int_variable(context, builder, value, name)
        } else {
            Self::create_float_variable(context, builder, value, name)
        }
    }

    pub fn create_float_variable<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        value: f64,
        name: &str,
    ) -> PointerValue<'a> {
        let f64_types = context.f64_type();
        let string_value = builder.build_alloca(f64_types, name);
        builder.build_store(string_value, f64_types.const_float(value));

        return string_value;
    }
    pub fn create_int_variable<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        value: f64,
        name: &str,
    ) -> PointerValue<'a> {
        let i64_types = context.i64_type();
        let string_value = builder.build_alloca(i64_types, name);
        builder.build_store(string_value, i64_types.const_int(value as u64, false));

        return string_value;
    }
}

pub struct DefaultFunction;

static PRINT_FN_NAME: &str = "printf";

impl DefaultFunction {
    pub fn get_default_function<'a>(
        context: &'a Context,
        module: &Module<'a>,
    ) -> FunctionValue<'a> {
        match module.get_function(PRINT_FN_NAME) {
            Some(f) => f,
            None => {
                let i32_types = context.i32_type();
                let print_fn_type = i32_types.fn_type(
                    &[inkwell::types::BasicMetadataTypeEnum::PointerType(
                        context.i8_type().ptr_type(inkwell::AddressSpace::Generic),
                    )],
                    true,
                );
                module.add_function(
                    PRINT_FN_NAME,
                    print_fn_type,
                    Some(inkwell::module::Linkage::External),
                )
            }
        }
    }
}

pub struct ParseExpr;
impl ParseExpr {
    pub fn parse_call_function_syntax<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        module: &Module<'a>,
        variable: &BTreeMap<String, PointerValue<'a>>,
        variable_metadata: &BTreeMap<String, VariableMetaType>,
        function_name_dist: &BTreeMap<String, String>,
        line: usize,
        config: &KeywordConfig,
        command: &AstNode,
    ) -> inkwell::values::CallSiteValue<'a> {
        let function_name = command.left[0]
            .op
            .get_ident_value()
            .unwrap()
            .get_function_name()
            .unwrap();
        let function_args = command.right.clone();
        if let Some(a) = config.default_function.get(&function_name) {
            if a.r#type == "print" {
                return Self::parse_call_print_function_syntax(
                    context,
                    builder,
                    module,
                    variable,
                    variable_metadata,
                    line,
                    function_name,
                    function_args,
                );
            } else {
                todo!()
            }
        } else if let Some(name) = function_name_dist.get(&function_name) {
            if let Some(function) = module.get_function(name) {
                let mut call_args = vec![];
                let mut must_remove = vec![];
                for args in function_args {
                    let value = args
                        .op
                        .get_value_value()
                        .unwrap()
                        .get_function_args()
                        .unwrap();
                    match value {
                        crate::parse::AstNodeValue::String(s) => {
                            let ptr = CompilerHelper::create_sting_variable(
                                context,
                                builder,
                                s.clone(),
                                "tmp",
                            );
                            call_args
                                .push(inkwell::values::BasicMetadataValueEnum::PointerValue(ptr));
                            must_remove.push(ptr);
                        }
                        crate::parse::AstNodeValue::Number(num) => {
                            if num.fract() == 0.0 {
                                call_args.push(inkwell::values::BasicMetadataValueEnum::IntValue(
                                    context.i64_type().const_int(num as u64, false),
                                ))
                            } else {
                                call_args.push(
                                    inkwell::values::BasicMetadataValueEnum::FloatValue(
                                        context.f64_type().const_float(num),
                                    ),
                                );
                            }
                        }
                        crate::parse::AstNodeValue::None => todo!(),
                        crate::parse::AstNodeValue::Bool(b) => {
                            call_args.push(inkwell::values::BasicMetadataValueEnum::IntValue(
                                context.bool_type().const_int(b as u64, false),
                            ));
                        }
                        crate::parse::AstNodeValue::Variable(name) => {
                            if let Some(v) = variable.get(&name) {
                                let variable_type = variable_metadata.get(&name).unwrap();
                                match variable_type {
                                    VariableMetaType::String => {
                                        call_args.push(
                                            inkwell::values::BasicMetadataValueEnum::PointerValue(
                                                *v,
                                            ),
                                        );
                                    }
                                    VariableMetaType::Number => {
                                        call_args.push(builder.build_load(*v, "").into());
                                    }
                                }
                            } else {
                                panic!("code kiểu beep gì vậy đã ko phải là biến mặc định rồi mà còn dùng biến ko được define(khai báo đó nếu ko bik từ này thì nên học thêm từ undefined đi) lỗi tại dòng này nè {} lo đi mà sửa đi",line);
                            }
                        }
                    }
                }
                let result = builder.build_call(function, &call_args, "function_return");
                for ptr in must_remove {
                    builder.build_free(ptr);
                }
                return result;
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
    fn parse_call_print_function_syntax<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        module: &Module<'a>,
        variable: &BTreeMap<String, PointerValue<'a>>,
        variable_metadata: &BTreeMap<String, VariableMetaType>,
        line: usize,
        _fn_name: String,
        fn_args: Vec<AstNode>,
    ) -> inkwell::values::CallSiteValue<'a> {
        let print_fn = DefaultFunction::get_default_function(context, module);
        let mut print_value = "".to_string();
        let mut print_args = vec![];
        for args in fn_args {
            let value = args
                .op
                .get_value_value()
                .unwrap()
                .get_function_args()
                .unwrap();
            match value {
                crate::parse::AstNodeValue::String(s) => {
                    print_value.push_str(&s);
                }
                crate::parse::AstNodeValue::Number(num) => {
                    print_value.push_str(&num.to_string());
                }
                crate::parse::AstNodeValue::None => {
                    todo!()
                }
                crate::parse::AstNodeValue::Bool(b) => {
                    print_value.push_str(&b.to_string());
                }
                crate::parse::AstNodeValue::Variable(name) => {
                    if let Some(v) = variable.get(&name) {
                        let variable_type = variable_metadata.get(&name).unwrap();
                        match variable_type {
                            VariableMetaType::String => {
                                print_value.push_str("%s");
                                print_args.push(inkwell::values::BasicValueEnum::PointerValue(*v));
                            }
                            VariableMetaType::Number => {
                                if v.get_type().get_element_type().is_int_type() {
                                    print_value.push_str("%d");
                                } else if v.get_type().get_element_type().is_float_type() {
                                    print_value.push_str("%f");
                                } else {
                                    todo!()
                                }
                                print_args.push(builder.build_load(*v, ""));
                            }
                        }
                    } else {
                        panic!("code kiểu beep gì vậy đã ko phải là biến mặc định rồi mà còn dùng biến ko được define(khai báo đó nếu ko bik từ này thì nên học thêm từ undefined đi) lỗi tại dòng này nè {} lo đi mà sửa đi",line);
                    }
                }
            }
        }
        // print_value.push_str("\n");
        let string_variable = CompilerHelper::create_sting_variable(
            context,
            builder,
            print_value,
            "print_string_tmp",
        );
        let mut call_args = Vec::new();
        call_args.push(inkwell::values::BasicMetadataValueEnum::PointerValue(
            string_variable,
        ));
        for a in print_args {
            match a {
                inkwell::values::BasicValueEnum::ArrayValue(_) => todo!(),
                inkwell::values::BasicValueEnum::IntValue(v) => {
                    call_args.push(inkwell::values::BasicMetadataValueEnum::IntValue(v));
                }
                inkwell::values::BasicValueEnum::FloatValue(f) => {
                    call_args.push(inkwell::values::BasicMetadataValueEnum::FloatValue(f));
                }
                inkwell::values::BasicValueEnum::PointerValue(v) => {
                    call_args.push(inkwell::values::BasicMetadataValueEnum::PointerValue(v));
                }
                inkwell::values::BasicValueEnum::StructValue(_) => todo!(),
                inkwell::values::BasicValueEnum::VectorValue(_) => todo!(),
            }
        }
        let result = builder.build_call(print_fn, &call_args, "call_printf_fn");
        builder.build_free(string_variable);
        result
    }
    pub fn parse_new_variable_syntax<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        _module: &Module<'a>,
        variable: &mut BTreeMap<String, PointerValue<'a>>,
        variable_metadata: &mut BTreeMap<String, VariableMetaType>,
        _line: usize,
        _config: &KeywordConfig,
        command: &AstNode,
    ) {
        let variable_name = command.left[0]
            .op
            .get_ident_value()
            .unwrap()
            .get_variable_name()
            .unwrap();
        let variable_value = if command.right.len() > 0 {
            Some(
                command.right[0]
                    .op
                    .get_value_value()
                    .unwrap()
                    .get_variable_value()
                    .unwrap(),
            )
        } else {
            None
        };

        match variable_value {
            Some(v) => match v {
                crate::parse::AstNodeValue::String(s) => {
                    let ptr =
                        CompilerHelper::create_sting_variable(context, &builder, s, &variable_name);
                    variable.insert(variable_name.clone(), ptr);
                    variable_metadata.insert(variable_name.clone(), VariableMetaType::String);
                }
                crate::parse::AstNodeValue::Number(n) => {
                    let ptr = CompilerHelper::create_number_variable(
                        context,
                        &builder,
                        n,
                        &variable_name,
                    );
                    variable.insert(variable_name.clone(), ptr);
                    variable_metadata.insert(variable_name.clone(), VariableMetaType::Number);
                }
                crate::parse::AstNodeValue::None => todo!(),
                crate::parse::AstNodeValue::Bool(_) => todo!(),
                crate::parse::AstNodeValue::Variable(_) => todo!(),
            },
            None => todo!(),
        }
    }
    pub fn parse_set_variable_syntax<'a>(
        context: &'a Context,
        builder: &Builder<'a>,
        _module: &Module<'a>,
        variable: &mut BTreeMap<String, PointerValue<'a>>,
        variable_metadata: &mut BTreeMap<String, VariableMetaType>,
        line: usize,
        _config: &KeywordConfig,
        command: &AstNode,
    ) {
        let variable_name = command.left[0]
            .op
            .get_ident_value()
            .unwrap()
            .get_variable_name()
            .unwrap();
        let variable_value = command.right[0]
            .op
            .get_value_value()
            .unwrap()
            .get_variable_value()
            .unwrap();

        let ptr = variable
            .get(&variable_name)
            .expect(&("variable is not defined ".to_string() + &line.to_string()));
        let variable_type = variable_metadata
            .get(&variable_name)
            .expect(&("variable is not true type ".to_string() + &line.to_string()));
        match variable_type {
            VariableMetaType::String => {
                if let AstNodeValue::String(s) = variable_value {
                    builder.build_free(*ptr);
                    let ptr =
                        CompilerHelper::create_sting_variable(context, builder, s, &variable_name);
                    variable.remove(&variable_name);
                    variable.insert(variable_name.clone(), ptr);
                } else {
                    panic!(
                        "{}",
                        &("variable type is not valid type ".to_string() + &line.to_string())
                    )
                }
            }
            VariableMetaType::Number => {
                if let AstNodeValue::Number(num) = variable_value {
                    if ptr.get_type().get_element_type().is_float_type() {
                        builder.build_store(*ptr, context.f64_type().const_float(num));
                    } else if ptr.get_type().get_element_type().is_int_type()
                        && (num.fract() == 0.0)
                    {
                        // int
                        builder.build_store(*ptr, context.i64_type().const_int(num as u64, false));
                    } else if ptr.get_type().get_element_type().is_int_type()
                        && !(num.fract() == 0.0)
                    {
                        // create new variable
                        let ptr = CompilerHelper::create_number_variable(
                            context,
                            builder,
                            num,
                            &variable_name,
                        );
                        variable.remove(&variable_name);
                        variable.insert(variable_name.clone(), ptr);
                    }
                } else {
                    panic!(
                        "{}",
                        &("variable type is not valid type ".to_string() + &line.to_string())
                    )
                }
            }
        }
    }
}
