use libc::{c_int, c_char};

include!(concat!("intercdep.rs"));

#[no_mangle]
pub extern "C" fn r_times_builtin(list: *mut WORD_LIST) -> i32 {
    println!("r_times_builtin call");

unsafe {

    if no_options(list) != 0 {
        return EX_USAGE;
    }

    let mut curr: libc::rusage = std::mem::zeroed();
    let mut kids: libc::rusage = std::mem::zeroed();
    libc::putchar(b'\n' as c_int);

    libc::getrusage(libc::RUSAGE_SELF, std::mem::transmute(&curr));
    libc::getrusage(libc::RUSAGE_CHILDREN, std::mem::transmute(&kids));

    print_timeval(stdout, std::mem::transmute(&curr.ru_utime));
    libc::putchar(b' ' as c_int);
    print_timeval(stdout, std::mem::transmute(&curr.ru_stime));
    libc::putchar(b'\n' as c_int);

    print_timeval(stdout, std::mem::transmute(&kids.ru_utime));
    libc::putchar(b' ' as c_int);
    print_timeval(stdout, std::mem::transmute(&kids.ru_stime));
    libc::putchar(b'\n' as c_int);

    return sh_chkwrite(EXECUTION_SUCCESS);
}
}
