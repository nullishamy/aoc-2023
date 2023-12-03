#[macro_export]
macro_rules! input {
    // Include the real input
    ($path: literal) => {
        include_str!($path)
    };

    // Use the sample
    ($path: literal, $sample: expr) => {
        $sample
    };

    // Use the real input, but with the sample expression provided
    // Allows for switching between them with a single change
    (~ $path: literal, $s: expr) => {
        input!($path)
    };
}