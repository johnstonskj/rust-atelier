#![allow(clippy::upper_case_acronyms)]

use crate::parser::error::ParserError;
use atelier_core::error::{Error, ErrorKind, Result as ModelResult};
use atelier_core::model::selector::{
    AttributeComparison, AttributeSelector, Comparator, Function, Key, KeyPathSegment,
    NeighborSelector, ScopedAttributeAssertion, ScopedAttributeSelector, ScopedValue, Selector,
    SelectorExpression, ShapeType, Value, VariableDefinition, VariableReference,
};
use atelier_core::model::values::Number;
use atelier_core::model::{Identifier, ShapeID};
use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "selector.pest"]
struct SelectorParser;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Parse a Smithy selector expression. These are not parsed by default when reading a Smithy
/// model as the selectors are not necessarily required for all processing cases.
///
pub fn parse_selector(input: &str) -> ModelResult<Selector> {
    let mut parsed = SelectorParser::parse(Rule::selector, input).map_err(from_pest_error)?;
    let top_pair = parsed.next().unwrap();
    trace!("{:#?}", top_pair);
    parse_selectors(top_pair)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_selectors(input_pair: Pair<'_, Rule>) -> ModelResult<Selector> {
    entry!("parse_selectors", input_pair);
    let mut results = Selector::default();
    match input_pair.as_rule() {
        Rule::selector => {
            for inner in input_pair.into_inner() {
                match inner.as_rule() {
                    Rule::selector_expression => {
                        results.add_expression(parse_selector_expression(inner)?);
                    }
                    Rule::EOI => {}
                    _ => unexpected!("parse_selectors", inner),
                }
            }
        }
        _ => unexpected!("parse_selectors", input_pair),
    }

    Ok(results)
}

fn parse_selector_expression(input_pair: Pair<'_, Rule>) -> ModelResult<SelectorExpression> {
    entry!("parse_selector_expression", input_pair);
    let inner: Pair<'_, Rule> = input_pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::selector_shape_types => Ok(parse_selector_shape_types(inner)?.into()),
        Rule::selector_attr => Ok(parse_selector_attr(inner)?.into()),
        Rule::selector_scoped_attr => Ok(parse_selector_scoped_attr(inner)?.into()),
        Rule::selector_forward_undirected_neighbor => {
            Ok(NeighborSelector::ForwardUndirected.into())
        }
        Rule::selector_reverse_undirected_neighbor => {
            Ok(NeighborSelector::ReverseUndirected.into())
        }
        Rule::selector_forward_directed_neighbor => Ok(NeighborSelector::ForwardDirected(
            parse_selector_directed_relationships(inner)?,
        )
        .into()),
        Rule::selector_reverse_directed_neighbor => Ok(NeighborSelector::ReverseDirected(
            parse_selector_directed_relationships(inner)?,
        )
        .into()),
        Rule::selector_forward_recursive_neighbor => {
            Ok(NeighborSelector::ForwardRecursiveDirected.into())
        }
        Rule::selector_function => Ok(parse_selector_function(inner)?.into()),
        Rule::selector_variable_set => Ok(parse_selector_variable_set(inner)?.into()),
        Rule::selector_variable_get => Ok(parse_selector_variable_get(inner)?.into()),
        _ => unexpected!("parse_selector_expression", inner),
    }
}

fn parse_selector_shape_types(input_pair: Pair<'_, Rule>) -> ModelResult<ShapeType> {
    entry!("parse_selector_shape_types", input_pair);
    match input_pair.into_inner().next() {
        None => Ok(ShapeType::Any),
        Some(inner) => match inner.as_rule() {
            Rule::identifier => Ok(ShapeType::from_str(inner.as_str())?),
            _ => unexpected!("parse_selector_shape_types", inner),
        },
    }
}

fn parse_selector_attr(input_pair: Pair<'_, Rule>) -> ModelResult<AttributeSelector> {
    entry!("parse_selector_attr", input_pair);
    let mut inner = input_pair.into_inner();
    let mut result = AttributeSelector::new(match inner.next() {
        None => expecting!("parse_selector_attr", Rule::selector_key),
        Some(inner) => match inner.as_rule() {
            Rule::selector_key => parse_selector_key(inner)?,
            _ => unexpected!("parse_selector_attr", inner),
        },
    });

    if let Some(inner) = inner.next() {
        match inner.as_rule() {
            Rule::selector_attr_comparison => {
                result.set_comparison(parse_selector_attr_comparison(inner)?)
            }
            _ => unexpected!("parse_selector_attr", inner),
        }
    }

    Ok(result)
}

fn parse_selector_attr_comparison(input_pair: Pair<'_, Rule>) -> ModelResult<AttributeComparison> {
    entry!("parse_selector_attr_comparison", input_pair);
    let mut inner = input_pair.into_inner();
    let next = inner.next().unwrap();
    let comparator = match next.as_rule() {
        Rule::selector_comparator => Comparator::from_str(next.as_str()).unwrap(),
        _ => unexpected!("parse_selector_attr_comparison", next),
    };

    let mut values: Vec<Value> = Default::default();
    let mut case_insensitive: bool = false;
    for next in inner {
        match next.as_rule() {
            Rule::selector_attr_values => {
                for inner_2 in next.into_inner() {
                    match inner_2.as_rule() {
                        Rule::selector_value => values.push(parse_selector_value(inner_2)?),
                        _ => unexpected!("parse_selector_attr_comparison", inner_2),
                    }
                }
            }
            Rule::case_comparison_suffix => {
                case_insensitive = true;
            }
            _ => unexpected!("parse_selector_attr_comparison", next),
        }
    }

    if case_insensitive {
        Ok(AttributeComparison::new_case_insensitive(
            comparator, &values,
        ))
    } else {
        Ok(AttributeComparison::new(comparator, &values))
    }
}

fn parse_selector_key(input_pair: Pair<'_, Rule>) -> ModelResult<Key> {
    entry!("parse_selector_key", input_pair);
    let mut inner = input_pair.into_inner();
    let first = inner.next().unwrap();
    let identifier = match first.as_rule() {
        Rule::identifier => Identifier::from_str(first.as_str()).unwrap(),
        _ => unexpected!("parse_selector_key", first),
    };
    Ok(Key::with_path(identifier, &parse_selector_path(inner)?))
}

fn parse_selector_path(input_pairs: Pairs<'_, Rule>) -> ModelResult<Vec<KeyPathSegment>> {
    let mut path: Vec<KeyPathSegment> = Default::default();
    for inner in input_pairs {
        match inner.as_rule() {
            Rule::selector_path => {
                for inner in inner.into_inner() {
                    path.push(parse_selector_path_segment(inner)?)
                }
            }
            _ => unexpected!("parse_selector_path", inner),
        }
    }
    Ok(path)
}

fn parse_selector_path_segment(input_pair: Pair<'_, Rule>) -> ModelResult<KeyPathSegment> {
    entry!("parse_selector_path_segment", input_pair);
    let inner = input_pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::selector_value => Ok(KeyPathSegment::Value(parse_selector_value(inner)?)),
        Rule::selector_function_property => Ok(KeyPathSegment::FunctionProperty(
            Identifier::from_str(inner.into_inner().next().unwrap().as_str()).unwrap(),
        )),
        _ => unexpected!("parse_selector_path_segment", inner),
    }
}

fn parse_selector_value(input_pair: Pair<'_, Rule>) -> ModelResult<Value> {
    entry!("parse_selector_value", input_pair);
    let inner = input_pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::selector_text => {
            let text = inner.as_str();
            Ok(Value::Text((&text[1..text.len() - 1]).to_string()))
        }
        Rule::number => {
            let number = inner.as_str();
            if number.contains('.') {
                Ok(Value::Number(Number::Float(f64::from_str(number).unwrap())))
            } else {
                Ok(Value::Number(Number::Integer(
                    i64::from_str(number).unwrap(),
                )))
            }
        }
        Rule::root_shape_id => {
            let inner = inner.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::identifier => Ok(Value::RootShapeIdentifier(
                    Identifier::from_str(inner.as_str()).unwrap(),
                )),
                Rule::absolute_root_shape_id => Ok(Value::AbsoluteRootShapeIdentifier(
                    ShapeID::from_str(inner.as_str()).unwrap(),
                )),
                _ => unexpected!("parse_selector_value", inner),
            }
        }
        _ => unexpected!("parse_selector_value", inner),
    }
}

fn parse_selector_directed_relationships(
    input_pair: Pair<'_, Rule>,
) -> ModelResult<Vec<Identifier>> {
    entry!("parse_selector_directed_relationships", input_pair);
    let mut relationships: Vec<Identifier> = Default::default();
    let outer = input_pair.into_inner().next().unwrap();
    match outer.as_rule() {
        Rule::selector_directed_relationships => {
            for inner in outer.into_inner() {
                match inner.as_rule() {
                    Rule::identifier => relationships.push(Identifier::from_str(inner.as_str())?),
                    _ => unexpected!("parse_selector_directed_relationships", inner),
                }
            }
        }
        _ => unexpected!("parse_selector_directed_relationships", outer),
    }
    Ok(relationships)
}

fn parse_selector_scoped_attr(input_pair: Pair<'_, Rule>) -> ModelResult<ScopedAttributeSelector> {
    entry!("parse_selector_scoped_attr", input_pair);
    let mut inner = input_pair.into_inner();
    let mut next = inner.next().unwrap();
    let key = if next.as_rule() == Rule::selector_key {
        let key = parse_selector_key(next)?;
        next = inner.next().unwrap();
        Some(key)
    } else {
        None
    };

    let mut assertions: Vec<ScopedAttributeAssertion> = Default::default();
    match next.as_rule() {
        Rule::selector_scoped_assertions => {
            for inner in next.into_inner() {
                match inner.as_rule() {
                    Rule::selector_scoped_assertion => {
                        assertions.push(parse_selector_scoped_assertion(inner)?);
                    }
                    _ => unexpected!("parse_selector_scoped_attr", inner),
                }
            }
        }
        _ => unexpected!("parse_selector_scoped_attr", next),
    };

    match key {
        None => Ok(ScopedAttributeSelector::new(&assertions)),
        Some(key) => Ok(ScopedAttributeSelector::with_key(key, &assertions)),
    }
}

fn parse_selector_scoped_assertion(
    input_pair: Pair<'_, Rule>,
) -> ModelResult<ScopedAttributeAssertion> {
    entry!("parse_selector_scoped_assertion", input_pair);
    let mut inner = input_pair.into_inner();
    let next = inner.next().unwrap();
    let value = match next.as_rule() {
        Rule::selector_scoped_value => parse_selector_scoped_value(next)?,
        _ => unexpected!("parse_selector_scoped_assertion-1", next),
    };

    let next = inner.next().unwrap();
    let comparator = match next.as_rule() {
        Rule::selector_comparator => Comparator::from_str(next.as_str()).unwrap(),
        _ => unexpected!("parse_selector_scoped_assertion-2", next),
    };

    let mut values: Vec<ScopedValue> = Default::default();
    let mut case_insensitive: bool = false;
    for next in inner {
        match next.as_rule() {
            Rule::selector_scoped_values => {
                for inner_2 in next.into_inner() {
                    match inner_2.as_rule() {
                        Rule::selector_scoped_value => {
                            values.push(parse_selector_scoped_value(inner_2)?)
                        }
                        _ => unexpected!("parse_selector_scoped_assertion-3", inner_2),
                    }
                }
            }
            Rule::case_comparison_suffix => {
                case_insensitive = true;
            }
            _ => unexpected!("parse_selector_scoped_assertion-4", next),
        }
    }

    if case_insensitive {
        Ok(ScopedAttributeAssertion::new_case_insensitive(
            value, comparator, &values,
        ))
    } else {
        Ok(ScopedAttributeAssertion::new(value, comparator, &values))
    }
}

fn parse_selector_scoped_value(input_pair: Pair<'_, Rule>) -> ModelResult<ScopedValue> {
    entry!("parse_selector_scoped_value", input_pair);
    let mut inner = input_pair.into_inner();
    let first = inner.next().unwrap();
    Ok(match first.as_rule() {
        Rule::selector_context_value => {
            ScopedValue::ContextValue(parse_selector_path(first.into_inner())?)
        }
        Rule::selector_value => ScopedValue::Value(parse_selector_value(first)?),
        _ => unexpected!("parse_selector_scoped_value", first),
    })
}

fn parse_selector_function(input_pair: Pair<'_, Rule>) -> ModelResult<Function> {
    entry!("parse_selector_function", input_pair);
    let mut inner = input_pair.into_inner();
    let first = inner.next().unwrap();
    let name = match first.as_rule() {
        Rule::identifier => Identifier::from_str(first.as_str())?,
        _ => unexpected!("parse_selector_function", first),
    };

    let next = inner.next().unwrap();
    let mut arguments: Vec<SelectorExpression> = Default::default();
    match next.as_rule() {
        Rule::selector_function_args => {
            for inner in next.into_inner() {
                match inner.as_rule() {
                    Rule::selector_expression => arguments.push(parse_selector_expression(inner)?),
                    _ => unexpected!("parse_selector_function", inner),
                }
            }
        }
        _ => unexpected!("parse_selector_function", next),
    };

    Ok(Function::new(name, &arguments))
}

fn parse_selector_variable_set(input_pair: Pair<'_, Rule>) -> ModelResult<VariableDefinition> {
    entry!("parse_selector_variable_set", input_pair);
    let mut outer = input_pair.into_inner();
    let first = outer.next().unwrap();
    let name = match first.as_rule() {
        Rule::identifier => Identifier::from_str(first.as_str())?,
        _ => unexpected!("parse_selector_variable_set", first),
    };

    let mut arguments: Vec<SelectorExpression> = Default::default();
    for inner in outer {
        match inner.as_rule() {
            Rule::selector_expression => arguments.push(parse_selector_expression(inner)?),
            _ => unexpected!("parse_selector_variable_set", inner),
        }
    }

    Ok(VariableDefinition::new(name, &arguments))
}

fn parse_selector_variable_get(input_pair: Pair<'_, Rule>) -> ModelResult<VariableReference> {
    entry!("parse_selector_variable_get", input_pair);
    let mut inner = input_pair.into_inner();
    let first = inner.next().unwrap();
    match first.as_rule() {
        Rule::identifier => Ok(VariableReference::new(Identifier::from_str(
            first.as_str(),
        )?)),
        _ => unexpected!("parse_selector_function", first),
    }
}

fn from_pest_error(e: PestError<Rule>) -> Error {
    error!("{}", e);
    Error::with_chain(
        e,
        ErrorKind::Deserialization("Smithy".to_string(), "pest".to_string(), None),
    )
}
