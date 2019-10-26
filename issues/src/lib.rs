#![feature(try_trait)]
extern crate rlsl_math;
use rlsl_math::prelude::*;
pub fn square(_: u32, val: f32) -> f32 {
    val * val
}

pub fn single_branch(_: u32, val: f32) -> f32 {
    if val > 1.0 {
        1.0
    } else {
        0.0
    }
}

pub fn break_loop(_: u32, val: f32) -> f32 {
    let mut sum = 0.0;
    let mut i = 0u32;
    while i > 100 {
        sum += 1.0;
        i += 1;
        if i < 50 {
            break;
        }
    }
    sum
}
// pub fn break_loop(_: u32, val: f32) -> f32 {
//     let mut sum = val;
//     let mut i = Some(0usize);
//     while let Some(idx) = i {
//         sum += 1.0;
//         if 100 < idx {
//             break;
//         }
//         i = Some(idx + 1);
//     }
//     sum
// }
pub fn reference(_: u32, val: f32) -> f32 {
    const LEN: usize = 2;
    let arr: [f32; LEN] = [val, 2.0 * val];
    trait Array<T> {
        fn get(&self, idx: usize) -> T;
        fn length(&self) -> usize;
    }
    impl Array<f32> for [f32; 2] {
        fn get(&self, i: usize) -> f32 {
            self[i]
        }
        fn length(&self) -> usize {
            2
        }
    }
    #[inline(never)]
    fn sum(arr: &impl Array<f32>) -> f32 {
        let mut sum = 42.0f32;
        let mut i = 0usize;
        //let len = arr.length();
        while i < arr.length() {
            sum += 1.0f32;
            i += 1usize;
        }
        sum
    }
    sum(&arr)
}

pub fn simple_loop(_: u32, val: f32) -> f32 {
    const LEN: usize = 2;
    let arr: [f32; LEN] = [val, val];
    let mut i = 0usize;
    let mut sum = 0.0;
    while i < 2 {
        i += 1;
        sum += 1.0;
    }
    sum
}

pub fn u32_add(_: u32, val: f32) -> f32 {
    let i = 1u32;
    let i2 = i + i;
    val
}

pub fn match_result(_: u32, val: f32) -> f32 {
    let foo = if val < 100.0 {
        Ok(1.0)
    } else {
        Err(-1.0)
    };
    match foo {
        Ok(val) => val,
        Err(val) => val,
    }
}

pub fn match_enum(_: u32, val: f32) -> f32 {
    enum Foo {
        A(f32),
        B(f32),
    }
    let foo = if val < 100.0 {
        Foo::A(1.0)
    } else {
        Foo::B(-1.0)
    };
    match foo {
        Foo::A(val) => val,
        Foo::B(val) => val,
    }
}

pub fn ok_or(_: u32, val: f32) -> f32 {
    let o = Some(val).ok_or(-1.0f32);
    match o {
        Ok(val) => val,
        Err(val) => val,
    }
}

pub fn option(_: u32, val: f32) -> f32 {
    let o = Some(val);
    if let Some(f) = o {
        f
    } else {
        -1.0
    }
}

pub fn questionmark_option(_: u32, val: f32) -> f32 {
    fn test(f: f32) -> Option<f32> {
        let o = if f > 42.0 { Some(f) } else { None };
        let r = o?;
        Some(r + 10.0)
    }
    if let Some(val) = test(val) {
        val
    } else {
        -1.0
    }
}
