use swc_core::ecma::ast::Lit;

pub fn get_lit_value(lit: &Lit) -> String {
    let str = match lit {
        Lit::Null(_null) => "null".to_string(),
        Lit::Regex(regex) => regex.exp.to_string(),
        Lit::JSXText(jsx_text) => jsx_text.value.to_string(),
        Lit::Str(str) => str.value.to_string(),
        Lit::Bool(bool) => bool.value.to_string(),
        Lit::BigInt(big_int) => big_int.value.to_string(),
        Lit::Num(num) => num.value.to_string(),
    };

    str
}
