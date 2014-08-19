struct FunctionDefinition<'a> {
    name: &'a str,
    arguments: &'a [uint]
}

struct Scope {
    childscopes: Vec<Scope>
}

struct TAC {
    left: VariableType<String, int>,
    right: TACRight
}

enum VariableType<String, int> {
    RealVar(String),
    TempVar(String),
    NumberConst(int)
}

enum ThreeAddressCode {
    SimpleInstr(VariableType<String, int>, TACRight),//Dest, (equals) (expr)
    CallInstr(String),
    Label(String),//name of label. When using goto label or call label, it goes to the instruction
    //following the label
    NopInstr
}

enum TACRight {
    Addition(VariableType<String, int>, VariableType<String, int>),
    Subtraction(VariableType<String, int>, VariableType<String, int>),
    Multiplication(VariableType<String, int>, VariableType<String, int>),
    Division(VariableType<String, int>, VariableType<String, int>),
    Assignment(VariableType<String, int>),
    Write(VariableType<String, int>)
}

pub fn interpret_tree<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>) -> Vec<ThreeAddressCode> {
    // let mut funcdefs = Vec::new();
    // println!("{}", syntaxtree);
    let mut tempvarcount: uint = 0;
    match syntaxtree {
        &box ::compiler::ProgramNode(ref listoffuncdefs) => {
            for def in listoffuncdefs.iter() {
                // println!("{}", def);
                match def {
                    &::compiler::FuncDeclNode(ismain, typestr, ref ident, ref args, ref block) => {
                        if ismain {
                            return interpret_block_node(block, &tempvarcount);
                        }
                        else {
                            println!("{}", ident);
                        }
                        let mut funcdefs = Vec::new();
                        funcdefs.push( (ident, args, block) );
                        continue;
                    },
                    _ => fail!()
                }
            }
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_block_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::BlockNode(ref stmtlistnode) => {
            return interpret_stmt_list_node(stmtlistnode, tempvarcount);
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_stmt_list_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::StmtListNode(ref listofnodes) => {
            for node in listofnodes.iter() {
                println!("{}", node);
                match node {
                    &::compiler::StmtNode(ref left, ref right) => {
                        interpret_stmt_node(node, tempvarcount);
                    },
                    _ => continue
                }
            }
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_stmt_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::StmtNode(ref ident, ref expr) => {
            match ident {
                &box ::compiler::IdentNode(identity) => {
                    /*let mut vec = Vec::new();
                    vec.push(SimpleInstr(identity.to_owned(), interpret_expr_node(expr,
                                                                                  tempvarcount)));
                    return vec;*/
                    return interpret_expr_node(expr, tempvarcount);
                },
                _ => fail!()
            }
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_expr_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::ExprNode(operation, ref left, ref right) => {
            println!("{}", operation);
            match (left, right) {
                (&box ::compiler::NumberNode(n), &box ::compiler::NumberNode(m)) => {
                    let mut tempvarname = String::new();
                    tempvarname = tempvarname.append("t");
                    tempvarname = tempvarname.append((*tempvarcount).to_string().as_slice());
                    let n_as_int: int = ::std::from_str::from_str(n).unwrap();
                    let m_as_int: int = ::std::from_str::from_str(m).unwrap();
                    let instr = match operation {
                        "+" => SimpleInstr(TempVar(tempvarname), 
                                           Addition(NumberConst(n_as_int),
                                           NumberConst(m_as_int))),
                        "-" => SimpleInstr(TempVar(tempvarname),
                                           Subtraction(NumberConst(n_as_int),
                                           NumberConst(m_as_int))),
                        "*" => SimpleInstr(TempVar(tempvarname),
                                           Multiplication(NumberConst(n_as_int),
                                           NumberConst(m_as_int))),
                        "/" => SimpleInstr(TempVar(tempvarname),
                                           Division(NumberConst(n_as_int),
                                           NumberConst(m_as_int))),
                        _ => fail!()
                    };
                    let mut vec = Vec::new();
                    vec.push(instr);
                    return vec;
                },
                _ => fail!()
            }
        },
        _ => fail!()
    }
    Vec::new()
}
