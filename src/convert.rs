use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{
    ltl::LTL,
    pdc::{Action, Domain, Problem, Value, Variable},
};

struct OutputMapping(Vec<(String, Vec<String>)>);

impl Display for OutputMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (predicate, outputs)) in self.0.iter().enumerate() {
            writeln!(f, "o{i}: {}({})", predicate, outputs.join(", "))?;
        }
        Ok(())
    }
}

pub fn collect_output(domain: &Domain, problem: &Problem) -> OutputMapping {
    let mut result = Vec::new();
    for predicate in &domain.predicates {
        let outputs: Vec<_> = predicate
            .parameters
            .iter()
            .map(|parameter| {
                let param_type = parameter.data_type.clone().unwrap();
                problem.variables_of_type(&param_type)
            })
            .multi_cartesian_product()
            .collect();
        // assign o_i to each output
        for output in outputs {
            result.push((
                predicate.name.clone(),
                output.iter().map(|it| it.name.clone()).collect_vec(),
            ));
        }
    }
    for action in &domain.actions {
        let outputs: Vec<_> = action
            .parameters
            .iter()
            .map(|parameter| {
                let param_type = parameter.data_type.clone().unwrap();
                problem.variables_of_type(&param_type)
            })
            .multi_cartesian_product()
            .collect();
        // assign o_i to each output
        for output in outputs {
            result.push((
                action.name.clone(),
                output.iter().map(|it| it.name.clone()).collect_vec(),
            ));
        }
    }
    OutputMapping(result)
}

fn convert_value(
    value: &Value,
    substitution: &HashMap<String, String>,
    output_mapping: &OutputMapping,
) -> LTL {
    match value {
        Value::And(values) => LTL::And(
            values
                .iter()
                .map(|value| convert_value(value, substitution, output_mapping))
                .collect(),
        ),
        Value::Or(values) => LTL::Or(
            values
                .iter()
                .map(|value| convert_value(value, substitution, output_mapping))
                .collect(),
        ),
        Value::Not(value) => LTL::Not(Box::new(convert_value(value, substitution, output_mapping))),
        Value::Call(name, params) => LTL::Atom(
            output_mapping
                .0
                .iter()
                .position(|(it_name, it_arguments)| {
                    let arguments = params
                        .iter()
                        .map(|param| substitution.get(param).unwrap())
                        .cloned()
                        .collect_vec();
                    it_name == name && it_arguments == &arguments
                })
                .unwrap() as _,
        ),
    }
}

fn convert_action_with_param_set(
    action: &Action,
    arguments: Vec<&Variable>,
    output_mapping: &OutputMapping,
) -> LTL {
    let mapping: HashMap<_, _> = action
        .parameters
        .iter()
        .zip(arguments.iter())
        .map(|(param, argument)| {
            let param_type = param.data_type.as_ref().unwrap();
            let value_type = &argument.data_type;
            // todo: a real type checking
            assert_eq!(param_type, value_type);
            (param.name.clone(), argument.name.clone())
        })
        .collect();
    let preconditions = LTL::And(
        action
            .preconditions
            .iter()
            .map(|precondition| convert_value(precondition, &mapping, output_mapping))
            .collect(),
    );
    let effects = LTL::And(
        action
            .effects
            .iter()
            .map(|postcondition| convert_value(postcondition, &mapping, output_mapping))
            .collect(),
    );
    let argument_names = arguments
        .iter()
        .map(|arg| arg.name.clone())
        .collect::<Vec<_>>();
    let call = LTL::Atom(
        output_mapping
            .0
            .iter()
            .position(|(name, it_arguments)| {
                name == &action.name && it_arguments == &argument_names
            })
            .unwrap() as _,
    );
    LTL::Globally(Box::new((call & preconditions).implies(LTL::next(effects))))
}

pub fn convert_action(action: &Action, problem: &Problem, output_mapping: &OutputMapping) -> LTL {
    let apply_action_with_param_sets: Vec<_> = action
        .parameters
        .iter()
        .map(|parameter| {
            let param_type = parameter.data_type.clone().unwrap();
            problem.variables_of_type(&param_type)
        })
        .multi_cartesian_product()
        .collect();
    apply_action_with_param_sets
        .into_iter()
        .map(|arguments| convert_action_with_param_set(action, arguments, output_mapping))
        .fold(LTL::Top, |acc, ltl| acc & ltl)
}

#[cfg(test)]
mod tests {
    use crate::pdc::{Parameter, Predicate};

    use super::*;

    #[test]
    fn test_collect_output() {
        let domain = Domain {
            name: "Domain".to_string(),
            types: vec![("Type".to_string(), None)],
            predicates: vec![
                Predicate {
                    name: "p0".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "a".to_string(),
                            data_type: Some("Type".to_string()),
                        },
                        Parameter {
                            name: "b".to_string(),
                            data_type: Some("Type".to_string()),
                        },
                    ],
                },
                Predicate {
                    name: "p1".to_string(),
                    parameters: vec![Parameter {
                        name: "c".to_string(),
                        data_type: Some("Type".to_string()),
                    }],
                },
            ],
            actions: vec![Action {
                name: "a0".to_string(),
                parameters: vec![
                    Parameter {
                        name: "q0".to_string(),
                        data_type: Some("Type".to_string()),
                    },
                    Parameter {
                        name: "q1".to_string(),
                        data_type: Some("Type".to_string()),
                    },
                ],
                preconditions: vec![Value::Call(
                    "p0".to_string(),
                    vec!["q0".to_string(), "q1".to_string()],
                )],
                effects: vec![Value::And(vec![
                    Value::Call("p1".to_string(), vec!["q0".to_string()]),
                    Value::Not(Box::new(Value::Call(
                        "p1".to_string(),
                        vec!["q1".to_string()],
                    ))),
                ])],
            }],
        };
        let problem = Problem {
            name: "Problem".to_string(),
            domain: domain.name.clone(),
            variables: vec![
                Variable {
                    name: "v1".to_string(),
                    data_type: "Type".to_string(),
                },
                Variable {
                    name: "v2".to_string(),
                    data_type: "Type".to_string(),
                },
            ],
            init: Vec::new(),
            goal: Vec::new(),
        };
        let output_mapping = collect_output(&domain, &problem);
        let action = convert_action(&domain.actions[0], &problem, &output_mapping);
        println!("{}", output_mapping);
        println!("{}", action);
    }
}
