#[macro_export]
macro_rules! knapsacks {
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
        = ProblemKnapsacks::<$knapsack_type, $length>::new();
        $(
            $knapsacks_name.add(Knapsack::<$knapsack_type, $length>::
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
        = ProblemKnapsacksBinary::<$knapsack_type, $length>::new();
        $(
            $knapsacks_name.add(KnapsackBinary::<$knapsack_type, $length>::
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
            $knapsacks_name<$knapsack_type, $length>:
            $(
                [$capacity];
            )*
        }
    };
}
