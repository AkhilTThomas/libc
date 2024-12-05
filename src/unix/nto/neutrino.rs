// use qtime_entry;
// use syspage_entry;
cfg_if! {
    if #[cfg(target_env = "nto80")]{
        use super::nto80::*;
    }
    else if #[cfg(any(target_env = "nto71", target_env = "nto70"))]{
        use super::nto7071::*;
    }
    else {
        // Unknown env
    }
}
pub type nto_job_t = ::sync_t;

s! {
    pub struct intrspin {
        pub value: ::c_uint, // volatile
    }

    pub struct iov_t {
        pub iov_base: *mut ::c_void,  // union
        pub iov_len: ::size_t,
    }

    pub struct _itimer {
        pub nsec: u64,
        pub interval_nsec: u64,
    }

    pub struct _msg_info64 {
        pub nd: u32,
        pub srcnd: u32,
        pub pid: ::pid_t,
        pub tid: i32,
        pub chid: i32,
        pub scoid: i32,
        pub coid: i32,
        pub priority: i16,
        pub flags: i16,
        pub msglen: isize,
        pub srcmsglen: isize,
        pub dstmsglen: isize,
        pub type_id: u32,
        reserved: u32,
    }

    pub struct _cred_info {
        pub ruid: ::uid_t,
        pub euid: ::uid_t,
        pub suid: ::uid_t,
        pub rgid: ::gid_t,
        pub egid: ::gid_t,
        pub sgid: ::gid_t,
        pub ngroups: u32,
        pub grouplist: [::gid_t; 8],
    }

    pub struct _client_info {
        pub nd: u32,
        pub pid: ::pid_t,
        pub sid: ::pid_t,
        pub flags: u32,
        pub cred: ::_cred_info,
    }

    pub struct _client_able {
        pub ability: u32,
        pub flags: u32,
        pub range_lo: u64,
        pub range_hi: u64,
    }

    pub struct nto_channel_config {
        pub event: ::sigevent,
        pub num_pulses: ::c_uint,
        pub rearm_threshold: ::c_uint,
        pub options: ::c_uint,
        reserved: [::c_uint; 3],
    }

    // TODO: The following structures are defined in a header file which doesn't
    //       appear as part of the default headers found in a standard installation
    //       of Neutrino 7.1 SDP.  Commented out for now.
    //pub struct _asyncmsg_put_header {
    //    pub err: ::c_int,
    //    pub iov: *mut ::iov_t,
    //    pub parts: ::c_int,
    //    pub handle: ::c_uint,
    //    pub cb: ::Option<
    //        unsafe extern "C" fn(
    //            err: ::c_int,
    //            buf: *mut ::c_void,
    //            handle: ::c_uint,
    //        ) -> ::c_int>,
    //    pub put_hdr_flags: ::c_uint,
    //}

    //pub struct _asyncmsg_connection_attr {
    //    pub call_back: ::Option<
    //        unsafe extern "C" fn(
    //            err: ::c_int,
    //            buff: *mut ::c_void,
    //            handle: ::c_uint,
    //        ) -> ::c_int>,
    //    pub buffer_size: ::size_t,
    //    pub max_num_buffer: ::c_uint,
    //    pub trigger_num_msg: ::c_uint,
    //    pub trigger_time: ::_itimer,
    //    reserve: ::c_uint,
    //}

    //pub struct _asyncmsg_connection_descriptor {
    //    pub flags: ::c_uint,
    //    pub sendq_size: ::c_uint,
    //    pub sendq_head: ::c_uint,
    //    pub sendq_tail: ::c_uint,
    //    pub sendq_free: ::c_uint,
    //    pub err: ::c_int,
    //    pub ev: ::sigevent,
    //    pub num_curmsg: ::c_uint,
    //    pub ttimer: ::timer_t,
    //    pub block_con: ::pthread_cond_t,
    //    pub mu: ::pthread_mutex_t,
    //    reserved: ::c_uint,
    //    pub attr: ::_asyncmsg_connection_attr,
    //    pub reserves: [::c_uint; 3],
    //    pub sendq: [::_asyncmsg_put_header; 1], // flexarray
    //}

    pub struct __c_anonymous_struct_ev {
        pub event: ::sigevent,
        pub coid: ::c_int,
    }

    pub struct _channel_connect_attr {  // union
        pub ev: ::__c_anonymous_struct_ev,
    }

    pub struct _sighandler_info {
        pub siginfo: ::siginfo_t,
        pub handler: ::Option<unsafe extern "C" fn(value: ::c_int)>,
        pub context: *mut ::c_void,
    }

    pub struct __c_anonymous_struct_time {
        pub length: ::c_uint,
        pub scale: ::c_uint,
    }

    pub struct _idle_hook {
        pub hook_size: ::c_uint,
        pub cmd: ::c_uint,
        pub mode: ::c_uint,
        pub latency: ::c_uint,
        pub next_fire: u64,
        pub curr_time: u64,
        pub tod_adjust: u64,
        pub resp: ::c_uint,
        pub time: __c_anonymous_struct_time,
        pub trigger: ::sigevent,
        pub intrs: *mut ::c_uint,
        pub block_stack_size: ::c_uint,
    }

    pub struct _clockadjust {
        pub tick_count: u32,
        pub tick_nsec_inc: i32,
    }

    pub struct _sched_info {
        pub priority_min: ::c_int,
        pub priority_max: ::c_int,
        pub interval: u64,
        pub priority_priv: ::c_int,
        reserved: [::c_int; 11],
    }

    pub struct _timer_info {
        pub itime: ::_itimer,
        pub otime: ::_itimer,
        pub flags: u32,
        pub tid: i32,
        pub notify: i32,
        pub clockid: ::clockid_t,
        pub overruns: u32,
        pub event: ::sigevent, // union
    }

    pub struct _clockperiod {
        pub nsec: u32,
        pub fract: i32,
    }
}

pub const SYSMGR_PID: u32 = 1;
pub const SYSMGR_CHID: u32 = 1;
pub const SYSMGR_COID: u32 = _NTO_SIDE_CHANNEL;
pub const SYSMGR_HANDLE: u32 = 0;

pub const _NTO_TIMEOUT_RECEIVE: i32 = 1 << STATE_RECEIVE;
pub const _NTO_TIMEOUT_SEND: i32 = 1 << STATE_SEND;
pub const _NTO_TIMEOUT_REPLY: i32 = 1 << STATE_REPLY;
pub const _NTO_TIMEOUT_SIGSUSPEND: i32 = 1 << STATE_SIGSUSPEND;
pub const _NTO_TIMEOUT_SIGWAITINFO: i32 = 1 << STATE_SIGWAITINFO;
pub const _NTO_TIMEOUT_NANOSLEEP: i32 = 1 << STATE_NANOSLEEP;
pub const _NTO_TIMEOUT_MUTEX: i32 = 1 << STATE_MUTEX;
pub const _NTO_TIMEOUT_CONDVAR: i32 = 1 << STATE_CONDVAR;
pub const _NTO_TIMEOUT_JOIN: i32 = 1 << STATE_JOIN;
pub const _NTO_TIMEOUT_INTR: i32 = 1 << STATE_INTR;
pub const _NTO_TIMEOUT_SEM: i32 = 1 << STATE_SEM;

pub const _NTO_MI_ENDIAN_BIG: u32 = 1;
pub const _NTO_MI_ENDIAN_DIFF: u32 = 2;
pub const _NTO_MI_UNBLOCK_REQ: u32 = 256;
pub const _NTO_MI_NET_CRED_DIRTY: u32 = 512;
pub const _NTO_MI_CONSTRAINED: u32 = 1024;
pub const _NTO_MI_CHROOT: u32 = 2048;
pub const _NTO_MI_BITS_DIFF: u32 = 8192;
pub const _NTO_MI_SANDBOX: u32 = 16384;

pub const _NTO_CI_ENDIAN_BIG: u32 = 0x00000001;
pub const _NTO_CI_BKGND_PGRP: u32 = 0x00000004;
pub const _NTO_CI_ORPHAN_PGRP: u32 = 0x00000008;
pub const _NTO_CI_STOPPED: u32 = 0x00000080;
pub const _NTO_CI_UNABLE: u32 = 0x00000100;
pub const _NTO_CI_TYPE_ID: u32 = 0x00000200;
pub const _NTO_CI_CHROOT: u32 = 0x00000800;
cfg_if! {
    if #[cfg(target_env = "nto80")] {
        pub const _NTO_CI_SANDBOX: u32 = 0x00004000;
        pub const _NTO_CI_GROUPS_INFO_ONLY: u32 = 0x40000000;
    }
    else {
        pub const _NTO_MI_BITS_64: u32 = 0x00001000;
    }
}
pub const _NTO_CI_LOADER: u32 = 0x00008000;
pub const _NTO_CI_FULL_GROUPS: u32 = 0x80000000;

pub const _NTO_TI_ACTIVE: u32 = 1;
pub const _NTO_TI_ABSOLUTE: u32 = 2;
pub const _NTO_TI_EXPIRED: u32 = 4;
pub const _NTO_TI_TOD_BASED: u32 = 8;
pub const _NTO_TI_TARGET_PROCESS: u32 = 16;
pub const _NTO_TI_REPORT_TOLERANCE: u32 = 32;
pub const _NTO_TI_PRECISE: u32 = 64;
pub const _NTO_TI_TOLERANT: u32 = 128;
pub const _NTO_TI_WAKEUP: u32 = 256;
pub const _NTO_TI_PROCESS_TOLERANT: u32 = 512;
pub const _NTO_TI_HIGH_RESOLUTION: u32 = 1024;

pub const _PULSE_TYPE: u32 = 0;
pub const _PULSE_SUBTYPE: u32 = 0;
pub const _PULSE_CODE_UNBLOCK: i32 = -32;
pub const _PULSE_CODE_DISCONNECT: i32 = -33;
pub const _PULSE_CODE_THREADDEATH: i32 = -34;
pub const _PULSE_CODE_COIDDEATH: i32 = -35;
pub const _PULSE_CODE_NET_ACK: i32 = -36;
pub const _PULSE_CODE_NET_UNBLOCK: i32 = -37;
pub const _PULSE_CODE_NET_DETACH: i32 = -38;
pub const _PULSE_CODE_RESTART: i32 = -39;
pub const _PULSE_CODE_NORESTART: i32 = -40;
pub const _PULSE_CODE_UNBLOCK_RESTART: i32 = -41;
pub const _PULSE_CODE_UNBLOCK_TIMER: i32 = -42;
pub const _PULSE_CODE_MINAVAIL: u32 = 0;
pub const _PULSE_CODE_MAXAVAIL: u32 = 127;

pub const _NTO_HARD_FLAGS_END: u32 = 1;

pub const _NTO_PULSE_IF_UNIQUE: u32 = 4096;
pub const _NTO_PULSE_REPLACE: u32 = 8192;

pub const _NTO_PF_NOCLDSTOP: u32 = 1;
pub const _NTO_PF_LOADING: u32 = 2;
pub const _NTO_PF_TERMING: u32 = 4;
pub const _NTO_PF_ZOMBIE: u32 = 8;
pub const _NTO_PF_NOZOMBIE: u32 = 16;
pub const _NTO_PF_FORKED: u32 = 32;
cfg_if! {
    if #[cfg(target_env = "nto80")] {
        pub const _NTO_PF_SPAWN_HELD: u32 = 0x00004000;
        pub const _NTO_PF_KILL_REQUEST: u32 = 524288;
        pub const _NTO_PF_CORE_REQUEST: u32 = 0x00200000;
        pub const _NTO_PF_COREDUMP_WAIT: u32 = 268435456;

    }
    else {
        pub const _NTO_PF_ORPHAN_PGRP: u32 = 64;
        pub const _NTO_PF_RING0: u32 = 32768;
        pub const _NTO_PF_DESTROYALL: u32 = 524288;
        pub const _NTO_PF_TERM_WAITING: u32 = 8388608;
        pub const _NTO_PF_NET: u32 = 268435456;
    }
}
pub const _NTO_PF_STOPPED: u32 = 128;
pub const _NTO_PF_DEBUG_STOPPED: u32 = 256;
pub const _NTO_PF_BKGND_PGRP: u32 = 512;
pub const _NTO_PF_NOISYNC: u32 = 1024;
pub const _NTO_PF_CONTINUED: u32 = 2048;
pub const _NTO_PF_CHECK_INTR: u32 = 4096;
pub const _NTO_PF_COREDUMP: u32 = 8192;
pub const _NTO_PF_SLEADER: u32 = 65536;
pub const _NTO_PF_WAITINFO: u32 = 131072;
pub const _NTO_PF_NOCOREDUMP: u32 = 1048576;
pub const _NTO_PF_WAITDONE: u32 = 4194304;
pub const _NTO_PF_ASLR: u32 = 16777216;
pub const _NTO_PF_EXECED: u32 = 33554432;
pub const _NTO_PF_APP_STOPPED: u32 = 67108864;
pub const _NTO_PF_64BIT: u32 = 134217728;
pub const _NTO_PF_NOLAZYSTACK: u32 = 536870912;
pub const _NTO_PF_NOEXEC_STACK: u32 = 1073741824;
pub const _NTO_PF_LOADER_PERMS: u32 = 2147483648;

pub const _NTO_TCTL_IO_PRIV: u32 = 1;
pub const _NTO_TCTL_THREADS_HOLD: u32 = 2;
pub const _NTO_TCTL_THREADS_CONT: u32 = 3;
pub const _NTO_TCTL_RUNMASK: u32 = 4;
pub const _NTO_TCTL_ALIGN_FAULT: u32 = 5;
pub const _NTO_TCTL_RUNMASK_GET_AND_SET: u32 = 6;
cfg_if! {
    if #[cfg(target_env = "nto80")] {
        pub const _NTO_TCTL_SIGSTACK: u32 =   23;
        pub const _NTO_TCTL_TLS_SET_TP: u32 = 24;
        pub const _NTO_TCTL_HYP_VCPU: u32 =   25;
    }
    else {
        pub const _NTO_TCTL_PERFCOUNT: u32 = 7;
        pub const _NTO_TCTL_SHR_MUTEX: u32 = 13;
        pub const _NTO_TCTL_NET_KIF_GET_AND_SET: u32 = 15;
        pub const _NTO_TCTL_LOW_LATENCY: u32 = 16;
    }
}
pub const _NTO_TCTL_ONE_THREAD_HOLD: u32 = 8;
pub const _NTO_TCTL_ONE_THREAD_CONT: u32 = 9;
pub const _NTO_TCTL_RUNMASK_GET_AND_SET_INHERIT: u32 = 10;
pub const _NTO_TCTL_NAME: u32 = 11;
pub const _NTO_TCTL_RCM_GET_AND_SET: u32 = 12;
pub const _NTO_TCTL_IO: u32 = 14;
pub const _NTO_TCTL_ADD_EXIT_EVENT: u32 = 17;
pub const _NTO_TCTL_DEL_EXIT_EVENT: u32 = 18;
pub const _NTO_TCTL_IO_LEVEL: u32 = 19;
pub const _NTO_TCTL_RESERVED: u32 = 2147483648;
pub const _NTO_TCTL_IO_LEVEL_INHERIT: u32 = 1073741824;
pub const _NTO_IO_LEVEL_NONE: u32 = 1;
pub const _NTO_IO_LEVEL_1: u32 = 2;
pub const _NTO_IO_LEVEL_2: u32 = 3;

pub const _NTO_THREAD_NAME_MAX: u32 = 100;

pub const _NTO_CHF_FIXED_PRIORITY: u32 = 1;
pub const _NTO_CHF_UNBLOCK: u32 = 2;
pub const _NTO_CHF_THREAD_DEATH: u32 = 4;
pub const _NTO_CHF_DISCONNECT: u32 = 8;
pub const _NTO_CHF_NET_MSG: u32 = 16;
pub const _NTO_CHF_SENDER_LEN: u32 = 32;
pub const _NTO_CHF_COID_DISCONNECT: u32 = 64;
pub const _NTO_CHF_REPLY_LEN: u32 = 128;
pub const _NTO_CHF_PULSE_POOL: u32 = 256;
cfg_if! {
    if #[cfg(any(target_env = "nto71", target_env = "nto70"))]{
        pub const _NTO_CHF_ASYNC_NONBLOCK: u32 = 512;
        pub const _NTO_CHF_ASYNC: u32 = 1024;
    }
}
pub const _NTO_CHF_GLOBAL: u32 = 2048;
pub const _NTO_CHF_PRIVATE: u32 = 4096;
pub const _NTO_CHF_MSG_PAUSING: u32 = 8192;
pub const _NTO_CHF_INHERIT_RUNMASK: u32 = 16384;
pub const _NTO_CHF_UNBLOCK_TIMER: u32 = 32768;

pub const _NTO_CHO_CUSTOM_EVENT: u32 = 1;

pub const _NTO_COF_CLOEXEC: u32 = 1;
pub const _NTO_COF_DEAD: u32 = 2;
cfg_if! {
    if #[cfg(target_env = "nto80")]{
        pub const _NTO_COF_NOINHERIT: u32 = 4;
        pub const _NTO_COF_LOCAL: u32 = 8;
        pub const _NTO_COF_ALLOWED: u32 = 128;
        pub const _NTO_COF_DUP2: u32 = 0x8000;
    }
    else if #[cfg(any(target_env = "nto71", target_env = "nto70"))]{
        pub const _NTO_COF_NETCON: u32 = 128;
    }
    else {
        //unknown env
    }
}
pub const _NTO_COF_NOSHARE: u32 = 64;
pub const _NTO_COF_NONBLOCK: u32 = 256;
pub const _NTO_COF_ASYNC: u32 = 512;
pub const _NTO_COF_GLOBAL: u32 = 1024;
pub const _NTO_COF_NOEVENT: u32 = 2048;
pub const _NTO_COF_INSECURE: u32 = 4096;
pub const _NTO_COF_REG_EVENTS: u32 = 8192;
pub const _NTO_COF_UNREG_EVENTS: u32 = 16384;
pub const _NTO_COF_MASK: u32 = 65535;

pub const _NTO_SIDE_CHANNEL: u32 = 1073741824;

pub const _NTO_CONNECTION_SCOID: u32 = 65536;
pub const _NTO_GLOBAL_CHANNEL: u32 = 1073741824;

cfg_if! {
    if #[cfg(target_env = "nto80")]{
        pub const _NTO_TIMEOUT_MASK: ::c_uint = (1 << STATE_TIMEOUT_MAX) - 1;
        pub const _NTO_TIMEOUT_ACTIVE: ::c_uint = 1 << STATE_TIMEOUT_MAX;
        pub const _NTO_TIMEOUT_NO_WAIT: ::c_uint = 1 << (STATE_TIMEOUT_MAX + 1);
    }
    else if #[cfg(any(target_env = "nto71", target_env = "nto70"))]{
        pub const _NTO_TIMEOUT_MASK: ::u32 = (1 << STATE_MAX) - 1;
        pub const _NTO_TIMEOUT_ACTIVE: u32 = 1 << STATE_MAX;
        pub const _NTO_TIMEOUT_IMMEDIATE: u32 = 1 << (STATE_MAX + 1);
    }
    else {
        // Unknown env
    }
}

pub const _NTO_IC_LATENCY: u32 = 0;

pub const _NTO_INTR_FLAGS_END: u32 = 1;
pub const _NTO_INTR_FLAGS_NO_UNMASK: u32 = 2;
pub const _NTO_INTR_FLAGS_PROCESS: u32 = 4;
pub const _NTO_INTR_FLAGS_TRK_MSK: u32 = 8;
pub const _NTO_INTR_FLAGS_ARRAY: u32 = 16;
pub const _NTO_INTR_FLAGS_EXCLUSIVE: u32 = 32;
pub const _NTO_INTR_FLAGS_FPU: u32 = 64;

pub const _NTO_INTR_CLASS_EXTERNAL: u32 = 0;
pub const _NTO_INTR_CLASS_SYNTHETIC: u32 = 2147418112;

pub const _NTO_INTR_SPARE: u32 = 2147483647;

pub const _NTO_HOOK_IDLE: u32 = 2147418113;
pub const _NTO_HOOK_OVERDRIVE: u32 = 2147418114;
pub const _NTO_HOOK_LAST: u32 = 2147418114;
pub const _NTO_HOOK_IDLE2_FLAG: u32 = 32768;

pub const _NTO_IH_CMD_SLEEP_SETUP: u32 = 1;
pub const _NTO_IH_CMD_SLEEP_BLOCK: u32 = 2;
pub const _NTO_IH_CMD_SLEEP_WAKEUP: u32 = 4;
pub const _NTO_IH_CMD_SLEEP_ONLINE: u32 = 8;
pub const _NTO_IH_RESP_NEEDS_BLOCK: u32 = 1;
pub const _NTO_IH_RESP_NEEDS_WAKEUP: u32 = 2;
pub const _NTO_IH_RESP_NEEDS_ONLINE: u32 = 4;
pub const _NTO_IH_RESP_SYNC_TIME: u32 = 16;
pub const _NTO_IH_RESP_SYNC_TLB: u32 = 32;
pub const _NTO_IH_RESP_SUGGEST_OFFLINE: u32 = 256;
pub const _NTO_IH_RESP_SLEEP_MODE_REACHED: u32 = 512;
pub const _NTO_IH_RESP_DELIVER_INTRS: u32 = 1024;

cfg_if! {
   if #[cfg(any(target_env = "nto71", target_env = "nto70"))]{
    pub const _NTO_KEYDATA_VTID: u32 = 2147483648;
    pub const _NTO_KEYDATA_PATHSIGN: u32 = 32768;
    pub const _NTO_KEYDATA_OP_MASK: u32 = 255;
    pub const _NTO_KEYDATA_VERIFY: u32 = 0;
    pub const _NTO_KEYDATA_CALCULATE: u32 = 1;
    pub const _NTO_KEYDATA_CALCULATE_REUSE: u32 = 2;
    pub const _NTO_KEYDATA_PATHSIGN_VERIFY: u32 = 32768;
    pub const _NTO_KEYDATA_PATHSIGN_CALCULATE: u32 = 32769;
    pub const _NTO_KEYDATA_PATHSIGN_CALCULATE_REUSE: u32 = 32770;
    
    pub const _NTO_TF_INTR_PENDING: u32 = 65536;
    pub const _NTO_TF_DETACHED: u32 = 131072;
    pub const _NTO_TF_SHR_MUTEX: u32 = 262144;
    pub const _NTO_TF_SHR_MUTEX_EUID: u32 = 524288;
    pub const _NTO_TF_THREADS_HOLD: u32 = 1048576;
    pub const _NTO_TF_UNBLOCK_REQ: u32 = 4194304;
    pub const _NTO_TF_ALIGN_FAULT: u32 = 16777216;
    pub const _NTO_TF_SSTEP: u32 = 33554432;
    pub const _NTO_TF_ALLOCED_STACK: u32 = 67108864;
    pub const _NTO_TF_NOMULTISIG: u32 = 134217728;
    pub const _NTO_TF_LOW_LATENCY: u32 = 268435456;
    pub const _NTO_TF_IOPRIV: u32 = 2147483648;

    pub const _NTO_READIOV_SEND: u32 = 0;
    pub const _NTO_READIOV_REPLY: u32 = 1;
    }
}


pub const _NTO_SCTL_SETPRIOCEILING: u32 = 1;
pub const _NTO_SCTL_GETPRIOCEILING: u32 = 2;
pub const _NTO_SCTL_SETEVENT: u32 = 3;
pub const _NTO_SCTL_MUTEX_WAKEUP: u32 = 4;
pub const _NTO_SCTL_MUTEX_CONSISTENT: u32 = 5;
pub const _NTO_SCTL_SEM_VALUE: u32 = 6;

pub const _NTO_CLIENTINFO_GETGROUPS: u32 = 1;
pub const _NTO_CLIENTINFO_GETTYPEID: u32 = 2;

extern "C" {
    pub fn ChannelCreate(__flags: ::c_uint) -> ::c_int;
    pub fn ChannelCreate_r(__flags: ::c_uint) -> ::c_int;
    pub fn ChannelCreatePulsePool(
        __flags: ::c_uint,
        __config: *const nto_channel_config,
    ) -> ::c_int;
    pub fn ChannelDestroy(__chid: ::c_int) -> ::c_int;
    pub fn ChannelDestroy_r(__chid: ::c_int) -> ::c_int;
    pub fn ConnectAttach(
        __nd: u32,
        __pid: ::pid_t,
        __chid: ::c_int,
        __index: ::c_uint,
        __flags: ::c_int,
    ) -> ::c_int;
    pub fn ConnectAttach_r(
        __nd: u32,
        __pid: ::pid_t,
        __chid: ::c_int,
        __index: ::c_uint,
        __flags: ::c_int,
    ) -> ::c_int;

    // TODO: The following function uses a structure defined in a header file
    //       which doesn't appear as part of the default headers found in a
    //       standard installation of Neutrino 7.1 SDP.  Commented out for now.
    //pub fn ConnectAttachExt(
    //    __nd: u32,
    //    __pid: ::pid_t,
    //    __chid: ::c_int,
    //    __index: ::c_uint,
    //    __flags: ::c_int,
    //    __cd: *mut _asyncmsg_connection_descriptor,
    //) -> ::c_int;
    pub fn ConnectDetach(__coid: ::c_int) -> ::c_int;
    pub fn ConnectDetach_r(__coid: ::c_int) -> ::c_int;
    pub fn ConnectServerInfo(__pid: ::pid_t, __coid: ::c_int, __info: *mut _msg_info64) -> ::c_int;
    pub fn ConnectServerInfo_r(
        __pid: ::pid_t,
        __coid: ::c_int,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn ConnectClientInfoExtraArgs(
        __scoid: ::c_int,
        __info_pp: *mut _client_info,
        __ngroups: ::c_int,
        __abilities: *mut _client_able,
        __nable: ::c_int,
        __type_id: *mut ::c_uint,
    ) -> ::c_int;
    pub fn ConnectClientInfoExtraArgs_r(
        __scoid: ::c_int,
        __info_pp: *mut _client_info,
        __ngroups: ::c_int,
        __abilities: *mut _client_able,
        __nable: ::c_int,
        __type_id: *mut ::c_uint,
    ) -> ::c_int;
    pub fn ConnectClientInfo(
        __scoid: ::c_int,
        __info: *mut _client_info,
        __ngroups: ::c_int,
    ) -> ::c_int;
    pub fn ConnectClientInfo_r(
        __scoid: ::c_int,
        __info: *mut _client_info,
        __ngroups: ::c_int,
    ) -> ::c_int;
    pub fn ConnectClientInfoExt(
        __scoid: ::c_int,
        __info_pp: *mut *mut _client_info,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn ClientInfoExtFree(__info_pp: *mut *mut _client_info) -> ::c_int;
    pub fn ConnectClientInfoAble(
        __scoid: ::c_int,
        __info_pp: *mut *mut _client_info,
        flags: ::c_int,
        abilities: *mut _client_able,
        nable: ::c_int,
    ) -> ::c_int;
    pub fn ConnectFlags(
        __pid: ::pid_t,
        __coid: ::c_int,
        __mask: ::c_uint,
        __bits: ::c_uint,
    ) -> ::c_int;
    pub fn ConnectFlags_r(
        __pid: ::pid_t,
        __coid: ::c_int,
        __mask: ::c_uint,
        __bits: ::c_uint,
    ) -> ::c_int;
    pub fn ChannelConnectAttr(
        __id: ::c_uint,
        __old_attr: *mut _channel_connect_attr,
        __new_attr: *mut _channel_connect_attr,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn MsgSend(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSend_r(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendnc(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendnc_r(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendsv(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendsv_r(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendsvnc(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendsvnc_r(
        __coid: ::c_int,
        __smsg: *const ::c_void,
        __sbytes: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendvs(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendvs_r(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendvsnc(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendvsnc_r(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __rmsg: *mut ::c_void,
        __rbytes: usize,
    ) -> ::c_long;
    pub fn MsgSendv(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendv_r(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendvnc(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    pub fn MsgSendvnc_r(
        __coid: ::c_int,
        __siov: *const ::iovec,
        __sparts: usize,
        __riov: *const ::iovec,
        __rparts: usize,
    ) -> ::c_long;
    
    pub fn MsgReceivePulse(
        __chid: ::c_int,
        __pulse: *mut ::c_void,
        __bytes: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgReceivePulse_r(
        __chid: ::c_int,
        __pulse: *mut ::c_void,
        __bytes: usize,
        __info: *mut _msg_info64,
    ) -> ::c_int;
    pub fn MsgSendPulsePtr(
        __coid: ::c_int,
        __priority: ::c_int,
        __code: ::c_int,
        __value: *mut ::c_void,
    ) -> ::c_int;
    pub fn MsgSendPulsePtr_r(
        __coid: ::c_int,
        __priority: ::c_int,
        __code: ::c_int,
        __value: *mut ::c_void,
    ) -> ::c_int;
    pub fn MsgRegisterEvent(__event: *mut ::sigevent, __coid: ::c_int) -> ::c_int;
    pub fn MsgRegisterEvent_r(__event: *mut ::sigevent, __coid: ::c_int) -> ::c_int;
    pub fn MsgUnregisterEvent(__event: *const ::sigevent) -> ::c_int;
    pub fn MsgUnregisterEvent_r(__event: *const ::sigevent) -> ::c_int;
 
    pub fn SignalKill(
        __nd: u32,
        __pid: ::pid_t,
        __tid: ::c_int,
        __signo: ::c_int,
        __code: ::c_int,
        __value: ::c_int,
    ) -> ::c_int;
    pub fn SignalKill_r(
        __nd: u32,
        __pid: ::pid_t,
        __tid: ::c_int,
        __signo: ::c_int,
        __code: ::c_int,
        __value: ::c_int,
    ) -> ::c_int;
    pub fn SignalKillSigval(
        __nd: u32,
        __pid: ::pid_t,
        __tid: ::c_int,
        __signo: ::c_int,
        __code: ::c_int,
        __value: *const ::sigval,
    ) -> ::c_int;
    pub fn SignalKillSigval_r(
        __nd: u32,
        __pid: ::pid_t,
        __tid: ::c_int,
        __signo: ::c_int,
        __code: ::c_int,
        __value: *const ::sigval,
    ) -> ::c_int;
    pub fn SignalReturn(__info: *mut _sighandler_info) -> ::c_int;
    pub fn SignalAction(
        __pid: ::pid_t,
        __sigstub: unsafe extern "C" fn(),
        __signo: ::c_int,
        __act: *const ::sigaction,
        __oact: *mut ::sigaction,
    ) -> ::c_int;
    pub fn SignalAction_r(
        __pid: ::pid_t,
        __sigstub: unsafe extern "C" fn(),
        __signo: ::c_int,
        __act: *const ::sigaction,
        __oact: *mut ::sigaction,
    ) -> ::c_int;
    pub fn SignalProcmask(
        __pid: ::pid_t,
        __tid: ::c_int,
        __how: ::c_int,
        __set: *const ::sigset_t,
        __oldset: *mut ::sigset_t,
    ) -> ::c_int;
    pub fn SignalProcmask_r(
        __pid: ::pid_t,
        __tid: ::c_int,
        __how: ::c_int,
        __set: *const ::sigset_t,
        __oldset: *mut ::sigset_t,
    ) -> ::c_int;
    pub fn SignalSuspend(__set: *const ::sigset_t) -> ::c_int;
    pub fn SignalSuspend_r(__set: *const ::sigset_t) -> ::c_int;
    pub fn SignalWaitinfo(__set: *const ::sigset_t, __info: *mut ::siginfo_t) -> ::c_int;
    pub fn SignalWaitinfo_r(__set: *const ::sigset_t, __info: *mut ::siginfo_t) -> ::c_int;
    pub fn SignalWaitinfoMask(
        __set: *const ::sigset_t,
        __info: *mut ::siginfo_t,
        __mask: *const ::sigset_t,
    ) -> ::c_int;
    pub fn SignalWaitinfoMask_r(
        __set: *const ::sigset_t,
        __info: *mut ::siginfo_t,
        __mask: *const ::sigset_t,
    ) -> ::c_int;
    pub fn ThreadCreate(
        __pid: ::pid_t,
        __func: unsafe extern "C" fn(__arg: *mut ::c_void) -> *mut ::c_void,
        __arg: *mut ::c_void,
        __attr: *const ::_thread_attr,
    ) -> ::c_int;
    pub fn ThreadCreate_r(
        __pid: ::pid_t,
        __func: unsafe extern "C" fn(__arg: *mut ::c_void) -> *mut ::c_void,
        __arg: *mut ::c_void,
        __attr: *const ::_thread_attr,
    ) -> ::c_int;

    pub fn ThreadDestroy(__tid: ::c_int, __priority: ::c_int, __status: *mut ::c_void) -> ::c_int;
    pub fn ThreadDestroy_r(__tid: ::c_int, __priority: ::c_int, __status: *mut ::c_void)
        -> ::c_int;
    pub fn ThreadDetach(__tid: ::c_int) -> ::c_int;
    pub fn ThreadDetach_r(__tid: ::c_int) -> ::c_int;
    pub fn ThreadJoin(__tid: ::c_int, __status: *mut *mut ::c_void) -> ::c_int;
    pub fn ThreadJoin_r(__tid: ::c_int, __status: *mut *mut ::c_void) -> ::c_int;
    pub fn ThreadCancel(__tid: ::c_int, __canstub: unsafe extern "C" fn()) -> ::c_int;
    pub fn ThreadCancel_r(__tid: ::c_int, __canstub: unsafe extern "C" fn()) -> ::c_int;
    pub fn ThreadCtl(__cmd: ::c_int, __data: *mut ::c_void) -> ::c_int;
    pub fn ThreadCtl_r(__cmd: ::c_int, __data: *mut ::c_void) -> ::c_int;
    pub fn ThreadCtlExt(
        __pid: ::pid_t,
        __tid: ::c_int,
        __cmd: ::c_int,
        __data: *mut ::c_void,
    ) -> ::c_int;
    pub fn ThreadCtlExt_r(
        __pid: ::pid_t,
        __tid: ::c_int,
        __cmd: ::c_int,
        __data: *mut ::c_void,
    ) -> ::c_int;

    pub fn InterruptHookTrace(
        __handler: ::Option<unsafe extern "C" fn(arg1: ::c_int) -> *const ::sigevent>,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptHookIdle(
        __handler: ::Option<unsafe extern "C" fn(arg1: *mut u64, arg2: *mut qtime_entry)>,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptHookIdle2(
        __handler: ::Option<
            unsafe extern "C" fn(arg1: ::c_uint, arg2: *mut syspage_entry, arg3: *mut _idle_hook),
        >,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptHookOverdriveEvent(__event: *const ::sigevent, __flags: ::c_uint) -> ::c_int;
    pub fn InterruptAttachEvent(
        __intr: ::c_int,
        __event: *const ::sigevent,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptAttachEvent_r(
        __intr: ::c_int,
        __event: *const ::sigevent,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptAttach(
        __intr: ::c_int,
        __handler: ::Option<
            unsafe extern "C" fn(__area: *mut ::c_void, __id: ::c_int) -> *const ::sigevent,
        >,
        __area: *const ::c_void,
        __size: ::c_int,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptAttach_r(
        __intr: ::c_int,
        __handler: ::Option<
            unsafe extern "C" fn(__area: *mut ::c_void, __id: ::c_int) -> *const ::sigevent,
        >,
        __area: *const ::c_void,
        __size: ::c_int,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptAttachArray(
        __intr: ::c_int,
        __handler: ::Option<
            unsafe extern "C" fn(__area: *mut ::c_void, __id: ::c_int) -> *const *const ::sigevent,
        >,
        __area: *const ::c_void,
        __size: ::c_int,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptAttachArray_r(
        __intr: ::c_int,
        __handler: ::Option<
            unsafe extern "C" fn(__area: *mut ::c_void, __id: ::c_int) -> *const *const ::sigevent,
        >,
        __area: *const ::c_void,
        __size: ::c_int,
        __flags: ::c_uint,
    ) -> ::c_int;
    pub fn InterruptDetach(__id: ::c_int) -> ::c_int;
    pub fn InterruptDetach_r(__id: ::c_int) -> ::c_int;
    pub fn InterruptWait(__flags: ::c_int, __timeout: *const u64) -> ::c_int;
    pub fn InterruptWait_r(__flags: ::c_int, __timeout: *const u64) -> ::c_int;

    pub fn SchedGet(__pid: ::pid_t, __tid: ::c_int, __param: *mut ::sched_param) -> ::c_int;
    pub fn SchedGet_r(__pid: ::pid_t, __tid: ::c_int, __param: *mut ::sched_param) -> ::c_int;
    pub fn SchedGetCpuNum() -> ::c_uint;
    pub fn SchedSet(
        __pid: ::pid_t,
        __tid: ::c_int,
        __algorithm: ::c_int,
        __param: *const ::sched_param,
    ) -> ::c_int;
    pub fn SchedSet_r(
        __pid: ::pid_t,
        __tid: ::c_int,
        __algorithm: ::c_int,
        __param: *const ::sched_param,
    ) -> ::c_int;
    pub fn SchedInfo(__pid: ::pid_t, __algorithm: ::c_int, __info: *mut ::_sched_info) -> ::c_int;
    pub fn SchedInfo_r(__pid: ::pid_t, __algorithm: ::c_int, __info: *mut ::_sched_info)
        -> ::c_int;
    pub fn SchedYield() -> ::c_int;
    pub fn SchedYield_r() -> ::c_int;
    pub fn SchedCtl(__cmd: ::c_int, __data: *mut ::c_void, __length: usize) -> ::c_int;
    pub fn SchedCtl_r(__cmd: ::c_int, __data: *mut ::c_void, __length: usize) -> ::c_int;
    pub fn SchedJobCreate(__job: *mut nto_job_t) -> ::c_int;
    pub fn SchedJobCreate_r(__job: *mut nto_job_t) -> ::c_int;
    pub fn SchedJobDestroy(__job: *mut nto_job_t) -> ::c_int;
    pub fn SchedJobDestroy_r(__job: *mut nto_job_t) -> ::c_int;

    pub fn TimerCreate(__id: ::clockid_t, __notify: *const ::sigevent) -> ::c_int;
    pub fn TimerCreate_r(__id: ::clockid_t, __notify: *const ::sigevent) -> ::c_int;
    pub fn TimerDestroy(__id: ::timer_t) -> ::c_int;
    pub fn TimerDestroy_r(__id: ::timer_t) -> ::c_int;
    pub fn TimerSettime(
        __id: ::timer_t,
        __flags: ::c_int,
        __itime: *const ::_itimer,
        __oitime: *mut ::_itimer,
    ) -> ::c_int;
    pub fn TimerSettime_r(
        __id: ::timer_t,
        __flags: ::c_int,
        __itime: *const ::_itimer,
        __oitime: *mut ::_itimer,
    ) -> ::c_int;
    pub fn TimerInfo(
        __pid: ::pid_t,
        __id: ::timer_t,
        __flags: ::c_int,
        __info: *mut ::_timer_info,
    ) -> ::c_int;
    pub fn TimerInfo_r(
        __pid: ::pid_t,
        __id: ::timer_t,
        __flags: ::c_int,
        __info: *mut ::_timer_info,
    ) -> ::c_int;
    pub fn TimerAlarm(
        __id: ::clockid_t,
        __itime: *const ::_itimer,
        __otime: *mut ::_itimer,
    ) -> ::c_int;
    pub fn TimerAlarm_r(
        __id: ::clockid_t,
        __itime: *const ::_itimer,
        __otime: *mut ::_itimer,
    ) -> ::c_int;

    pub fn SyncTypeCreate(
        __type: ::c_uint,
        __sync: *mut ::sync_t,
        __attr: *const ::_sync_attr,
    ) -> ::c_int;
    pub fn SyncTypeCreate_r(
        __type: ::c_uint,
        __sync: *mut ::sync_t,
        __attr: *const ::_sync_attr,
    ) -> ::c_int;
    pub fn SyncDestroy(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncDestroy_r(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncCtl(__cmd: ::c_int, __sync: *mut ::sync_t, __data: *mut ::c_void) -> ::c_int;
    pub fn SyncCtl_r(__cmd: ::c_int, __sync: *mut ::sync_t, __data: *mut ::c_void) -> ::c_int;
    pub fn SyncMutexEvent(__sync: *mut ::sync_t, event: *const ::sigevent) -> ::c_int;
    pub fn SyncMutexEvent_r(__sync: *mut ::sync_t, event: *const ::sigevent) -> ::c_int;
    pub fn SyncMutexLock(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncMutexLock_r(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncMutexUnlock(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncMutexUnlock_r(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncCondvarWait(__sync: *mut ::sync_t, __mutex: *mut ::sync_t) -> ::c_int;
    pub fn SyncCondvarWait_r(__sync: *mut ::sync_t, __mutex: *mut ::sync_t) -> ::c_int;
    pub fn SyncCondvarSignal(__sync: *mut ::sync_t, __all: ::c_int) -> ::c_int;
    pub fn SyncCondvarSignal_r(__sync: *mut ::sync_t, __all: ::c_int) -> ::c_int;
    pub fn SyncSemPost(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncSemPost_r(__sync: *mut ::sync_t) -> ::c_int;
    pub fn SyncSemWait(__sync: *mut ::sync_t, __tryto: ::c_int) -> ::c_int;
    pub fn SyncSemWait_r(__sync: *mut ::sync_t, __tryto: ::c_int) -> ::c_int;

    pub fn ClockTime(__id: ::clockid_t, _new: *const u64, __old: *mut u64) -> ::c_int;
    pub fn ClockTime_r(__id: ::clockid_t, _new: *const u64, __old: *mut u64) -> ::c_int;
    pub fn ClockAdjust(
        __id: ::clockid_t,
        _new: *const ::_clockadjust,
        __old: *mut ::_clockadjust,
    ) -> ::c_int;
    pub fn ClockAdjust_r(
        __id: ::clockid_t,
        _new: *const ::_clockadjust,
        __old: *mut ::_clockadjust,
    ) -> ::c_int;
    pub fn ClockPeriod(
        __id: ::clockid_t,
        _new: *const ::_clockperiod,
        __old: *mut ::_clockperiod,
        __reserved: ::c_int,
    ) -> ::c_int;
    pub fn ClockPeriod_r(
        __id: ::clockid_t,
        _new: *const ::_clockperiod,
        __old: *mut ::_clockperiod,
        __reserved: ::c_int,
    ) -> ::c_int;
    pub fn ClockId(__pid: ::pid_t, __tid: ::c_int) -> ::c_int;
    pub fn ClockId_r(__pid: ::pid_t, __tid: ::c_int) -> ::c_int;

    //
    //TODO: The following commented out functions are implemented in assembly.
    //      We can implmement them either via a C stub or rust's inline assembly.
    //
    //pub fn InterruptEnable();
    //pub fn InterruptDisable();
    pub fn InterruptMask(__intr: ::c_int, __id: ::c_int) -> ::c_int;
    pub fn InterruptUnmask(__intr: ::c_int, __id: ::c_int) -> ::c_int;
    //pub fn InterruptLock(__spin: *mut ::intrspin);
    //pub fn InterruptUnlock(__spin: *mut ::intrspin);
    //pub fn InterruptStatus() -> ::c_uint;
}
