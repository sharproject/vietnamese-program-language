use std::ptr;
#[allow(dead_code)]
pub fn append_vec<T>(dst: &mut Vec<T>, src: &Vec<T>) {
    let src_len = src.len();
    let dst_len = dst.len();

    // Ensure that `dst` has enough capacity to hold all of `src`.
    dst.reserve(src_len);

    unsafe {
        // The call to offset is always safe because `Vec` will never
        // allocate more than `isize::MAX` bytes.
        let dst_ptr = dst.as_mut_ptr().offset(dst_len as isize);
        let src_ptr = src.as_ptr();

        // The two regions cannot overlap because mutable references do
        // not alias, and two different vectors cannot own the same
        // memory.
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);

        // Notify `dst` that it now holds the contents of `src`.
        dst.set_len(dst_len + src_len);
    }
}
