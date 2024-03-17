#![feature(generic_const_exprs)]

const fn func1<T, U>() -> bool {
    true
}
const fn func2<T, U>() -> bool {
    false
}

trait CompareResult {
    const R1: bool;
    const R2: bool;
}

struct CompareResultHolder<const R1: bool, const R2: bool>;

impl<const R1: bool, const R2: bool> CompareResult for CompareResultHolder<R1, R2> {
    const R1: bool = R1;
    const R2: bool = R2;
}

trait CompareTypeList<T> {
    type Compare: CompareResult;
}

impl<T, U> CompareTypeList<U> for T
where
    [(); func1::<T, U>() as usize]:,
    [(); func2::<T, U>() as usize]:,
{
    type Compare = CompareResultHolder<{ func1::<T, U>() }, { func2::<T, U>() }>;
}

#[test]
fn aasdf() {
    let a = CompareResultHolder::<{ !false }, { false }>;
}
