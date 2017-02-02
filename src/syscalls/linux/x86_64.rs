#![allow(unused_assignments)]
#![allow(non_snake_case)]

use libc::{c_int, c_char, size_t, c_uint, c_long, pollfd, off_t, c_ulong, stat, sigaction, sigset_t, loff_t, iovec, fd_set, timeval, key_t, shmid_ds, timespec, sockaddr, c_void, pid_t, msghdr, rusage, c_uchar, msqid_ds, uid_t, gid_t, mode_t, timezone, sysinfo, rlimit, sched_param, stack_t, utimbuf, statfs, time_t, clockid_t, epoll_event, mq_attr, mqd_t, rlimit64};
use types::{itimerval, file_handle, sembuf, msgbuf, linux_dirent, cap_user_header_t, cap_user_data_t, old_utsname, ustat, __sysctl_args, timex, pt_regs, qid_t, mmsghdr, getcpu_cache, kexec_segment, key_serial_t, robust_list_head, aio_context_t, io_event, iocb, linux_dirent64, timer_t, itimerspec};

macro_rules! syscall {
    ($id:expr, $name:ident) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name() -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a) -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b) -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c) -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d) -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e) -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) "{r8}"(e) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
    ($id:expr, $name:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {
        #[inline(always)]
        #[no_mangle]
        pub unsafe extern fn $name(a: $a, b: $b, c: $c, d: $d, e: $e, f:$f) -> c_int {
            let mut ret: c_int = $id;
            asm!("syscall" :
                 "+{rax}"(ret) :
                 "{rdi}"(a), "{rsi}"(b), "{rdx}"(c), "{r10}"(d) "{r8}"(e), "{r9}"(f) :
                 "rdi", "rsi", "rdx", "rcx", "r8", "r9", "r10", "r11", "memory" :
                 "volatile");
            ret
        }
    };
}

syscall!(000, sys_read, c_uint, *mut c_char, size_t);
syscall!(001, sys_write, c_uint, *const c_char, size_t);
syscall!(002, sys_open, *const c_char, c_int, c_int);
syscall!(003, sys_close, c_uint);
syscall!(004, sys_stat, *const c_char, *mut stat);
syscall!(005, sys_fstat, c_uint, *mut stat);
syscall!(006, sys_lstat, *const c_char, *mut stat);
syscall!(007, sys_poll, *mut pollfd, c_uint, c_long);
syscall!(008, sys_lseek, c_uint, off_t, c_uint);
syscall!(009,
         sys_mmap,
         c_ulong,
         c_ulong,
         c_ulong,
         c_ulong,
         c_ulong,
         c_ulong);
syscall!(010, sys_mprotect, c_ulong, size_t, c_ulong);
syscall!(011, sys_munmap, c_ulong, size_t);
syscall!(012, sys_brk, c_ulong);
syscall!(013, sys_rt_sigaction, c_int, *const sigaction, *mut sigaction, size_t);
syscall!(014, sys_rt_sigprocmask, c_int, *mut sigset_t, *mut sigset_t, size_t);
syscall!(015, sys_rt_sigreturn, c_ulong);
syscall!(016, sys_ioctl, c_uint, c_uint, c_ulong);
syscall!(017, sys_pread64, c_ulong, *mut c_char, size_t, loff_t);
syscall!(018, sys_pwrite64, c_uint, *const c_char, size_t, loff_t);
syscall!(019, sys_readv, c_ulong, *const iovec, c_ulong);
syscall!(020, sys_writev, c_ulong, *const iovec, c_ulong);
syscall!(021, sys_access, *const c_char, c_int);
syscall!(022, sys_pipe, *mut c_int);
syscall!(023, sys_select, c_int, *mut fd_set, *mut fd_set, *mut fd_set, *mut timeval);
syscall!(024, sys_sched_yield);
syscall!(025, sys_mremap, c_ulong, c_ulong, c_ulong, c_ulong, c_ulong);
syscall!(026, sys_msync, c_ulong, size_t, c_int);
syscall!(027, sys_mincore, c_ulong, size_t, *mut c_uchar);
syscall!(028, sys_madvise, c_ulong, size_t, c_int);
syscall!(029, sys_shmget, key_t, size_t, c_int);
syscall!(030, sys_shmat, c_int, *mut c_char, c_int);
syscall!(031, sys_shmctl, c_int, c_int, *mut shmid_ds);
syscall!(032, sys_dup, c_uint);
syscall!(033, sys_dup2, c_uint, c_uint);
syscall!(034, sys_pause);
syscall!(035, sys_nanosleep, *mut timespec, *mut timespec);
syscall!(036, sys_getitimer, c_int, *mut itimerval);
syscall!(037, sys_alarm, c_uint);
syscall!(038, sys_setitimer, c_int, *mut itimerval, *mut itimerval);
syscall!(039, sys_getpid);
syscall!(040, sys_sendfile, c_int, c_int, *mut off_t, size_t);
syscall!(041, sys_socket, c_int, c_int, c_int);
syscall!(042, sys_connect, c_int, *mut sockaddr, c_int);
syscall!(043, sys_accept, c_int, *mut sockaddr, *mut c_int);
syscall!(044, sys_sendto, c_int, *mut c_void, size_t, c_uint, *mut sockaddr, c_int);
syscall!(045, sys_recvfrom, c_int, *mut c_void, size_t, c_uint, *mut sockaddr, *mut c_int);
syscall!(046, sys_sendmsg, c_int, *mut msghdr, c_uint);
syscall!(047, sys_recvmsg, c_int, *mut msghdr, c_uint);
syscall!(048, sys_shutdown, c_int, c_int);
syscall!(049, sys_bind, c_int, *mut sockaddr, c_int);
syscall!(050, sys_listen, c_int, c_int);
syscall!(051, sys_getsockname, c_int, *mut sockaddr, *mut c_int);
syscall!(052, sys_getpeername, c_int, *mut sockaddr, *mut c_int);
syscall!(053, sys_socketpair, c_int, c_int, c_int, *mut c_int);
syscall!(054, sys_setsockopt, c_int, c_int, c_int, *mut c_char, c_int);
syscall!(055, sys_getsockopt, c_int, c_int, c_int, *mut c_char, *mut c_int);
syscall!(056, sys_clone, c_ulong, c_ulong, *mut c_void, *mut c_void);
syscall!(057, sys_fork);
syscall!(058, sys_vfork);
syscall!(059, sys_execve, *const c_char, *const *const c_char, *const *const c_char);
syscall!(060, sys_exit, c_int);
syscall!(061, sys_wait4, pid_t, *mut c_int, c_int, *mut rusage);
syscall!(062, sys_kill, pid_t, c_int);
syscall!(063, sys_uname, *mut old_utsname);
syscall!(064, sys_semget, key_t, c_int, c_int);
syscall!(065, sys_semop, c_int, *mut sembuf, c_uint);
// TODO syscall!(066, sys_semctl, c_int, c_int, c_int, union semun);
syscall!(067, sys_shmdt, *mut c_char);
syscall!(068, sys_msgget, key_t, c_int);
syscall!(069, sys_msgsnd, c_int, *mut msgbuf, size_t, c_int);
syscall!(070, sys_msgrcv, c_int, *mut msgbuf, size_t, c_long, c_int);
syscall!(071, sys_msgctl, c_int, c_int, *mut msqid_ds);
syscall!(072, sys_fcntl, c_uint, c_uint, c_ulong);
syscall!(073, sys_flock, c_uint, c_uint);
syscall!(074, sys_fsync, c_uint);
syscall!(075, sys_fdatasync, c_uint);
syscall!(076, sys_truncate, *const c_char, c_long);
syscall!(077, sys_ftruncate, c_uint, c_ulong);
syscall!(078, sys_getdents, c_uint, *mut linux_dirent, c_uint);
syscall!(079, sys_getcwd, *mut c_char, c_ulong);
syscall!(080, sys_chdir, *const c_char);
syscall!(081, sys_fchdir, c_uint);
syscall!(082, sys_rename, *const c_char, *const c_char);
syscall!(083, sys_mkdir, *const c_char, c_int);
syscall!(084, sys_rmdir, *const c_char);
syscall!(085, sys_creat, *const c_char, c_int);
syscall!(086, sys_link, *const c_char, *const c_char);
syscall!(087, sys_unlink, *const c_char);
syscall!(088, sys_symlink, *const c_char, *const c_char);
syscall!(089, sys_readlink, *const c_char, *mut c_char, c_int);
syscall!(090, sys_chmod, *const c_char, mode_t);
syscall!(091, sys_fchmod, c_uint, mode_t);
syscall!(092, sys_chown, *const c_char, uid_t, gid_t);
syscall!(093, sys_fchown, c_uint, uid_t, gid_t);
syscall!(094, sys_lchown, *const c_char, uid_t, gid_t);
syscall!(095, sys_umask, c_int);
syscall!(096, sys_gettimeofday, *mut timeval, *mut timezone);
syscall!(097, sys_getrlimit, c_uint, *mut rlimit);
syscall!(098, sys_getrusage, c_int, *mut rusage);
syscall!(099, sys_sysinfo, *mut sysinfo);
syscall!(100, sys_times, *mut sysinfo);
syscall!(101, sys_ptrace, c_long, c_long, c_ulong, c_ulong);
syscall!(102, sys_getuid);
syscall!(103, sys_syslog, c_int, *mut c_char, c_int);
syscall!(104, sys_getgid);
syscall!(105, sys_setuid, uid_t);
syscall!(106, sys_setgid, gid_t);
syscall!(107, sys_geteuid);
syscall!(108, sys_getegid);
syscall!(109, sys_setpgid, pid_t, pid_t);
syscall!(110, sys_getppid);
syscall!(111, sys_getpgrp);
syscall!(112, sys_setsid);
syscall!(113, sys_setreuid, uid_t, uid_t);
syscall!(114, sys_setregid, gid_t, gid_t);
syscall!(115, sys_getgroups, c_int, *mut gid_t);
syscall!(116, sys_setgroups, c_int, *mut gid_t);
syscall!(117, sys_setresuid, *mut uid_t, *mut uid_t, *mut uid_t);
syscall!(118, sys_getresuid, *mut uid_t, *mut uid_t, *mut uid_t);
syscall!(119, sys_setresgid, gid_t, gid_t, gid_t);
syscall!(120, sys_getresgid, *mut gid_t, *mut gid_t, *mut gid_t);
syscall!(121, sys_getpgid, pid_t);
syscall!(122, sys_setfsuid, uid_t);
syscall!(123, sys_setfsgid, gid_t);
syscall!(124, sys_getsid, pid_t);
syscall!(125, sys_capget, cap_user_header_t, cap_user_data_t);
syscall!(126, sys_capset, cap_user_header_t, *const cap_user_data_t);
syscall!(127, sys_rt_sigpending, *mut sigset_t, size_t);
// TODO syscall!(128, sys_rt_sigtimedwait, *sigset_t, *mut siginfo_t, *timespec, size_t);
// TODO syscall!(129, sys_rt_sigqueueinfo, pid_t, c_int, *mut siginfo_t);
syscall!(130, sys_rt_sigsuspend, *mut sigset_t, size_t);
syscall!(131, sys_sigaltstack, *const stack_t, *mut stack_t);
syscall!(132, sys_utime, *mut c_char, *mut utimbuf);
syscall!(133, sys_mknod, *const c_char, c_int, c_uint);
// syscall!(134, sys_uselib, NOT);
syscall!(135, sys_personality, c_uint);
syscall!(136, sys_ustat, c_uint, *mut ustat);
syscall!(137, sys_statfs, *const c_char, *mut statfs);
syscall!(138, sys_fstatfs, c_uint, *mut statfs);
syscall!(139, sys_sysfs, c_int, c_ulong, c_ulong);
syscall!(140, sys_getpriority, c_int, c_int);
syscall!(141, sys_setpriority, c_int, c_int, c_int);
syscall!(142, sys_sched_setparam, pid_t, *mut sched_param);
syscall!(143, sys_sched_getparam, pid_t, *mut sched_param);
syscall!(144, sys_sched_setscheduler, pid_t, c_int, *mut sched_param);
syscall!(145, sys_sched_getscheduler, pid_t);
syscall!(146, sys_sched_get_priority_max, c_int);
syscall!(147, sys_sched_get_priority_min, c_int);
syscall!(148, sys_sched_rr_get_c_interval, pid_t, *mut timespec);
syscall!(149, sys_mlock, c_ulong, size_t);
syscall!(150, sys_munlock, c_ulong, size_t);
syscall!(151, sys_mlockall, c_int);
syscall!(152, sys_munlockall);
syscall!(153, sys_vhangup);
syscall!(154, sys_modify_ldt, c_int, *mut c_void, c_ulong);
syscall!(155, sys_pivot_root, *const c_char, *const c_char);
syscall!(156, sys__sysctl, *mut __sysctl_args);
// This causes llvm to segfault syscall!(157, sys_prctl, c_int, c_ulong, c_ulong, c_ulong, ();, c_ulong)
// TODO syscall!(158, sys_arch_prctl, *mut task_struct, c_int, *mut c_ulong);
syscall!(159, sys_adjtimex, *mut timex);
syscall!(160, sys_setrlimit, c_uint, *mut rlimit);
syscall!(161, sys_chroot, *const c_char);
syscall!(162, sys_sync);
syscall!(163, sys_acct, *const c_char);
syscall!(164, sys_settimeofday, *mut timeval, *mut timezone);
syscall!(165, sys_mount, *mut c_char, *mut c_char, *mut c_char, c_ulong, *mut c_void);
syscall!(166, sys_umount2, *const c_char, c_int);
syscall!(167, sys_swapon, *const c_char, c_int);
syscall!(168, sys_swapoff, *const c_char);
syscall!(169, sys_reboot, c_int, c_int, c_uint, *mut c_void);
syscall!(170, sys_sethostname, *mut c_char, c_int);
syscall!(171, sys_setdomainname, *mut c_char, c_int);
syscall!(172, sys_iopl, c_uint, *mut pt_regs);
syscall!(173, sys_ioperm, c_ulong, c_ulong, c_int);
// syscall!(174, sys_create_module, REMOVED IN Linux 2.);
syscall!(175, sys_init_module, *mut c_void, c_ulong, *const c_char);
syscall!(176, sys_delete_module, *const c_char, c_uint);
// syscall!(177, sys_get_kernel_syms, REMOVED IN Linux 2.);
// syscall!(178, sys_query_module, REMOVED IN Linux 2.);
syscall!(179, sys_quotactl, c_uint, *const c_char, qid_t, *mut c_void);
// syscall!(180, sys_nfsservctl, NOT);
// syscall!(181, sys_getpmsg, NOT);
// syscall!(182, sys_putpmsg, NOT);
// syscall!(183, sys_afs_syscall, NOT);
// syscall!(184, sys_tuxcall, NOT);
// syscall!(185, sys_security, NOT);
syscall!(186, sys_gettid);
syscall!(187, sys_readahead, c_int, loff_t, size_t);
syscall!(188, sys_setxattr, *const c_char, *const c_char, *const c_void, size_t, c_int);
syscall!(189, sys_lsetxattr, *const c_char, *const c_char, *const c_void, size_t, c_int);
syscall!(190, sys_fsetxattr, c_int, *const c_char, *const c_void, size_t, c_int);
syscall!(191, sys_getxattr, *const c_char, *const c_char, *mut c_void, size_t);
syscall!(192, sys_lgetxattr, *const c_char, *const c_char, *mut c_void, size_t);
syscall!(193, sys_fgetxattr, c_int, *const c_char, *mut c_void, size_t);
syscall!(194, sys_listxattr, *const c_char, *mut c_char, size_t);
syscall!(195, sys_llistxattr, *const c_char, *mut c_char, size_t);
syscall!(196, sys_flistxattr, c_int, *mut c_char, size_t);
syscall!(197, sys_removexattr, *const c_char, *const c_char);
syscall!(198, sys_lremovexattr, *const c_char, *const c_char);
syscall!(199, sys_fremovexattr, c_int, *const c_char);
// TODO syscall!(200, sys_tkill, pid_t, ing);
syscall!(201, sys_time, *mut time_t);
syscall!(202, sys_futex, *mut u32, c_int, u32, *mut timespec, *mut u32, u32);
syscall!(203, sys_sched_setaffinity, pid_t, c_uint, *mut c_ulong);
syscall!(204, sys_sched_getaffinity, pid_t, c_uint, *mut c_ulong);
// syscall!(205, sys_set_thread_area, NOT IMPLEMENTED. Use);
syscall!(206, sys_io_setup, c_uint, *mut aio_context_t);
syscall!(207, sys_io_destroy, aio_context_t);
syscall!(208, sys_io_getevents, aio_context_t, c_long, c_long, *mut io_event);
syscall!(209, sys_io_submit, aio_context_t, c_long, *mut *mut iocb);
syscall!(210, sys_io_cancel, aio_context_t, *mut iocb, *mut *mut io_event);
// syscall!(211, sys_get_thread_area, NOT IMPLEMENTED. Use);
syscall!(212, sys_lookup_dcookie, u64, c_long, c_long);
syscall!(213, sys_epoll_create, c_int);
// syscall!(214, sys_epoll_ctl_old, NOT);
// syscall!(215, sys_epoll_wait_old, NOT);
syscall!(216,
         sys_remap_file_pages,
         c_ulong,
         c_ulong,
         c_ulong,
         c_ulong,
         c_ulong);
syscall!(217, sys_getdents64, c_uint, *mut linux_dirent64, c_uint);
syscall!(218, sys_set_tid_address, *mut c_int);
syscall!(219, sys_restart_syscall);
syscall!(220, sys_semtimedop, c_int, *mut sembuf, c_uint, *const timespec);
syscall!(221, sys_fadvise64, c_int, loff_t, size_t, c_int);
// TODO syscall!(222, sys_timer_create, *clockid_t, *mut sigevent, *mut timer_t);
syscall!(223, sys_timer_settime, timer_t, c_int, *const itimerspec, *mut itimerspec);
syscall!(224, sys_timer_gettime, timer_t, *mut itimerspec);
syscall!(225, sys_timer_getoverrun, timer_t);
syscall!(226, sys_timer_delete, timer_t);
syscall!(227, sys_clock_settime, *const clockid_t, *const timespec);
syscall!(228, sys_clock_gettime, *const clockid_t, *mut timespec);
syscall!(229, sys_clock_getres, *const clockid_t, *mut timespec);
syscall!(230, sys_clock_nanosleep, *const clockid_t, c_int, *const timespec, *mut timespec);
syscall!(231, sys_exit_group, c_int);
syscall!(232, sys_epoll_wait, c_int, *mut epoll_event, c_int, c_int);
syscall!(233, sys_epoll_ctl, c_int, c_int, c_int, *mut epoll_event);
syscall!(234, sys_tgkill, pid_t, pid_t, c_int);
syscall!(235, sys_utimes, *const c_char, *mut timeval); // WARNING *mut c_char
// syscall!(236, sys_vserver, NOT);
syscall!(237, sys_mbind, c_ulong, c_ulong, c_ulong, *mut c_ulong, c_ulong, c_uint);
syscall!(238, sys_set_mempolicy, c_int, *mut c_ulong, c_ulong);
syscall!(239, sys_get_mempolicy, *mut c_int, *mut c_ulong, c_ulong, c_ulong, c_ulong);
syscall!(240, sys_mq_open, *const c_char, c_int, mode_t, *mut mq_attr);
syscall!(241, sys_mq_unlink, *const c_char);
syscall!(242, sys_mq_timedsend, mqd_t, *const c_char, size_t, c_uint, *const timespec);
syscall!(243, sys_mq_timedreceive, mqd_t, *mut c_char, size_t, *mut c_uint, *const timespec);
// TODO syscall!(244, sys_mq_notify, mqd_t, *sigevent);
syscall!(245, sys_mq_getsetattr, mqd_t, *const mq_attr, *mut mq_attr);
syscall!(246, sys_kexec_load, c_ulong, c_ulong, *mut kexec_segment, c_ulong);
// TODO syscall!(247, sys_waitid, c_int, pid_t, *mut siginfo, c_int, *mut rusage);
syscall!(248, sys_add_key, *const c_char, *const c_char, *const c_void, size_t);
syscall!(249, sys_request_key, *const c_char, *const c_char, *const c_char, key_serial_t);
syscall!(250, sys_keyctl, c_int, c_ulong, c_ulong, c_ulong, c_ulong);
syscall!(251, sys_ioprio_set, c_int, c_int, c_int);
syscall!(252, sys_ioprio_get, c_int, c_int);
syscall!(253, sys_inotify_init);
syscall!(254, sys_inotify_add_watch, c_int, *const c_char, u32);
syscall!(255, sys_inotify_rm_watch, c_int, i32);
syscall!(256, sys_migrate_pages, pid_t, c_ulong, *const c_ulong, *const c_ulong);
syscall!(257, sys_openat, c_int, *const c_char, c_int, c_int);
syscall!(258, sys_mkdirat, c_int, *const c_char, c_int);
syscall!(259, sys_mknodat, c_int, *const c_char, c_int, c_uint);
syscall!(260, sys_fchownat, c_int, *const c_char, uid_t, gid_t, c_int);
syscall!(261, sys_futimesat, c_int, *const c_char, *mut timeval);
syscall!(262, sys_newfstatat, c_int, *const c_char, *mut stat, c_int);
syscall!(263, sys_unlinkat, c_int, *const c_char, c_int);
syscall!(264, sys_renameat, c_int, *const c_char, c_int, *const c_char);
syscall!(265, sys_linkat, c_int, *const c_char, c_int, *const c_char, c_int);
syscall!(266, sys_symlinkat, *const c_char, c_int, *const c_char);
syscall!(267, sys_readlinkat, c_int, *const c_char, *mut c_char, c_int);
syscall!(268, sys_fchmodat, c_int, *const c_char, mode_t);
syscall!(269, sys_faccessat, c_int, *const c_char, c_int);
syscall!(270, sys_pselect6, c_int, *mut fd_set, *mut fd_set, *mut fd_set, *mut timespec,
         *mut c_void);
syscall!(271, sys_ppoll, *mut pollfd, c_uint, *mut timespec, *const sigset_t, size_t);
syscall!(272, sys_unshare, c_ulong);
syscall!(273, sys_set_robust_list, *mut robust_list_head, size_t);
syscall!(274, sys_get_robust_list, c_int, *mut *mut robust_list_head, *mut size_t);
syscall!(275, sys_splice, c_int, *mut loff_t, c_int, *mut loff_t, size_t, c_uint);
syscall!(276, sys_tee, c_int, c_int, size_t, c_uint);
syscall!(277, sys_sync_file_range, c_long, loff_t, loff_t, c_long);
syscall!(278, sys_vmsplice, c_int, *const iovec, c_ulong, c_uint);
syscall!(279, sys_move_pages, pid_t, c_ulong, *mut *const c_void, *const c_int, *mut c_int, c_int);
syscall!(280, sys_utimensat, c_int, *const c_char, *mut timespec, c_int);
syscall!(281, sys_epoll_pwait, c_int, *mut epoll_event, c_int, c_int, *const sigset_t, size_t);
syscall!(282, sys_signalfd, c_int, *mut sigset_t, size_t);
syscall!(283, sys_timerfd_create, c_int, c_int);
syscall!(284, sys_eventfd, c_uint);
syscall!(285, sys_fallocate, c_long, c_long, loff_t, loff_t);
syscall!(286, sys_timerfd_settime, c_int, c_int, *const itimerspec, *mut itimerspec);
syscall!(287, sys_timerfd_gettime, c_int, *mut itimerspec);
syscall!(288, sys_accept4, c_int, *mut sockaddr, *mut c_int, c_int);
syscall!(289, sys_signalfd4, c_int, *mut sigset_t, size_t, c_int);
syscall!(290, sys_eventfd2, c_uint, c_int);
syscall!(291, sys_epoll_create1, c_int);
syscall!(292, sys_dup3, c_uint, c_uint, c_int);
syscall!(293, sys_pipe2, *mut c_int, c_int);
syscall!(294, sys_inotify_init1, c_int);
syscall!(295, sys_preadv, c_ulong, *const iovec, c_ulong, c_ulong, c_ulong);
syscall!(296, sys_pwritev, c_ulong, *const iovec, c_ulong, c_ulong, c_ulong);
// TODO syscall!(297, sys_rt_tgsigqueueinfo, pid_t, pid_t, c_int, *mut siginfo_t);
// TODO syscall!(298, sys_perf_event_open, *mut perf_event_attr, pid_t, c_int, c_int, c_ulong);
syscall!(299, sys_recvmmsg, c_int, *mut msghdr, c_uint, c_uint, *mut timespec);
syscall!(300, sys_fanotify_init, c_uint, c_uint);
syscall!(301, sys_fanotify_mark, c_long, c_long, u64, c_long, c_long);
syscall!(302, sys_prlimit64, pid_t, c_uint, *const rlimit64, *mut rlimit64);
syscall!(303, sys_name_to_handle_at, c_int, *const c_char, *mut file_handle, *mut c_int, c_int);
syscall!(304, sys_open_by_handle_at, c_int, *const c_char, *mut file_handle, *mut c_int, c_int);
syscall!(305, sys_clock_adjtime, clockid_t, *mut timex);
syscall!(306, sys_syncfs, c_int);
syscall!(307, sys_sendmmsg, c_int, *mut mmsghdr, c_uint, c_uint);
syscall!(308, sys_setns, c_int, c_int);
syscall!(309, sys_getcpu, *mut c_uint, *mut c_uint, *mut getcpu_cache);
syscall!(310, sys_process_vm_readv, pid_t, *const iovec, c_ulong, *const iovec, c_ulong, c_ulong);
syscall!(311, sys_process_vm_writev, pid_t, *const iovec, c_ulong, *const iovec, c_ulong, c_ulong);
