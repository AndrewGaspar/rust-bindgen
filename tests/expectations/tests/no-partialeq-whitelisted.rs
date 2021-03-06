/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct NoPartialEq {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NoPartialEq() {
    assert_eq!(
        ::std::mem::size_of::<NoPartialEq>(),
        4usize,
        concat!("Size of: ", stringify!(NoPartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<NoPartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(NoPartialEq))
    );
    assert_eq!(
        unsafe { &(*(0 as *const NoPartialEq)).i as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(NoPartialEq),
            "::",
            stringify!(i)
        )
    );
}
impl Clone for NoPartialEq {
    fn clone(&self) -> Self {
        *self
    }
}
