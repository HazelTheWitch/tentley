use proc_macro::TokenStream;

use quote::TokenStreamExt;

use proc_macro2::{Delimiter, Spacing, TokenTree};
use proc_macro2::{Group, Punct};

use quote::quote;

use syn::{
    parse::{Error, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Result, Token, Type,
};

struct Matrix {
    rows: Vec<Vec<Expr>>,
    cols: usize,
    ty: Option<Type>,
}

impl Matrix {
    fn rows(&self) -> usize {
        self.rows.len()
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();

        for row in self.rows.iter() {
            let mut row_tokens = proc_macro2::TokenStream::new();

            if let Some(ty) = &self.ty {
                row_tokens.append_separated(
                    row.into_iter()
                        .map(|element| proc_macro2::TokenStream::from(quote! { #element as #ty })),
                    Punct::new(',', Spacing::Alone)
                );
            } else {
                row_tokens.append_separated(row.into_iter(), Punct::new(',', Spacing::Alone));
            }

            tokens.append(Group::new(Delimiter::Bracket, row_tokens));
            tokens.append(Punct::new(',', Spacing::Alone));
        }

        proc_macro2::TokenStream::from(TokenTree::Group(Group::new(Delimiter::Bracket, tokens)))
    }
}

type MatrixRow = Punctuated<Expr, Token![,]>;

impl Parse for Matrix {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut rows = Vec::new();
        let mut cols = None;

        let mut ty = None;
        let mut needs_ty = true;

        while !input.is_empty() {
            let span = input.span();
            if !needs_ty {
                let row = MatrixRow::parse_separated_nonempty(input)?;

                if let Some(cols) = cols {
                    if row.len() != cols {
                        let row_index = rows.len();

                        let error_message = format!(
                            "Unexpected number of elements in row: {}. Expected {}. Found {}",
                            row_index,
                            cols,
                            row.len()
                        );

                        return Err(Error::new(span, error_message));
                    }
                } else {
                    cols = Some(row.len());
                }

                rows.push(row.into_iter().collect());

                if !input.is_empty() {
                    input.parse::<Token![;]>()?;
                }
            } else {
                let parsed_ty = input.parse::<Type>();

                if let Ok(parsed_ty) = parsed_ty {
                    input.parse::<Token![;]>()?;

                    ty = Some(parsed_ty);
                }

                needs_ty = false;
            }
        }

        Ok(Matrix {
            rows,
            cols: cols.unwrap_or(0),
            ty,
        })
    }
}

struct Vector {
    data: Vec<Expr>,
    ty: Option<Type>,
}

impl Vector {
    fn as_row_vector(self) -> Matrix {
        let cols = self.data.len();

        Matrix {
            rows: vec![self.data],
            cols,
            ty: self.ty,
        }
    }

    fn as_column_vector(self) -> Matrix {
        Matrix {
            rows: self.data.into_iter().map(|e| vec![e]).collect(),
            cols: 1,
            ty: self.ty,
        }
    }
}

impl Parse for Vector {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ty = None;
        let parsed_ty = input.parse::<Type>();

        if let Ok(parsed_ty) = parsed_ty {
            input.parse::<Token![;]>()?;

            ty = Some(parsed_ty);
        }
        
        let data = MatrixRow::parse_separated_nonempty(input)?;

        Ok(Vector {
            data: data.into_iter().collect(),
            ty,
        })
    }
}

#[proc_macro]
pub fn mat(stream: TokenStream) -> TokenStream {
    let matrix = parse_macro_input!(stream as Matrix);

    let rows = matrix.rows();
    let cols = matrix.cols();

    let tokens = matrix.to_tokens();

    TokenStream::from(quote! {
       tentley::prelude::Matrix::<_, #rows, #cols>::new(#tokens)
    })
}

#[proc_macro]
pub fn vector(stream: TokenStream) -> TokenStream {
    let vector = parse_macro_input!(stream as Vector);

    let matrix = vector.as_column_vector();

    let rows = matrix.rows();
    let cols = matrix.cols();

    let tokens = matrix.to_tokens();

    TokenStream::from(quote! {
       tentley::prelude::Matrix::<_, #rows, #cols>::new(#tokens)
    })
}

#[proc_macro]
pub fn row_vector(stream: TokenStream) -> TokenStream {
    let vector = parse_macro_input!(stream as Vector);

    let matrix = vector.as_row_vector();

    let rows = matrix.rows();
    let cols = matrix.cols();
    let ty = match &matrix.ty {
        Some(ty) => quote! { #ty },
        None => quote! { _ }
    };

    let tokens = matrix.to_tokens();

    TokenStream::from(quote! {
       tentley::prelude::Matrix::<#ty, #rows, #cols>::new(#tokens)
    })
}
