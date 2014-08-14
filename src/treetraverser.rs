// struct Scope {
//     childscopes: Vec<Scope>
// }

pub fn interpret_tree<'a>(syntaxtree: Box<::compiler::AstNode<'a>>) {
    // let mut funcdefs = Vec::new();
    // println!("{}", syntaxtree);
    match syntaxtree {
        box ::compiler::ProgramNode(listoffuncdefs) => {
            for def in listoffuncdefs.iter() {
                println!("{}", def);
                match def {
                    &::compiler::FuncDeclNode(ismain, typestr, ref ident, ref args, ref block) => {
                        continue;
                    },
                    _ => fail!()
                }
            }
        },
        _ => fail!()
    }
}
