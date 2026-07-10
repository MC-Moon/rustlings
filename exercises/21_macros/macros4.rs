// TODO: Fix the compiler error by adding one or two characters.

macro_rules! replace_with_placeholder {
    ($_ignored:expr, $sub:tt) => {
        $sub
    };
}

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($($val:expr),*) =>{
        // it calls recursive method
        // $(
        //     // my_macro!($val);
        // )*

        // This way is only work by numeric inputs.
        // println!(
        //     "Entire values: {:?}", [$($val),*]
        // );

        // This way support numeric and str types but it requires new replace_with_placeholder macro
        println!(
            concat!("Entire values:" $(, " {}", replace_with_placeholder!($val, ""))*),
            $($val),*
        );
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(111, 222);
    my_macro!(111, 222, 333, "ㅁㄴㅇㅁㄴㅇ");
}
