#[deriving(Show)]
#[deriving(Clone)]
enum AstNode<'a> {
    ProgramNode(Vec<AstNode<'a>>),
    BlockNode(Box<AstNode<'a>>),
    StmtListNode(Vec<AstNode<'a>>),
    StmtNode(Box<AstNode<'a>>, Box<AstNode<'a>>),
    ReturnStmtNode(Box<AstNode<'a>>), // Right (right of RETURN)
    WriteStmtNode(Box<AstNode<'a>>),
    FuncDeclNode(bool, &'a str, Box<AstNode<'a>>, Box<AstNode<'a>>, Box<AstNode<'a>>), // Type, IdentNode, Args, Block
    FuncArgListNode(Vec<AstNode<'a>>),
    FuncArgNode(&'a str, Box<AstNode<'a>>), // Type, IdentNode
    FuncCallNode(Box<AstNode<'a>>, Box<AstNode<'a>>), // IdentNode, Args
    FuncCallArgListNode(Vec<AstNode<'a>>),
    FuncCallArgNode(Box<AstNode<'a>>),
    DeclarNode(&'a str, Box<AstNode<'a>>),
    IdentNode(&'a str),
    ExprNode(&'a str, Box<AstNode<'a>>, Box<AstNode<'a>>), // Operator, Left, Right
    NumberNode(&'a str),
    FailureNode
}

pub fn compile<'a>(tokens: Vec<&'a str>) -> AstNode<'a> {
    return parse_program(tokens.as_slice());
}

pub fn parse_program<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    // println!("program");
    // println!("{}", tokens);
    // for (index, token) in tokens.iter().enumerate() {
    //    println!("{}", token);
    // }
    // ProgramNode(box parse_stmt_list(tokens.as_slice()))
    let mut children = Vec::new();
    if tokens.len() > 0 && tokens[0] == "INT" && tokens[2] == "(" {
        for (index, token) in tokens.slice_from(3).iter().enumerate() {
            match *token {
                "END" => {
                    children.push(parse_func_decl(tokens.slice_to(index+4)));
                    if tokens.slice_from(index+4).len() > 0 {
                        match parse_program(tokens.slice_from(index+4)) {
                            ProgramNode(listofchildren) => {
                                children = children.append(listofchildren.as_slice());
                            },
                            _ => children.push(FailureNode)
                        }
                        return ProgramNode(children);
                    }
                },
                _ => continue
            }
        }
    }
    else if tokens.len() > 0 && tokens[0] == "INT" && tokens[1] == "MAIN" &&
        tokens[3] == "(" {
            for (index, token) in tokens.slice_from(4).iter().enumerate() {
                match *token {
                    "END" => {
                        children.push(parse_func_decl(tokens.slice_to(index+5)));
                        if tokens.slice_from(index+5).len() > 0 {
                            match parse_program(tokens.slice_from(index+5)) {
                                ProgramNode(listofchildren) => {
                                    children = children.append(listofchildren.as_slice());
                                },
                                _ => children.push(FailureNode)
                            }
                            return ProgramNode(children);
                        }
                    },
                    _ => continue
                }
            }
    }
    return ProgramNode(children);
}

fn parse_block<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    // println!("{}", tokens);
    if tokens[0] == "BEGIN" && tokens[tokens.len()-1] == "END" {
        return BlockNode(box parse_stmt_list(tokens.slice(1,tokens.len()-1)));
    }
    return FailureNode;
}

fn parse_func_decl<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    // println!("FuncDecl");
    // println!("{}", tokens);
    if tokens[0] == "INT" && tokens[1] == "MAIN" && tokens[3] == "(" {
        for (index, token) in tokens.slice_from(4).iter().enumerate() {
            match *token {
                ")" => {
                    return FuncDeclNode(
                                        true,
                                        tokens[0], 
                                        box parse_ident(tokens.slice(2,3)),
                                        box parse_func_arg_list(tokens.slice(4, index+4)),
                                        box parse_block(tokens.slice_from(index+5))
                                        );
                },
                _ => continue
            }
        }
    }
    else if tokens[0] == "INT" && tokens[2] == "(" {
        for (index, token) in tokens.slice_from(3).iter().enumerate() {
            match *token {
                ")" => {
                    return FuncDeclNode(
                                        false,
                                        tokens[0],
                                        box parse_ident(tokens.slice(1,2)),
                                        box parse_func_arg_list(tokens.slice(3, index+3)),
                                        box parse_block(tokens.slice_from(index+4))
                                        );
                },
                _ => continue
            }
        }
    }
    println!("Failing.");
    return FailureNode;
}

fn parse_func_arg_list<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    let mut children = Vec::new();
    if tokens.len() == 2 {
        children.push(parse_func_arg(tokens.slice_to(2)));
    }
    else if tokens.len() > 2 {
        children.push(parse_func_arg(tokens.slice_to(2)));
        match parse_func_arg_list(tokens.slice_from(3)) {
            FuncArgListNode(listofnodes) => {
                children = children.append(listofnodes.as_slice());
            },
            _ => return FailureNode
        }
    }
    return FuncArgListNode(children);
}

fn parse_func_arg<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    if tokens.len() == 2 {
        return FuncArgNode(tokens[0], box IdentNode(tokens[1]));
    }
    return FailureNode;
}

fn parse_stmt_list<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    let mut children = Vec::new();
    let mut prev_end = 0u;
    for (index, token) in tokens.iter().enumerate() {
        match *token {
            "BEGIN" => {
            let mut nestcount = 1u;
                for (innerindex, innertoken) in tokens.slice_from(index+1).iter().enumerate() {
                    // println!("{}", tokens.slice_from(index+1));
                    match *innertoken {
                        "BEGIN" => {
                            nestcount += 1;
                        },
                        "END" => {
                            nestcount -= 1;
                            if nestcount == 0 {
                                // println!("{}", tokens.slice(index, innerindex+2));
                                children.push(parse_block(tokens.slice(index, innerindex+2)));
                                if tokens.slice_from(innerindex+2).len() > 0 {
                                    match parse_stmt_list(tokens.slice_from(innerindex+1)) {
                                        StmtListNode(listofnodes) => {
                                            children = children.append(listofnodes.as_slice());
                                        },
                                        FailureNode => children.push(FailureNode),
                                        _ => fail!()
                                    }
                                }
                            }
                            return StmtListNode(children);
                        },
                        _ => {
                            continue;
                        }
                    }
                }
                return FailureNode;
            },
            ";" => {
                children.push(parse_stmt(tokens.slice(prev_end, index+1)));
                prev_end = index+1
            },
            _ => continue
        }
    }
    if children.len() > 0 {
        return StmtListNode(children);
    }
    return FailureNode;
}

fn parse_stmt<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    for (index, token) in tokens.iter().enumerate().rev() {
        match *token {
            ";" => {
                for (innerindex, innertoken) in tokens.slice_to(index).iter().enumerate().rev() {
                    match *innertoken {
                        ":=" => return StmtNode(box parse_ident(tokens.slice_to(innerindex)),
                                                box parse_expr(tokens.slice(innerindex+1,
                                                                            tokens.len()-1))
                                                ),
                        _ => continue
                    }
                }
                if tokens[0] == "RETURN" {
                    return parse_return_stmt(tokens);
                }
                else if tokens[0] == "WRITE" {
                    return parse_write_stmt(tokens);
                }
                return parse_decl(tokens);
            },
            _ => continue
        }
    }
    return FailureNode;
}

fn parse_return_stmt<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    if tokens[0] == "RETURN" && tokens[tokens.len()-1] == ";" {
        return ReturnStmtNode(box parse_expr(tokens.slice(1, tokens.len()-1)))
    }
    return FailureNode;
}

fn parse_write_stmt<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    if tokens[0] == "WRITE" && tokens[tokens.len()-1] == ";" && 
        tokens[1] == "(" && tokens[tokens.len()-2] == ")" {
            return WriteStmtNode(box parse_func_call_args_list(tokens.slice(2, tokens.len()-2)));
    }
    return FailureNode;
}
fn parse_decl<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    match tokens[0] {
        "INT" => {
            return DeclarNode("INT", box parse_ident(tokens.slice(1,2)));
        },
        _ => return FailureNode
    }
}

fn parse_ident<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    if tokens.len() == 1 {
        return IdentNode(tokens[0]);
    }
    return FailureNode;
}

fn parse_expr<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    if tokens.len() == 1 {
        match from_str::<int>(tokens[0].as_slice()) {
            Some(n) => return NumberNode(tokens[0]),
            None => return IdentNode(tokens[0])
        }
    }
    for (index, token) in tokens.iter().enumerate().rev() {
    	match *token {
            "+" | "-" => {
                return ExprNode(*token, 
                    box parse_expr(tokens.slice_to(index)),
                    box parse_expr(tokens.slice_from(index+1))
                    );
            },
            ")" => {
                break;
            },
            "(" => {
                return FailureNode;
            },
            _ => continue
        }
    }
    for (index, token) in tokens.iter().enumerate().rev() {
    	match *token {
            "*" | "/" => {
                return ExprNode(*token, 
                    box parse_expr(tokens.slice_to(index)),
                    box parse_expr(tokens.slice_from(index+1))
                    );
            },
            ")" => {
                break;
            },
            "(" => {
                return FailureNode;
            },
            _ => continue
        }
    }
    for (index, token) in tokens.iter().enumerate().rev() {
        match *token {
            ")" => {
                let mut parencount = 1u;
                for (innerindex, innertoken) in tokens.slice_to(index).iter().enumerate().rev() {
                    match *innertoken {
                        ")" => {
                            parencount += 1u;
                        },
                        "(" => {
                            parencount -= 1u;
                            if parencount == 0u {
                                if tokens.slice_to(innerindex).len() == 1 {
                                    return parse_func_call(tokens);
                                }
                                for (leftindex, lefttoken) in
                                    tokens.slice_to(innerindex).iter().enumerate().rev() {
                                    match *lefttoken {
                                        "+" | "-" => {
                                            return ExprNode(*lefttoken, 
                                                            box
                                                            parse_expr(tokens.slice_to(leftindex)),
                                                            box
                                                            parse_expr(tokens.slice_from(leftindex+1))
                                                            );
                                        },
                                        _ => continue
                                    }
                                }
                                for (leftindex, lefttoken) in
                                    tokens.slice_to(innerindex).iter().enumerate().rev() {
                                    match *lefttoken {
                                        "*" | "/" => {
                                            return ExprNode(*lefttoken, 
                                                            box
                                                            parse_expr(tokens.slice_to(leftindex)),
                                                            box
                                                            parse_expr(tokens.slice_from(leftindex+1))
                                                            );
                                        },
                                        _ => continue
                                    }
                                }
                                return parse_expr(tokens.slice(innerindex+1, index));
                            }
                        },
                        _ => continue
                    }
                }
            },
            "(" => {
                return FailureNode;
            },
            _ => continue
        }
    }
    return FailureNode;
}

fn parse_func_call<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    if tokens[1] == "(" && tokens[tokens.len()-1] == ")" {
        return FuncCallNode(box parse_ident(tokens.slice(0,1)), 
                            box parse_func_call_args_list(tokens.slice(2,tokens.len()-1)));
    }
    return FailureNode;
}

fn parse_func_call_args_list<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    let mut children = Vec::new();
    let mut parencount = 0u;
    if tokens.len() == 0 {
        return FuncCallArgListNode(Vec::new());
    }
    for (index, token) in tokens.iter().enumerate() {
        match *token {
            "(" => {
                parencount += 1;
            },
            ")" => {
                if parencount > 0 {
                    parencount -= 1;
                }
                else {
                    return FailureNode;
                }
            },
            "," => {
                if parencount == 0 {
                    children.push(parse_func_call_arg(tokens.slice_to(index)));
                    match parse_func_call_args_list(tokens.slice_from(index+1)) {
                        FuncCallArgListNode(listofnodes) => {
                            children = children.append(listofnodes.as_slice());
                        },
                        _ => children.push(FailureNode)
                    }
                }
            },
            _ => continue
        }
    }
    if children.len() == 0 {
        children.push(parse_func_call_arg(tokens));
        return FuncCallArgListNode(children);
    }
    else if children.len() > 0 {
        return FuncCallArgListNode(children);
    }
    return FailureNode;
}

fn parse_func_call_arg<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    return parse_expr(tokens);
}

fn parse_number(single_token: &[&str]) {

}
