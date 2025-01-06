#[macro_export]
macro_rules! assert_float {
    ($left:expr, $right:expr) => {
        //Check if the left and right expressions are equal
        if ($left - $right).abs() < f32::EPSILON {
            //If they are equal, do nothing
        } else {
            //If they are not equal, panic
            panic!("assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right);
        }
    };
    ($left:expr, $right:expr, $epsilon:expr) => {
        //Check if the left and right expressions are equal
        if ($left - $right).abs() < $epsilon {
            //If they are equal, do nothing
        } else {
            //If they are not equal, panic
            panic!("assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`", $left, $right);
        }
    };
    ($left:expr, $right:expr, $epsilon:expr, $msg:expr) => {
        //Check if the left and right expressions are equal
        if ($left - $right).abs() <= $epsilon {
            //If they are equal, do nothing
        } else {
            //If they are not equal, panic
            panic!("assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`\n diff: |`{:?}`| > `{:?}`\n msg: `{:?}`", $left, $right, ($left - $right), $epsilon, $msg);
        }
    };
}