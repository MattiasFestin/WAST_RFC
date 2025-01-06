pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

mod wast;

#[cfg(test)]
mod tests {
    

    // use wit_bindgen::generate;

    // generate!({
    //     inline: r#"
    //         package wast:wast-snapshot-preview1@0.2.0;

    //         interface setup {
    //             /// Set up the test environment
    //             before-all: func() -> ();

    //             /// Set up the test environment before each test
    //             before-each: func() -> ();

    //             /// Tear down the test environment
    //             after-all: func() -> ();

    //             /// Tear down the test environment after each test
    //             after-each: func() -> ();
    //         }

    //         interface asserting {
    //             /// Assert equality between two 32-bit integers
    //             assert-s32-eq: func(expected: s32, actual: s32) -> ();

    //             /// Assert that a 32-bit integer is within a specified range
    //             assert-s32-range: func(value: s32, min: s32, max: s32) -> ();

    //             /// Assert equality between two 64-bit integers
    //             assert-s64-eq: func(expected: s64, actual: s64) -> ();

    //             /// Assert that a 64-bit integer is within a specified range
    //             assert-s64-range: func(value: s64, min: s64, max: s64) -> ();

    //             /// Assert that two 32-bit floats are not within the provided epsilon of each other
    //             assert-f32-near: func(a: f32, b: f32, epsilon: f32) -> ();

    //             /// Assert that two 64-bit floats are within the provided epsilon of each other
    //             assert-f64-near: func(a: f64, b: f64, epsilon: f64) -> ();
    //         }

    //         world wast {
    //             import setup;
    //             export asserting;
    //         }
    //     "#,
    // });
    
    use crate::assert_float;


    
    #[no_mangle]
    pub extern "C" fn wast_before_all() -> () {
        
    }

    #[no_mangle]
    pub extern "C" fn wast_before_each() -> () {
        
    }

    #[no_mangle]
    pub extern "C" fn wast_after_all() -> () {
        
    }

    #[no_mangle]
    pub extern "C" fn wast_after_each() -> () {
        
    }

    #[test]
    #[no_mangle]
    pub extern "C" fn wast_test_add_ints_should_presere_identity() -> () {
        let result = crate::add(0, 3);
        assert_eq!(result, 3i32, "add(0, 3) should be 3");
    }

    #[test]
    #[no_mangle]
    pub extern "C" fn wast_test_add_floats_should_presere_identity() -> () {
        let result = crate::add_f32(0.1, 0.2);

        // assert_eq!(0.2 + 0.1, 0.3, "add(0.1, 0.2) should be 0.3");
        assert_float!(0.2 + 0.1, 0.3f64, f64::EPSILON, "add(0.1, 0.2) should be 0.3");
        
        // crate::assert_float!(result, 3.139f32, 0.001, "add(0.0, 3.14) should be 3.14");
    }
    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn it_works() {
    //         let result = add(2, 2);
    //         assert-eq!(result, 4);
    //     }
    // }

    //WASI mocks
    //func $fd_write (param i32 i32 i32 i32) (result i32)
    #[no_mangle]
    pub extern "C" fn fd_write(fd: i32, iovs: i32, iovs_len: i32, nwritten: i32) -> i32 {
        0
    }

}