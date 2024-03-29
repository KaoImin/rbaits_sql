use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{BinOp, Expr, ItemFn, Lit, Member};

use crate::proc_macro::TokenStream;

// fn is_name_char(arg: char) -> bool {
//     match arg {
//         '.' |
//         '_' |
//         '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
//         'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
//         'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
//         => {
//             return true;
//         }
//         _ => {}
//     }
//     return false;
// }

fn is_param_char(arg: char) -> bool {
    match arg {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
        => {
            return true;
        }
        _ => {}
    }
    return false;
}


fn token_steam_string(arg: proc_macro2::TokenStream) -> String {
    arg.to_token_stream().to_string().trim().to_string()
}


fn convert_to_arg_access(context: &str, arg: Expr, as_proxy: bool, ignore: &[&str]) -> Expr {
    match arg {
        Expr::Path(b) => {
            let token = b.to_token_stream().to_string();
            if token == "null" {
                return syn::parse_str::<Expr>("serde_json::Value::Null").unwrap();
            }
            if token == "sql" {
                return Expr::Path(b);
            }
            if token == "arg" {
                return Expr::Path(b);
            }
            if token == "args" {
                return Expr::Path(b);
            }
            let param = token_steam_string(b.to_token_stream());
            let mut fetch_from_arg = true;
            for x in ignore {
                if param.eq(x) {
                    fetch_from_arg = false;
                }
            }
            if fetch_from_arg {
                return syn::parse_str::<Expr>(&format!(" &arg[\"{}\"]", param)).unwrap();
            } else {
                return syn::parse_str::<Expr>(&format!("{}", param)).unwrap();
            }
        }
        Expr::MethodCall(mut b) => {
            let ex = *(b.receiver.clone());
            let s = ex.to_token_stream().to_string();
            for x in s.chars() {
                if is_param_char(x) {
                    b.receiver = Box::new(convert_to_arg_access(context, *b.receiver.clone(), as_proxy, ignore));
                    return Expr::MethodCall(b);
                }
                break;
            }
            return Expr::MethodCall(b);
        }
        Expr::Binary(mut b) => {
            b.left = Box::new(convert_to_arg_access(context, *b.left.clone(), as_proxy, ignore));
            b.right = Box::new(convert_to_arg_access(context, *b.right.clone(), as_proxy, ignore));
            match b.op {
                BinOp::Add(_) => {
                    let left_token = b.left.to_token_stream().to_string();
                    let right_token = b.right.to_token_stream().to_string();
                    if left_token.trim().ends_with("\"") && left_token.trim().starts_with("\"") {
                        return syn::parse_str::<Expr>(&format!("(String::from({})).op_add({})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                    }else{
                        return syn::parse_str::<Expr>(&format!("({}).op_add(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                    }
                }
                BinOp::And(_) => {
                    b.left = Box::new(syn::parse_str::<Expr>(&format!("bool::op_from({})", b.left.to_token_stream().to_string().trim())).unwrap());
                    b.right = Box::new(syn::parse_str::<Expr>(&format!("bool::op_from({})", b.right.to_token_stream().to_string().trim())).unwrap());
                }
                BinOp::Or(_) => {
                    b.left = Box::new(syn::parse_str::<Expr>(&format!("bool::op_from({})", b.left.to_token_stream().to_string().trim())).unwrap());
                    b.right = Box::new(syn::parse_str::<Expr>(&format!("bool::op_from({})", b.right.to_token_stream().to_string().trim())).unwrap());
                }
                
                /// The `-` operator (subtraction)
                BinOp::Sub(_) => {
                  return syn::parse_str::<Expr>(&format!("({}).op_sub(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `*` operator (multiplication)
                BinOp::Mul(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_mul(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `/` operator (division)
                BinOp::Div(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_div(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `%` operator (modulus)
                BinOp::Rem(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_rem(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `&` operator (bitwise and)
                BinOp::BitAnd(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_bitand(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `|` operator (bitwise or)
                BinOp:: BitOr(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_bitor(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `==` operator (equality)
                BinOp:: Eq(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_eq(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `<` operator (less than)
                BinOp:: Lt(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_lt(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `<=` operator (less than or equal to)
                BinOp:: Le(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_le(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `!=` operator (not equal to)
                BinOp:: Ne(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_ne(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `>=` operator (greater than or equal to)
                BinOp:: Ge(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_ge(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `>` operator (greater than)
                BinOp:: Gt(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_gt(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `^` operator (bitwise xor)
                BinOp:: BitXor(_)=> {
                    return syn::parse_str::<Expr>(&format!("({}).op_bitxor(&{})",b.left.to_token_stream(),b.right.to_token_stream())).unwrap();
                }
                /// The `<<` operator (shift left)
                BinOp::  Shl(_)=> {panic!("unsupported token << ")}
                /// The `>>` operator (shift right)
                BinOp:: Shr(_)=> {panic!("unsupported token >> ")}
                /// The `+=` operator
                BinOp:: AddEq(_)=> {panic!("unsupported token += ")}
                /// The `-=` operator
                BinOp:: SubEq(_)=> {panic!("unsupported token -= ")}
                /// The `*=` operator
                BinOp:: MulEq(_)=> {panic!("unsupported token *= ")}
                /// The `/=` operator
                BinOp:: DivEq(_)=> {panic!("unsupported token /= ")}
                /// The `%=` operator
                BinOp:: RemEq(_)=> {panic!("unsupported token %= ")}
                /// The `^=` operator
                BinOp:: BitXorEq(_)=> {panic!("unsupported token ^= ")}
                /// The `&=` operator
                BinOp:: BitAndEq(_)=> {panic!("unsupported token &= ")}
                /// The `|=` operator
                BinOp::BitOrEq(_)=> {panic!("unsupported token |= ")}
                /// The `<<=` operator
                BinOp:: ShlEq(_)=> {panic!("unsupported token <<= ")}
                /// The `>>=` operator
                BinOp:: ShrEq(_)=> {panic!("unsupported token >>= ")}
            }
            return Expr::Binary(b);
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, as_proxy, ignore));
            if b.op.to_token_stream().to_string().trim() == "-" {
                return syn::parse_str::<Expr>(&format!(" (0.op_sub(&{}))", b.expr.to_token_stream().to_string().trim())).unwrap();
            }
            return Expr::Unary(b);
        }
        Expr::Paren(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, as_proxy, ignore));
            return Expr::Paren(b);
        }
        Expr::Field(mut b) => {
            b.base = Box::new(convert_to_arg_access(context, *b.base.clone(), as_proxy, ignore));
            return match b.member.clone() {
                Member::Named(n) => {
                    let s = b.member.to_token_stream().to_string();
                    let vs: Vec<&str> = s.split(".").collect();
                    let mut token = String::new();
                    for x in vs {
                        if x.ends_with("()") {
                            token.push_str(".");
                            token.push_str(x.trim());
                        } else {
                            let x = x.trim();
                            //format index
                            let xs: Vec<&str> = x.split("[").collect();
                            if xs.len() > 1 {
                                let mut is_first = true;
                                for x in xs {
                                    if is_first {
                                        token.push_str("[\"");
                                        token.push_str(x.trim());
                                        token.push_str("\"]");
                                    } else {
                                        token.push_str("[");
                                        token.push_str(x.trim());
                                    }
                                    is_first = false;
                                }
                            } else {
                                token.push_str("[\"");
                                token.push_str(x.trim());
                                token.push_str("\"]");
                            }
                        }
                    }
                    syn::parse_str::<Expr>(&format!("{}{}", b.base.to_token_stream(), token)).unwrap()
                }
                Member::Unnamed(unamed) => {
                    Expr::Field(b)
                }
            };
        }
        Expr::Reference(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, as_proxy, ignore));
            let result = Expr::Reference(b);
            return result;
        }
        Expr::Index(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr, as_proxy, ignore));
            let result = Expr::Index(b);
            return syn::parse_str::<Expr>(&format!("{}", result.to_token_stream().to_string())).unwrap();
        }
        Expr::Let(mut let_expr) => {
            let_expr.expr = Box::new(convert_to_arg_access(context, *let_expr.expr, as_proxy, ignore));
            return Expr::Let(let_expr);
        }
        Expr::Lit(b) => {
            match b.lit.clone() {
                Lit::Str(_) => {}
                Lit::ByteStr(_) => {}
                Lit::Byte(_) => {}
                Lit::Char(_) => {}
                Lit::Int(i) => {
                    //cast int to i64
                    return syn::parse_str::<Expr>(&format!("{}i64", i)).unwrap();
                }
                Lit::Float(f) => {
                    //cast int to f64
                    return syn::parse_str::<Expr>(&format!("{}f64", f)).unwrap();
                }
                Lit::Bool(_) => {}
                Lit::Verbatim(_) => {}
            }
            return Expr::Lit(b);
        }
        _ => {
            println!("_def:{:?}", expr_type(arg.clone()));
            return arg;
        }
    }
}


fn expr_type(expr: Expr) -> String {
    match expr {
        Expr::Array(_) => { format!("Array") }
        Expr::Assign(_) => { format!("Assign") }
        Expr::AssignOp(_) => { format!("AssignOp") }
        Expr::Async(_) => { format!("Async") }
        Expr::Await(_) => { format!("Await") }
        Expr::Binary(_) => { format!("Binary") }
        Expr::Block(_) => { format!("Block") }
        Expr::Box(_) => { format!("Box") }
        Expr::Break(_) => { format!("Break") }
        Expr::Call(_) => { format!("Call") }
        Expr::Cast(_) => { format!("Cast") }
        Expr::Closure(_) => { format!("Closure") }
        Expr::Continue(_) => { format!("Continue") }
        Expr::Field(_) => { format!("Field") }
        Expr::ForLoop(_) => { format!("ForLoop") }
        Expr::Group(_) => { format!("Group") }
        Expr::If(_) => { format!("If") }
        Expr::Index(_) => { format!("Index") }
        Expr::Let(_) => { format!("Let") }
        Expr::Lit(_) => { format!("Lit") }
        Expr::Loop(_) => { format!("Loop") }
        Expr::Macro(_) => { format!("Macro") }
        Expr::Match(_) => { format!("Match") }
        Expr::MethodCall(_) => { format!("MethodCall") }
        Expr::Paren(_) => { format!("Paren") }
        Expr::Path(_) => { format!("Path") }
        Expr::Range(_) => { format!("Range") }
        Expr::Reference(_) => { format!("Reference") }
        Expr::Repeat(_) => { format!("Repeat") }
        Expr::Return(_) => { format!("Return") }
        Expr::Struct(_) => { format!("Struct") }
        Expr::Try(_) => { format!("Try") }
        Expr::TryBlock(_) => { format!("TryBlock") }
        Expr::Tuple(_) => { format!("Tuple") }
        Expr::Type(_) => { format!("Type") }
        Expr::Unary(_) => { format!("Unary") }
        Expr::Unsafe(_) => { format!("Unsafe") }
        Expr::Verbatim(_) => { format!("Verbatim") }
        Expr::While(_) => { format!("While") }
        Expr::Yield(_) => { format!("Yield") }
        Expr::__TestExhaustive(_) => { format!("__TestExhaustive") }
    }
}


pub fn impl_fn(context: &str, func_name_ident: &str, args: &str, serialize_result: bool, as_proxy: bool, ignore: &[&str]) -> proc_macro2::TokenStream {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();
    string_data = string_data.replace(".string()", ".to_string()");
    //convert string define
    let mut last_char = '_';
    let mut string_data_new = String::new();
    for x in string_data.chars() {
        if x == '\'' && last_char != '\\' {
            string_data_new.push('\"');
        } else {
            string_data_new.push(x);
        }
        last_char = x;
    }
    string_data = string_data_new;
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str: {} fail for: {}", string_data, t.err().unwrap().to_string())
    }
    let mut t = t.unwrap();
    t = convert_to_arg_access(context, t, as_proxy, ignore);
    string_data = t.to_token_stream().to_string();
    string_data = string_data.replace(" . ", ".");
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str: {} fail for: {}", string_data, t.err().unwrap().to_string())
    }
    let t = t.unwrap();
    let mut result_impl = quote! { result };
    if serialize_result {
        result_impl = quote! {serde_json::json!(result)};
    }
    if func_name_ident.is_empty() || func_name_ident.eq("\"\"") {
        return quote! {
         {
           let result={#t};
           #result_impl
        }
    }.to_token_stream();
    } else {
        let func_name_ident = Ident::new(&func_name_ident.to_string(), Span::call_site());
        return quote! {
        pub fn #func_name_ident(arg:&serde_json::Value) -> serde_json::Value {
           let result={#t};
           #result_impl
        }
    }.to_token_stream();
    }
}