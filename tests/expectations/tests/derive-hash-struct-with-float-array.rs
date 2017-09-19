/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]



/// A struct containing an array of floats that cannot derive hash/eq but can derive partialeq and partialord
#[repr(C)]
#[derive(Debug, Default, Copy, PartialOrd, PartialEq)]
pub struct foo {
    pub bar: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        12usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(0 as *const foo)).bar as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(foo),
            "::",
            stringify!(bar)
        )
    );
}
impl Clone for foo {
    fn clone(&self) -> Self {
        *self
    }
}
