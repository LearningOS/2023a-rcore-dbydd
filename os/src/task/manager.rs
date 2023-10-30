//!Implementation of [`TaskManager`]

use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use lazy_static::*;

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    pub task_vec: VecDeque<Arc<TaskControlBlock>>,
    pub stride_map: BTreeMap<usize, (isize, isize)>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            task_vec: VecDeque::new(),
            stride_map: BTreeMap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.task_vec.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch_min(&mut self) -> Option<Arc<TaskControlBlock>> {
        match (&self.stride_map)
            .into_iter()
            .min_by(|l, r| l.1 .0.cmp(&r.1 .0))
        {
            Some(min) => self.task_vec.remove(min.0.clone()),
            None => None,
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn find_and_op<T>(pid: isize, fun: T) -> isize
where
    T: Fn(&Arc<TaskControlBlock>) -> isize,
{
    fun(TASK_MANAGER
        .exclusive_access()
        .task_vec
        .iter()
        .find(|t| t.getpid() as isize == pid)
        .unwrap())
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch_min()
}
