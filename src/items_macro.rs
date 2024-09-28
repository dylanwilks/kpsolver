#[macro_export]
macro_rules! items {
    (
    $items_name:ident 
    <
        $item_type:ty,
        $length:literal,
        $count_type:tt
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
                                             $count_type,
                                             usize($($quantity,)? 1)
                                             unbound($($quantity,)? unbound)),
                });
            )*
        };
    (@items_quantity, usize, 
     usize($quantity_usize:expr $(, $_default_usize:expr)?)
     unbound($quantity_unbound:expr $(, $_default_unbound:expr)?)) => {
        $quantity_usize
    };
    (@items_quantity, unbound, 
     usize($quantity_usize:expr $(, $_default_usize:expr)?)
     unbound($quantity_unbound:expr $(, $_default_unbound:expr)?)) => {
        $quantity_usize
    };
}
