//! Helper functions to insert a range of items into a Vec.
//!
//! With the functions in this crate, you can insert a slice or the contents of another Vec into
//! the middle of a Vec. Whereas `insert` allows you to insert a single element into the middle of
//! a Vec, and `append` and `extend` allow you to add a collection of elements to the end of a Vec,
//! the `splice` functions allow you to take a Vec or slice and insert its contents to the middle
//! of a Vec.
//!
//! # Examples
//!
//! Using `splice_copy` allows for an efficient implementation for types which implement Copy:
//!
//! ```rust
//! let mut dest = vec![1u8, 2, 3, 4];
//! let src = vec![5u8, 6];
//!
//! splice::splice_copy(&mut dest, 2, &src);
//!
//! assert_eq!(dest, vec![1, 2, 5, 6, 3, 4]);
//! ```

use std::ptr;

/// Clone the contents of the given slice into the given Vec at the given index, shifting all
/// existing elements after that position to the right.
///
/// # Panics
///
/// Panics if `index` is out of bounds of `dest`.
///
/// # Examples
///
/// ```rust
/// let mut dest: Vec<String> = vec!["one".into(), "two".into()];
/// let src: Vec<String> = vec!["three".into(), "four".into()];
///
/// splice::splice_clone(&mut dest, 1, &src);
///
/// assert_eq!(dest, vec!["one", "three", "four", "two"]);
/// ```
pub fn splice_clone<T: Clone>(dest: &mut Vec<T>, index: usize, src: &[T]) {
    assert!(index <= dest.len(), "index of out of bounds of the Vec");

    dest.reserve(src.len());

    unsafe {
        let dest_ptr = dest.as_mut_ptr().offset(index as isize);
        let shift_ptr = dest_ptr.offset(src.len() as isize);
        let shift_len = dest.len() - index;

        ptr::copy(dest_ptr, shift_ptr, shift_len);

        for (idx, elem) in src.iter().enumerate() {
            ptr::write(dest_ptr.offset(idx as isize), elem.clone());
        }

        let len = dest.len() + src.len();
        dest.set_len(len);
    }
}

/// Copies the contents of the given slice into the given Vec at the given index, shifting all
/// existing elements after that position to the right.
///
/// # Panics
///
/// Panics if `index` is out of bounds of `dest`.
///
/// # Examples
///
/// ```rust
/// let mut dest = vec![1u8, 2, 3, 4];
/// let src = vec![5u8, 6];
///
/// splice::splice_copy(&mut dest, 2, &src);
///
/// assert_eq!(dest, vec![1, 2, 5, 6, 3, 4]);
/// ```
pub fn splice_copy<T: Copy>(dest: &mut Vec<T>, index: usize, src: &[T]) {
    assert!(index <= dest.len(), "index of out of bounds of the Vec");

    dest.reserve(src.len());

    unsafe {
        let dest_ptr = dest.as_mut_ptr().offset(index as isize);
        let shift_ptr = dest_ptr.offset(src.len() as isize);
        let shift_len = dest.len() - index;

        ptr::copy(dest_ptr, shift_ptr, shift_len);
        ptr::copy_nonoverlapping(src.as_ptr(), dest_ptr, src.len());

        let len = dest.len() + src.len();
        dest.set_len(len);
    }
}

/// Moves the contents of the source Vec into the destination Vec at the given index, shifting all
/// existing elements after that position to the right and leaving `src` empty.
///
/// # Panics
///
/// Panics if `index` is out of bounds of `dest`.
///
/// # Examples
///
/// ```rust
/// // making a struct here so we have something that's not Copy/Clone
/// #[derive(PartialEq, Debug)]
/// struct Item(u8);
///
/// let mut dest = vec![Item(1), Item(2), Item(3), Item(4)];
/// let mut src = vec![Item(5), Item(6)];
///
/// splice::splice(&mut dest, 2, &mut src);
///
/// assert_eq!(dest, vec![Item(1), Item(2), Item(5), Item(6), Item(3), Item(4)]);
/// assert!(src.is_empty());
/// ```
///
/// To only move part of a Vec, move that segment out with `drain` first:
///
/// ```rust
/// #[derive(PartialEq, Debug)]
/// struct Item(u8);
///
/// let mut dest = vec![Item(1), Item(2), Item(3), Item(4)];
/// let mut src = vec![Item(5), Item(6), Item(7), Item(8)];
/// let mut sub_src = src.drain(1..3).collect::<Vec<_>>();
///
/// splice::splice(&mut dest, 2, &mut sub_src);
///
/// assert_eq!(dest, vec![Item(1), Item(2), Item(6), Item(7), Item(3), Item(4)]);
/// assert_eq!(src, vec![Item(5), Item(8)]);
/// assert!(sub_src.is_empty());
/// ```
pub fn splice<T>(dest: &mut Vec<T>, index: usize, src: &mut Vec<T>) {
    assert!(index <= dest.len(), "index of out of bounds of the Vec");

    dest.reserve(src.len());

    unsafe {
        let dest_ptr = dest.as_mut_ptr().offset(index as isize);
        let shift_ptr = dest_ptr.offset(src.len() as isize);
        let shift_len = dest.len() - index;

        ptr::copy(dest_ptr, shift_ptr, shift_len);
        ptr::copy_nonoverlapping(src.as_ptr(), dest_ptr, src.len());

        let len = dest.len() + src.len();
        dest.set_len(len);
        src.set_len(0);
    }
}

#[test]
fn splice_clone_test() {
    let mut asdf: Vec<String> = vec!["asdf".into(), "qwop".into()];
    let qwop: Vec<String> = vec!["girp".into(), "zxcv".into()];

    splice_clone(&mut asdf, 1, &qwop);

    assert_eq!(asdf, vec!["asdf", "girp", "zxcv", "qwop"]);
}

#[test]
fn splice_copy_test() {
    let mut asdf = vec![1u8, 2, 3, 4];
    let qwop = vec![5, 6];

    splice_copy(&mut asdf, 2, &qwop);

    assert_eq!(asdf, vec![1, 2, 5, 6, 3, 4]);
}

#[test]
fn splice_test() {
    let mut asdf = vec![1u8, 2, 3, 4];
    let mut qwop = vec![5, 6];

    splice(&mut asdf, 2, &mut qwop);

    assert_eq!(asdf, vec![1, 2, 5, 6, 3, 4]);
    assert!(qwop.is_empty());
}
