#[deriving(Show)]
#[deriving(Clone)]
enum AstNode<'a> {
    ProgramNode(Vec<AstNode<'a>>),
    BlockNode(Box<AstNode<'a>>),
    StmtListNode(Vec<AstNode<'a>>),
    StmtNode(Box<AstNode<'a>>, Box<AstNode<'a>>),
    FuncDeclNode(&'a str, Box<AstNode<'a>>, Box<AstNode<'a>>, Box<AstNode<'a>>), // Type, IdentNode, Args, Block
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
    return ProgramNode(children);
}

fn parse_block<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    println!("{}", tokens);
    if tokens[0] == "BEGIN" && tokens[tokens.len()-1] == "END" {
        return BlockNode(box parse_stmt_list(tokens.slice(1,tokens.len()-1)));
    }
    return FailureNode;
}

fn parse_func_decl<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    // println!("FuncDecl");
    // println!("{}", tokens);
    if tokens[0] == "INT" && tokens[2] == "(" {
        for (index, token) in tokens.slice_from(3).iter().enumerate() {
            match *token {
                ")" => {
                    return FuncDeclNode(
                                        tokens[0],
                                        box parse_ident(tokens.slice(1,2)),
                                        box parse_func_args(tokens.slice(3, index+3)),
                                        box parse_block(tokens.slice_from(index+4))
                                        );
                },
                _ => continue
            }
        }
    }
    return FailureNode;
}

fn parse_func_args<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    FailureNode
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
                return parse_decl(tokens);
            },
            _ => continue
        }
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
            _ => continue
        }
    }
    return FailureNode;
}

fn parse_number(single_token: &[&str]) {

}
