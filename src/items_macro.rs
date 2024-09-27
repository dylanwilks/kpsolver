#[macro_export]
macro_rules! items {
    (
    $items_name:ident 
    <
        $item_type:ty,
        $length:literal,
        $count_type:ty
    >:
    $(
        $val:expr,
        [$($weights:expr),+]
        $(, $quantity:expr)?;
    )*
    ) => {
            let mut $items_name = $crate::item::
            ProblemItems::<$item_type, $length, $count_type>::new();
            $(
                $items_name.add($crate::item::
                Item::<$item_type, $length, $count_type> {
                    value: $val,
                    weights: [$($weights),*],
                    quantity: $crate::items!(@items_quantity, 
                                             $($quantity,)?
                                             <$count_type as $crate::
                                                             unbound_struct::
                                                             Quantity>::
                                                             identity()),
                });
            )*
        };
    (@items_quantity, $quantity:expr $(, $_default:expr)?) => {
        $quantity
    };
}
