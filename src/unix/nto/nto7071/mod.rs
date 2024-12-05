use unix::nto::*;


pub type pthread_mutex_t = ::sync_t;
pub type pthread_mutexattr_t = ::_sync_attr;
pub type pthread_cond_t = ::sync_t;
pub type pthread_condattr_t = ::_sync_attr;
pub type pthread_rwlockattr_t = ::_sync_attr;
pub type pthread_key_t = ::c_int;
pub type pthread_spinlock_t = sync_t;
pub type pthread_barrierattr_t = _sync_attr;
pub type sem_t = sync_t;
pub type tcflag_t = ::c_uint;
pub type sa_family_t = u8;

s! {
    pub struct unpcbid {
        pub unp_pid: ::pid_t,
        pub unp_euid: ::uid_t,
        pub unp_egid: ::gid_t,
    }

    pub struct bpf_stat {
        pub bs_recv: u64,
        pub bs_drop: u64,
        pub bs_capt: u64,
        bs_padding: [u64; 13],
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [i8; 8],
    }

    pub struct in_pktinfo {
        pub ipi_addr: ::in_addr,
        pub ipi_ifindex: ::c_uint,
    }

    pub struct _sync_attr {
        pub __protocol: ::c_int,
        pub __flags: ::c_int,
        pub __prioceiling: ::c_int,
        pub __clockid: ::c_int,
        pub __count: ::c_int,
        __reserved: [::c_int; 3],
    }

    pub struct mmsghdr {
        pub msg_hdr: ::msghdr,
        pub msg_len: ::c_uint,
    }

    pub struct mallinfo {
        pub arena: ::c_int,
        pub ordblks: ::c_int,
        pub smblks: ::c_int,
        pub hblks: ::c_int,
        pub hblkhd: ::c_int,
        pub usmblks: ::c_int,
        pub fsmblks: ::c_int,
        pub uordblks: ::c_int,
        pub fordblks: ::c_int,
        pub keepcost: ::c_int,
    }

    #[repr(packed)]
    pub struct arphdr {
        pub ar_hrd: u16,
        pub ar_pro: u16,
        pub ar_hln: u8,
        pub ar_pln: u8,
        pub ar_op: u16,
    }

    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct qtime_entry {
        pub cycles_per_sec: u64,
        pub nsec_tod_adjust: u64, // volatile
        pub nsec: u64,            // volatile
        pub nsec_inc: u32,
        pub boot_time: u32,
        pub adjust: _clockadjust,
        pub timer_rate: u32,
        pub timer_scale: i32,
        pub timer_load: u32,
        pub intr: i32,
        pub epoch: u32,
        pub flags: u32,
        pub rr_interval_mul: u32,
        pub timer_load_hi: u32,
        pub nsec_stable: u64,      // volatile
        pub timer_load_max: u64,
        pub timer_prog_time: u32,
        spare: [u32; 7],
    }

    pub struct _thread_attr {
        pub __flags: ::c_int,
        pub __stacksize: ::size_t,
        pub __stackaddr: *mut ::c_void,
        pub __exitfunc: ::Option<unsafe extern "C" fn(_fake: *mut ::c_void)>,
        pub __policy: ::c_int,
        pub __param: ::__sched_param,
        pub __guardsize: ::c_uint,
        pub __prealloc: ::c_uint,
        __spare: [::c_int; 2],
    }

    pub struct sockaddr_dl {
        pub sdl_len: ::c_uchar,
        pub sdl_family: ::sa_family_t,
        pub sdl_index: u16,
        pub sdl_type: ::c_uchar,
        pub sdl_nlen: ::c_uchar,
        pub sdl_alen: ::c_uchar,
        pub sdl_slen: ::c_uchar,
        pub sdl_data: [::c_char; 12],
    }
}

s_no_extra_traits! {
    pub struct pthread_rwlock_t {
        pub __active: ::c_int,
        pub __blockedwriters: ::c_int,
        pub __blockedreaders: ::c_int,
        pub __heavy: ::c_int,
        pub __lock: ::pthread_mutex_t,     // union
        pub __rcond: ::pthread_cond_t,     // union
        pub __wcond: ::pthread_cond_t,     // union
        pub __owner: ::c_uint,
        pub __spare: ::c_uint,
    }

    pub struct sync_t {
        __u: ::c_uint,                     // union
        pub __owner: ::c_uint,
    }

    pub struct syspage_entry_info {
        pub entry_off: u16,
        pub entry_size: u16,
    }
    pub struct syspage_array_info {
        entry_off: u16,
        entry_size: u16,
        element_size: u16,
    }

    #[repr(align(8))]
    pub struct syspage_entry {
        pub size: u16,
        pub total_size: u16,
        pub type_: u16,
        pub num_cpu: u16,
        pub system_private: syspage_entry_info,
        pub old_asinfo: syspage_entry_info,
        pub __mangle_name_to_cause_compilation_errs_meminfo: syspage_entry_info,
        pub hwinfo: syspage_entry_info,
        pub old_cpuinfo: syspage_entry_info,
        pub old_cacheattr: syspage_entry_info,
        pub qtime: syspage_entry_info,
        pub callout: syspage_entry_info,
        pub callin: syspage_entry_info,
        pub typed_strings: syspage_entry_info,
        pub strings: syspage_entry_info,
        pub old_intrinfo: syspage_entry_info,
        pub smp: syspage_entry_info,
        pub pminfo: syspage_entry_info,
        pub old_mdriver: syspage_entry_info,
        spare0: [u32; 1],
        __reserved: [u8; 160], // anonymous union with architecture dependent structs
        pub new_asinfo: syspage_array_info,
        pub new_cpuinfo: syspage_array_info,
        pub new_cacheattr: syspage_array_info,
        pub new_intrinfo: syspage_array_info,
        pub new_mdriver: syspage_array_info,
    }
}

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_UNIX: ::c_int = AF_LOCAL;
pub const AF_INET: ::c_int = 2;
pub const AF_IPX: ::c_int = 23;
pub const AF_APPLETALK: ::c_int = 16;
pub const AF_INET6: ::c_int = 28;
pub const AF_ROUTE: ::c_int = 17;
pub const AF_SNA: ::c_int = 11;
pub const AF_BLUETOOTH: ::c_int = 31;
pub const AF_ISDN: ::c_int = 26;
pub const pseudo_AF_HDRCMPLT: ::c_int = 30;
pub const pseudo_AF_PIP: ::c_int = 25;
pub const pseudo_AF_RTIP: ::c_int = 22;
pub const pseudo_AF_XTP: ::c_int = 19;

pub const PF_ARP: ::c_int = 28;
pub const PF_CCITT: ::c_int = 10;
pub const PF_CHAOS: ::c_int = 5;
pub const PF_CNT: ::c_int = 21;
pub const PF_COIP: ::c_int = 20;
pub const PF_DATAKIT: ::c_int = 9;
pub const PF_DECnet: ::c_int = 12;
pub const PF_DLI: ::c_int = 13;
pub const PF_ECMA: ::c_int = 8;
pub const PF_HYLINK: ::c_int = 15;
pub const PF_IMPLINK: ::c_int = 3;
pub const PF_ISO: ::c_int = 7;
pub const PF_LAT: ::c_int = 14;
pub const PF_LINK: ::c_int = 18;
pub const PF_NATM: ::c_int = 27;
pub const PF_OSI: ::c_int = 7;
pub const PF_PIP: ::c_int = 25;
pub const PF_PUP: ::c_int = 4;
pub const PF_RTIP: ::c_int = 22;
pub const PF_XTP: ::c_int = 19;
pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_UNIX: ::c_int = PF_LOCAL;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_INET: ::c_int = AF_INET;
pub const PF_IPX: ::c_int = AF_IPX;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_INET6: ::c_int = AF_INET6;
pub const pseudo_AF_KEY: ::c_int = 29;
pub const PF_KEY: ::c_int = pseudo_AF_KEY;
pub const PF_ROUTE: ::c_int = AF_ROUTE;
pub const PF_SNA: ::c_int = AF_SNA;
pub const PF_ISDN: ::c_int = AF_ISDN;

pub const IFF_UP: ::c_int = 0x00000001;
pub const IFF_BROADCAST: ::c_int = 0x00000002;
pub const IFF_DEBUG: ::c_int = 0x00000004;
pub const IFF_LOOPBACK: ::c_int = 0x00000008;
pub const IFF_POINTOPOINT: ::c_int = 0x00000010;
pub const IFF_NOTRAILERS: ::c_int = 0x00000020;
pub const IFF_RUNNING: ::c_int = 0x00000040;
pub const IFF_NOARP: ::c_int = 0x00000080;
pub const IFF_PROMISC: ::c_int = 0x00000100;
pub const IFF_ALLMULTI: ::c_int = 0x00000200;
pub const IFF_MULTICAST: ::c_int = 0x00008000;

pub const IP_TOS: ::c_int = 3;
pub const IP_TTL: ::c_int = 4;
pub const IP_HDRINCL: ::c_int = 2;
pub const IP_OPTIONS: ::c_int = 1;
pub const IP_RECVOPTS: ::c_int = 5;
pub const IP_RETOPTS: ::c_int = 8;
pub const IP_PKTINFO: ::c_int = 25;
pub const IP_IPSEC_POLICY_COMPAT: ::c_int = 22;
pub const IP_MULTICAST_IF: ::c_int = 9;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;

pub const IP_DEFAULT_MULTICAST_TTL: ::c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: ::c_int = 1;

pub const IPV6_IPSEC_POLICY_COMPAT: ::c_int = 28;

pub const TCP_NODELAY: ::c_int = 0x01;
pub const TCP_MAXSEG: ::c_int = 0x02;
pub const TCP_MD5SIG: ::c_int = 0x10;
pub const TCP_KEEPALIVE: ::c_int = 0x04;

pub const F_SETOWN: ::c_int = 36;
pub const IFF_ACCEPTRTADV: ::c_int = 0x40000000;
pub const IFF_IP6FORWARDING: ::c_int = 0x20000000;
pub const IFF_LINK0: ::c_int = 0x00001000;
pub const IFF_LINK1: ::c_int = 0x00002000;
pub const IFF_LINK2: ::c_int = 0x00004000;
pub const IFF_OACTIVE: ::c_int = 0x00000400;
pub const IFF_SHIM: ::c_int = 0x80000000;
pub const IFF_SIMPLEX: ::c_int = 0x00000800;
pub const IHFLOW: tcflag_t = 0x00000001;
pub const IIDLE: tcflag_t = 0x00000008;
pub const IP_RECVDSTADDR: ::c_int = 7;
pub const IP_RECVIF: ::c_int = 20;
pub const IPTOS_ECN_NOTECT: u8 = 0x00;
pub const IUCLC: tcflag_t = 0x00000200;
pub const IUTF8: tcflag_t = 0x0004000;

pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_IEEE802: u16 = 6;
pub const ARPHRD_ARCNET: u16 = 7;
pub const ARPHRD_IEEE1394: u16 = 24;

// sysctl.h
pub const HW_MACHINE: ::c_int = 1;
pub const HW_MODEL: ::c_int = 2;
pub const HW_NCPU: ::c_int = 3;
pub const HW_BYTEORDER: ::c_int = 4;
pub const HW_PHYSMEM: ::c_int = 5;
pub const HW_USERMEM: ::c_int = 6;
pub const HW_PAGESIZE: ::c_int = 7;
pub const HW_DISKNAMES: ::c_int = 8;
pub const HW_IOSTATS: ::c_int = 9;
pub const HW_MACHINE_ARCH: ::c_int = 10;
pub const HW_ALIGNBYTES: ::c_int = 11;
pub const HW_CNMAGIC: ::c_int = 12;
pub const HW_PHYSMEM64: ::c_int = 13;
pub const HW_USERMEM64: ::c_int = 14;
pub const HW_IOSTATNAMES: ::c_int = 15;
pub const HW_MAXID: ::c_int = 15;

pub const USER_CS_PATH: ::c_int = 1;
pub const USER_BC_BASE_MAX: ::c_int = 2;
pub const USER_BC_DIM_MAX: ::c_int = 3;
pub const USER_BC_SCALE_MAX: ::c_int = 4;
pub const USER_BC_STRING_MAX: ::c_int = 5;
pub const USER_COLL_WEIGHTS_MAX: ::c_int = 6;
pub const USER_EXPR_NEST_MAX: ::c_int = 7;
pub const USER_LINE_MAX: ::c_int = 8;
pub const USER_RE_DUP_MAX: ::c_int = 9;
pub const USER_POSIX2_VERSION: ::c_int = 10;
pub const USER_POSIX2_C_BIND: ::c_int = 11;
pub const USER_POSIX2_C_DEV: ::c_int = 12;
pub const USER_POSIX2_CHAR_TERM: ::c_int = 13;
pub const USER_POSIX2_FORT_DEV: ::c_int = 14;
pub const USER_POSIX2_FORT_RUN: ::c_int = 15;
pub const USER_POSIX2_LOCALEDEF: ::c_int = 16;
pub const USER_POSIX2_SW_DEV: ::c_int = 17;
pub const USER_POSIX2_UPE: ::c_int = 18;
pub const USER_STREAM_MAX: ::c_int = 19;
pub const USER_TZNAME_MAX: ::c_int = 20;
pub const USER_ATEXIT_MAX: ::c_int = 21;
pub const USER_MAXID: ::c_int = 22;

pub const CTL_UNSPEC: ::c_int = 0;
pub const CTL_KERN: ::c_int = 1;
pub const CTL_VM: ::c_int = 2;
pub const CTL_VFS: ::c_int = 3;
pub const CTL_NET: ::c_int = 4;
pub const CTL_DEBUG: ::c_int = 5;
pub const CTL_HW: ::c_int = 6;
pub const CTL_MACHDEP: ::c_int = 7;
pub const CTL_USER: ::c_int = 8;
pub const CTL_QNX: ::c_int = 9;
pub const CTL_PROC: ::c_int = 10;
pub const CTL_VENDOR: ::c_int = 11;
pub const CTL_EMUL: ::c_int = 12;
pub const CTL_SECURITY: ::c_int = 13;
pub const CTL_MAXID: ::c_int = 14;

pub const KERN_ARGMAX: ::c_int = 8;
pub const KERN_ARND: ::c_int = 81;
pub const KERN_BOOTTIME: ::c_int = 21;
pub const KERN_CLOCKRATE: ::c_int = 12;
pub const KERN_FILE: ::c_int = 15;
pub const KERN_HOSTID: ::c_int = 11;
pub const KERN_HOSTNAME: ::c_int = 10;
pub const KERN_IOV_MAX: ::c_int = 38;
pub const KERN_JOB_CONTROL: ::c_int = 19;
pub const KERN_LOGSIGEXIT: ::c_int = 46;
pub const KERN_MAXFILES: ::c_int = 7;
pub const KERN_MAXID: ::c_int = 83;
pub const KERN_MAXPROC: ::c_int = 6;
pub const KERN_MAXVNODES: ::c_int = 5;
pub const KERN_NGROUPS: ::c_int = 18;
pub const KERN_OSRELEASE: ::c_int = 2;
pub const KERN_OSREV: ::c_int = 3;
pub const KERN_OSTYPE: ::c_int = 1;
pub const KERN_POSIX1: ::c_int = 17;
pub const KERN_PROC: ::c_int = 14;
pub const KERN_PROC_ALL: ::c_int = 0;
pub const KERN_PROC_ARGS: ::c_int = 48;
pub const KERN_PROC_ENV: ::c_int = 3;
pub const KERN_PROC_GID: ::c_int = 7;
pub const KERN_PROC_PGRP: ::c_int = 2;
pub const KERN_PROC_PID: ::c_int = 1;
pub const KERN_PROC_RGID: ::c_int = 8;
pub const KERN_PROC_RUID: ::c_int = 6;
pub const KERN_PROC_SESSION: ::c_int = 3;
pub const KERN_PROC_TTY: ::c_int = 4;
pub const KERN_PROC_UID: ::c_int = 5;
pub const KERN_PROF: ::c_int = 16;
pub const KERN_SAVED_IDS: ::c_int = 20;
pub const KERN_SECURELVL: ::c_int = 9;
pub const KERN_VERSION: ::c_int = 4;
pub const KERN_VNODE: ::c_int = 13;

pub const SO_DEBUG: ::c_int = 0x0001;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_TYPE: ::c_int = 0x1008;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_BINDTODEVICE: ::c_int = 0x0800;
pub const SO_TIMESTAMP: ::c_int = 0x0400;
pub const SO_ACCEPTCONN: ::c_int = 0x0002;

pub const SO_FIB: ::c_int = 0x100a;
pub const SO_OVERFLOWED: ::c_int = 0x1009;
pub const SO_SETFIB: ::c_int = 0x100a;
pub const SO_TXPRIO: ::c_int = 0x100b;
pub const SO_USELOOPBACK: ::c_int = 0x0040;
pub const SO_VLANPRIO: ::c_int = 0x100c;

pub const FIONBIO: ::c_int = -2147195266;
pub const FIOASYNC: ::c_int = -2147195267;
pub const FIOCLEX: ::c_int = 26113;
pub const FIOGETOWN: ::c_int = 1074030203;
pub const FIONCLEX: ::c_int = 26114;
pub const FIONREAD: ::c_int = 1074030207;
pub const FIONSPACE: ::c_int = 1074030200;
pub const FIONWRITE: ::c_int = 1074030201;
pub const FIOSETOWN: ::c_int = -2147195268;

pub const IPTOS_ECN_NOT_ECT: u8 = 0x00;
pub const M_KEEP: ::c_int = 4;

pub const MNT_NOEXEC: ::c_int = 0x02;
pub const MNT_NOSUID: ::c_int = 0x04;
pub const MNT_RDONLY: ::c_int = 0x01;

pub const AF_ARP: ::c_int = 28;
pub const AF_CCITT: ::c_int = 10;
pub const AF_CHAOS: ::c_int = 5;
pub const AF_CNT: ::c_int = 21;
pub const AF_COIP: ::c_int = 20;
pub const AF_DATAKIT: ::c_int = 9;
pub const AF_DECnet: ::c_int = 12;
pub const AF_DLI: ::c_int = 13;
pub const AF_E164: ::c_int = 26;
pub const AF_ECMA: ::c_int = 8;
pub const AF_HYLINK: ::c_int = 15;
pub const AF_IEEE80211: ::c_int = 32;
pub const AF_IMPLINK: ::c_int = 3;
pub const AF_ISO: ::c_int = 7;
pub const AF_LAT: ::c_int = 14;
pub const AF_LINK: ::c_int = 18;
pub const AF_NATM: ::c_int = 27;
pub const AF_NS: ::c_int = 6;
pub const AF_OSI: ::c_int = 7;
pub const AF_PUP: ::c_int = 4;

pub const LOCAL_CONNWAIT: ::c_int = 0x0002;
pub const LOCAL_CREDS: ::c_int = 0x0001;
pub const LOCAL_PEEREID: ::c_int = 0x0003;

pub const STATE_DEAD: ::c_int = 0x00;
pub const STATE_RUNNING: ::c_int = 0x01;
pub const STATE_READY: ::c_int = 0x02;
pub const STATE_STOPPED: ::c_int = 0x03;
pub const STATE_SEND: ::c_int = 0x04;
pub const STATE_RECEIVE: ::c_int = 0x05;
pub const STATE_REPLY: ::c_int = 0x06;
pub const STATE_STACK: ::c_int = 0x07;
pub const STATE_WAITTHREAD: ::c_int = 0x08;
pub const STATE_WAITPAGE: ::c_int = 0x09;
pub const STATE_SIGSUSPEND: ::c_int = 0x0a;
pub const STATE_SIGWAITINFO: ::c_int = 0x0b;
pub const STATE_NANOSLEEP: ::c_int = 0x0c;
pub const STATE_MUTEX: ::c_int = 0x0d;
pub const STATE_CONDVAR: ::c_int = 0x0e;
pub const STATE_JOIN: ::c_int = 0x0f;
pub const STATE_INTR: ::c_int = 0x10;
pub const STATE_SEM: ::c_int = 0x11;
pub const STATE_WAITCTX: ::c_int = 0x12;
pub const STATE_NET_SEND: ::c_int = 0x13;
pub const STATE_NET_REPLY: ::c_int = 0x14;
pub const STATE_MAX: ::c_int = 0x18;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __u: 0x80000000,
    __owner: 0xffffffff,
};

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __active: 0,
    __blockedwriters: 0,
    __blockedreaders: 0,
    __heavy: 0,
    __lock: PTHREAD_MUTEX_INITIALIZER,
    __rcond: PTHREAD_COND_INITIALIZER,
    __wcond: PTHREAD_COND_INITIALIZER,
    __owner: -2i32 as ::c_uint,
    __spare: 0,
};

// Network related functions are provided by libsocket and regex
// functions are provided by libregex.
// In QNX <=7.0, libregex functions were included in libc itself.
#[link(name = "socket")]
#[cfg_attr(not(target_env = "nto70"), link(name = "regex"))]
extern "C" {
    pub fn sem_destroy(sem: *mut sem_t) -> ::c_int;
    pub fn sem_init(sem: *mut sem_t, pshared: ::c_int, value: ::c_uint) -> ::c_int;
    pub fn fdatasync(fd: ::c_int) -> ::c_int;
    pub fn getpriority(which: ::c_int, who: ::id_t) -> ::c_int;
    pub fn setpriority(which: ::c_int, who: ::id_t, prio: ::c_int) -> ::c_int;
    pub fn mkfifoat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t) -> ::c_int;
    pub fn mknodat(
        __fd: ::c_int,
        pathname: *const ::c_char,
        mode: ::mode_t,
        dev: ::dev_t,
    ) -> ::c_int;

    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn setgroups(ngroups: ::c_int, ptr: *const ::gid_t) -> ::c_int;

    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t, advise: ::c_int) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn nl_langinfo(item: ::nl_item) -> *mut ::c_char;

    pub fn utimensat(
        dirfd: ::c_int,
        path: *const ::c_char,
        times: *const ::timespec,
        flag: ::c_int,
    ) -> ::c_int;

    pub fn ptsname_r(fd: ::c_int, buf: *mut ::c_char, buflen: ::size_t) -> *mut ::c_char;
    pub fn clearenv() -> ::c_int;
   
    pub fn execvpe(
        file: *const ::c_char,
        argv: *const *const ::c_char,
        envp: *const *const ::c_char,
    ) -> ::c_int;

    pub fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int;
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);
    pub fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int;

    pub fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
    pub fn readv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;

    pub fn sendmsg(fd: ::c_int, msg: *const ::msghdr, flags: ::c_int) -> ::ssize_t;
    pub fn recvmsg(fd: ::c_int, msg: *mut ::msghdr, flags: ::c_int) -> ::ssize_t;

    pub fn uname(buf: *mut ::utsname) -> ::c_int;

    pub fn getpeereid(socket: ::c_int, euid: *mut ::uid_t, egid: *mut ::gid_t) -> ::c_int;

    pub fn abs(i: ::c_int) -> ::c_int;
    pub fn labs(i: ::c_long) -> ::c_long;
    pub fn rand() -> ::c_int;
    pub fn srand(seed: ::c_uint);

    pub fn setgrent();
    pub fn endgrent();
    pub fn getgrent() -> *mut ::group;
    pub fn setspent();
    pub fn endspent();

    pub fn shm_open(name: *const c_char, oflag: ::c_int, mode: mode_t) -> ::c_int;

    pub fn ftok(pathname: *const ::c_char, proj_id: ::c_int) -> ::key_t;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;

    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t, len: ::off_t) -> ::c_int;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char, suffixlen: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn sigtimedwait(
        set: *const sigset_t,
        info: *mut siginfo_t,
        timeout: *const ::timespec,
    ) -> ::c_int;
    pub fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> ::c_int;

    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);

    pub fn glob(
        pattern: *const c_char,
        flags: ::c_int,
        errfunc: ::Option<extern "C" fn(epath: *const c_char, errno: ::c_int) -> ::c_int>,
        pglob: *mut ::glob_t,
    ) -> ::c_int;
    pub fn globfree(pglob: *mut ::glob_t);

    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int;

    pub fn shm_unlink(name: *const ::c_char) -> ::c_int;

    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);

    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;

    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;

    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::ssize_t;
    pub fn mkstemps(template: *mut ::c_char, suffixlen: ::c_int) -> ::c_int;

    pub fn getdomainname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
    pub fn setdomainname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn sync();
   
    pub fn umount(target: *const ::c_char, flags: ::c_int) -> ::c_int;
    pub fn sched_get_priority_max(policy: ::c_int) -> ::c_int;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::c_void) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const ::timespec) -> ::c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut ::c_int) -> ::c_int;
    pub fn sched_setparam(pid: ::pid_t, param: *const ::sched_param) -> ::c_int;
    pub fn mount(
        special_device: *const ::c_char,
        mount_directory: *const ::c_char,
        flags: ::c_int,
        mount_type: *const ::c_char,
        mount_data: *const ::c_void,
        mount_datalen: ::c_int,
    ) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut ::sched_param) -> ::c_int;
    
    pub fn sched_getscheduler(pid: ::pid_t) -> ::c_int;
    pub fn clock_nanosleep(
        clk_id: ::clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;
   
    pub fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn sched_get_priority_min(policy: ::c_int) -> ::c_int;
   
    pub fn sched_setscheduler(
        pid: ::pid_t,
        policy: ::c_int,
        param: *const ::sched_param,
    ) -> ::c_int;
    pub fn sigsuspend(mask: *const ::sigset_t) -> ::c_int;
    pub fn getgrgid_r(
        gid: ::gid_t,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn sem_close(sem: *mut sem_t) -> ::c_int;
    pub fn getdtablesize() -> ::c_int;
    pub fn getgrnam_r(
        name: *const ::c_char,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn initgroups(user: *const ::c_char, group: ::gid_t) -> ::c_int;
    pub fn sem_open(name: *const ::c_char, oflag: ::c_int, ...) -> *mut sem_t;
    pub fn getgrnam(name: *const ::c_char) -> *mut ::group;
   
    pub fn sem_unlink(name: *const ::c_char) -> ::c_int;
    pub fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int;
   
    pub fn sigwait(set: *const sigset_t, sig: *mut ::c_int) -> ::c_int;
    
    pub fn getgrgid(gid: ::gid_t) -> *mut ::group;
    pub fn getgrouplist(
        user: *const ::c_char,
        group: ::gid_t,
        groups: *mut ::gid_t,
        ngroups: *mut ::c_int,
    ) -> ::c_int;
   
    pub fn getitimer(which: ::c_int, curr_value: *mut ::itimerval) -> ::c_int;
    pub fn setitimer(
        which: ::c_int,
        value: *const ::itimerval,
        ovalue: *mut ::itimerval,
    ) -> ::c_int;
    pub fn posix_spawn(
        pid: *mut ::pid_t,
        path: *const ::c_char,
        file_actions: *const ::posix_spawn_file_actions_t,
        attrp: *const ::posix_spawnattr_t,
        argv: *const *mut ::c_char,
        envp: *const *mut ::c_char,
    ) -> ::c_int;
    pub fn posix_spawnp(
        pid: *mut ::pid_t,
        file: *const ::c_char,
        file_actions: *const ::posix_spawn_file_actions_t,
        attrp: *const ::posix_spawnattr_t,
        argv: *const *mut ::c_char,
        envp: *const *mut ::c_char,
    ) -> ::c_int;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> ::c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> ::c_int;
    pub fn posix_spawnattr_getsigdefault(
        attr: *const posix_spawnattr_t,
        default: *mut ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_setsigdefault(
        attr: *mut posix_spawnattr_t,
        default: *const ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_getsigmask(
        attr: *const posix_spawnattr_t,
        default: *mut ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_setsigmask(
        attr: *mut posix_spawnattr_t,
        default: *const ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_getflags(
        attr: *const posix_spawnattr_t,
        flags: *mut ::c_short,
    ) -> ::c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: ::c_short) -> ::c_int;
    pub fn posix_spawnattr_getpgroup(
        attr: *const posix_spawnattr_t,
        flags: *mut ::pid_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: ::pid_t) -> ::c_int;
    pub fn posix_spawnattr_getschedpolicy(
        attr: *const posix_spawnattr_t,
        flags: *mut ::c_int,
    ) -> ::c_int;
    pub fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: ::c_int) -> ::c_int;
    pub fn posix_spawnattr_getschedparam(
        attr: *const posix_spawnattr_t,
        param: *mut ::sched_param,
    ) -> ::c_int;
    pub fn posix_spawnattr_setschedparam(
        attr: *mut posix_spawnattr_t,
        param: *const ::sched_param,
    ) -> ::c_int;

    pub fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> ::c_int;
    pub fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> ::c_int;
    pub fn posix_spawn_file_actions_addopen(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
        path: *const ::c_char,
        oflag: ::c_int,
        mode: ::mode_t,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_addclose(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
        newfd: ::c_int,
    ) -> ::c_int;
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut ::FILE;
    pub fn faccessat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::c_int,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn inotify_rm_watch(fd: ::c_int, wd: ::c_int) -> ::c_int;
    pub fn inotify_init() -> ::c_int;
    pub fn inotify_add_watch(fd: ::c_int, path: *const ::c_char, mask: u32) -> ::c_int;

    pub fn gettid() -> ::pid_t;


    pub fn getnameinfo(
        sa: *const ::sockaddr,
        salen: ::socklen_t,
        host: *mut ::c_char,
        hostlen: ::socklen_t,
        serv: *mut ::c_char,
        servlen: ::socklen_t,
        flags: ::c_int,
    ) -> ::c_int;

    pub fn sendmmsg(
        sockfd: ::c_int,
        msgvec: *mut ::mmsghdr,
        vlen: ::c_uint,
        flags: ::c_uint,
    ) -> ::c_int;
    pub fn recvmmsg(
        sockfd: ::c_int,
        msgvec: *mut ::mmsghdr,
        vlen: ::c_uint,
        flags: ::c_uint,
        timeout: *mut ::timespec,
    ) -> ::c_int;

    pub fn mallopt(param: ::c_int, value: i64) -> ::c_int;
    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;

    pub fn ctermid(s: *mut ::c_char) -> *mut ::c_char;
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;

    pub fn mallinfo() -> ::mallinfo;
    pub fn getpwent_r(
        pwd: *mut ::passwd,
        buf: *mut ::c_char,
        __bufsize: ::c_int,
        __result: *mut *mut ::passwd,
    ) -> ::c_int;
   
    pub fn sysctl(
        _: *const ::c_int,
        _: ::c_uint,
        _: *mut ::c_void,
        _: *mut ::size_t,
        _: *const ::c_void,
        _: ::size_t,
    ) -> ::c_int;

    pub fn getrlimit(resource: ::c_int, rlim: *mut ::rlimit) -> ::c_int;
    pub fn setrlimit(resource: ::c_int, rlp: *const ::rlimit) -> ::c_int;

    pub fn lio_listio(
        __mode: ::c_int,
        __list: *const *mut aiocb,
        __nent: ::c_int,
        __sig: *mut sigevent,
    ) -> ::c_int;

    pub fn dl_iterate_phdr(
        callback: ::Option<
            unsafe extern "C" fn(
                // The original .h file declares this as *const, but for consistency with other platforms,
                // changing this to *mut to make it easier to use.
                // Maybe in v0.3 all platforms should use this as a *const.
                info: *mut dl_phdr_info,
                size: ::size_t,
                data: *mut ::c_void,
            ) -> ::c_int,
        >,
        data: *mut ::c_void,
    ) -> ::c_int;

    pub fn memset_s(s: *mut ::c_void, smax: ::size_t, c: ::c_int, n: ::size_t) -> ::c_int;

    pub fn regcomp(
        __preg: *mut ::regex_t,
        __pattern: *const ::c_char,
        __cflags: ::c_int,
    ) -> ::c_int;
    pub fn regexec(
        __preg: *const ::regex_t,
        __str: *const ::c_char,
        __nmatch: ::size_t,
        __pmatch: *mut ::regmatch_t,
        __eflags: ::c_int,
    ) -> ::c_int;
    pub fn regerror(
        __errcode: ::c_int,
        __preg: *const ::regex_t,
        __errbuf: *mut ::c_char,
        __errbuf_size: ::size_t,
    ) -> ::size_t;
    pub fn regfree(__preg: *mut ::regex_t);
    pub fn dirfd(__dirp: *mut ::DIR) -> ::c_int;
    pub fn dircntl(dir: *mut ::DIR, cmd: ::c_int, ...) -> ::c_int;

    pub fn aio_cancel(__fd: ::c_int, __aiocbp: *mut ::aiocb) -> ::c_int;
    pub fn aio_error(__aiocbp: *const ::aiocb) -> ::c_int;
    pub fn aio_fsync(__operation: ::c_int, __aiocbp: *mut ::aiocb) -> ::c_int;
    pub fn aio_read(__aiocbp: *mut ::aiocb) -> ::c_int;
    pub fn aio_return(__aiocpb: *mut ::aiocb) -> ::ssize_t;
    pub fn aio_suspend(
        __list: *const *const ::aiocb,
        __nent: ::c_int,
        __timeout: *const ::timespec,
    ) -> ::c_int;
    pub fn aio_write(__aiocpb: *mut ::aiocb) -> ::c_int;

    pub fn mq_close(__mqdes: ::mqd_t) -> ::c_int;
    pub fn mq_getattr(__mqdes: ::mqd_t, __mqstat: *mut ::mq_attr) -> ::c_int;
    pub fn mq_notify(__mqdes: ::mqd_t, __notification: *const ::sigevent) -> ::c_int;
    pub fn mq_open(__name: *const ::c_char, __oflag: ::c_int, ...) -> ::mqd_t;
    pub fn mq_receive(
        __mqdes: ::mqd_t,
        __msg_ptr: *mut ::c_char,
        __msg_len: ::size_t,
        __msg_prio: *mut ::c_uint,
    ) -> ::ssize_t;
    pub fn mq_send(
        __mqdes: ::mqd_t,
        __msg_ptr: *const ::c_char,
        __msg_len: ::size_t,
        __msg_prio: ::c_uint,
    ) -> ::c_int;
    pub fn mq_setattr(
        __mqdes: ::mqd_t,
        __mqstat: *const mq_attr,
        __omqstat: *mut mq_attr,
    ) -> ::c_int;
    pub fn mq_timedreceive(
        __mqdes: ::mqd_t,
        __msg_ptr: *mut ::c_char,
        __msg_len: ::size_t,
        __msg_prio: *mut ::c_uint,
        __abs_timeout: *const ::timespec,
    ) -> ::ssize_t;
    pub fn mq_timedsend(
        __mqdes: ::mqd_t,
        __msg_ptr: *const ::c_char,
        __msg_len: ::size_t,
        __msg_prio: ::c_uint,
        __abs_timeout: *const ::timespec,
    ) -> ::c_int;
    pub fn mq_unlink(__name: *const ::c_char) -> ::c_int;
    pub fn __get_errno_ptr() -> *mut ::c_int;

    // System page, see https://www.qnx.com/developers/docs/7.1#com.qnx.doc.neutrino.building/topic/syspage/syspage_about.html
    pub static mut _syspage_ptr: *mut syspage_entry;

    // Function on the stack after a call to pthread_create().  This is used
    // as a sentinel to work around an infitnite loop in the unwinding code.
    pub fn __my_thread_exit(value_ptr: *mut *const ::c_void);

    pub fn MsgReceive(
        __chid: ::c_int,
        __msg: *mut ::c_void,
        __bytes: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReceive_r(
        __chid: ::c_int,
        __msg: *mut ::c_void,
        __bytes: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReceivev(
        __chid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReceivev_r(
        __chid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReceivePulsev(
        __chid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReceivePulsev_r(
        __chid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReply(
        __rcvid: ::c_int,
        __status: ::c_long,
        __msg: *const ::c_void,
        __bytes: usize,
    ) -> ::c_int;
    pub fn MsgReply_r(
        __rcvid: ::c_int,
        __status: ::c_long,
        __msg: *const ::c_void,
        __bytes: usize,
    ) -> ::c_int;
    pub fn MsgReplyv(
        __rcvid: ::c_int,
        __status: ::c_long,
        __iov: *const ::iovec,
        __parts: usize,
    ) -> ::c_int;
    pub fn MsgReplyv_r(
        __rcvid: ::c_int,
        __status: ::c_long,
        __iov: *const ::iovec,
        __parts: usize,
    ) -> ::c_int;
    pub fn MsgRead(
        __rcvid: ::c_int,
        __msg: *mut ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgRead_r(
        __rcvid: ::c_int,
        __msg: *mut ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgReadv(
        __rcvid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgReadv_r(
        __rcvid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWrite(
        __rcvid: ::c_int,
        __msg: *const ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWrite_r(
        __rcvid: ::c_int,
        __msg: *const ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWritev(
        __rcvid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWritev_r(
        __rcvid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgInfo(__rcvid: ::c_int, __info: *mut _msg_info64) -> ::c_int;
    pub fn MsgInfo_r(__rcvid: ::c_int, __info: *mut _msg_info64) -> ::c_int;
    pub fn MsgCurrent(__rcvid: ::c_int) -> ::c_int;
    pub fn MsgCurrent_r(__rcvid: ::c_int) -> ::c_int;
    pub fn MsgSendAsyncGbl(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __msg_prio: ::c_uint,
    ) -> ::c_int;
    pub fn MsgSendAsync(__coid: ::c_int) -> ::c_int;
    pub fn MsgReceiveAsyncGbl(
        __chid: ::c_int,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
        __info: *mut _msg_info64,
        __coid: ::c_int,
    ) -> ::c_int;
    pub fn MsgReceiveAsync(__chid: ::c_int, __iov: *const ::iovec, __parts: ::c_uint) -> ::c_int;
    pub fn MsgPause(__rcvid: ::c_int, __cookie: ::c_uint) -> ::c_int;
    pub fn MsgPause_r(__rcvid: ::c_int, __cookie: ::c_uint) -> ::c_int;
    pub fn MsgError(__rcvid: ::c_int, __err: ::c_int) -> ::c_int;
    pub fn MsgError_r(__rcvid: ::c_int, __err: ::c_int) -> ::c_int;
    pub fn SignalFault(__sigcode: ::c_uint, __regs: *mut ::c_void, __refaddr: usize) -> ::c_int;
    pub fn SyncMutexRevive(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncMutexRevive_r(__sync: *mut ::sync_t) -> ::c_int;
    pub fn MsgSendPulse(
        __coid: ::c_int,
        __priority: ::c_int,
        __code: ::c_int,
        __value: ::c_int,
    ) -> ::c_int;
    pub fn MsgSendPulse_r(
        __coid: ::c_int,
        __priority: ::c_int,
        __code: ::c_int,
        __value: ::c_int,
    ) -> ::c_int;
    pub fn MsgReadiov(
        __rcvid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
        __flags: ::c_int,
    ) -> isize;
    pub fn MsgReadiov_r(
        __rcvid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
        __flags: ::c_int,
    ) -> isize;
    pub fn MsgKeyData(
        __rcvid: ::c_int,
        __oper: ::c_int,
        __key: u32,
        __newkey: *mut u32,
        __iov: *const ::iovec,
        __parts: ::c_int,
    ) -> ::c_int;
    pub fn MsgKeyData_r(
        __rcvid: ::c_int,
        __oper: ::c_int,
        __key: u32,
        __newkey: *mut u32,
        __iov: *const ::iovec,
        __parts: ::c_int,
    ) -> ::c_int;
    pub fn MsgDeliverEvent(__rcvid: ::c_int, __event: *const ::sigevent) -> ::c_int;
    pub fn MsgDeliverEvent_r(__rcvid: ::c_int, __event: *const ::sigevent) -> ::c_int;
    pub fn MsgVerifyEvent(__rcvid: ::c_int, __event: *const ::sigevent) -> ::c_int;
    pub fn MsgVerifyEvent_r(__rcvid: ::c_int, __event: *const ::sigevent) -> ::c_int;
    pub fn ChannelCreateExt(
        __flags: ::c_uint,
        __mode: ::mode_t,
        __bufsize: usize,
        __maxnumbuf: ::c_uint,
        __ev: *const ::sigevent,
        __cred: *mut _cred_info,
    ) -> ::c_int;
    pub fn TimerTimeout(
        __id: ::clockid_t,
        __flags: ::c_int,
        __notify: *const ::sigevent,
        __ntime: *const u64,
        __otime: *mut u64,
    ) -> ::c_int;
    pub fn TimerTimeout_r(
        __id: ::clockid_t,
        __flags: ::c_int,
        __notify: *const ::sigevent,
        __ntime: *const u64,
        __otime: *mut u64,
    ) -> ::c_int;

    pub fn SchedWaypoint(
        __job: *mut nto_job_t,
        __new: *const i64,
        __max: *const i64,
        __old: *mut i64,
    ) -> ::c_int;
    pub fn SchedWaypoint_r(
        __job: *mut nto_job_t,
        __new: *const i64,
        __max: *const i64,
        __old: *mut i64,
    ) -> ::c_int;
    pub fn InterruptCharacteristic(
        __type: ::c_int,
        __id: ::c_int,
        __new: *mut ::c_uint,
        __old: *mut ::c_uint,
    ) -> ::c_int;
    pub fn InterruptCharacteristic_r(
        __type: ::c_int,
        __id: ::c_int,
        __new: *mut ::c_uint,
        __old: *mut ::c_uint,
    ) -> ::c_int;
}