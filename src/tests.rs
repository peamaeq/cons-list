use crate::Cons;
use crate::List;

#[test]
fn new_list_is_empty() {
    let l: List<()> = List::new();

    assert!(l.is_empty());
}

#[test]
fn empty_list_is_zero_len() {
    let l: List<()> = List::new();

    assert_eq!(l.len(), 0);
}

#[test]
fn list_len_works_correctly() {
    let mut l: List<i32> = List::new();

    l.push(1);
    l.push(2);
    l.push(3);

    assert_eq!(l.len(), 3);
}

#[test]
fn items_are_popped_in_lifo_order() {
    let mut l: List<i32> = List::new();

    l.push(1);
    l.push(2);
    l.push(3);

    assert_eq!(l.pop(), Some(3));
    assert_eq!(l.pop(), Some(2));
    assert_eq!(l.pop(), Some(1));
}

#[test]
fn list_can_be_consed() {
    let mut l: List<i32> = List::new();

    l.push(1);

    assert!(matches!(l.cons(), Cons::Cons(1, _)));
}

#[test]
fn tail_can_be_consed() {
    let mut l: List<i32> = List::new();

    l.push(1);
    l.push(2);
    l.push(3);

    let tail = l
        .cons()
        .tail()
        .map(|list| list.into_iter().collect::<Vec<i32>>());

    assert_eq!(tail, Some(vec![2, 1]));
}

#[test]
fn no_tail_when_consing_one_element_list() {
    let mut l: List<i32> = List::new();

    l.push(1);

    assert!(matches!(l.cons().tail(), None));
}

#[test]
fn list_can_be_iterated() {
    let mut l: List<i32> = List::new();

    l.push(1);
    l.push(2);
    l.push(3);

    let collected: Vec<i32> = l.iter().copied().collect();

    assert_eq!(collected, vec![3, 2, 1]);
}

#[test]
fn list_can_be_mutably_iterated() {
    let mut l: List<i32> = List::new();

    l.push(1);
    l.push(2);
    l.push(3);

    l.iter_mut().for_each(|el| *el *= 2);

    let collected: Vec<i32> = l.iter().copied().collect();

    assert_eq!(collected, vec![6, 4, 2]);
}

#[test]
fn list_can_be_transformed_into_iterator() {
    let mut l: List<i32> = List::new();

    l.push(1);
    l.push(2);
    l.push(3);

    let collected: Vec<i32> = l.into_iter().collect();

    assert_eq!(collected, vec![3, 2, 1]);
}
