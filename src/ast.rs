use std::fmt;

#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    Variable(String),
    Lambda(String, Box<Token>),
    Application(Box<Token>, Box<Token>)
}

use self::Token::*;

fn get_variables(token: &Token) -> Vec<String> {
    match token {
        &Variable(ref literal) => vec![literal.clone()],
        &Lambda(ref param, ref body) => {
            let mut body_vars = get_variables(body);
            body_vars.push(param.clone());
            body_vars
        },
        &Application(ref left, ref right) => {
            let mut left_vars = get_variables(left);
            let mut right_vars = get_variables(right);
            left_vars.append(&mut right_vars);
            left_vars //TODO Make this distinct
        }
    }
}

fn replace_variable(token: &Token, old_literal: &String, new_literal: &String) -> Token {
    match token {
        &Variable(ref literal) => {
            if literal == old_literal {
                Variable(new_literal.clone())
            } else {
                Variable(literal.clone())
            }
        },
        &Lambda(ref param, ref body) => {
            let new_param = if param == old_literal {
                    new_literal.clone()
                } else {
                    param.clone()
                };
            let new_body = Box::new(replace_variable(body, old_literal, new_literal));
            Lambda(new_param, new_body)
        },
        &Application(ref left, ref right) => {
            let new_left = Box::new(replace_variable(left, old_literal, new_literal));
            let new_right = Box::new(replace_variable(right, old_literal, new_literal));
            Application(new_left, new_right)
        }
    }
}

fn find_new_variable_name(variables_in_scope: &Vec<String>) -> String {
    (0..).map(|i| i.to_string()).find(|s| variables_in_scope.iter().find(|&x| *x == *s).is_none()).unwrap()
}

fn replace_parameter(parameter: &String, template: &Token, insert: &Token, variables_in_scope: &Vec<String>) -> Token {
    match template {
        &Variable(ref literal)  => {
            if literal == parameter {
                insert.clone()
            } else {
                template.clone()
            }
        },
        &Lambda(ref param, ref body) => {
            let lambda =
                if variables_in_scope.contains(param) {
                    let mut all_variables_in_scope = get_variables(body);
                    all_variables_in_scope.extend(variables_in_scope.clone()); //TODO Distinct
                    let new_variable_name = find_new_variable_name(&all_variables_in_scope);
                    replace_variable(template, param, &new_variable_name)
                } else {
                    template.clone()
                };
            if let Lambda(ref lambda_param, ref lambda_body) = lambda {
                let new_body = Box::new(replace_parameter(parameter, lambda_body, insert, variables_in_scope));
                Lambda(lambda_param.clone(), new_body)
            } else {
                panic!("Not a Lambda anymore");
            }
        },
        &Application(ref left, ref right) => {
            let new_left = Box::new(replace_parameter(parameter, left, insert, variables_in_scope));
            let new_right = Box::new(replace_parameter(parameter, right, insert, variables_in_scope));
            Application(new_left, new_right)
        }
    }
}

impl Token {
    fn evaluate_once(&self, variables_in_scope: &Vec<String>) -> Token {
        match self {
            &Variable(_) => self.clone(),
            &Lambda(ref param, ref body) => {
                let new_var_in_scope = [param.clone()];
                let new_variables_in_scope: Vec<_> = variables_in_scope.iter().chain(new_var_in_scope.iter()).cloned().collect();
                Lambda(param.clone(), Box::new(body.evaluate_once(&new_variables_in_scope)))
            },
            &Application(ref left, ref right) => {
                if let &Lambda(ref param, ref body) = left as &Token {
                    let mut all_variables_in_scope = get_variables(right);
                    all_variables_in_scope.push(param.clone());
                    all_variables_in_scope.extend(variables_in_scope.clone());
                    replace_parameter(param, body, right, &all_variables_in_scope)
                } else {
                    Application(Box::new(left.evaluate_once(variables_in_scope)),
                                Box::new(right.evaluate_once(variables_in_scope)))
                }
            }
        }
    }

    pub fn evaluate(&self) -> Token {
        let variables_in_scope = Vec::new();
        let mut old_evaluated = self.evaluate_once(&variables_in_scope);
        loop {
            let evaluated = old_evaluated.evaluate_once(&variables_in_scope);
            if old_evaluated.to_string() == evaluated.to_string() {
                return evaluated;
            }
            old_evaluated = evaluated;
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Variable(ref x) => write!(f, "{}", x.clone()),
            &Lambda(ref param, ref body) => write!(f, "(λ{}.{})", param.clone(), body),
            &Application(ref left, ref right) => write!(f, "({} {})", left, right)
        }
    }
}


