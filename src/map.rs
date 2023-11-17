#[macro_export]
macro_rules! hashmap {
    ( $( $key:tt => $arg:expr), * $(,)?) => {
        let mut tmp = ::std::collections::HashMap::new();
        $( 
          tmp.insert($key, $arg); 
        )*
        tmp
    };
}