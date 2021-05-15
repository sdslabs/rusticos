use crate::println;
use alloc::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum SyscallIndex {
    Dummy0 = 0x595ca11a,
    Dummy1 = 0x595ca11b,
}

struct Syscall {
    pub func: fn(u64, u64, u64, u64) -> i64,
}

impl Syscall {
    fn new(f: fn(u64, u64, u64, u64) -> i64) -> Syscall {
        Syscall { func: f }
    }

    pub fn execute(&self, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
        (self.func)(arg0, arg1, arg2, arg3)
    }
}

#[inline(never)]
fn dummy_syscall_0(a: u64, b: u64, c: u64, d: u64) -> i64 {
    println!("sys0 {:x} {:x} {:x} {:x}", a, b, c, d);
    123
}

#[inline(never)]
fn dummy_syscall_1(a: u64, b: u64, c: u64, d: u64) -> i64 {
    println!("sys1 {:x} {:x} {:x} {:x}", a, b, c, d);
    456
}

pub fn process_syscalls(addr: u64, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let syscall = SYSCALLS.get(&addr);
    match syscall {
        Some(handler) => handler.execute(arg0, arg1, arg2, arg3),
        _ => -1,
    }
}

lazy_static! {
    static ref SYSCALLS: BTreeMap<u64, Syscall> = {
        let mut syscalls = BTreeMap::new();
        syscalls.insert(SyscallIndex::Dummy0 as u64, Syscall::new(dummy_syscall_0));
        syscalls.insert(SyscallIndex::Dummy1 as u64, Syscall::new(dummy_syscall_1));
        syscalls
    };
}
