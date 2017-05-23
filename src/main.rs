
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

extern crate llvm;
use llvm::*;

fn main() {
    let mut state = State::new();
    state.def_var("hello", Tast::Num(12));

    let comp = compile(&state, None);
    println!("{:?}", comp);
}


#[derive(Debug)]
enum Ast {
    Atom(String),
    List(Vec<Ast>),
}

#[derive(Debug, PartialEq, Clone)]
enum Tast {
    Unit,
    Num(i64),
    Var(String),
    List(Vec<Tast>),
}

struct State {
    vars: HashMap<String, Tast>,
}

impl State {
    fn new() -> Self {
        State { vars: HashMap::new() }
    }

    fn def_var<T: Into<String>>(&mut self, name: T, value: Tast) {
        self.vars.insert(name.into(), value);
    }
}

fn interpret(state: &State, tast: &Tast) -> Tast {
    let plus = Tast::Var("+".into());

    match *tast {
        ref unit @ Tast::Unit => unit.clone(),
        Tast::Num(v) => Tast::Num(v),
        Tast::Var(ref name) => state.vars[name].clone(),
        Tast::List(ref children) if children.is_empty() => Tast::Unit,
        Tast::List(ref children) => {
            let (first, rest) = children.split_first().unwrap(); // not empty
            if first == &plus {
                let result = rest.into_iter().fold(0, |acc, elem| {
                    acc +
                    match *elem {
                        Tast::Num(v) => v,
                        _ => panic!(),
                    }
                });
                return Tast::Num(result);
            } else {
                unimplemented!()
            }
        }
    }
}

struct Comp<'a> {
    context: &'a Context,
    module: CSemiBox<'a, Module>,
}

fn compile<'a>(state: &State, tast: Option<&Tast>) -> Comp<'a> {
    let context = unsafe { Context::get_global() };

    let module = Module::new("name", context);

    for (name, value) in state.vars.iter() {
        let value = match *value {
            Tast::Unit => Compile::compile((), &context),
            Tast::Num(v) => v.compile(&context),
            _ => unimplemented!(),
        };

        module.add_global_variable(&*name, value);
    }

    Comp {
        context: context,
        module: module,
    }
}

#[test]
fn test1() {
    let state = State::new();
    let tast = Tast::Num(1);
    assert_eq!(interpret(&state, &tast), Tast::Num(1));


    let mut state = State::new();
    state.def_var("hello", Tast::Num(1));
    assert_eq!(interpret(&state, &Tast::Var("hello".into())), Tast::Num(1));

    assert_eq!(interpret(&state, &Tast::List(vec![])), Tast::Unit);
    assert_eq!(interpret(&state,
                         &Tast::List(vec![Tast::Var("+".into()), Tast::Num(1), Tast::Num(1)])),
               Tast::Num(2));
}


use std::fmt::{self, Debug, Formatter};

impl<'a> Debug for Comp<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Comp({:#?})", self.module)
    }
}
