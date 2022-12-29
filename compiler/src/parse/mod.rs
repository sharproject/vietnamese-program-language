use std::collections::BTreeMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default)]
pub struct KeywordConfig {
    pub default_function: BTreeMap<String, DefaultFunctionType>,
    pub variable_keyword: Vec<String>,
}

#[derive(Debug)]
pub struct DefaultFunctionType {
    pub name: String,
    pub r#type: String,
}

lazy_static! {
    static ref CALL_FUNCTION_REGEX: Regex = Regex::new(r"(\w+)\s?(\((.+)?\))").unwrap();
    static ref SPECIAL_CALL_FUNCTION_REGEX: Regex = Regex::new(r"(\w+)\s?:\s?(.+)?").unwrap();
    static ref SET_VARIABLE_REGEX: Regex = Regex::new(r"(\w+)\s?=\s?(.+)").unwrap();
    static ref OPS_LIST: Vec<String> = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
    ];
}

pub fn parse_config(config: String) -> KeywordConfig {
    let mut result = KeywordConfig::default();
    for s in config.split("\n") {
        if s.trim().is_empty() {
            continue;
        }
        // sample value is in=print
        let tmp = s.split("=").collect::<Vec<&str>>();
        result.default_function.insert(
            tmp[0].to_string(),
            DefaultFunctionType {
                name: tmp[0].to_string(),
                r#type: tmp[1].to_string(),
            },
        );
        if tmp[1] == "variable" {
            result.variable_keyword.push(tmp[0].to_string());
        }
    }
    return result;
}
pub type Ast = Vec<AstNode>;

#[derive(Debug, Default, Clone)]
pub struct AstNode {
    pub op: Operation,
    pub left: Ast,
    pub right: Ast,
    pub raw: String,
}

#[derive(Debug, Default, Clone)]
pub enum AstNodeValue {
    String(String),
    Number(f64),
    #[default]
    None,
    Bool(bool),
    Variable(String),
    Operation(Box<AstNode>),
}

#[derive(Debug, Default, Clone)]
pub enum Operation {
    #[default]
    None,
    Ident(IdentType),
    Value(ValueMetaType),
    Call,
    NewVariable,
    SetVariable,
    IntOperation(IntOperationType),
}

#[derive(Debug, Default, Clone)]
pub enum IntOperationType {
    Plus,
    Minus,
    Times,
    Divide,
    #[default]
    None,
}

impl IntOperationType {
    pub fn from_string_symbol(data: &str) -> Self {
        match data.trim() {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Times,
            "/" => Self::Divide,
            _ => {
                panic!("{} cái phép tính beep gì đây", data)
            }
        }
    }
    pub fn to_symbol_string(&self) -> String {
        match self {
            IntOperationType::Plus => "+",
            IntOperationType::Minus => "-",
            IntOperationType::Times => "*",
            IntOperationType::Divide => "/",
            _ => {
                panic!("cái phép tính beep gì dậy trời")
            }
        }
        .to_string()
    }
}

impl Operation {
    pub fn get_ident_value(&self) -> Option<IdentType> {
        return if let Operation::Ident(v) = self {
            Some(v.clone())
        } else {
            None
        };
    }
    pub fn get_value_value(&self) -> Option<ValueMetaType> {
        match self {
            Operation::Value(v) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn get_int_operation(&self) -> Option<IntOperationType> {
        match self {
            Self::IntOperation(v) => Some(v.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ValueMetaType {
    #[default]
    None,
    FunctionArg(AstNodeValue),
    VariableValue(AstNodeValue),
    MathValue(AstNodeValue),
}

impl ValueMetaType {
    pub fn get_function_args(&self) -> Option<AstNodeValue> {
        return if let ValueMetaType::FunctionArg(fa) = self {
            Some(fa.clone())
        } else {
            None
        };
    }
    pub fn get_variable_value(&self) -> Option<AstNodeValue> {
        return if let ValueMetaType::VariableValue(var_val) = self {
            Some(var_val.clone())
        } else {
            None
        };
    }
    pub fn get_math_value(&self) -> Option<AstNodeValue> {
        return if let ValueMetaType::MathValue(var_val) = self {
            Some(var_val.clone())
        } else {
            None
        };
    }
}

#[derive(Debug, Clone)]
pub struct IdentType {
    pub data: String,
    pub metadata: IdentMetaDataType,
}

impl IdentType {
    pub fn get_function_name(&self) -> Option<String> {
        match self.metadata {
            IdentMetaDataType::FunctionName => Some(self.data.clone()),
            _ => None,
        }
    }
    pub fn get_variable_name(&self) -> Option<String> {
        match self.metadata {
            IdentMetaDataType::VariableName => Some(self.data.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IdentMetaDataType {
    FunctionName,
    VariableName,
}

#[derive(Debug, Default)]
pub struct ContextType {
    pub variable: std::collections::BTreeMap<String, AstNodeValue>,
}

pub fn parse_string_to_ast(data: String, config: &KeywordConfig) -> Ast {
    let mut result = Ast::default();
    let mut context = ContextType::default();
    for line in data.split("\n") {
        if line.starts_with("#") {
            continue;
        }
        if line.trim().len() <= 0 {
            continue;
        }
        result.push(parse_string(line.trim(), config, &mut context))
    }
    return result;
}

fn parse_string(data: &str, config: &KeywordConfig, context: &mut ContextType) -> AstNode {
    let mut result = AstNode::default();

    result.raw = data.to_string();
    let new_variable_long_style_regex = regex::Regex::new(&format!(
        r"({})\s(\w+)(\s+)?(=)?(.+)?",
        config.variable_keyword.join("|")
    ))
    .unwrap();

    if let Some(p) = CALL_FUNCTION_REGEX.captures(data) {
        result = parse_call_function_syntax(
            p.get(1).unwrap().as_str(),
            p.get(2).unwrap().as_str(),
            context,
        );
    } else if let Some(p) = SPECIAL_CALL_FUNCTION_REGEX.captures(data) {
        result = parse_call_function_syntax(
            p.get(1).unwrap().as_str(),
            p.get(2).unwrap().as_str(),
            context,
        );
    } else if let Some(p) = new_variable_long_style_regex.captures(data) {
        result.op = Operation::NewVariable;
        result
            .left
            .push(parse_variable_name(p.get(2).unwrap().as_str()));

        if let Some(data) = p.get(5) {
            result
                .right
                .push(parse_variable_value(data.as_str(), context));
        }
        context.variable.insert(
            p.get(2).unwrap().as_str().to_string(),
            if result.right.len() == 0 {
                AstNodeValue::default()
            } else {
                result.right[0]
                    .op
                    .get_value_value()
                    .unwrap()
                    .get_variable_value()
                    .unwrap()
            },
        );
    } else if let Some(p) = SET_VARIABLE_REGEX.captures(data) {
        result.op = Operation::SetVariable;
        result
            .left
            .push(parse_variable_name(p.get(1).unwrap().as_str()));
        result
            .right
            .push(parse_variable_value(p.get(2).unwrap().as_str(), context))
    }
    result.raw = data.to_string();
    return result;
}

fn parse_variable_value(data: &str, context: &mut ContextType) -> AstNode {
    let mut result = AstNode::default();
    result.op = Operation::Value(ValueMetaType::VariableValue(AstNodeValue::from_string(
        data,
        &context.variable,
    )));
    result.raw = data.to_string();
    return result;
}

fn parse_call_function_syntax(name: &str, data: &str, context: &mut ContextType) -> AstNode {
    let mut result = AstNode::default();
    result.op = Operation::Call;
    result.left.push(parse_function_name(name));
    for v in parse_function_args(data, context) {
        result.right.push(v);
    }
    result
}

fn parse_function_name(name: &str) -> AstNode {
    let mut node = AstNode::default();
    node.op = Operation::Ident(IdentType {
        data: name.to_string(),
        metadata: IdentMetaDataType::FunctionName,
    });
    node.raw = name.to_string();
    return node;
}
fn parse_variable_name(name: &str) -> AstNode {
    let mut node = AstNode::default();
    node.op = Operation::Ident(IdentType {
        data: name.to_string(),
        metadata: IdentMetaDataType::VariableName,
    });
    node.raw = name.to_string();
    return node;
}

fn parse_function_args(raw_args: &str, context: &mut ContextType) -> Ast {
    let mut ast = Ast::default();
    if raw_args.starts_with("(")
        && raw_args.ends_with(")")
        && !check_string_is_math_operation(raw_args)
    {
        let content = remove_first_and_last(raw_args);
        for raw in content.split(",") {
            let mut node = AstNode::default();
            node.op = Operation::Value(ValueMetaType::FunctionArg(AstNodeValue::from_string(
                raw,
                &context.variable,
            )));
            node.raw = raw.to_string();
            ast.push(node.clone());
        }
    } else {
        for raw in raw_args.split(",") {
            let mut node = AstNode::default();
            node.op = Operation::Value(ValueMetaType::FunctionArg(AstNodeValue::from_string(
                raw,
                &context.variable,
            )));
            node.raw = raw.to_string();
            ast.push(node.clone());
        }
    }
    return ast;
}
fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
impl AstNodeValue {
    pub fn from_string(
        data: &str,
        variable_list: &std::collections::BTreeMap<String, AstNodeValue>,
    ) -> Self {
        let use_data = data.trim();
        if use_data.starts_with("\"") && use_data.ends_with("\"") {
            return Self::String(remove_first_and_last(use_data.trim()).to_string());
        } else if use_data.starts_with("(")
            && use_data.ends_with(")")
            && !check_string_is_math_operation(use_data)
        {
            return Self::from_string(remove_first_and_last(use_data.trim()).trim(), variable_list);
        } else if use_data.parse::<f64>().is_ok() {
            return Self::Number(use_data.parse::<f64>().unwrap());
        } else if use_data == "true" {
            return Self::Bool(true);
        } else if use_data == "false" {
            return Self::Bool(false);
        } else if use_data.len() > 0 && variable_list.contains_key(&use_data.to_string()) {
            return Self::Variable(use_data.to_string());
        } else if check_string_is_math_operation(use_data) {
            let node = parse_math_operation(use_data, false, variable_list);
            return Self::Operation(Box::new(node));
        } else {
            return Self::default();
        }
    }
}
fn parse_math_operation(
    data: &str,
    by_pass_math_check: bool,
    variable_list: &std::collections::BTreeMap<String, AstNodeValue>,
) -> AstNode {
    let mut result = AstNode::default();
    result.raw = data.to_string();
    if !check_string_is_math_operation(data) {
        if data.starts_with("\"") && data.ends_with("\"") {
            panic!("cant calculator with string");
        } else if (data == "true") | (data == "false") {
            panic!("cant calculator with bool");
        } else {
            result.op = Operation::Value(ValueMetaType::MathValue(AstNodeValue::from_string(
                data,
                variable_list,
            )))
        }
    } else if data.starts_with("(") && data.ends_with(")") && by_pass_math_check {
        return parse_math_operation(
            remove_first_and_last(data.trim()).trim(),
            false,
            variable_list,
        );
    } else {
        let mut bra = 0;
        let mut i = 0;
        let mut tmp = "".to_string();
        let mut char_list = data
            .to_string()
            .replace(" ", "")
            .chars()
            .collect::<Vec<_>>()
            .into_iter();
        let mut op = vec![];
        let mut value = vec![];
        while let Some(mut c) = char_list.next() {
            i = i + 1;
            tmp.push(c);
            if c == '(' {
                bra = bra + 1;
            } else if c == ')' {
                bra = bra - 1
            }

            if bra == 0 && i != 0 {
                let mut pushable = true;
                if c.is_numeric() {
                    while let Some(ch) = char_list.next() {
                        if ch.is_numeric() {
                            tmp.push(ch);
                        } else {
                            // let mut new_char_list = vec![];
                            // new_char_list.push(c);
                            // new_char_list.extend_from_slice(char_list.as_slice());
                            // char_list.clone_from(&new_char_list.into_iter());
                            c = ch;
                            value.push(tmp.clone());
                            pushable = false;
                            break;
                        }
                    }
                }

                for o in OPS_LIST.clone() {
                    if c.to_string() == o {
                        op.push(IntOperationType::from_string_symbol(&c.to_string()));
                        pushable = false;
                        tmp = "".to_string();
                    }
                }

                if pushable {
                    value.push(tmp.clone());
                    tmp = "".to_string();
                }
            }
        }
        {
            let index = 0;
            let left = parse_math_operation(&value.remove(index), true, variable_list);
            result.op = Operation::IntOperation(op.remove(0));
            result.left.push(left);
            let mut right_op = "".to_string();
            for i in 0..value.len() {
                if value[i].parse::<f64>().is_ok() {
                    if i > 0 {
                        right_op.push_str(&op[i - 1].to_symbol_string());
                    }
                    right_op.push_str(&value[i]);
                } else {
                    right_op.push_str(&value[i]);
                    if i > 0 {
                        right_op.push_str(&op[i - 1].to_symbol_string());
                    }
                }
            }
            let right = parse_math_operation(&right_op, true, variable_list);
            result.right.push(right);
        }
    }
    result
}

fn check_string_is_math_operation(data: &str) -> bool {
    let mut result = false;
    let mut setted = false;
    for o in OPS_LIST.clone() {
        if data.contains(&o) && !setted {
            setted = true;
            result = data.contains(&o);
        }
    }
    return result;
}
