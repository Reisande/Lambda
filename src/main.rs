use std::rc::Rc;
use std::borrow::Borrow;

#[derive(Debug)]
enum Top {
    Variable(char),
    Lambda(char, Rc<Top>),
    Application(Rc<Top>, Rc<Top>),
}

// TODO: convert this to use a string builder so this fn isn't O(n^2)
fn pretty_print(t: &Rc<Top>) -> String {
    match (*t).borrow() {
        Top::Variable(v) => return v.to_string(),
        Top::Lambda(parameter, body) => {
            return String::from("λ(")
                + &parameter.to_string()
                + &String::from(".")
                + &pretty_print(body)
                + &String::from(")");
        }
        Top::Application(caller, argument) => {
            return String::from("(")
                + &pretty_print(caller)
                + &String::from(" ")
                + &pretty_print(argument)
                + &String::from(")");
        }
    }
}
 
// goes through the entirely of the tree representation and replaces an argument
// with a value
fn replace_parameter(t: &Rc<Top>, argument: &Rc<Top>, name: char) -> Rc<Top> {
    match (*t).borrow() {
        Top::Variable(v) => {
            if *v == name {
                return Rc::clone(argument);
            } 
        },
        Top::Lambda(parameter, ref body) => {
            if *parameter != name {
                return Rc::new(Top::Lambda(*parameter, replace_parameter(body, argument, name)));
            } 
        },
        Top::Application(caller, inner_argument)=> {
            return Rc::new(Top::Application(replace_parameter(caller, argument, name),
                                             replace_parameter(inner_argument, argument, name)));
        }
    }
    Rc::clone(t)
}

fn main() {
    let top = Rc::new(Top::Application(
        Rc::new(Top::Lambda(
            'y',
            Rc::new(Top::Variable('y')),
        )),
        Rc::new(Top::Variable('x')),
    ));

    println!("Evaluation: {:?}", pretty_print(&top));

    let replaced = replace_parameter(&top, &Rc::new(Top::Lambda(
            'z',
            Rc::new(Top::Variable('z')),
    )), 'x');
    
    println!("Replaced parameter: {:?}", pretty_print(&replaced));
}
