//!Implementation of [`TaskManager`]

use core::usize;

use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use lazy_static::*;

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    pub task_vec: VecDeque<Arc<TaskControlBlock>>,
    pub stride_map: BTreeMap<usize, (usize, usize)>,
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

    pub fn check_map_manager(&mut self, pid: usize) {
        let contains_key = self.stride_map.contains_key(&pid);
        if !contains_key {
            self.stride_map.insert(pid, (0, 16));
        }
    }

    /// Take a process out of the ready queue
    pub fn fetch_min(&mut self) -> Option<Arc<TaskControlBlock>> {
        let btree_map = &self.stride_map;
        // if btree_map.len() != 0 && self.task_vec.len() == 1 && self.task_vec[0].pid.0 == 0 {
        //     return None;
        // }
        // btree_map
        //     .into_iter()
        //     .map(|c| (c.0, c.1 .0))
        //     .for_each(|t| print!("({},{})", t.0, t.1));
        match btree_map.into_iter().min_by(|l, r| l.1 .0.cmp(&r.1 .0)) {
            Some(min) => self.task_vec.remove(min.0.clone()),
            None => {
                self.task_vec
                    .clone()
                    .iter()
                    .for_each(|t| self.check_map_manager(t.pid.0.clone()));
                println!("all inited");
                self.fetch_min()
            }
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
    trace!("kernel: TaskManager::add_task");
    println!("added {}", task.pid.0);
    let mut exclusive_access = TASK_MANAGER.exclusive_access();
    exclusive_access.check_map_manager(task.getpid());
    exclusive_access.add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    trace!("kernel: TaskManager::fetch_task");
    let fetch_min = TASK_MANAGER.exclusive_access().fetch_min();
    fetch_min
}
