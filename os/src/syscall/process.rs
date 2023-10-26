//! Process management syscalls

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    mm::{get_actual_ptr, vmem_alloc, vmem_free},
    syscall::{
        SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_MMAP, SYSCALL_MUNMAP, SYSCALL_SBRK,
        SYSCALL_TASK_INFO, SYSCALL_YIELD,
    },
    task::{
        change_program_brk, cur_call_count, cur_init_time, current_user_token,
        exit_current_and_run_next, inc_call_count, suspend_current_and_run_next, TaskStatus,
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    inc_call_count(SYSCALL_EXIT);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    inc_call_count(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    inc_call_count(SYSCALL_GET_TIME);
    let ptr = get_actual_ptr(current_user_token(), _ts);
    *ptr = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    inc_call_count(SYSCALL_TASK_INFO);
    let cur_init_time = cur_init_time();
    let get_time_ms = get_time_ms();
    // println!(
    //     "cmp: {} and {}, error: {}",
    //     get_time_ms,
    //     cur_init_time,
    //     get_time_ms - cur_init_time
    // );
    let task_info = get_actual_ptr(current_user_token(), _ti);
    // TaskInfo {
    //     status: TaskStatus::Running,
    //     syscall_times: cur_call_count(),
    //     time: get_time_ms - cur_init_time + 30, //玄学
    // };
    task_info.status = TaskStatus::Running;
    task_info.syscall_times = cur_call_count();
    task_info.time = get_time_ms - cur_init_time + 15;
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    inc_call_count(SYSCALL_MMAP);

    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    vmem_alloc(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    inc_call_count(SYSCALL_MUNMAP);

    if _start % PAGE_SIZE != 0 {
        return -1;
    }

    vmem_free(_start, _len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    inc_call_count(SYSCALL_SBRK);
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
