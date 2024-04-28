// Library crate

#[allow(dead_code)]
#[allow(unused_variables)]
#[macro_export]
macro_rules! cr_vec {
    ( $( $x:expr ), * ) => {{
        let mut temp_vec = Vec::new();
        $(
          temp_vec.push($x);
        )*
        temp_vec
    }};
}
