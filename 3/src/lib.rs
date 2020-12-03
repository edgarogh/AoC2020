#![recursion_limit="1450"]

pub mod peano {
    #[derive(Default)]
    pub struct Zero;
    #[derive(Default)]
    pub struct Succ<T>(T);

    pub type One = Succ<Zero>;


    pub trait PeanoValue {
        fn value() -> u64;
    }

    impl PeanoValue for Zero {
        #[inline]
        fn value() -> u64 {
            0
        }
    }

    impl<V: PeanoValue> PeanoValue for Succ<V> {
        #[inline]
        fn value() -> u64 {
            1 + V::value()
        }
    }
}

macro_rules! map {
    // Function(T, U) = { ... }
    ($n:ident <$sparams:ty> : $nt:ident = {
        $($(<$($gen:ident),+>)? ($($param:ty),*) $([where $($w:tt)+])? => $r:ty;)*
    }) => {
        type $n<T> = <T as $nt>::Res;

        trait $nt {
            type Res;
        }

        // for each "line" inside the "= { ... }"
        $(
        impl$(<$($gen),+>)? $nt for ($($param,)*) $(where $($w)+)? {
            type Res = $r;
        }
        )*
    };
}

pub mod option {
    #[derive(Default)]
    pub struct None;
    pub struct Some<T>(T);
}

pub mod list {
    #[derive(Default)]
    pub struct Nil;
    pub struct Cons<T, S>(T, S);
}

#[cfg(not(any(test, feature = "test-compile")))]
macro_rules! input_line {
    (@ .) => { peano::Zero };
    (@ #) => { peano::One };
    ([$t:tt $($rest:tt)*]) => { list::Cons<input_line!(@ $t), input_line!([$($rest)*])> };
    ([]) => { list::Nil };
}

#[cfg(not(any(test, feature = "test-compile")))]
macro_rules! input {
    ([$($t:tt)+] $($rest:tt)*) => { list::Cons<input_line!([$($t)+]), input!($($rest)*)> };
    () => { list::Nil };
}

/// Retrieves the zero-indexed `Int`'th element of `List`
map! {
    ListIndex<(List, Int)> : TrListIndex = {
        <Index>(list::Nil, Index) => option::None;
        <Element, Tail>(list::Cons<Element, Tail>, peano::Zero) => option::Some<Element>;
        <Element, Tail, Index>(list::Cons<Element, Tail>, peano::Succ<Index>) [where (Tail, Index): TrListIndex] => ListIndex<(Tail, Index)>;
    }
}

/// Retrieves the zero-indexed `Int`'th element of `List`, modulo the list's
/// length
map! {
    ListIndexLoop<(Base, List, Index)> : TrListIndexLoop = {
        <Base, Index>(Base, list::Nil, Index)
            [where (Base, Base, Index): TrListIndexLoop]
            => ListIndexLoop<(Base, Base, Index)>;
        <Base, Element, Tail>(Base, list::Cons<Element, Tail>, peano::Zero)
            => Element;
        <Base, Element, Tail, Index>(Base, list::Cons<Element, Tail>, peano::Succ<Index>)
            [where (Base, Tail, Index): TrListIndexLoop]
            => ListIndexLoop<(Base, Tail, Index)>;
    }
}

type Succ3<T> = peano::Succ<peano::Succ<peano::Succ<T>>>;

map! {
    TreeCount<(List2D, Y, X)> : TrTreeCount = {
        <Y, X>(list::Nil, Y, X) => list::Nil;
        <Y, X, Row, Tail>(list::Cons<Row, Tail>, Y, X)
            [where (Row, Row, X): TrListIndexLoop, (Tail, peano::Succ<Y>, Succ3<X>): TrTreeCount]
            => list::Cons<ListIndexLoop<(Row, Row, X)>, TreeCount<(Tail, peano::Succ<Y>, Succ3<X>)>>;
    }
}

/// Sums a [list][crate::list] whose elements are 1s or 0s into a peano integer
map! {
    ListSum<(List,)> : TrListSum = {
        (list::Nil) => peano::Zero;
        <Tail>(list::Cons<peano::Zero, Tail>) [where (Tail,): TrListSum] => ListSum<(Tail,)>;
        <Tail>(list::Cons<peano::One, Tail>) [where (Tail,): TrListSum] => peano::Succ<ListSum<(Tail,)>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use peano::*;
    use list::*;


    #[test]
    fn list_index() {
        struct A;
        struct B;
        struct C;
        struct D;

        type L = Cons<A, Cons<B, Cons<C, Cons<D, Nil>>>>;
        type Two = Succ<Succ<Zero>>;
        type ShouldBeSomeC = ListIndex<(L, Two)>;

        // Compile-time type equality check
        let some_c: ShouldBeSomeC = return;
        let some_c_for_sure: option::Some<C> = some_c;
    }

    #[test]
    fn list_index_loop() {
        struct A;
        struct B;
        struct C;
        struct D;

        type L = Cons<A, Cons<B, Cons<C, Cons<D, Nil>>>>;
        type Five = Succ<Succ<Succ<Succ<Succ<Zero>>>>>;
        type ShouldBeB = ListIndexLoop<(L, L, Five)>;

        // Compile-time type equality check
        let b: ShouldBeB = return;
        let b_for_sure: B = b;
    }

    #[test]
    fn list_sum() {
        type ListEq2 = Cons<Zero, Cons<One, Cons<One, Cons<Zero, Nil>>>>;
        type ListEq2Sum = ListSum<(ListEq2,)>;

        assert_eq!(<ListEq2Sum as peano::PeanoValue<u64>>::value(), 2);
    }
}

#[cfg(not(any(test, feature = "test-compile")))]
include!(concat!(env!("OUT_DIR"), "/input_txt.rs"));
#[cfg(not(any(test, feature = "test-compile")))]
type R = TreeCount<(Input, peano::Zero, peano::Zero)>;
#[cfg(not(any(test, feature = "test-compile")))]
type Sum = ListSum<(R,)>;

#[cfg(any(test, feature = "test-compile"))]
type Sum = peano::Succ<peano::Succ<peano::Zero>>;

#[no_mangle]
extern "C" fn get_trees() -> u64 {
    <Sum as peano::PeanoValue>::value()
}
