use backtrace::Backtrace;

#[test]
fn test_backtrace() {
    println!("stack depth={}", stack_frame_depth());
}

#[inline]
fn stack_frame_depth() -> usize {
    Backtrace::new_unresolved().frames().len()
}

