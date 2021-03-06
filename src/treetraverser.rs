#[deriving(Show)]
#[deriving(Clone)]
enum VariableType {
    RealVar(String),
    TempVar(String),
    NumberConst(int)
}

#[deriving(Show)]
#[deriving(Clone)]
enum ThreeAddressCode {
    SimpleInstr(VariableType, TACRight),//Dest, (equals) (expr)
    // CallInstr(String),
    Label(String),//name of label. When using goto label or call label, it goes to the instruction
    //following the label
    Write(Vec<VariableType>),
    Return(VariableType),
    NopInstr
}

#[deriving(Show)]
#[deriving(Clone)]
enum TACRight {
    Addition(VariableType, VariableType),
    Subtraction(VariableType, VariableType),
    Multiplication(VariableType, VariableType),
    Division(VariableType, VariableType),
    Assignment(VariableType),
    CallInstr(String, Vec<String>)
}

pub fn interpret_tree<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>) -> Vec<ThreeAddressCode> {
    // let mut funcdefs = Vec::new();
    // println!("{}", syntaxtree);
    let mut tempvarcount: uint = 0;
    match syntaxtree {
        &box ::compiler::ProgramNode(ref listoffuncdefs) => {
            let mut vec = Vec::new();
            for def in listoffuncdefs.iter() {
                // println!("{}", def);
                match def {
                    &::compiler::FuncDeclNode(ismain, typestr, ref ident, ref args, ref block) => {
                        /*if ismain {
                            return interpret_block_node(block, &mut tempvarcount);
                        }
                        else {
                            // println!("{}", ident);
                        }
                        let mut funcdefs = Vec::new();
                        funcdefs.push( (ident, args, block) );*/
                        println!("Func");
                        if ismain || true {
                            vec = vec.append(interpret_func_decl_node(def, &mut tempvarcount).as_slice());
                        }
                    },
                    _ => fail!()
                }
            }
            return vec;
        },
        _ => fail!()
    }
    Vec::new()
}

fn interpret_func_decl_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &mut uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::FuncDeclNode(ismain, typestr, ref ident, ref args, ref block) => {
            if ismain || true {
                match ident {
                    &box ::compiler::IdentNode(identity) => {
                        return Vec::new().append_one(Label(identity.to_string()))
                        .append(interpret_block_node(block, tempvarcount).as_slice());
                    },
                    _ => fail!()
                }
            }
            else {
                return Vec::new();
            }
        },
        _ => fail!()
    }
}

fn interpret_block_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &mut uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::BlockNode(ref stmtlistnode) => {
            return interpret_stmt_list_node(stmtlistnode, tempvarcount);
        },
        _ => fail!()
    }
}

fn interpret_stmt_list_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &mut uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &box ::compiler::StmtListNode(ref listofnodes) => {
            let mut vec = Vec::new();
            for node in listofnodes.iter() {
                println!("{}", node);
                match node {
                    &::compiler::StmtNode(_,_) => {
                        vec = vec.append(interpret_stmt_node(node, tempvarcount).as_slice());
                    },
                    &::compiler::DeclarNode(_,_) => {
                        vec = vec.append(interpret_decl_node(node, tempvarcount).as_slice());
                    },
                    &::compiler::WriteStmtNode(_) => {
                        vec = vec.append(interpret_write_stmt_node(node, tempvarcount).as_slice());
                    },
                    &::compiler::ReturnStmtNode(_) => {
                        vec = vec.append(interpret_return_stmt_node(node,
                                                                    tempvarcount).as_slice());
                    },
                    _ => continue
                }
            }
            return vec;
        },
        _ => fail!()
    }
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

fn interpret_stmt_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &mut uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::StmtNode(ref ident, ref expr) => {
            match ident {
                &box ::compiler::IdentNode(identity) => {
                    match interpret_expr_node(expr, Some(RealVar(identity.to_string())), tempvarcount) {
                        (tacode, somevar) => return tacode,
                    }
                },
                _ => fail!()
            }
        },
        _ => fail!()
    }
}

fn interpret_write_stmt_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &mut uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::WriteStmtNode(ref argslist) => {
            return Vec::new().append_one(Write(Vec::new()));
        },
        _ => fail!()
    }
}

fn interpret_return_stmt_node<'a>(syntaxtree: &::compiler::AstNode<'a>, tempvarcount: &mut uint) -> Vec<ThreeAddressCode> {
    match syntaxtree {
        &::compiler::ReturnStmtNode(ref argslist) => {
            match argslist {
                &box ::compiler::ExprNode(_,_,_) => {
                    match interpret_expr_node(argslist, None, tempvarcount) {
                        (tac, outvar) => {
                            return tac.append_one(Return(NumberConst(0)));
                        }
                    }
                },
                &box ::compiler::IdentNode(identity) => {
                    return Vec::new().append_one(Return(RealVar(identity.to_string())));
                },
                _ => fail!()
            }
        },
        _ => fail!()
    }
}

fn interpret_expr_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, realout: Option<VariableType>, tempvarcount: &mut uint)
    -> (Vec<ThreeAddressCode>,Option<VariableType>) {
        match syntaxtree {
            &box ::compiler::ExprNode(operation, ref left, ref right) => {
                // println!("{}", operation);
                println!("{}", syntaxtree);
                match interpret_expr_leftside(left, tempvarcount) {
                    (leftvecinstr, leftoutvar) => {
                        match interpret_expr_rightside(right, tempvarcount) {
                            (rightvecinstr, rightoutvar) => {
                                let comp = match operation {
                                    "+" => Addition(leftoutvar, rightoutvar),
                                    "-" => Subtraction(leftoutvar, rightoutvar),
                                    "*" => Multiplication(leftoutvar, rightoutvar),
                                    "/" => Division(leftoutvar, rightoutvar),
                                    _ => fail!()
                                };
                                let outvar = match realout {
                                    Some(variablename) => variablename,
                                    None => generate_tempvar(tempvarcount)
                                };
                                // println!("Left {}", leftvecinstr);
                                // println!("Middle {}", SimpleInstr(outvar.clone(), comp.clone()));
                                // println!("Right {}", rightvecinstr);
                                return (leftvecinstr
                                        .append(rightvecinstr.as_slice())
                                        .append_one(SimpleInstr(outvar.clone(), comp)), Some(outvar.clone()));
                            }
                        }
                    }
                }
            },
            &box ::compiler::IdentNode(identity) => {
                fail!();
            },
            _ => fail!()
        }
}

fn interpret_expr_leftside<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &mut uint) -> (Vec<ThreeAddressCode>,VariableType) {
    match syntaxtree {
        &box ::compiler::NumberNode(n) => {
            let n_as_int: int = ::std::from_str::from_str(n).unwrap();
            return (Vec::new(), NumberConst(n_as_int));
        },
        &box ::compiler::ExprNode(_,_,_) => {
            match interpret_expr_node(syntaxtree, None, tempvarcount) {
                tacode => {
                    match tacode {
                        (innercode, innerovar) => return (innercode, innerovar.unwrap()),
                    }
                }
            }
        },
        &box ::compiler::FuncCallNode(ref identity, ref funcargs) => {
            return interpret_func_call_node(syntaxtree, tempvarcount);
        },
        &box ::compiler::IdentNode(identity) => {
            return (Vec::new(), RealVar(identity.to_string()));
        },
        _ => fail!()
    }
}

fn interpret_expr_rightside<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &mut uint) -> (Vec<ThreeAddressCode>,VariableType) {
    match syntaxtree {
        &box ::compiler::NumberNode(n) => {
            let n_as_int: int = ::std::from_str::from_str(n).unwrap();
            return (Vec::new(), NumberConst(n_as_int));
        },
        &box ::compiler::ExprNode(_,_,_) => {
            match interpret_expr_node(syntaxtree, None, tempvarcount) {
                tacode => {
                    match tacode {
                        (innercode, innerovar) => return (innercode, innerovar.unwrap()),
                    }
                }
            }
            // return (interpret_expr_node(syntaxtree, tempvarcount), generate_tempvar(tempvarcount));
        },
        &box ::compiler::FuncCallNode(ref identity, ref funcargs) => {
            return interpret_func_call_node(syntaxtree, tempvarcount);
        },
        &box ::compiler::IdentNode(identity) => {
            return (Vec::new(), RealVar(identity.to_string()));
        },
        _ => fail!()
    }
}

fn interpret_func_call_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &mut uint) -> (Vec<ThreeAddressCode>,VariableType) {
    match syntaxtree {
        &box ::compiler::FuncCallNode(ref identity, ref funcargs) => {
            let mut vec = Vec::new();
            match interpret_func_arg_list_node(funcargs, tempvarcount) {
                (thac, arglist) => {
                    let outvar = generate_tempvar(tempvarcount);
                    vec.push(SimpleInstr(outvar.clone(), CallInstr(identity.to_string(),
                                                                             arglist
                                                                             )));
                    return (vec, outvar.clone());
                }
            }
        },
        _ => fail!()
    }
}

fn interpret_func_arg_list_node<'a>(syntaxtree: &Box<::compiler::AstNode<'a>>, tempvarcount: &uint) -> (Vec<ThreeAddressCode>,Vec<String>) {
    (Vec::new(), Vec::new())
}

fn generate_tempvar(tempvarcount: &mut uint) -> VariableType {
    let mut varname = String::new();
    varname = varname.append("t").append((*tempvarcount).to_string().as_slice());
    *tempvarcount += 1;
    return TempVar(varname);
}
