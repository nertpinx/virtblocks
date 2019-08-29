// Virt Blocks
//
// Copyright (C) 2019 Red Hat, Inc.
//
// This software is distributed under the terms of the MIT License.
// See the LICENSE file in the top level directory for details.

/// Test all the things with this simple object
pub struct Ultra<'a> {
    number: i32,
    callback: Option<Box<dyn Fn(i32) + 'a>>,
}

impl Default for Ultra<'_> {
    fn default() -> Self {
        Self {
            number: 1,
            callback: None,
        }
    }
}

impl<'a> Ultra<'a> {
    pub fn set_cb<F>(&mut self, cb: F)
    where
        F: Fn(i32) + 'a,
    {
        self.callback = Some(Box::new(cb));
    }

    pub fn unset_cb(&mut self) {
        self.callback = None;
    }

    pub fn call_me(&mut self) {
        let num = self.number;

        self.number += 1;

        let cb = match &self.callback {
            Some(cb) => cb,
            None => return,
        };

        cb(num);
    }
}
