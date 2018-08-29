#[macro_use]
extern crate assert_futures;
extern crate futures;

use futures::future;
use futures::stream;

#[test]
fn test_assert_future() {
    let laz = future::lazy(|| future::ok::<(), ()>(assert_eq!(1 + 1, 2)));
    assert_future!(laz);
}

#[test]
#[should_panic]
fn test_panic_assert_future() {
    let laz = future::lazy(|| future::ok::<(), ()>(assert_eq!(1 + 1, 3)));
    assert_future!(laz);

    let imm = future::err::<(), _>(1);
    assert_future!(imm);
}

#[test]
fn test_assert_future_eq() {
    let imm = future::ok::<u32, u32>(1u32);
    assert_future_eq!(imm, Ok(1u32));

    let imm = future::err::<u32, u32>(2u32);
    assert_future_eq!(imm, Err(2u32));
}

#[test]
#[should_panic]
fn test_panic_assert_future_eq() {
    let imm = future::ok::<u32, u32>(1u32);
    assert_future_eq!(imm, Ok(3u32));

    let imm = future::ok::<u32, u32>(1u32);
    assert_future_eq!(imm, Err(1u32));
}

#[test]
fn test_assert_stream_eq() {
    let imm = stream::iter_ok::<_, ()>(vec![1, 2, 3]);
    assert_stream_eq!(imm, Ok(vec![1, 2, 3]));

    let imm = stream::iter_result(vec![Ok(1), Err(2), Ok(3)]);
    assert_stream_eq!(imm, Err(2));
}

#[test]
#[should_panic]
fn test_panic_assert_stream_eq() {
    let imm = stream::iter_ok::<_, ()>(vec![1, 2, 3]);
    assert_stream_eq!(imm, Ok(vec![1, 3]));

    let imm = stream::iter_result(vec![Ok(1), Err(2), Ok(3)]);
    assert_stream_eq!(imm, Err(1));
}

#[test]
fn test_assert_stream_result_eq() {
    let imm = stream::iter_result(vec![Ok(1), Err(2), Ok(3)]);
    assert_stream_result_eq!(imm, vec![Ok(1), Err(2), Ok(3)]);
}

#[test]
#[should_panic]
fn test_panic_assert_stream_result_eq() {
    let imm = stream::iter_result(vec![Ok(1), Err(2), Ok(3)]);
    assert_stream_result_eq!(imm, vec![Err(1), Err(2), Ok(3)]);
}
