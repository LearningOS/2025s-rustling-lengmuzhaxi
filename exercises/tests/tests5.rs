// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    // tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.
/// # Safety
///
/// - The `address` must be a valid, non-null, and properly aligned pointer to a mutable `u32` value.
/// - The caller must ensure that the memory at `address` is owned and not accessed elsewhere concurrently.
/// - The memory at `address` must be initialized before dereferencing.
unsafe fn modify_by_address(address: usize) {
    let ptr = address as *mut u32;
    
    // SAFETY: Caller must ensure that `ptr` is valid, non-null, and properly aligned.
    if !ptr.is_null() && ptr.align_offset(std::mem::align_of::<u32>()) == 0 {
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        
        // SAFETY: `&mut t` is a valid and unique mutable reference to `u32`,
        // ensuring `modify_by_address` receives a valid pointer.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        
        assert_eq!(t, 0xAABBCCDD);
    }

    #[test]
    fn test_null_pointer_no_panic() {
        // SAFETY: We are explicitly testing a null pointer case.
        unsafe { modify_by_address(0) }; // Should not crash or modify anything.
    }
}
}