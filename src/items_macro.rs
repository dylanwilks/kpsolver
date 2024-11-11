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
macro_rules! items_unbound {
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
            $items_name<$item_type, $length, unbound>:
                $(
                    $val, [$weight] $(, $quantity)?;
                )*
        }
    };
}

#[macro_export]
macro_rules! items_binary {
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
            ProblemItemsBinary::<$item_type, 
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
                                     >::identity()
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

