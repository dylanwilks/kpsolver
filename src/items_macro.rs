#[macro_export]
macro_rules! items {
    ($type:ty, $length:expr) => {$crate::Item::ProblemItems::<$type, $length>::new()};
    ($type:ty, $length:expr, unbounded) => {
        $crate::Item::ProblemItems::<$type, $length, unbounded>::new()
    };

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
                           $length
                           >::new();
            $(
                $items_name.add($crate::item::
                Item::<$item_type, 
                       $length
                       > {
                    value: $val,
                    weights: [$($weights),*],
                    quantity: $crate::items!(
                                    @items_quantity, 
                                    $($quantity,)?  
                                    <$item_type as $crate::
                                          compatible_problem_type_trait::
                                          CompatibleProblemType
                                     >::identity()
                              ),
                
                });
            )*
        };

    (
    $items_name:ident 
    <
        $item_type:ty,
        $length:literal,
        unbounded
    >:
    $(
        $val:expr,
        [$($weights:expr),+]
        $(, $quantity:expr)?;
    )*
    ) => {
            let mut $items_name = $crate::item::
            UnboundedProblemItems::<$item_type, 
                                    $length, 
                                    >::new();
            $(
                $items_name.add($crate::item::
                UnboundedItem::<$item_type, 
                                $length,
                                > {
                    value: $val,
                    weights: [$($weights),*],
                    quantity: $crate::items!(
                                    @items_quantity, 
                                    $($quantity,)?  
                                    unbounded
                              ),
                
                });
            )*
    };

    (
    $items_name:ident 
    <
        $item_type:ty,
        1,
    >:
    $(
        $val:expr,
        $weight:expr
        $(, $quantity:expr)?;
    )*
    ) => {
        items! {
            $items_name<$item_type, 1>:
                $(
                    $val, [$weight] $(, $quantity)?;
                )*
        }
    };
    (@items_quantity, $quantity:expr $(, $_default:expr)?) => {
        $quantity
    };
}

#[macro_export]
macro_rules! items_unbounded {
    ($type:ty, $length:expr) => {$crate::Item::UnboundedProblemItems::<$type, $length>::new()};

    (
    $items_name:ident 
    <
        $item_type:ty,
        $length:literal,
    >:
    $(
        $val:expr,
        $weight:expr
        $(, $quantity:expr)?;
    )*
    ) => {
        items! {
            $items_name<$item_type, $length, unbounded>:
                $(
                    $val, [$weight] $(, $quantity)?;
                )*
        }
    };
}

#[macro_export]
macro_rules! items_binary {
    ($type:ty, $length:expr) => {$crate::Item::BinaryProblemItems::<$type, $length>::new()};

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
            BinaryProblemItems::<$item_type, 
                                 $length
                                 >::new();
            $(
                $items_name.add($crate::item::
                Item::<$item_type, 
                       $length
                       > {
                    value: $val,
                    weights: [$($weights),*],
                    quantity: $crate::items_binary!(
                                    @items_quantity, 
                                    $($quantity,)?  
                                    <$item_type as $crate::
                                          compatible_problem_type_trait::
                                          CompatibleProblemType
                                     >::identity()
                              ),
                
                });
            )*
        };

    (
    $items_name:ident 
    <
        $item_type:ty,
        1
    >:
    $(
        $val:expr,
        $weight:expr
        $(, $quantity:expr)?;
    )*
    ) => {
        items_binary! {
            $items_name<$item_type, 1>:
                $(
                    $val, [$weight] $(, $quantity)?;
                )*
        }
    };
    (@items_quantity, $quantity:expr $(, $_default:expr)?) => {
        $quantity
    };
}

