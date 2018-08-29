pub extern crate futures;
pub extern crate tokio;

pub mod _ext {
    pub use futures;
    pub use tokio;
}

#[macro_export]
macro_rules! assert_future_eq {
    ($fut:expr, $val:expr) => {{
        use $crate::_ext::tokio::runtime::current_thread::Runtime;

        let mut _runtime = Runtime::new().unwrap();

        assert_eq!(_runtime.block_on($fut), $val);
    }};
}

#[macro_export]
macro_rules! assert_future {
    ($fut:expr) => {{
        use $crate::_ext::tokio::runtime::current_thread::Runtime;

        let mut _runtime = Runtime::new().unwrap();

        assert!(_runtime.block_on($fut).is_ok());
    }};
}

#[macro_export]
macro_rules! assert_stream_eq {
    ($str:expr, $vec:expr) => {{
        use $crate::_ext::futures::prelude::*;

        assert_future_eq!($str.collect(), $vec);
    }};
}

#[macro_export]
macro_rules! assert_stream_result_eq {
    ($str:expr, $vec_result:expr) => {{
        fn _wrap<T>(a: T) -> Result<T, ()> {
            Ok(a)
        }
        assert_stream_eq!($str.then(_wrap), Ok($vec_result));
    }};
}
