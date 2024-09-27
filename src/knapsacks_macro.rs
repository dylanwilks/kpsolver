#[macro_export]
macro_rules! knapsacks {
    (
    $knapsacks_name:ident
    <
        $knapsack_type:ty,
        $length:literal
    >:
    $(
        [$($capacity:expr),+]
    )*
    ) => {
        let mut $knapsacks_name 
        = ProblemKnapsacks::<$knapsack_type, $length>::new();
        $(
            $knapsacks_name.add(Knapsack::<$knapsack_type, $length>::
                new([$($capacity),*]));
        )*
    };
}
