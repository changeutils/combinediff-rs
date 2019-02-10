//!
//! The Combinediff library.
//!

use std::collections::VecDeque;

use log::*;

use patch_rs::Patch;

pub fn combinediff(mut patch_1: Patch, mut patch_2: Patch, context_radius: usize) -> Patch {
    let mut combinediff = Patch {
        input: patch_1.input.to_owned(),
        output: patch_2.output.to_owned(),
        contexts: VecDeque::new(),
    };

    trace!("DRAINING BOTH PATCHES");
    while !patch_1.contexts.is_empty() && !patch_2.contexts.is_empty() {
        let p1 = patch_1.contexts.front().unwrap();
        let p2 = patch_2.contexts.front().unwrap();
        if p1.header.file1_l <= p2.header.file1_l {
            let context = patch_1.contexts.pop_front().unwrap();
            let reduced = context.reduce(context_radius);
            for mut context in reduced.into_iter() {
                combinediff.contexts.push_back(context);
            }
        } else {
            let context = patch_2.contexts.pop_front().unwrap();
            let reduced = context.reduce(context_radius);
            for mut context in reduced.into_iter() {
                combinediff.contexts.push_back(context);
            }
        }
    }
    trace!("DRAINING FIRST PATCH");
    while !patch_1.contexts.is_empty() {
        let context = patch_1.contexts.pop_front().unwrap();
        let reduced = context.reduce(context_radius);
        for mut context in reduced.into_iter() {
            combinediff.contexts.push_back(context);
        }
    }
    trace!("DRAINING SECOND PATCH");
    while !patch_2.contexts.is_empty() {
        let context = patch_2.contexts.pop_front().unwrap();
        let reduced = context.reduce(context_radius);
        for mut context in reduced.into_iter() {
            combinediff.contexts.push_back(context);
        }
    }

    combinediff
}
