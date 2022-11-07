// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

#![allow(unused_imports)]

use core::{time::Duration, sync::atomic::AtomicBool};

use kernel::{
    kasync::executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    prelude::*,
    c_str,
    spawn_task,
    sync::{Arc, ArcBorrow},
    task::Task, delay::coarse_sleep, sysctl::Sysctl, Mode,
};

module! {
    type: RustMinimal,
    name: "rust_minimal",
    author: "Rust for Linux Contributors",
    description: "Rust minimal sample",
    license: "GPL",
}

struct RustMinimal {
    numbers: Vec<i32>,
    #[allow(unused)]
    sys_val: Sysctl<AtomicBool>,
}

#[allow(unused)]
async fn loop_fn(_executor: Arc<impl Executor>) {
    loop {
        let task = Task::current();
        pr_info!("Timed execution, pid={:?}!", task.pid());
        coarse_sleep(Duration::from_millis(1000));
    }
}

impl kernel::Module for RustMinimal {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust minimal sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let sys_val = Sysctl::register(c_str!("fs"), c_str!("module_param_name"), AtomicBool::new(false), Mode::from_int(0)).unwrap();
        pr_info!("VALUE: {:?}", sys_val.get());

        let mut numbers = Vec::new();
        numbers.try_push(72)?;
        numbers.try_push(108)?;
        numbers.try_push(200)?;

        let task = Task::current();
        pr_info!("Have task, pid={:?}!", task.pid());

        let _handle = WqExecutor::try_new(kernel::workqueue::system())?;
        //spawn_task!(handle.executor(), loop_fn(handle.executor().into()))?;
        //spawn_task!(handle.executor(), loop_fn(handle.executor().into()))?;

        Ok(RustMinimal { numbers, sys_val })
    }
}

impl Drop for RustMinimal {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust minimal sample (exit)\n");
    }
}
