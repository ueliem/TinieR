#[deriving(Show)]
enum VariableType {
    RealVar(String),
    TempVar(String),
    NumberConst(int)
}

#[deriving(Show)]
enum ThreeAddressCode {
    SimpleInstr(VariableType, TACRight),//Dest, (equals) (expr)
    CallInstr(String),
    Label(String),//name of label. When using goto label or call label, it goes to the instruction
    //following the label
    NopInstr
}

#[deriving(Show)]
enum TACRight {
    Addition(VariableType, VariableType),
    Subtraction(VariableType, VariableType),
    Multiplication(VariableType, VariableType),
    Division(VariableType, VariableType),
    Assignment(VariableType),
    Write(VariableType)
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
}

fn interpret_stmt_list_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::StmtListNode(ref listofnodes) => {
            for node in listofnodes.iter() {
                println!("{}", node);
                match node {
                    &::compiler::StmtNode(_,_) => {
                        interpret_stmt_node(node, tempvarcount);
                    },
                    &::compiler::DeclarNode(_,_) => {
                        interpret_decl_node(node, tempvarcount);
                    },
                    _ => continue
                }
            }
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_decl_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::DeclarNode(typestr, ref identity) => {
            let mut declinstr = match identity {
                &box ::compiler::IdentNode(idstr) => SimpleInstr(RealVar(idstr.to_string()), Assignment(NumberConst(0))),
                _ => fail!()
            };
            // println!("{}", declinstr);
            let mut vec = Vec::new();
            vec.push(declinstr);
            return vec;
        },
        _ => fail!()
    }
}

fn interpret_stmt_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::StmtNode(ref ident, ref expr) => {
            match ident {
                &box ::compiler::IdentNode(_) => {
                    return interpret_expr_node(expr, tempvarcount);
                },
                _ => fail!()
            }
        },
        _ => fail!()
    }
}

fn interpret_expr_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::ExprNode(operation, ref left, ref right) => {
            println!("{}", operation);
            interpret_expr_leftside(left, tempvarcount);
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_expr_leftside<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> (Vec<ThreeAddressCode>,VariableType) {
    match syntaxtree {
        &box ::compiler::NumberNode(n) => {
            let n_as_int: int = ::std::from_str::from_str(n).unwrap();
            return (Vec::new(), NumberConst(n_as_int));
        },
        &box ::compiler::ExprNode(_,_,_) => {
            return (interpret_expr_node(syntaxtree, tempvarcount), TempVar("NotReal".to_string()));
        },
        &box ::compiler::FuncCallNode(ref identity, ref funcargs) => {
            println!("Handling FuncCallNode poorly.");
            fail!();
        },
        &box ::compiler::IdentNode(identity) => {
            return (Vec::new(), RealVar(identity.to_string()));
        },
        _ => fail!()
    }
}

fn interpret_expr_rightside<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> (Vec<ThreeAddressCode>,VariableType) {
    match syntaxtree {
        &box ::compiler::NumberNode(n) => {
            let n_as_int: int = ::std::from_str::from_str(n).unwrap();
            return (Vec::new(), NumberConst(n_as_int));
        },
        &box ::compiler::ExprNode(_,_,_) => {
            return (interpret_expr_node(syntaxtree, tempvarcount), TempVar("NotReal".to_string()));
        },
        &box ::compiler::FuncCallNode(ref identity, ref funcargs) => {
            println!("Handling FuncCallNode poorly.");
            fail!();
        },
        &box ::compiler::IdentNode(identity) => {
            return (Vec::new(), RealVar(identity.to_string()));
        },
        _ => fail!()
    }
}
