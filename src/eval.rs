use swc_core::ecma::ast::*;

/// A subset of Babel's `path.evaluate()`. `None` stands for Babel's `confident: false`.
///
/// ponytail: literals, template literals without substitutions, operators over those, and
/// identifiers resolved through a never-reassigned binding — which is how Babel folds
/// `let value = "World"` into the template. Babel goes further (`Math.*`, `String()`, array
/// members); widen this if a real source turns up that regresses.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Undefined,
    /// An object or array literal: confidently evaluable, but not a primitive.
    Object,
}

impl Value {
    pub(crate) fn as_static_text(&self) -> Option<String> {
        match self {
            Value::Str(s) => Some(s.clone()),
            Value::Num(n) => Some(number_to_string(*n)),
            _ => None,
        }
    }

    pub(crate) fn truthy(&self) -> bool {
        match self {
            Value::Str(s) => !s.is_empty(),
            Value::Num(n) => *n != 0.0 && !n.is_nan(),
            Value::Bool(b) => *b,
            Value::Null | Value::Undefined => false,
            Value::Object => true,
        }
    }
}

/// JavaScript's `String(number)` for the integral and simple cases the templates need.
pub(crate) fn number_to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".into();
    }
    if n.is_infinite() {
        return if n > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    if n == n.trunc() && n.abs() < 1e21 {
        let mut s = format!("{}", n as i64);
        if n == 0.0 && n.is_sign_negative() {
            s = "0".into();
        }
        return s;
    }
    let mut s = format!("{n}");
    if s.ends_with(".0") {
        s.truncate(s.len() - 2);
    }
    s
}

pub(crate) fn evaluate(expr: &Expr, bindings: &crate::scope::Bindings) -> Option<Value> {
    evaluate_at(expr, bindings, 0)
}

/// Guards against `let a = a` and friends resolving forever.
const MAX_DEPTH: u32 = 16;

fn evaluate_at(expr: &Expr, bindings: &crate::scope::Bindings, depth: u32) -> Option<Value> {
    if depth > MAX_DEPTH {
        return None;
    }
    match expr {
        Expr::Paren(paren) => evaluate_at(&paren.expr, bindings, depth + 1),
        Expr::Lit(lit) => match lit {
            Lit::Str(s) => Some(Value::Str(s.value.to_atom_lossy().to_string())),
            Lit::Num(n) => Some(Value::Num(n.value)),
            Lit::Bool(b) => Some(Value::Bool(b.value)),
            Lit::Null(_) => Some(Value::Null),
            Lit::BigInt(_) | Lit::Regex(_) | Lit::JSXText(_) => None,
        },
        // Babel's evaluateQuasis: a template folds when every substitution folds.
        Expr::Tpl(tpl) => {
            let mut out = String::new();
            for (index, quasi) in tpl.quasis.iter().enumerate() {
                out.push_str(&quasi.cooked.as_ref()?.to_atom_lossy());
                if let Some(expr) = tpl.exprs.get(index) {
                    out.push_str(&to_string(&evaluate_at(expr, bindings, depth + 1)?)?);
                }
            }
            Some(Value::Str(out))
        }
        Expr::Ident(ident) => match &*ident.sym {
            "undefined" => Some(Value::Undefined),
            "NaN" => Some(Value::Num(f64::NAN)),
            "Infinity" => Some(Value::Num(f64::INFINITY)),
            _ => {
                let binding = bindings.get(ident)?;
                // Babel deopts on reassignment and on a reference that precedes the end of the
                // declaration.
                if binding.reassigned || ident.span.lo < binding.decl_end {
                    return None;
                }
                let value = evaluate_at(binding.init.as_ref()?, bindings, depth + 1)?;
                // Determined by experiment against Babel: a primitive folds through however
                // many references, but an object or array only through a single one — with
                // more than one use it cannot tell whether the value was mutated in between.
                if matches!(value, Value::Object) && binding.references > 1 {
                    return None;
                }
                Some(value)
            }
        },
        Expr::Unary(unary) => {
            let arg = evaluate_at(&unary.arg, bindings, depth + 1)?;
            match unary.op {
                UnaryOp::Minus => Some(Value::Num(-to_number(&arg)?)),
                UnaryOp::Plus => Some(Value::Num(to_number(&arg)?)),
                UnaryOp::Bang => Some(Value::Bool(!arg.truthy())),
                UnaryOp::Void => Some(Value::Undefined),
                UnaryOp::TypeOf => Some(Value::Str(
                    match arg {
                        Value::Str(_) => "string",
                        Value::Num(_) => "number",
                        Value::Bool(_) => "boolean",
                        Value::Undefined => "undefined",
                        Value::Null | Value::Object => "object",
                    }
                    .into(),
                )),
                UnaryOp::Tilde => Some(Value::Num(!(to_number(&arg)? as i32) as f64)),
                UnaryOp::Delete => None,
            }
        }
        // Babel short-circuits these: `"Hi" || x` folds even when `x` does not.
        Expr::Bin(bin) if matches!(bin.op, BinaryOp::LogicalOr | BinaryOp::LogicalAnd) => {
            let left = evaluate_at(&bin.left, bindings, depth + 1)?;
            let right = evaluate_at(&bin.right, bindings, depth + 1);
            let decided = if bin.op == BinaryOp::LogicalOr {
                left.truthy()
            } else {
                !left.truthy()
            };
            if !decided && right.is_none() {
                return None;
            }
            Some(if decided { left } else { right? })
        }
        Expr::Bin(bin) => {
            let left = evaluate_at(&bin.left, bindings, depth + 1)?;
            let right = evaluate_at(&bin.right, bindings, depth + 1)?;
            match bin.op {
                BinaryOp::Add => match (&left, &right) {
                    (Value::Str(_), _) | (_, Value::Str(_)) => {
                        Some(Value::Str(to_string(&left)? + &to_string(&right)?))
                    }
                    _ => Some(Value::Num(to_number(&left)? + to_number(&right)?)),
                },
                BinaryOp::Sub => Some(Value::Num(to_number(&left)? - to_number(&right)?)),
                BinaryOp::Mul => Some(Value::Num(to_number(&left)? * to_number(&right)?)),
                BinaryOp::Div => Some(Value::Num(to_number(&left)? / to_number(&right)?)),
                BinaryOp::Mod => Some(Value::Num(to_number(&left)? % to_number(&right)?)),
                BinaryOp::EqEqEq => Some(Value::Bool(left == right)),
                BinaryOp::NotEqEq => Some(Value::Bool(left != right)),
                // Loose equality over primitives: same type compares as strict, null and
                // undefined are equal to each other only, anything else compares as numbers.
                BinaryOp::EqEq | BinaryOp::NotEq => {
                    let equal = match (&left, &right) {
                        (Value::Object, _) | (_, Value::Object) => return None,
                        (Value::Null | Value::Undefined, Value::Null | Value::Undefined) => true,
                        (Value::Null | Value::Undefined, _)
                        | (_, Value::Null | Value::Undefined) => false,
                        (Value::Str(a), Value::Str(b)) => a == b,
                        _ => to_number(&left)? == to_number(&right)?,
                    };
                    Some(Value::Bool(if bin.op == BinaryOp::EqEq {
                        equal
                    } else {
                        !equal
                    }))
                }
                BinaryOp::Lt | BinaryOp::LtEq | BinaryOp::Gt | BinaryOp::GtEq => {
                    let ordering = match (&left, &right) {
                        (Value::Str(a), Value::Str(b)) => a.partial_cmp(b),
                        _ => to_number(&left)?.partial_cmp(&to_number(&right)?),
                    };
                    let Some(ordering) = ordering else {
                        return Some(Value::Bool(false)); // NaN compares false either way
                    };
                    Some(Value::Bool(match bin.op {
                        BinaryOp::Lt => ordering.is_lt(),
                        BinaryOp::LtEq => ordering.is_le(),
                        BinaryOp::Gt => ordering.is_gt(),
                        _ => ordering.is_ge(),
                    }))
                }
                BinaryOp::Exp => Some(Value::Num(to_number(&left)?.powf(to_number(&right)?))),
                BinaryOp::BitAnd => Some(Value::Num((to_int32(&left)? & to_int32(&right)?) as f64)),
                BinaryOp::BitOr => Some(Value::Num((to_int32(&left)? | to_int32(&right)?) as f64)),
                BinaryOp::BitXor => Some(Value::Num((to_int32(&left)? ^ to_int32(&right)?) as f64)),
                BinaryOp::LShift => Some(Value::Num(
                    ((to_int32(&left)?) << (to_uint32(&right)? & 31)) as f64,
                )),
                BinaryOp::RShift => Some(Value::Num(
                    ((to_int32(&left)?) >> (to_uint32(&right)? & 31)) as f64,
                )),
                BinaryOp::ZeroFillRShift => Some(Value::Num(
                    ((to_uint32(&left)?) >> (to_uint32(&right)? & 31)) as f64,
                )),
                _ => None,
            }
        }
        // Babel calls side-effect-free methods on string and number literals. Only the ones
        // whose result does not depend on UTF-16 index or length semantics are folded here:
        // getting `slice` or `padStart` subtly wrong would bake a *wrong* string into the
        // template, which is worse than leaving the expression dynamic.
        Expr::Call(call) => {
            let Callee::Expr(callee) = &call.callee else {
                return None;
            };
            let Expr::Member(member) = &**callee else {
                return None;
            };
            let MemberProp::Ident(method) = &member.prop else {
                return None;
            };
            let receiver = evaluate_at(&member.obj, bindings, depth + 1)?;
            let args = call
                .args
                .iter()
                .map(|arg| {
                    if arg.spread.is_some() {
                        return None;
                    }
                    evaluate_at(&arg.expr, bindings, depth + 1)
                })
                .collect::<Option<Vec<_>>>()?;
            let text = match &receiver {
                Value::Str(s) => s.clone(),
                Value::Num(n) if &*method.sym == "toString" && args.is_empty() => {
                    return Some(Value::Str(number_to_string(*n)));
                }
                _ => return None,
            };
            match (&*method.sym, args.as_slice()) {
                ("repeat", [count]) => {
                    let count = to_number(count)?;
                    // A negative or non-integral count throws in JS; leave it dynamic.
                    if count < 0.0 || count.fract() != 0.0 || count > 10_000.0 {
                        return None;
                    }
                    Some(Value::Str(text.repeat(count as usize)))
                }
                ("toUpperCase", []) => Some(Value::Str(text.to_uppercase())),
                ("toLowerCase", []) => Some(Value::Str(text.to_lowercase())),
                ("trim", []) => Some(Value::Str(text.trim().to_string())),
                ("trimStart", []) => Some(Value::Str(text.trim_start().to_string())),
                ("trimEnd", []) => Some(Value::Str(text.trim_end().to_string())),
                ("concat", rest) => {
                    let mut out = text;
                    for value in rest {
                        out.push_str(&to_string(value)?);
                    }
                    Some(Value::Str(out))
                }
                _ => None,
            }
        }
        Expr::Cond(cond) => {
            if evaluate_at(&cond.test, bindings, depth + 1)?.truthy() {
                evaluate_at(&cond.cons, bindings, depth + 1)
            } else {
                evaluate_at(&cond.alt, bindings, depth + 1)
            }
        }
        Expr::Array(array) => array
            .elems
            .iter()
            .all(|elem| {
                elem.as_ref().is_none_or(|e| {
                    e.spread.is_none() && evaluate_at(&e.expr, bindings, depth + 1).is_some()
                })
            })
            .then_some(Value::Object),
        // Babel evaluates a computed key rather than giving up on it, so `{['a-b']: v}` is
        // just as constant as `{'a-b': v}`. Only a key it cannot evaluate deopts the object.
        Expr::Object(object) => object
            .props
            .iter()
            .all(|prop| match prop {
                PropOrSpread::Spread(_) => false,
                PropOrSpread::Prop(prop) => match &**prop {
                    Prop::KeyValue(kv) => {
                        let key_ok = match &kv.key {
                            PropName::Computed(computed) => {
                                evaluate_at(&computed.expr, bindings, depth + 1).is_some()
                            }
                            _ => true,
                        };
                        key_ok && evaluate_at(&kv.value, bindings, depth + 1).is_some()
                    }
                    // Babel's parser has no shorthand node: `{x}` is a key/value property
                    // whose value is the identifier, and it evaluates like one.
                    Prop::Shorthand(ident) => {
                        evaluate_at(&Expr::Ident(ident.clone()), bindings, depth + 1).is_some()
                    }
                    // An accessor or method deopts, as `isObjectMethod` does upstream.
                    _ => false,
                },
            })
            .then_some(Value::Object),
        _ => None,
    }
}

fn to_number(value: &Value) -> Option<f64> {
    match value {
        Value::Num(n) => Some(*n),
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        Value::Null => Some(0.0),
        Value::Undefined => Some(f64::NAN),
        Value::Str(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                Some(0.0)
            } else {
                trimmed.parse().ok()
            }
        }
        Value::Object => None,
    }
}

/// JavaScript's ToInt32 / ToUint32 for the bitwise operators.
fn to_int32(value: &Value) -> Option<i32> {
    Some(to_uint32(value)? as i32)
}

fn to_uint32(value: &Value) -> Option<u32> {
    let n = to_number(value)?;
    if !n.is_finite() {
        return Some(0);
    }
    Some(n.trunc().rem_euclid(4294967296.0) as u32)
}

fn to_string(value: &Value) -> Option<String> {
    match value {
        Value::Str(s) => Some(s.clone()),
        Value::Num(n) => Some(number_to_string(*n)),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => Some("null".into()),
        Value::Undefined => Some("undefined".into()),
        Value::Object => None,
    }
}
