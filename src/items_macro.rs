#[macro_export]
macro_rules! items {
    (
    $items_name:ident 
    <
        $item_type:ty,
        $length:literal
    >:
    $(
        $val:expr,
        [$($weights:expr),+]
        $(, $quantity:expr)?;
    )*
    ) => {
            let mut $items_name = $crate::item::
            ProblemItems::<$item_type, 
                           $length, 
                           $item_type
                           >::new();
            $(
                $items_name.add($crate::item::
                Item::<$item_type, 
                       $length, 
                       $item_type
                       > {
                    value: $val,
                    weights: [$($weights),*],
                    quantity: $crate::items!(
                                    @items_quantity, 
                                    $($quantity,)?  
                                    <$item_type as $crate::
                                          compatible_problem_type_trait::
                                          CompatibleProblemType
                                     >::default_quantity()
                              ),
                
                });
            )*
        };
    (@items_quantity, $quantity:expr $(, $_default:expr)?) => {
        $quantity
    };

    (
    $items_name:ident 
    <
        $item_type:ty,
        $length:literal,
        unbound
    >:
    $(
        $val:expr,
        [$($weights:expr),+]
        $(, $quantity:expr)?;
    )*
    ) => {
            let mut $items_name = $crate::item::
            ProblemItems::<$item_type, 
                           $length, 
                           unbound
                           >::new();
            $(
                $items_name.add($crate::item::
                Item::<$item_type, 
                       $length,
                       unbound
                       > {
                    value: $val,
                    weights: [$($weights),*],
                    quantity: $crate::items!(
                                    @items_quantity, 
                                    $($quantity,)?  
                                    unbound
                              ),
                
                });
            )*
        };
    (@items_quantity, $quantity:expr $(, $_default:expr)?) => {
        $quantity
    };
}
