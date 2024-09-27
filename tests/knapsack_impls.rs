use kpsolver::{
    ItemBound, Knapsack
};

#[test]
fn add_usize_1() {
    let item1 = ItemBound::<u32, 2>::new(1.0, [1, 1], 1);
    let item2 = ItemBound::<u32, 2>::new(1.0, [1, 1], 1);
    let item3 = ItemBound::<u32, 2>::new(2.0, [1, 1], 1);
    let item_test = item1.clone();
    let mut knapsack = Knapsack::<u32, 2>::new([1, 1]);
    assert!(knapsack.add(item1));
    assert!(!knapsack.add(item2));
    assert!(!knapsack.add(item3));
    assert_eq!(knapsack.peek_item_list(), [item_test]);
    assert_eq!(knapsack.peek_item(1.0, [1, 1]).unwrap().quantity, 1); 
    assert_eq!(knapsack.peek_item(2.0, [1, 1]), None);
    assert_eq!(knapsack.value(), 1.0);
    assert_eq!(knapsack.weights(), &[1, 1]);
}

#[test]
fn add_usize_2() {
    let item1 = ItemBound::<u32, 2>::new(1.0, [1, 1], 1);
    let item2 = ItemBound::<u32, 2>::new(1.0, [1, 1], 2);
    let mut knapsack = Knapsack::<u32, 2>::new([3, 3]);
    knapsack.add(item1);
    knapsack.add(item2);
    assert_eq!(knapsack.peek_item(1.0, [1, 1]).unwrap().quantity, 3);
    assert_eq!(knapsack.value(), 3.0);
    assert_eq!(knapsack.weights(), &[3, 3]);
}

#[test]
fn add_f64_1() {
    let item1 = ItemBound::<f64, 2>::new(1.0, [1.5, 1.5], 1);
    let item2 = ItemBound::<f64, 2>::new(1.0, [1.5, 1.5], 1);
    let item3 = ItemBound::<f64, 2>::new(2.0, [1.5, 1.5], 1);
    let item_test = item1.clone();
    let mut knapsack = Knapsack::<f64, 2>::new([1.5, 1.5]);
    assert!(knapsack.add(item1));
    assert!(!knapsack.add(item2));
    assert!(!knapsack.add(item3));
    assert_eq!(knapsack.peek_item_list(), [item_test]);
    assert_eq!(knapsack.peek_item(1.0, [1.5, 1.5]).unwrap().quantity, 1);
    assert_eq!(knapsack.peek_item(2.0, [1.5, 1.5]), None);
    assert_eq!(knapsack.value(), 1.0);
    assert_eq!(knapsack.weights(), &[1.5, 1.5]);
}

#[test]
fn add_f64_2() {
    let item1 = ItemBound::<f64, 2>::new(1.0, [1.5, 1.5], 1);
    let item2 = ItemBound::<f64, 2>::new(1.0, [1.5, 1.5], 2);
    let mut knapsack = Knapsack::<f64, 2>::new([4.5, 4.5]);
    knapsack.add(item1);
    knapsack.add(item2);
    assert_eq!(knapsack.peek_item(1.0, [1.5, 1.5]).unwrap().quantity, 3);
    assert_eq!(knapsack.value(), 3.0);
    assert_eq!(knapsack.weights(), &[4.5, 4.5]);
}

#[test]
fn add_mut() {
    let mut item = ItemBound::<u32, 1>::new(1.0, [1], 2);
    let mut knapsack = Knapsack::<u32, 1>::new([1]);
    knapsack.add_mut(&mut item, 1);
    assert_eq!(item.quantity, 1);
    assert_eq!(knapsack.peek_item_list(), [item]);
}

/*
#[test]
fn add_mut() {
    let item = ItemUnbound::<usize, 1>::new(1.0, [1]);
    let item_test = ItemBound::<usize, 1>::new(1.0, 1, [1]);
    let mut knapsack = Knapsack::<usize, 1>::new([1]);
    knapsack.add_mut(&item, 1);
    assert_eq!(item.quantity, unbound);
    assert_eq!(knapsack.peek_item_list(), [item_test]);
}
*/

#[test]
fn remove_usize_some() {
    let item = ItemBound::<u32, 2>::new(1.0, [1, 1], 1);
    let item_test = item.clone();
    let mut knapsack = Knapsack::<u32, 2>::new([1, 1]);
    knapsack.add(item);
    assert_eq!(knapsack.remove(item_test.clone()), Some(item_test));
    assert!(knapsack.peek_item_list().is_empty());
}

#[test]
fn remove_usize_none_1() {
    let mut knapsack = Knapsack::<u32, 2>::new([1, 1]);
    assert_eq!(knapsack.remove(ItemBound::<u32, 2>::new(1.0, [1, 1], 1)), None);
}

#[test]
fn remove_item_usize_none_2() {
    let mut knapsack = Knapsack::<u32, 2>::new([1, 1]);
    knapsack.add(ItemBound::<u32, 2>::new(1.0, [1, 1], 1));
    assert_eq!(knapsack.remove(ItemBound::<u32, 2>::new(1.0, [1, 1], 2)), None);
}

#[test]
fn remove_item_f64_some() {
    let item = ItemBound::<f64, 2>::new(1.0, [1.0, 1.0], 1);
    let item_test = item.clone();
    let mut knapsack = Knapsack::<f64, 2>::new([1.0, 1.0]);
    knapsack.add(item);
    assert_eq!(knapsack.remove(item_test.clone()), Some(item_test));
    assert!(knapsack.peek_item_list().is_empty());
}

#[test]
fn remove_item_f64_none_1() {
    let mut knapsack = Knapsack::<f64, 2>::new([1.0, 1.0]);
    assert_eq!(knapsack.remove(ItemBound::<f64, 2>::new(1.0, [1.0, 1.0], 1)), None);
}

#[test]
fn remove_item_f64_none_2() {
    let mut knapsack = Knapsack::<f64, 2>::new([1.0, 1.0]);
    knapsack.add(ItemBound::<f64, 2>::new(1.0, [1.0, 1.0], 1));
    assert_eq!(knapsack.remove(ItemBound::<f64, 2>::new(1.0, [1.0, 1.0], 2)), None);
}
