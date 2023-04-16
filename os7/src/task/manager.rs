use super::{TaskControlBlock};
use crate::sync::UPSafeCell;
use alloc::collections::{VecDeque, BinaryHeap};
use alloc::sync::Arc;
use lazy_static::*;


pub const BIG_STRIDE: usize = 0xffffffff;
pub const PRIORITY_INIT: usize = 16;
pub const PASS_INIT: usize = 0;


pub trait TaskManager {
    fn new() -> Self;
    fn add(&mut self, task: Arc<TaskControlBlock>);
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>>;
}

pub struct StrideManager {
    ready_queue: BinaryHeap<Arc<TaskControlBlock>>,
}

impl TaskManager for  StrideManager{
    fn new() -> Self {
        Self { ready_queue: BinaryHeap::new(), }
    }

    fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(task);
    }

    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let next_tcb = self.ready_queue.pop();
        next_tcb.clone().unwrap().inner_exclusive_access().add_stride();
        next_tcb
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<StrideManager> = unsafe {
        UPSafeCell::new(StrideManager::new())
    };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}

// pub struct TaskManager {
//     ready_queue: VecDeque<Arc<TaskControlBlock>>,
// }

// // YOUR JOB: FIFO->Stride
// /// A simple FIFO scheduler.
// impl TaskManager {
//     pub fn new() -> Self {
//         Self {
//             ready_queue: VecDeque::new(),
//         }
//     }
//     /// Add process back to ready queue
//     pub fn add(&mut self, task: Arc<TaskControlBlock>) {
//         self.ready_queue.push_back(task);
//     }
//     /// Take a process out of the ready queue
//     pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
//         self.ready_queue.pop_front()
//     }
// }

// lazy_static! {
//     /// TASK_MANAGER instance through lazy_static!
//     pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
//         unsafe { UPSafeCell::new(TaskManager::new()) };
// }

// pub fn add_task(task: Arc<TaskControlBlock>) {
//     TASK_MANAGER.exclusive_access().add(task);
// }

// pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
//     TASK_MANAGER.exclusive_access().fetch()
// }