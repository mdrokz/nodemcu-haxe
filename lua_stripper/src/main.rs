use std::collections::HashMap;

use rslua::{
    lexer::Lexer,
    lexer::LexerConfig,
    parser::Parser,
    tokens::{Token, TokenType, TokenValue},
};

use rslua::ast::*;
use rslua::ast_walker::*;
use rslua::types::*;

use std::fs::read_to_string;
use std::fs::write;

const LUA_CODE: &'static str = r#"
    function test()
    end

    function test1()
    end

    function test2()
    end

    function test3()
    end

    function test4()
    end

    function test()
    end

    test5()
    test5()
"#;

#[derive(Debug, Clone)]
struct Function {
    pub start: Token,
    pub name: Token,
    pub params: Vec<Token>,
    pub end: Token,
}

struct LuaWriter {
    output: String,
    indent: usize,
    depth: usize,
}

#[allow(dead_code)]
impl LuaWriter {
    pub fn new() -> Self {
        LuaWriter {
            output: String::new(),
            indent: 2,
            depth: 0,
        }
    }

    pub fn run(&mut self, block: &Block) -> &str {
        self.output.clear();
        ast_walker::walk_block(block, self).unwrap();
        &self.output
    }

    fn append(&mut self, content: &str) {
        self.output.push_str(content);
    }

    fn incline(&mut self) {
        self.output.push_str("\n");
        self.output.push_str(&" ".repeat(self.depth * self.indent));
    }

    fn space(&mut self) {
        self.output.push_str(" ");
    }

    fn append_space(&mut self, content: &str) {
        self.append(content);
        self.space();
    }

    fn space_append(&mut self, content: &str) {
        self.space();
        self.append(content);
    }

    fn space_append_space(&mut self, content: &str) {
        self.space();
        self.append(content);
        self.space();
    }

    fn append_inc(&mut self, content: &str) {
        self.append(content);
        self.incline();
    }

    fn end(&mut self) {
        self.leave_scope();
        self.append("end");
    }

    fn enter_scope(&mut self) {
        self.depth += 1;
    }

    fn leave_scope(&mut self) {
        self.depth -= 1;
        for _i in 0..self.indent {
            self.output.pop();
        }
    }
}

type WriteResult<T> = Result<T, ()>;
type WriteSuccess = WriteResult<()>;

impl AstVisitor for LuaWriter {
    fn stat_sep(&mut self) {
        self.incline();
    }

    fn begin_if(&mut self, _cond: &Expr) -> WriteResult<bool> {
        self.append_space("if");
        Ok(false)
    }

    fn then(&mut self, _block: &Block) -> WriteResult<bool> {
        self.space();
        self.enter_scope();
        self.append_inc("then");
        Ok(false)
    }

    fn begin_else_if(&mut self, _cond: &Expr) -> WriteResult<bool> {
        self.leave_scope();
        self.append_space("elseif");
        Ok(false)
    }

    fn begin_else(&mut self, _block: &Block) -> WriteResult<bool> {
        self.leave_scope();
        self.append("else");
        self.enter_scope();
        self.incline();
        Ok(false)
    }

    fn end_if(&mut self) {
        self.end();
    }

    fn begin_while(&mut self, _cond: &Expr) -> WriteResult<bool> {
        self.append_space("while");
        Ok(false)
    }

    fn begin_while_block(&mut self, _block: &Block) -> WriteResult<bool> {
        self.enter_scope();
        self.space();
        self.append_inc("do");
        Ok(false)
    }

    fn end_while(&mut self) {
        self.end();
    }

    fn begin_do_block(&mut self, _block: &Block) -> WriteResult<bool> {
        self.enter_scope();
        self.space();
        self.append_inc("do");
        Ok(false)
    }

    fn end_do_block(&mut self) {
        self.end();
    }

    fn for_num(&mut self, fornum: &ForNum) -> WriteResult<bool> {
        self.append_space("for");
        self.append(&format!("{} = ", fornum.var));
        Ok(false)
    }

    fn for_list(&mut self, forlist: &ForList) -> WriteResult<bool> {
        self.append_space("for");
        for (n, var) in forlist.vars.iter().enumerate() {
            self.append(var);
            if n < forlist.vars.len() - 1 {
                self.append(", ");
            }
        }
        self.space_append_space("in");
        Ok(false)
    }

    fn begin_for_block(&mut self, _block: &Block) -> WriteResult<bool> {
        self.enter_scope();
        self.space();
        self.append_inc("do");
        Ok(false)
    }

    fn end_for(&mut self) {
        self.end();
    }

    fn begin_repeat(&mut self, _block: &Block) -> WriteResult<bool> {
        self.enter_scope();
        self.append_inc("repeat");
        Ok(false)
    }

    fn until(&mut self) {
        self.leave_scope();
        self.incline();
        self.append_space("until");
    }

    fn end_repeat(&mut self) {
        self.incline();
    }

    fn func(&mut self, funcstat: &FuncStat) {
        match funcstat.func_type {
            FuncType::Local => self.append_space("local function"),
            FuncType::Global => self.append_space("function"),
        };
        let func_name = &funcstat.func_name;
        let mut fields = func_name.fields.iter();
        if let Some(name) = fields.next() {
            self.append(name);
            while let Some(name) = fields.next() {
                self.append(".");
                self.append(name);
            }
            if let Some(method) = &func_name.method {
                self.append(":");
                self.append(method);
            }
        }
    }

    fn local_stat(&mut self, stat: &LocalStat) -> WriteSuccess {
        self.append_space("local");
        for (n, name) in stat.names.iter().enumerate() {
            self.append(name);
            if n < stat.names.len() - 1 {
                self.append(", ");
            }
        }
        self.space();
        if stat.exprs.len() > 0 {
            self.append_space("=");
            ast_walker::walk_exprlist(&stat.exprs, self)?;
        }
        Ok(())
    }

    fn label_stat(&mut self, stat: &LabelStat) -> WriteSuccess {
        self.append(&format!("::{}::", stat.label));
        Ok(())
    }

    fn ret_stat(&mut self, stat: &RetStat) -> WriteSuccess {
        self.append_space("return");
        ast_walker::walk_exprlist(&stat.exprs, self)?;
        Ok(())
    }

    fn break_stat(&mut self, _stat: &BreakStat) -> WriteSuccess {
        self.append("break");
        Ok(())
    }

    fn goto_stat(&mut self, stat: &GotoStat) -> WriteSuccess {
        self.append(&format!("goto {}", stat.label));
        Ok(())
    }

    fn assign_stat(&mut self, stat: &AssignStat) -> WriteSuccess {
        for (n, suffix) in stat.left.iter().enumerate() {
            ast_walker::walk_assinable(suffix, self)?;
            if n < stat.left.len() - 1 {
                self.append_space(",");
            }
        }
        self.space_append_space("=");
        ast_walker::walk_exprlist(&stat.right, self)?;
        Ok(())
    }

    fn call_stat(&mut self, stat: &CallStat) -> WriteSuccess {
        ast_walker::walk_assinable(&stat.call, self)?;
        Ok(())
    }

    fn expr_sep(&mut self) {
        self.append(", ");
    }

    fn nil(&mut self) {
        self.append("nil");
    }

    fn true_(&mut self) {
        self.append("true");
    }

    fn false_(&mut self) {
        self.append("false");
    }

    fn float(&mut self, f: FloatType) {
        let string = if f.fract() == 0.0 {
            format!("{}.0", f)
        } else {
            f.to_string()
        };
        self.append(&string);
    }

    fn int(&mut self, i: IntType) {
        self.append(&i.to_string());
    }

    fn string(&mut self, s: &str) {
        self.append(s);
    }

    fn vararg(&mut self) {
        self.append("...");
    }

    fn anonymous_func(&mut self) {
        self.append_space("function");
    }

    fn begin_func_body(&mut self, body: &FuncBody) -> WriteResult<bool> {
        self.append("(");
        for (n, param) in body.params.iter().enumerate() {
            match param {
                Param::VarArg => self.append("..."),
                Param::Name(s) => self.append(s),
            }
            if n < body.params.len() - 1 {
                self.append(", ");
            }
        }
        self.enter_scope();
        self.append_inc(")");
        Ok(false)
    }

    fn end_func_body(&mut self) {
        self.end();
    }

    fn begin_table(&mut self, t: &Table) -> WriteResult<bool> {
        if t.fields.len() > 0 {
            self.enter_scope();
            self.append_inc("{");
        } else {
            self.append("{}");
        }
        Ok(false)
    }

    fn end_table(&mut self, t: &Table) {
        if t.fields.len() > 0 {
            self.leave_scope();
            self.append("}");
        }
    }

    fn field_sep(&mut self) {
        self.append_inc(",");
    }

    fn field_kv_sep(&mut self) {
        self.space_append_space("=");
    }

    fn begin_field_key(&mut self, key: &FieldKey) -> WriteResult<bool> {
        match key {
            FieldKey::Expr(_) => self.append_space("["),
            _ => (),
        }
        Ok(false)
    }

    fn end_field_key(&mut self, key: &FieldKey) {
        match key {
            FieldKey::Expr(_) => self.space_append("]"),
            _ => (),
        }
    }

    fn binop(&mut self, op: BinOp) {
        let string = match op {
            BinOp::Or => "or",
            BinOp::And => "and",
            BinOp::Eq => "==",
            BinOp::Ne => "~=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::BOr => "|",
            BinOp::BXor => "~",
            BinOp::BAnd => "&",
            BinOp::Shl => "<<",
            BinOp::Shr => ">>",
            BinOp::Concat => "..",
            BinOp::Add => "+",
            BinOp::Minus => "-",
            BinOp::Mul => "*",
            BinOp::Mod => "%",
            BinOp::Div => "/",
            BinOp::IDiv => "//",
            BinOp::Pow => "^",
            _ => unreachable!(),
        };
        self.space_append_space(string);
    }

    fn unop(&mut self, op: UnOp) {
        match op {
            UnOp::Minus => self.append("-"),
            UnOp::BNot => self.append("~"),
            UnOp::Not => self.append_space("not"),
            UnOp::Len => self.append("#"),
            _ => unreachable!(),
        }
    }

    fn name(&mut self, name: &str) {
        self.append(name);
    }

    fn attr(&mut self, attr: &str) {
        self.append(".");
        self.append(attr);
    }

    fn method(&mut self, method: &str) {
        self.append(":");
        self.append(method);
    }

    fn begin_index(&mut self, _expr: &Expr) -> WriteResult<bool> {
        self.append("[");
        Ok(false)
    }

    fn end_index(&mut self) {
        self.append("]");
    }

    fn begin_func_args(&mut self, _args: &FuncArgs) -> WriteResult<bool> {
        self.append("(");
        Ok(false)
    }

    fn end_func_args(&mut self) {
        self.append(")");
    }

    fn begin_paren_expr(&mut self, _expr: &Expr) -> WriteResult<bool> {
        self.append("(");
        Ok(false)
    }

    fn end_paren_expr(&mut self) {
        self.append(")");
    }

    fn comment(&mut self, comment: &CommentStat) {
        self.append(&format!("--{}", comment.comment));
    }
}

struct Stripper {
    tokens: Vec<Token>,
    parser: Parser,
}

impl Stripper {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer::new();
        let parser = Parser::new();

        Self {
            tokens: lexer.run(input).unwrap(),
            parser,
        }
    }

    fn strip_functions(&mut self) -> Block {
        let parser = &mut self.parser;

        let tokens = &self.tokens;
        let mut func_list: Vec<Function> = Vec::new();

        let mut tk: Vec<Token> = tokens.clone();
        let mut func_tk: Vec<Token> = Vec::new();

        let fname_list: Vec<&String> = tokens
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v.t == TokenType::Function {
                    if let TokenValue::Str(s) = &tokens[i + 1].value {
                        Some(s)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for (i, token) in tokens.iter().enumerate() {
            if token.t == TokenType::Function {
                for v in &tokens[i + 1..] {
                    if v.t == TokenType::End {
                        let f = Function {
                            start: token.clone(),
                            name: func_tk[0].clone(),
                            params: func_tk[1..].to_vec(),
                            end: v.clone(),
                        };
                        func_list.push(f);
                        func_tk.clear();
                        break;
                    } else {
                        func_tk.push(v.clone());
                    }
                }
            }
        }

        for (i, function) in fname_list.iter().enumerate() {
            let mut map = HashMap::<String, ()>::new();
            let name = function.clone();
            let usage: Vec<&Token> = tokens
                .iter()
                .filter(|v| {
                    if let TokenValue::Str(s) = &v.value {
                        if name == s && map.contains_key(s) {
                            true
                        } else {
                            map.insert(s.clone(), ());
                            false
                        }
                    } else {
                        false
                    }
                })
                .collect();
            if usage.len() == 0 {
                let ff = &func_list[i];
                tk = tk
                    .into_iter()
                    .filter_map(|t| {
                        if t == ff.start || t == ff.end || t == ff.name {
                            None
                        } else {
                            let v: Vec<&Token> =
                                ff.params.iter().filter(|ptk| &t == ptk.clone()).collect();
                            if v.len() > 0 {
                                None
                            } else {
                                Some(t)
                            }
                        }
                    })
                    .collect();
            }
        }

        parser.run(tk).unwrap()
    }
}

fn main() {
    let code = read_to_string("../out.lua").unwrap();

    let mut stripper = Stripper::new(&code);

    let block = stripper.strip_functions();

    let mut lua_writer= LuaWriter::new();

    let parsed_code = lua_writer.run(&block);

    write("./t.lua",parsed_code).unwrap();

    // println!("{:?}", LuaWriter::new().run(&block));
}
