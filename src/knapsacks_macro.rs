#[macro_export]
macro_rules! knapsacks {
    ($type:ty, $length:expr) => {$crate::knapsack::ProblemKnapsacks::<$type, $length>::new()};

    (
    $knapsacks_name:ident
    <
        $knapsack_type:ty,
        $length:literal
    >:
    $(
        [$($capacity:expr),+];
    )*
    ) => {
        let mut $knapsacks_name
        = $crate::knapsack::ProblemKnapsacks::<$knapsack_type, $length>::new();
        $(
            $knapsacks_name.add($crate::knapsack::Knapsack::<$knapsack_type, $length>::
                new([$($capacity),*]));
        )*
    };

    (
    $knapsacks_name:ident
    <
        $knapsack_type:ty,
        1
    >:
    $(
        $capacity:expr;
    )*
    ) => {
        knapsacks! {
            $knapsacks_name<$knapsack_type, $length>:
            $(
                [$capacity];
            )*
        }
    };
}

#[macro_export]
macro_rules! knapsacks_binary {
    ($type:ty, $length:expr) => {$crate::knapsack::BinaryProblemKnapsacks::<$type, $length>::new()};

    (
    $knapsacks_name:ident
    <
        $knapsack_type:ty,
        $length:literal
    >:
    $(
        [$($capacity:expr),+];
    )*
    ) => {
        let mut $knapsacks_name
        = $crate::knapsack::BinaryProblemKnapsacks::<$knapsack_type, $length>::new();
        $(
            $knapsacks_name.add($crate::knapsack::BinaryKnapsack::<$knapsack_type, $length>::
                new([$($capacity),*]));
        )*
    };

    (
    $knapsacks_name:ident
    <
        $knapsack_type:ty,
        1
    >:
    $(
        $capacity:expr;
    )*
    ) => {
        knapsacks_binary! {
            $knapsacks_name<$knapsack_type, 1>:
            $(
                [$capacity];
            )*
        }
    };
}
