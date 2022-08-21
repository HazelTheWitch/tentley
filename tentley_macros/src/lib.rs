use proc_macro::TokenStream;

use quote::TokenStreamExt;

use proc_macro2::{Delimiter, Spacing, TokenTree};
use proc_macro2::{Group, Punct};

use quote::quote;

use syn::{
    parse::{Error, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Result, Token,
};

struct Matrix {
    rows: Vec<Vec<Expr>>,
    cols: usize,
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

            let row_iter = row.iter();

            row_tokens.append_separated(row_iter, Punct::new(',', Spacing::Alone));

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

        while !input.is_empty() {
            let span = input.span();
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
        }

        Ok(Matrix {
            rows,
            cols: cols.unwrap_or(0),
        })
    }
}

struct Vector {
    data: Vec<Expr>,
}

impl Vector {
    fn as_row_vector(self) -> Matrix {
        let cols = self.data.len();

        Matrix {
            rows: vec![self.data],
            cols,
        }
    }

    fn as_column_vector(self) -> Matrix {
        Matrix {
            rows: self.data.into_iter().map(|e| vec![e]).collect(),
            cols: 1,
        }
    }
}

impl Parse for Vector {
    fn parse(input: ParseStream) -> Result<Self> {
        let data = MatrixRow::parse_separated_nonempty(input)?;

        Ok(Vector {
            data: data.into_iter().collect(),
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

    let tokens = matrix.to_tokens();

    TokenStream::from(quote! {
       tentley::prelude::Matrix::<_, #rows, #cols>::new(#tokens)
    })
}
