use std::cell::Ref;
use std::cell::RefCell;
use std::thread;
#[test]
fn test_basics() {
    let c = RefCell::new(5);

    let five = c.into_inner();

    assert_eq!(5, five);
    let cell = RefCell::new(five);
    let old_value = cell.replace(6);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(6));
    let old_value2 = cell.replace_with(|&mut old| old + 1);
    // let old_value3 = cell.replace_with(|&mut old| old + 1);
    let bb = cell.borrow();
    assert_eq!(old_value2, 6);
    assert_eq!(cell, RefCell::new(7));
    let c = RefCell::new(5);
    let d = RefCell::new(6);
    c.swap(&d);
    assert_eq!(c, RefCell::new(6));
    assert_eq!(d, RefCell::new(5));
    assert_eq!(*bb, 7);

    let borrowed_five = c.borrow(); //now borrowed_five is ref ,use deref * to visit data
    let borrowed_five2 = c.borrow();
    assert_eq!(*borrowed_five, 6);
    assert_eq!(*borrowed_five2, 6);
    let c = RefCell::new((5, 'b'));
    let b1: Ref<(u32, char)> = c.borrow();
    let b11: Ref<(u32, char)> = c.borrow();
    let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
    let b3: Ref<char> = Ref::map(b11, |t| &t.1);
    assert_eq!(*b2, 5);
    assert_eq!(*b3, 'b');
    assert_eq!(c, RefCell::new((5, 'b')));

    let cell = RefCell::new([1, 2, 3, 4]);
    let borrow = cell.borrow();
    let borrow2 = cell.borrow();
    let (begin, end) = Ref::map_split(borrow, |slice| slice.split_at(2));
    let (begin2, end2) = Ref::map_split(borrow2, |slice| slice.split_at(1));
    assert_eq!(*begin, [1, 2]);
    assert_eq!(*begin2, [1]);
    assert_eq!(*end, [3, 4]);
    assert_eq!(*end2, [2, 3, 4]);
    assert_eq!(cell, RefCell::new([1, 2, 3, 4]));
    let result = thread::spawn(move || {
        let c = RefCell::new(5);
        let m = c.borrow_mut();

        let b = c.borrow(); // this causes a panic
    })
    .join();

    assert!(result.is_err());
}
