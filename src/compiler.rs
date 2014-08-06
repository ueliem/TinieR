#[deriving(Show)]
enum AstNode<'a> {
    ProgramNode,
    StmtListNode(&'a [AstNode<'a>]),
    StmtNode(Box<AstNode<'a>>, Box<AstNode<'a>>),
    IdentNode(&'a str),
    ExprNode(&'a str, Box<AstNode<'a>>, Box<AstNode<'a>>), // Operator, Left, Right
    NumberNode(&'a str),
    FailureNode
}

pub fn compile<'a>(tokens: Vec<&'a str>) -> AstNode {
    // for (index, token) in tokens.iter().enumerate() {
    //    println!("{}", token);
    // }
    parse_stmt_list(tokens.as_slice())
}

fn parse_stmt_list<'a>(tokens: &[&'a str]) -> AstNode<'a> {
    let mut children = Vec::new();
    let mut tokenstack = Vec::new();
    for (index, token) in tokens.iter().enumerate().rev() {
        match *token {
            ";" => {
                tokenstack.push(*token);
                children.push(parse_stmt(tokenstack.as_slice()));
                tokenstack = Vec::new();
            },
            _ => tokenstack.push(*token)
        }
    }
    if children.len() > 0 {
        return StmtListNode(children.as_slice());
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
                                                box parse_expr(tokens.slice_from(index+1))
                                                ),
                        _ => continue
                    }
                }
                return FailureNode;
            },
            _ => continue
        }
    }
    return FailureNode;
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
            "*" | "/" | "+" | "-" => {
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
