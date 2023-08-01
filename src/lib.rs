#![allow(unused_macros,unused_imports,non_snake_case)]
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};
/// This macro can be used as a header for competitive programming
macro_rules! header {
    () => {
#![allow(unused_macros,unused_imports,non_snake_case)]
use std::cmp::*;
use std::{
    collections::{HashMap,VecDeque,BTreeMap,BTreeSet},
    iter::{Chain,Once,Repeat},
    fmt::Debug
};
use rust_template_macro::FromInput;
use std::{convert::TryInto, str::FromStr};
extern "C" {
    fn getchar() -> u8;
}
trait FromInput
where
    Self: Sized,
{
    fn from_input() -> Self;

    fn input_iter(n: usize) -> std::iter::Map<std::ops::Range<usize>, fn(usize) -> Self> {
        return (0..n).map(|_| Self::from_input());
    }
}

impl<T: FromStr> FromInput for T {
    fn from_input() -> Self {
        let mut x = String::new();
        loop {
            let token = unsafe { getchar() };
            if token == 32 || token == 10 {
                break;
            }
            x.push(token as char);
        }
        if let Ok(x) = x.parse::<T>() {
            return x;
        }
        panic!("parse failed")
    }
}
trait ExtraVec<T: Copy> {
    fn gi<I: TryInto<usize>>(&self, index: I) -> Option<T>;
    fn g(&self, index: usize) -> Option<T>;
}
impl<T: Copy> ExtraVec<T> for Vec<T> {
    fn gi<I: TryInto<usize>>(&self, index: I) -> Option<T> {
        return index.try_into().ok().and_then(|x| self.get(x).cloned());
    }
    fn g(&self, index: usize) -> Option<T> {
        return self.get(index).cloned();
    }
}
impl<T: Copy> ExtraVec<T> for [T] {
    fn g(&self, index: usize) -> Option<T> {
        self.get(index).cloned()
    }
    fn gi<I: TryInto<usize>>(&self, index: I) -> Option<T> {
        index.try_into().ok().and_then(|x| self.get(x).cloned())
    }
}
trait ExtraOption<T> {
    fn uo(self, default: T) -> T;
    fn ue(self, f: impl Fn() -> T) -> T;
    fn u(self) -> T;
}
impl<T> ExtraOption<T> for Option<T> {
    fn u(self) -> T {
        self.unwrap()
    }
    fn uo(self, default: T) -> T {
        self.unwrap_or(default)
    }
    fn ue(self, f: impl Fn() -> T) -> T {
        self.unwrap_or_else(|| f())
    }
}
trait ExtraUsize {
    fn cs(self, rhs: usize) -> Option<usize>;
    fn sa(self, rhs: usize) -> usize;
    fn ss(self, rhs: usize) -> usize;
}
impl ExtraUsize for usize {
    fn cs(self, rhs: usize) -> Option<usize> {
        self.checked_sub(rhs)
    }
    fn sa(self, rhs: usize) -> usize {
        self.saturating_add(rhs)
    }
    fn ss(self, rhs: usize) -> usize {
        self.saturating_sub(rhs)
    }
}
macro_rules! product {
    ($first:expr, $($next:expr),*) => (
        $first$(
            .flat_map(move |e| std::iter::repeat(e)
                .zip($next.clone()))
        )*
    );
}
        
    };
}

///Implementation for from_derive 
#[proc_macro_derive(FromInput)]
pub fn from_input_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    let struct_name = &ast.ident;

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &ast.data
    {
        &fields.named
    } else {
        panic!("`FromInput` can only be derived for named structs.");
    };

    // Generate the field parsing code for each field
    let field_parsing = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Named field expected");
        let field_type = &field.ty;
        quote! {
            #field_name: #field_type::from_input(),
        }
    });

    // Generate the implementation code
    let expanded = quote! {
        impl FromInput for #struct_name {
            fn from_input() -> Self {
                #struct_name {
                    #(#field_parsing)*
                }
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}
