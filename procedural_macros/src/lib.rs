// Library crate

/*
  The second form of macros is the procedural macro, which acts more like a
  function (and is a type of procedure). Procedural macros accept some code as an input,
  operate on that code, and produce some code as an output rather than matching against patterns and
  replacing the code with other code as declarative macros do. The three kinds of procedural macros are
  custom derive, attribute-like, and function-like, and all work in a similar fashion.

  When creating procedural macros, the definitions must reside in their own crate with a special crate type.
  This is for complex technical reasons that we hope to eliminate in the future. In Listing 19-29,
  we show how to define a procedural macro, where some_attribute is a placeholder for using a specific macro variety.
*/

/*
  Structure of procedural macro:

  #[some_attribute]
  pub fn some_name(input: TokenStream) -> TokenStream {
    //...
  }
*/

#[allow(dead_code)]
#[allow(unused_variables)]

pub trait HelloMacro {
    fn hello_macro();
}
