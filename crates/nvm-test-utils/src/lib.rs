use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, ExprAssign, ExprBinary, ExprLit, ItemFn, Lit};

#[proc_macro_attribute]
pub fn machine_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = syn::parse::<ItemFn>(input).unwrap();
    let fn_name = &input_fn.sig.ident;
    let inner_name = syn::Ident::new(&format!("__inner_{}", fn_name), fn_name.span());

    let init_states: Vec<ExprAssign> = input_fn.attrs.iter()
        .filter(|attr| attr.path().is_ident("machine_state"))
        .map(|attr| attr.parse_args::<ExprAssign>().unwrap())
        .collect();

    let init_code = init_states.iter().map(|assign| {
        let left = &assign.left;
        let right = &assign.right;
        match &**left {
            Expr::Path(expr_path) => {
                let segments = &expr_path.path.segments;
                if segments.len() == 2 && segments[0].ident == "Register" {
                    let variant = &segments[1].ident;

                    quote! {
                        machine.set_register(Register::#variant, #right);
                    }
                } else {
                    panic!("Invalid path: {}", expr_path.into_token_stream());
                }
            },
            Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) => {
                quote! {
                    machine.memory_mut().data[(#int_lit) as usize] = #right;
                }
            },
            Expr::Binary(ExprBinary { left: left_operand, op: operation, right: right_operand, .. }) => {
                quote! {
                    machine.memory_mut().data[(#left_operand #operation #right_operand) as usize] = #right;
                }
            }
            _ => {
                panic!("Invalid left expression: {}", left.into_token_stream());
            }
        }
    });

    let mut inner_fn = input_fn.clone();
    inner_fn.sig.ident = inner_name.clone();
    inner_fn.attrs.clear();

    let output = quote! {
        #inner_fn

        #[test]
        fn #fn_name() {
            let mut machine = Machine::default();
            #(#init_code)*
            #inner_name(machine)
        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn machine_state(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}