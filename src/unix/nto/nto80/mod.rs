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
pub type u_int = ::c_uint;
pub type tcflag_t = ::c_uint;

pub type sa_family_t = u8;
pub type rcvid_t = i64;

s! {
    pub struct bpf_stat {
        pub bs_recv: u_int,
        pub bs_drop: u_int,
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8],
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
        pub msg_len: ::ssize_t,
    }

    pub struct mallinfo {
        pub arena: ::size_t,
        pub ordblks: ::size_t,
        pub smblks: ::size_t,
        pub hblks: ::size_t,
        pub hblkhd: ::size_t,
        pub usmblks: ::size_t,
        pub fsmblks: ::size_t,
        pub uordblks: ::size_t,
        pub fordblks: ::size_t,
        pub keepcost: ::size_t,
    }

    #[repr(packed)]
    pub struct arphdr {
        pub ar_hrd: ::c_ushort,
        pub ar_pro: ::c_ushort,
        pub ar_hln: ::c_uchar,
        pub ar_pln: ::c_uchar,
        pub ar_op: ::c_ushort,
    }

    pub struct sockaddr {
        pub sa_len: ::c_uchar,
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct qtime_entry {
        pub cycles_per_sec: u64,
        pub nsec_tod_adjust: u64, // volatile
        pub nsec_inc: u32,
        pub boot_time: u32,
        pub adjust: _clockadjust,
        pub timer_period: u32,
        pub timer_scale: i32,
        pub intr: i32,
        pub epoch: u32,
        pub flags: u32,
        pub rr_interval_mul: u32,
        pub timer_load_max: u64,
        pub boot_cc: u64,
        pub tick_period_cc: u64,
        spare: [u64; 3],
    }

    pub struct _thread_attr {
        pub __flags: ::c_int,
        pub __stacksize: ::size_t,
        pub __stackaddr: *mut ::c_void,
        pub __reserved0: ::uintptr_t,
        pub __policy: ::c_int,
        pub __param: ::__sched_param,
        pub __guardsize: ::c_uint,
        pub __prealloc: ::c_uint,
        __reserved1: ::uintptr_t,
    }

     pub struct sockaddr_dl {
        pub sdl_len: ::c_uchar,
        pub sdl_family: ::c_uchar,
        pub sdl_index: ::c_ushort,
        pub sdl_type: ::c_uchar,
        pub sdl_nlen: ::c_uchar,
        pub sdl_alen: ::c_uchar,
        pub sdl_slen: ::c_uchar,
        pub sdl_data: [::c_char; 46],
    }
}

s_no_extra_traits! {
    pub struct pthread_rwlock_t {
        __count: ::c_uint,
        pub __owner: ::c_uint,
    }

    pub struct sync_t {
        __u: ::c_uint,            // union
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

    // #[repr(C)]
    // pub struct version {
    //     pub major: u16, 
    //     pub minor: u16,
    // }

    // #[repr(C)]
    // #[cfg(target_arch = "x86_64")]
    // pub struct x86_64SyspageEntry {
    //     pub spare: [u64; 4],
    // }

    // #[repr(C)]
    // #[cfg(target_arch = "aarch64")]
    // pub struct aarch64SyspageEntry {
    //     pub L0_vaddr: u64,
    //     pub gic_map: syspage_entry_info,
    //     pub gicr_map: syspage_entry_info,
    //     pub idreg_dump: syspage_entry_info,
    //     pub rsvd: [u64; 6],
    // }

    // #[repr(C)]
    // pub union Un {
    //     #[cfg(target_arch = "x86_64")]
    //     pub x86_64: x86_64SyspageEntry,
    //     #[cfg(target_arch = "aarch64")]
    //     pub aarch64: aarch64SyspageEntry,
    //     pub filler: [u64; 20],               // Filler for unused space
    // }

    #[repr(align(8))]
    pub struct syspage_entry {
        pub size: u16,
        pub total_size: u16,
        pub type_: u16,
        pub num_cpu: u16,
        pub system_private: syspage_entry_info,
        // pub version: version,
        pub hwinfo: syspage_entry_info,
        pub qtime: syspage_entry_info,
        pub callout: syspage_entry_info,
        pub typed_strings: syspage_entry_info,
        pub strings: syspage_entry_info,
        pub smp: syspage_entry_info,
        // pub un: Un,
        __reserved: [u8; 160],
        pub asinfo: syspage_array_info,
        pub cpuinfo: syspage_array_info,
        pub cacheattr: syspage_array_info,
        pub intrinfo: syspage_array_info,
        pub hypinfo: syspage_entry_info,
        pub cluster: syspage_array_info,
    }
}

// Address families
pub const AF_UNSPEC: ::c_int = 0;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_UNIX: ::c_int = AF_LOCAL;
pub const AF_INET: ::c_int = 2;
pub const AF_IMPLINK: ::c_int = 3;
pub const AF_PUP: ::c_int = 4;
pub const AF_CHAOS: ::c_int = 5;
pub const AF_NETBIOS: ::c_int = 6;
pub const AF_ISO: ::c_int = 7;
pub const AF_OSI: ::c_int = AF_ISO;
pub const AF_ECMA: ::c_int = 8;
pub const AF_DATAKIT: ::c_int = 9;
pub const AF_CCITT: ::c_int = 10;
pub const AF_SNA:  ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_DLI: ::c_int = 13;
pub const AF_LAT: ::c_int = 14;
pub const AF_HYLINK: ::c_int = 15;
pub const AF_APPLETALK: ::c_int = 16;
pub const AF_ROUTE: ::c_int = 17;
pub const AF_LINK: ::c_int = 18;
pub const pseudo_AF_XTP: ::c_int = 19;
pub const AF_COIP: ::c_int = 20;
pub const AF_CNT: ::c_int = 21;
pub const pseudo_AF_RTIP: ::c_int = 22;
pub const AF_IPX: ::c_int = 23;
pub const AF_SIP: ::c_int = 24;
pub const pseudo_AF_PIP: ::c_int = 25;
pub const AF_ISDN: ::c_int = 26;
pub const AF_E164: ::c_int = AF_ISDN;
pub const pseudo_AF_KEY: ::c_int = 27;
pub const AF_INET6: ::c_int = 28; 
pub const AF_NATM: ::c_int =		29;		 
pub const AF_ATM: ::c_int =		30;		 
pub const pseudo_AF_HDRCMPLT: ::c_int = 31;
pub const AF_NETGRAPH: ::c_int =	32;		 
pub const AF_SLOW: ::c_int =		33;		 
pub const AF_SCLUSTER: ::c_int =	34;		 
pub const AF_ARP: ::c_int =		35;
pub const AF_BLUETOOTH: ::c_int =	36;	
pub const AF_IEEE80211: ::c_int =	37;	
pub const AF_NETLINK: ::c_int =	38;		 
pub const AF_INET_SDP: ::c_int =	40;		 
pub const AF_INET6_SDP: ::c_int =	42;	
pub const AF_HYPERV: ::c_int =	43;		 
pub const AF_MAX: ::c_int =		43;

pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_UNIX: ::c_int = PF_LOCAL;
pub const PF_INET: ::c_int = AF_INET;
pub const PF_IMPLINK: ::c_int = AF_IMPLINK;
pub const PF_PUP: ::c_int = AF_PUP;
pub const PF_CHAOS: ::c_int = AF_CHAOS;
pub const PF_NETBIOS: ::c_int = AF_NETBIOS;
pub const PF_ISO: ::c_int = AF_ISO;
pub const PF_OSI: ::c_int = AF_ISO;
pub const PF_DATAKIT: ::c_int = AF_DATAKIT;
pub const PF_ECMA: ::c_int = AF_ECMA;
pub const PF_CCITT: ::c_int = AF_CCITT;
pub const PF_SNA: ::c_int = AF_SNA;
pub const PF_DECnet: ::c_int = AF_DECnet;
pub const PF_DLI: ::c_int = AF_DLI;
pub const PF_LAT: ::c_int = AF_LAT;
pub const PF_HYLINK: ::c_int = AF_HYLINK;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_ROUTE: ::c_int = AF_ROUTE;
pub const PF_LINK: ::c_int = AF_LINK;
pub const PF_XTP: ::c_int = pseudo_AF_XTP;
pub const PF_COIP: ::c_int = AF_COIP;
pub const PF_CNT: ::c_int = AF_CNT;
pub const PF_SIP: ::c_int = AF_SIP;
pub const PF_IPX: ::c_int = AF_IPX;
pub const PF_RTIP: ::c_int = pseudo_AF_RTIP;
pub const PF_PIP: ::c_int = pseudo_AF_PIP;
pub const PF_ISDN: ::c_int = AF_ISDN;
pub const PF_KEY: ::c_int = pseudo_AF_KEY;
pub const PF_INET6: ::c_int = AF_INET6;
pub const PF_NATM: ::c_int = AF_NATM;
pub const PF_ATM: ::c_int =		AF_ATM;
pub const PF_NETGRAPH: ::c_int =	AF_NETGRAPH;
pub const PF_SLOW: ::c_int =		AF_SLOW;
pub const PF_SCLUSTER: ::c_int =	AF_SCLUSTER;
pub const PF_ARP: ::c_int = AF_ARP;


pub const IFF_UP: ::c_int = 0x1;
pub const IFF_BROADCAST: ::c_int = 0x2;
pub const IFF_DEBUG: ::c_int = 0x4;
pub const IFF_LOOPBACK: ::c_int = 0x8;
pub const IFF_POINTOPOINT: ::c_int = 0x10;
pub const IFF_KNOWSEPOCH: ::c_int = 0x20;
pub const IFF_DRV_RUNNING: ::c_int = 0x40;
pub const IFF_NOARP: ::c_int = 0x80;
pub const IFF_PROMISC: ::c_int = 0x100;
pub const IFF_ALLMULTI: ::c_int = 0x200;
pub const IFF_DRV_OACTIVE: ::c_int = 0x400;	
pub const IFF_SIMPLEX: ::c_int = 0x800;		
pub const IFF_LINK0: ::c_int = 	0x1000;		
pub const IFF_LINK1: ::c_int = 	0x2000;		
pub const IFF_LINK2: ::c_int = 	0x4000;		
pub const IFF_ALTPHYS: ::c_int = IFF_LINK2;	
pub const IFF_MULTICAST: ::c_int = 0x8000;
pub const IFF_CANTCONFIG: ::c_int =	    0x10000;
pub const IFF_PPROMISC: ::c_int =	    0x20000;	
pub const IFF_MONITOR: ::c_int =	    0x40000;	
pub const IFF_STATICARP: ::c_int =	    0x80000;	
pub const IFF_DYING: ::c_int =	        0x200000;	
pub const IFF_RENAMING: ::c_int =	    0x400000;	
pub const IFF_NOGROUP: ::c_int =	    0x800000;	
pub const IFF_NETLINK_1: ::c_int =	    0x1000000;

pub const F_SETOWN: ::c_int = 36;

pub const IHFLOW: tcflag_t = 0x00000001;
pub const IIDLE: tcflag_t = 0x00000008;
pub const IPTOS_ECN_NOTECT: u8 = 0x00;
pub const IUCLC: tcflag_t = 0x00000200;
pub const IUTF8: tcflag_t = 0x0004000;

// netinet/in.h
pub const IP_OPTIONS: ::c_int = 1;
pub const IP_HDRINCL: ::c_int = 2;
pub const IP_TOS: ::c_int = 3;
pub const IP_TTL: ::c_int = 4;
pub const IP_RECVOPTS: ::c_int = 5;
pub const IP_RECVRETOPTS: ::c_int = 6;
pub const IP_RECVDSTADDR: ::c_int = 7;
pub const IP_SENDSRCADDR: ::c_int = IP_RECVDSTADDR;
pub const IP_RETOPTS: ::c_int = 8;
pub const IP_MULTICAST_IF: ::c_int = 9;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;
pub const IP_MULTICAST_VIF: ::c_int = 14;
pub const IP_RSVP_ON: ::c_int = 15;
pub const IP_RSVP_OFF: ::c_int = 16;
pub const IP_RSVP_VIF_ON: ::c_int = 17;
pub const IP_RSVP_VIF_OFF: ::c_int = 18;
pub const IP_PORTRANGE: ::c_int = 19;
pub const IP_RECVIF: ::c_int = 20;
pub const IP_IPSEC_POLICY: ::c_int = 21;
// 22 unused; was IP_FAITH 
pub const IP_ONESBCAST: ::c_int = 23;
pub const IP_BINDANY: ::c_int = 24;
pub const IP_BINDMULTI: ::c_int = 25;
pub const IP_RSS_LISTEN_BUCKET: ::c_int = 26;
pub const IP_ORIGDSTADDR: ::c_int = 27;
pub const IP_RECVORIGDSTADDR: ::c_int = IP_ORIGDSTADDR;

pub const IP_DEFAULT_MULTICAST_TTL: ::c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: ::c_int = 1;

pub const IP_MAX_MEMBERSHIPS: ::c_int = 4095;

pub const IP_MAX_GROUP_SRC_FILTER: ::c_int = 512;
pub const IP_MAX_SOCK_SRC_FILTER: ::c_int = 128;
pub const IP_MAX_SOCK_MUTE_FILTER: ::c_int = 128;

pub const IPV6_IPSEC_POLICY: ::c_int = 28;
// 29; unused; was IPV6_FAITH 
// ipv6 firewall
pub const IPV6_FW_ADD: ::c_int   = 30;
pub const IPV6_FW_DEL: ::c_int   = 31;
pub const IPV6_FW_FLUSH: ::c_int = 32;
pub const IPV6_FW_ZERO: ::c_int  = 33;
pub const IPV6_FW_GET: ::c_int   = 34;

pub const IPV6_PREFER_TEMPADDR: ::c_int =	63;
pub const IPV6_BINDANY: ::c_int =		64; 
pub const IPV6_BINDMULTI: ::c_int =		65; 
pub const IPV6_RSS_LISTEN_BUCKET: ::c_int =	66;
pub const IPV6_FLOWID: ::c_int =		67; 
pub const IPV6_FLOWTYPE: ::c_int =		68;  
pub const IPV6_RSSBUCKETID: ::c_int =	69;  
pub const IPV6_RECVFLOWID: ::c_int =		70; 
pub const IPV6_RECVRSSBUCKETID: ::c_int =	71; 
pub const IPV6_ORIGDSTADDR: ::c_int =	72;
pub const IPV6_RECVORIGDSTADDR: ::c_int =	IPV6_ORIGDSTADDR;


pub const TCP_NODELAY: ::c_int =	1;	
pub const TCP_MAXSEG: ::c_int =	    2;	
pub const TCP_NOPUSH: ::c_int =	    4;	
pub const TCP_NOOPT: ::c_int =	    8;	
pub const TCP_MD5SIG: ::c_int =	    16;	
pub const TCP_INFO: ::c_int =	    32;	
pub const TCP_STATS: ::c_int =	    33;	
pub const TCP_LOG: ::c_int =		34;	
pub const TCP_LOGBUF: ::c_int =	    35;	
pub const TCP_LOGID: ::c_int =	    36;	
pub const TCP_LOGDUMP: ::c_int =	37;	
pub const TCP_LOGDUMPID: ::c_int =	38;

pub const TCP_TXTLS_ENABLE: ::c_int = 39;	
pub const TCP_TXTLS_MODE: ::c_int =	40;	
pub const TCP_RXTLS_ENABLE: ::c_int = 41;	
pub const TCP_RXTLS_MODE: ::c_int =	42;	
pub const TCP_IWND_NB: ::c_int =	43;	
pub const TCP_IWND_NSEG: ::c_int =	44;	
pub const TCP_LOGID_CNT: ::c_int =	46;	
pub const TCP_LOG_TAG: ::c_int =	47;	
pub const TCP_USER_LOG: ::c_int =	48;	
pub const TCP_CONGESTION: ::c_int =	64;	
pub const TCP_CCALGOOPT: ::c_int =	65;	
pub const TCP_MAXUNACKTIME: ::c_int = 68;	
pub const TCP_MAXPEAKRATE: ::c_int = 69;	
pub const TCP_IDLE_REDUCE: ::c_int = 70;	
pub const TCP_REMOTE_UDP_ENCAPS_PORT: ::c_int = 71;	
pub const TCP_DELACK: ::c_int =  	72;	
pub const TCP_FIN_IS_RST: ::c_int = 73;	
pub const TCP_LOG_LIMIT: ::c_int =  74;	
pub const TCP_SHARED_CWND_ALLOWED: ::c_int = 75;
pub const TCP_PROC_ACCOUNTING: ::c_int = 76;
pub const TCP_USE_CMP_ACKS: ::c_int = 77; 
pub const TCP_PERF_INFO: ::c_int =	78;	 
pub const TCP_KEEPINIT: ::c_int =	128;	 
pub const TCP_KEEPIDLE: ::c_int =	256;	 
pub const TCP_KEEPINTVL: ::c_int =	512;	 
pub const TCP_KEEPCNT: ::c_int =	1024;	 
pub const TCP_FASTOPEN: ::c_int =	1025;
pub const TCP_PCAP_OUT: ::c_int =	2048;
pub const TCP_PCAP_IN: ::c_int =	4096;	
pub const TCP_FUNCTION_BLK: ::c_int = 8192;	
// Options for Rack and BBR 
pub const TCP_REUSPORT_LB_NUMA: ::c_int =  1026;
pub const TCP_RACK_MBUF_QUEUE: ::c_int =   1050; 
pub const TCP_RACK_PROP: ::c_int =	      1051; 
pub const TCP_RACK_TLP_REDUCE: ::c_int =   1052; 
pub const TCP_RACK_PACE_REDUCE: ::c_int =  1053; 
pub const TCP_RACK_PACE_MAX_SEG: ::c_int = 1054; 
pub const TCP_RACK_PACE_ALWAYS: ::c_int =  1055; 
pub const TCP_RACK_PROP_RATE: ::c_int =    1056; 
pub const TCP_RACK_PRR_SENDALOT: ::c_int = 1057; 
pub const TCP_RACK_MIN_TO: ::c_int =       1058; 
pub const TCP_RACK_EARLY_RECOV: ::c_int =  1059; 
pub const TCP_RACK_EARLY_SEG: ::c_int =    1060; 
pub const TCP_RACK_REORD_THRESH: ::c_int = 1061; 
pub const TCP_RACK_REORD_FADE: ::c_int =   1062; 
pub const TCP_RACK_TLP_THRESH: ::c_int =   1063; 
pub const TCP_RACK_PKT_DELAY: ::c_int =    1064; 
pub const TCP_RACK_TLP_INC_VAR: ::c_int =  1065; 
pub const TCP_BBR_IWINTSO: ::c_int =	      1067; 
pub const TCP_BBR_RECFORCE: ::c_int =      1068; 
pub const TCP_BBR_STARTUP_PG: ::c_int =    1069; 
pub const TCP_BBR_DRAIN_PG: ::c_int =      1070; 
pub const TCP_BBR_RWND_IS_APP: ::c_int =   1071; 
pub const TCP_BBR_PROBE_RTT_INT: ::c_int = 1072; 
pub const TCP_BBR_ONE_RETRAN: ::c_int =    1073; 
pub const TCP_BBR_STARTUP_LOSS_EXIT: ::c_int = 1074;	
pub const TCP_BBR_USE_LOWGAIN: ::c_int =   1075; 
pub const TCP_BBR_LOWGAIN_THRESH: ::c_int = 1076; 
pub const TCP_BBR_TSLIMITS: ::c_int = 1076;	   
pub const TCP_BBR_LOWGAIN_HALF: ::c_int =  1077; 
pub const TCP_BBR_PACE_OH: ::c_int =        1077;
pub const TCP_BBR_LOWGAIN_FD: ::c_int =    1078; 
pub const TCP_BBR_HOLD_TARGET: ::c_int = 1078;	
pub const TCP_BBR_USEDEL_RATE: ::c_int =   1079; 
pub const TCP_BBR_MIN_RTO: ::c_int =       1080; 
pub const TCP_BBR_MAX_RTO: ::c_int =	      1081; 
pub const TCP_BBR_REC_OVER_HPTS: ::c_int = 1082; 
pub const TCP_BBR_UNLIMITED: ::c_int =     1083; 
pub const TCP_BBR_ALGORITHM: ::c_int =     1083; 
pub const TCP_BBR_DRAIN_INC_EXTRA: ::c_int = 1084; 
pub const TCP_BBR_STARTUP_EXIT_EPOCH: ::c_int = 1085; 
pub const TCP_BBR_PACE_PER_SEC: ::c_int =   1086;
pub const TCP_BBR_PACE_DEL_TAR: ::c_int =   1087;
pub const TCP_BBR_PACE_SEG_MAX: ::c_int =   1088;
pub const TCP_BBR_PACE_SEG_MIN: ::c_int =   1089;
pub const TCP_BBR_PACE_CROSS: ::c_int =     1090;
pub const TCP_RACK_IDLE_REDUCE_HIGH: ::c_int = 1092; 
pub const TCP_RACK_MIN_PACE: ::c_int =      1093; 
pub const TCP_RACK_MIN_PACE_SEG: ::c_int =  1094;	
pub const TCP_RACK_GP_INCREASE: ::c_int =   1094;	
pub const TCP_RACK_TLP_USE: ::c_int =       1095;
pub const TCP_BBR_ACK_COMP_ALG: ::c_int =   1096; 
pub const TCP_BBR_TMR_PACE_OH: ::c_int =    1096;	
pub const TCP_BBR_EXTRA_GAIN: ::c_int =     1097;
pub const TCP_RACK_DO_DETECTION: ::c_int =  1097;	
pub const TCP_BBR_RACK_RTT_USE: ::c_int =   1098;	
pub const TCP_BBR_RETRAN_WTSO: ::c_int =    1099;
pub const TCP_DATA_AFTER_CLOSE: ::c_int =   1100;
pub const TCP_BBR_PROBE_RTT_GAIN: ::c_int = 1101;
pub const TCP_BBR_PROBE_RTT_LEN: ::c_int =  1102;
pub const TCP_BBR_SEND_IWND_IN_TSO: ::c_int = 1103;	
pub const TCP_BBR_USE_RACK_RR: ::c_int =	 1104;	
pub const TCP_BBR_USE_RACK_CHEAT: ::c_int = TCP_BBR_USE_RACK_RR;
pub const TCP_BBR_HDWR_PACE: ::c_int =      1105;
pub const TCP_BBR_UTTER_MAX_TSO: ::c_int =  1106;
pub const TCP_BBR_EXTRA_STATE: ::c_int =    1107;
pub const TCP_BBR_FLOOR_MIN_TSO: ::c_int =  1108;
pub const TCP_BBR_MIN_TOPACEOUT: ::c_int =  1109;
pub const TCP_BBR_TSTMP_RAISES: ::c_int =   1110;
pub const TCP_BBR_POLICER_DETECT: ::c_int = 1111;
pub const TCP_BBR_RACK_INIT_RATE: ::c_int = 1112;
pub const TCP_RACK_RR_CONF: ::c_int =	1113; 
pub const TCP_RACK_CHEAT_NOT_CONF_RATE: ::c_int = TCP_RACK_RR_CONF;
pub const TCP_RACK_GP_INCREASE_CA: ::c_int =   1114;	
pub const TCP_RACK_GP_INCREASE_SS: ::c_int =   1115;
pub const TCP_RACK_GP_INCREASE_REC: ::c_int =  1116;
pub const TCP_RACK_FORCE_MSEG: ::c_int =	1117;	
pub const TCP_RACK_PACE_RATE_CA: ::c_int =  1118; 
pub const TCP_RACK_PACE_RATE_SS: ::c_int =  1119; 
pub const TCP_RACK_PACE_RATE_REC: ::c_int =  1120; 
pub const TCP_NO_PRR: ::c_int =         	1122; 
pub const TCP_RACK_NONRXT_CFG_RATE: ::c_int = 1123; 
pub const TCP_SHARED_CWND_ENABLE: ::c_int =   1124; 
pub const TCP_TIMELY_DYN_ADJ: ::c_int =       1125; 
pub const TCP_RACK_NO_PUSH_AT_MAX: ::c_int = 1126; 
pub const TCP_RACK_PACE_TO_FILL: ::c_int = 1127;  
pub const TCP_SHARED_CWND_TIME_LIMIT: ::c_int = 1128; 
pub const TCP_RACK_PROFILE: ::c_int = 1129;	 
pub const TCP_HDWR_RATE_CAP: ::c_int = 1130;   
pub const TCP_PACING_RATE_CAP: ::c_int = 1131;  
pub const TCP_HDWR_UP_ONLY: ::c_int = 1132;	  
pub const TCP_RACK_ABC_VAL: ::c_int = 1133;	 
pub const TCP_REC_ABC_VAL: ::c_int = 1134;	  
pub const TCP_RACK_MEASURE_CNT: ::c_int = 1135;
pub const TCP_DEFER_OPTIONS: ::c_int = 1136;    
pub const TCP_FAST_RSM_HACK: ::c_int = 1137;    
pub const TCP_RACK_PACING_BETA: ::c_int = 1138;
pub const TCP_RACK_PACING_BETA_ECN: ::c_int = 1139;
pub const TCP_RACK_TIMER_SLOP: ::c_int = 1140;

// sysctl.h
pub const HW_MACHINE: ::c_int = 1;
pub const HW_MODEL: ::c_int = 2;
pub const HW_NCPU: ::c_int = 3;
pub const HW_BYTEORDER: ::c_int = 4;
pub const HW_PHYSMEM: ::c_int = 5;
pub const HW_USERMEM: ::c_int = 6;
pub const HW_PAGESIZE: ::c_int = 7;
pub const HW_DISKNAMES: ::c_int = 8;
pub const HW_DISKSTATS: ::c_int = 9;
pub const HW_FLOATINGPT: ::c_int = 10;
pub const HW_MACHINE_ARCH: ::c_int = 11;
pub const HW_REALMEM: ::c_int = 12;

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
pub const USER_LOCALBASE: ::c_int = 21;

pub const CTL_SYSCTL: ::c_int = 0;
pub const CTL_KERN: ::c_int = 1;
pub const CTL_VM: ::c_int = 2;
pub const CTL_VFS: ::c_int = 3;
pub const CTL_NET: ::c_int = 4;
pub const CTL_DEBUG: ::c_int = 5;
pub const CTL_HW: ::c_int = 6;
pub const CTL_MACHDEP: ::c_int = 7;
pub const CTL_USER: ::c_int = 8;
pub const CTL_P1003_1B: ::c_int = 9;

pub const CTL_SYSCTL_DEBUG: ::c_int	     = 0;	
pub const CTL_SYSCTL_NAME: ::c_int		 = 1;	
pub const CTL_SYSCTL_NEXT: ::c_int		 = 2;	
pub const CTL_SYSCTL_NAME2OID: ::c_int	 = 3;	
pub const CTL_SYSCTL_OIDFMT: ::c_int	 = 4;	
pub const CTL_SYSCTL_OIDDESCR: ::c_int	 = 5;	
pub const CTL_SYSCTL_OIDLABEL: ::c_int	 = 6;	
pub const CTL_SYSCTL_NEXTNOSKIP: ::c_int = 7;

pub const CTL_P1003_1B_ASYNCHRONOUS_IO: ::c_int =		1;
pub const CTL_P1003_1B_MAPPED_FILES: ::c_int =		    2;
pub const CTL_P1003_1B_MEMLOCK: ::c_int =			    3;
pub const CTL_P1003_1B_MEMLOCK_RANGE: ::c_int =		    4;
pub const CTL_P1003_1B_MEMORY_PROTECTION: ::c_int =		5;
pub const CTL_P1003_1B_MESSAGE_PASSING: ::c_int =		6;
pub const CTL_P1003_1B_PRIORITIZED_IO: ::c_int =		    7;
pub const CTL_P1003_1B_PRIORITY_SCHEDULING: ::c_int =	8;
pub const CTL_P1003_1B_REALTIME_SIGNALS: ::c_int =		9;
pub const CTL_P1003_1B_SEMAPHORES: ::c_int =			   10;
pub const CTL_P1003_1B_FSYNC: ::c_int =			       11;
pub const CTL_P1003_1B_SHARED_MEMORY_OBJECTS: ::c_int = 12;
pub const CTL_P1003_1B_SYNCHRONIZED_IO: ::c_int =	   13;
pub const CTL_P1003_1B_TIMERS: ::c_int =			       14;
pub const CTL_P1003_1B_AIO_LISTIO_MAX: ::c_int =		   15;
pub const CTL_P1003_1B_AIO_MAX: ::c_int =			   16;
pub const CTL_P1003_1B_AIO_PRIO_DELTA_MAX: ::c_int =	   17;
pub const CTL_P1003_1B_DELAYTIMER_MAX: ::c_int =		   18;
pub const CTL_P1003_1B_MQ_OPEN_MAX: ::c_int =		   19;
pub const CTL_P1003_1B_PAGESIZE: ::c_int =			   20;
pub const CTL_P1003_1B_RTSIG_MAX: ::c_int =			   21;
pub const CTL_P1003_1B_SEM_NSEMS_MAX: ::c_int =		   22;
pub const CTL_P1003_1B_SEM_VALUE_MAX: ::c_int =		   23;
pub const CTL_P1003_1B_SIGQUEUE_MAX: ::c_int =		   24;
pub const CTL_P1003_1B_TIMER_MAX: ::c_int =			   25;

	
pub const IPPROTO_ST: ::c_int =		    7;		
pub const IPPROTO_PIGP: ::c_int =		9;	
pub const IPPROTO_RCCMON: ::c_int =		10;	
pub const IPPROTO_NVPII: ::c_int =		11;		
pub const IPPROTO_ARGUS: ::c_int =		13;	
pub const IPPROTO_EMCON: ::c_int =		14;	
pub const IPPROTO_XNET: ::c_int =		15;	
pub const IPPROTO_CHAOS: ::c_int =		16;	
pub const IPPROTO_MUX: ::c_int =		18;		
pub const IPPROTO_MEAS: ::c_int =		19;	
pub const IPPROTO_HMP: ::c_int =		20;		
pub const IPPROTO_PRM: ::c_int =		21;			
pub const IPPROTO_TRUNK1: ::c_int =		23;	
pub const IPPROTO_TRUNK2: ::c_int =		24;	
pub const IPPROTO_LEAF1: ::c_int =		25;	
pub const IPPROTO_LEAF2: ::c_int =		26;	
pub const IPPROTO_RDP: ::c_int =		27;		
pub const IPPROTO_IRTP: ::c_int =		28;	
pub const IPPROTO_BLT: ::c_int =		30;		
pub const IPPROTO_NSP: ::c_int =		31;		
pub const IPPROTO_INP: ::c_int =		32;		
pub const IPPROTO_DCCP: ::c_int =		33;
pub const IPPROTO_3PC: ::c_int =		34;		
pub const IPPROTO_IDPR: ::c_int =		35;	
pub const IPPROTO_XTP: ::c_int =		36;		
pub const IPPROTO_DDP: ::c_int =		37;		
pub const IPPROTO_CMTP: ::c_int =		38;	
pub const IPPROTO_TPXX: ::c_int =		39;	
pub const IPPROTO_IL: ::c_int =		    40;		
pub const IPPROTO_SDRP: ::c_int =		42;	
pub const IPPROTO_IDRP: ::c_int =		45;		
pub const IPPROTO_MHRP: ::c_int =		48;	
pub const IPPROTO_BHA: ::c_int =		49;			
pub const IPPROTO_INLSP: ::c_int =		52;	
pub const IPPROTO_SWIPE: ::c_int =		53;	
pub const IPPROTO_NHRP: ::c_int =		54;	
pub const IPPROTO_TLSP: ::c_int =		56;	
pub const IPPROTO_SKIP: ::c_int =		57;	
pub const IPPROTO_AHIP: ::c_int =		61;	
pub const IPPROTO_CFTP: ::c_int =		62;	
pub const IPPROTO_HELLO: ::c_int =		63;	
pub const IPPROTO_SATEXPAK: ::c_int =	64;	
pub const IPPROTO_KRYPTOLAN: ::c_int =	65;	
pub const IPPROTO_RVD: ::c_int =		66;		
pub const IPPROTO_IPPC: ::c_int =		67;	
pub const IPPROTO_ADFS: ::c_int =		68;	
pub const IPPROTO_SATMON: ::c_int =		69;	
pub const IPPROTO_VISA: ::c_int =		70;	
pub const IPPROTO_IPCV: ::c_int =		71;	
pub const IPPROTO_CPNX: ::c_int =		72;	
pub const IPPROTO_CPHB: ::c_int =		73;	
pub const IPPROTO_WSN: ::c_int =		74;		
pub const IPPROTO_PVP: ::c_int =		75;		
pub const IPPROTO_BRSATMON: ::c_int =	76;	
pub const IPPROTO_ND: ::c_int =		77;		
pub const IPPROTO_WBMON: ::c_int =		78;	
pub const IPPROTO_WBEXPAK: ::c_int =		79;		
pub const IPPROTO_VMTP: ::c_int =		81;	
pub const IPPROTO_SVMTP: ::c_int =		82;	
pub const IPPROTO_VINES: ::c_int =		83;	
pub const IPPROTO_TTP: ::c_int =		84;		
pub const IPPROTO_IGP: ::c_int =		85;		
pub const IPPROTO_DGP: ::c_int =		86;		
pub const IPPROTO_TCF: ::c_int =		87;		
pub const IPPROTO_IGRP: ::c_int =		88;	
pub const IPPROTO_OSPFIGP: ::c_int =		89;	
pub const IPPROTO_SRPC: ::c_int =		90;	
pub const IPPROTO_LARP: ::c_int =		91;	
pub const IPPROTO_MTP: ::c_int =		92;		
pub const IPPROTO_AX25: ::c_int =		93;	
pub const IPPROTO_IPEIP: ::c_int =		94;	
pub const IPPROTO_MICP: ::c_int =		95;	
pub const IPPROTO_SCCSP: ::c_int =		96;	
pub const IPPROTO_APES: ::c_int =		99;	
pub const IPPROTO_GMTP: ::c_int =		100;		
pub const IPPROTO_MH: ::c_int =		135;		
pub const IPPROTO_UDPLITE: ::c_int =		136;	
pub const IPPROTO_HIP: ::c_int =		139;		
pub const IPPROTO_SHIM6: ::c_int =		140;	
pub const IPPROTO_PGM: ::c_int =		113;		
pub const IPPROTO_MPLS: ::c_int =		137;	
pub const IPPROTO_PFSYNC: ::c_int =		240;	
pub const IPPROTO_OLD_DIVERT: ::c_int =	254;				
pub const IPPROTO_SEND: ::c_int =		259;		

pub const KERN_OSTYPE: ::c_int = 1;
pub const KERN_OSRELEASE: ::c_int = 2;
pub const KERN_OSREV: ::c_int = 3;
pub const KERN_VERSION: ::c_int = 4;
pub const KERN_MAXVNODES: ::c_int = 5;
pub const KERN_MAXPROC: ::c_int = 6;
pub const KERN_MAXFILES: ::c_int = 7;
pub const KERN_ARGMAX: ::c_int = 8;
pub const KERN_SECURELVL: ::c_int = 9;
pub const KERN_HOSTNAME: ::c_int = 10;
pub const KERN_HOSTID: ::c_int = 11;
pub const KERN_CLOCKRATE: ::c_int = 12;
pub const KERN_VNODE: ::c_int = 13;
pub const KERN_PROC: ::c_int = 14;
pub const KERN_FILE: ::c_int = 15;
pub const KERN_PROF: ::c_int = 16;
pub const KERN_POSIX1: ::c_int = 17;
pub const KERN_NGROUPS: ::c_int = 18;
pub const KERN_JOB_CONTROL: ::c_int = 19;
pub const KERN_SAVED_IDS: ::c_int = 20;
pub const KERN_BOOTTIME: ::c_int = 21;
pub const KERN_NISDOMAINNAME: ::c_int = 22;
pub const KERN_UPDATEINTERVAL: ::c_int =	23;
pub const KERN_OSRELDATE: ::c_int =		24;
pub const KERN_NTP_PLL: ::c_int =		25;
pub const KERN_BOOTFILE: ::c_int =		26;
pub const KERN_MAXFILESPERPROC: ::c_int = 27;
pub const KERN_MAXPROCPERUID: ::c_int =	28;
pub const KERN_DUMPDEV: ::c_int =		29;
pub const KERN_IPC: ::c_int =		    30;
pub const KERN_DUMMY: ::c_int =		    31;
pub const KERN_PS_STRINGS: ::c_int =		32;
pub const KERN_USRSTACK: ::c_int =		33;
pub const KERN_LOGSIGEXIT: ::c_int =	34;
pub const KERN_IOV_MAX: ::c_int =		35;
pub const KERN_HOSTUUID: ::c_int =		36;
pub const KERN_ARND: ::c_int = 37;
pub const KERN_MAXPHYS: ::c_int = 38;
pub const KERN_LOCKF: ::c_int = 39;

// sys/sysctl.h
pub const KERN_PROC_ALL: ::c_int = 0;
pub const KERN_PROC_PID: ::c_int = 1;
pub const KERN_PROC_PGRP: ::c_int = 2;
pub const KERN_PROC_SESSION: ::c_int = 3;
pub const KERN_PROC_TTY: ::c_int = 4;
pub const KERN_PROC_UID: ::c_int = 5;
pub const KERN_PROC_RUID: ::c_int = 6;
pub const KERN_PROC_ARGS: ::c_int = 7;
pub const KERN_PROC_PROC: ::c_int = 8;
pub const KERN_PROC_SV_NAME: ::c_int = 9;
pub const KERN_PROC_RGID: ::c_int = 10;
pub const KERN_PROC_GID: ::c_int = 11;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const KERN_PROC_OVMMAP: ::c_int =	13;	
pub const KERN_PROC_OFILEDESC: ::c_int =	14;	
pub const KERN_PROC_KSTACK: ::c_int =	15;	
pub const KERN_PROC_INC_THREAD: ::c_int =	0x10;	
pub const KERN_PROC_VMMAP: ::c_int =		32;	
pub const KERN_PROC_FILEDESC: ::c_int =	33;	
pub const KERN_PROC_GROUPS: ::c_int =	34;	
pub const KERN_PROC_ENV: ::c_int =		35;	
pub const KERN_PROC_AUXV: ::c_int =		36;	
pub const KERN_PROC_RLIMIT: ::c_int =	37;	
pub const KERN_PROC_PS_STRINGS: ::c_int =	38;	
pub const KERN_PROC_UMASK: ::c_int =		39;	
pub const KERN_PROC_OSREL: ::c_int =		40;	
pub const KERN_PROC_SIGTRAMP: ::c_int =	41;	
pub const KERN_PROC_CWD: ::c_int =		42;	
pub const KERN_PROC_NFDS: ::c_int =		43;	
pub const KERN_PROC_SIGFASTBLK: ::c_int =	44;

pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_IEEE802: u16 = 6;
pub const ARPHRD_FRELAY: u16 = 15;
pub const ARPHRD_IEEE1394: u16 = 24;
pub const ARPHRD_INFINIBAND: u16 = 32;

pub const _NTO_SYNC_NONRECURSIVE: ::c_uint = 0x80000000;
pub const _NTO_SYNC_MUTEX_FREE: ::c_uint = 0x00000000;
pub const CLOCK_REALTIME: ::clockid_t = 0;

// socket options flag
pub const SO_DEBUG: ::c_int = 0x00000001;
pub const SO_ACCEPTCONN: ::c_int = 0x00000002;
pub const SO_REUSEADDR: ::c_int = 0x00000004;
pub const SO_KEEPALIVE: ::c_int = 0x00000008;
pub const SO_DONTROUTE: ::c_int = 0x00000010;
pub const SO_BROADCAST: ::c_int = 0x00000020;

pub const SO_LINGER:    ::c_int = 0x00000080;
pub const SO_OOBINLINE: ::c_int = 0x00000100;
pub const SO_REUSEPORT: ::c_int = 0x00000800;
pub const SO_TIMESTAMP: ::c_int = 0x00000800;
pub const SO_NOSIGPIPE: ::c_int = 0x00000800;
pub const SO_ACCEPTFILTER: ::c_int = 0x00001000;
pub const SO_BINTIME: ::c_int =	0x00002000;	
pub const SO_NO_OFFLOAD: ::c_int =	0x00004000;	
pub const SO_NO_DDP: ::c_int =	0x00008000;	
pub const SO_REUSEPORT_LB: ::c_int =	0x00010000;
pub const SO_RERROR: ::c_int =	0x00020000;

// Additional options
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;

// jail.h
pub const JAIL_API_VERSION: ::c_int = 2;

pub const JAIL_CREATE: ::c_int =	0x01;	
pub const JAIL_UPDATE: ::c_int =	0x02;	
pub const JAIL_ATTACH: ::c_int =	0x04;	
pub const JAIL_DYING: ::c_int  =	0x08;	
pub const JAIL_SET_MASK: ::c_int =	0x0f;
pub const JAIL_GET_MASK: ::c_int =	0x08;

pub const JAIL_SYS_DISABLE: ::c_int =	0;
pub const JAIL_SYS_NEW: ::c_int =		1;
pub const JAIL_SYS_INHERIT: ::c_int =	2;

pub const FIONBIO: ::c_int = -2147195266;
pub const FIOASYNC: ::c_int = -2147195267;
pub const FIOCLEX: ::c_int = 26113;
pub const FIOGETOWN: ::c_int = 1074030203;
pub const FIONCLEX: ::c_int = 26114;
pub const FIONREAD: ::c_int = 1074030207;
pub const FIOSETOWN: ::c_int = -2147195268;

pub const LOCAL_PEERCRED: ::c_int = 0x0001; 
pub const LOCAL_CREDS: ::c_int = 0x0002;
pub const LOCAL_CREDS_PERSISTENT: ::c_int = 0x0003;
pub const LOCAL_CONNWAIT: ::c_int = 0x0004;

// sys/states.h
pub const STATE_DEAD: ::c_uint = 0;
pub const STATE_RUNNING: ::c_uint = 1;
pub const STATE_READY: ::c_uint = 2;
pub const STATE_STOPPED: ::c_uint = 3;
pub const STATE_SEND: ::c_uint = 4;
pub const STATE_RECEIVE: ::c_uint = 5;
pub const STATE_REPLY: ::c_uint = 6;
pub const STATE_MQ_SEND: ::c_uint = 7;
pub const STATE_MQ_RECEIVE: ::c_uint = 8;
pub const STATE_WAITPAGE: ::c_uint = 9;
pub const STATE_SIGSUSPEND: ::c_uint = 10;
pub const STATE_SIGWAITINFO: ::c_uint = 11;
pub const STATE_NANOSLEEP: ::c_uint = 12;
pub const STATE_MUTEX: ::c_uint = 13;
pub const STATE_CONDVAR: ::c_uint = 14;
pub const STATE_JOIN: ::c_uint = 15;
pub const STATE_INTR: ::c_uint = 16;
pub const STATE_SEM: ::c_uint = 17;
pub const STATE_WAITCTX: ::c_uint = 18;
pub const STATE_RWLOCK_READ: ::c_uint = 19;
pub const STATE_RWLOCK_WRITE: ::c_uint = 20;
pub const STATE_BARRIER: ::c_uint = 21;
pub const STATE_TIMEOUT_MAX: ::c_uint = 24; 
pub const STATE_CREATE: ::c_uint = 24;
pub const STATE_DESTROY: ::c_uint = 25;
pub const STATE_MUON_MUTEX: ::c_uint = 26;
pub const STATE_TRACEBUFFER: ::c_uint = 27;
pub const STATE_INTR_ATTACH_EV: ::c_uint = 28;
pub const STATE_TIMER_DELEGATE: ::c_uint = 29;
pub const STATE_MAX: ::c_uint = 63;

pub const PTHREAD_BARRIER_SERIAL_THREAD: ::c_int = -1;
pub const PTHREAD_CREATE_JOINABLE: ::c_int = 0x00;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 0x01;

pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 3;
pub const PTHREAD_STACK_MIN: ::size_t = 256;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = 0;
pub const PTHREAD_MUTEX_STALLED: ::c_int = 0x00;
pub const PTHREAD_MUTEX_ROBUST: ::c_int = 0x10;
pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0x00;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 0x01;

pub const PTHREAD_KEYS_MAX: usize = 128;

pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __u: CLOCK_REALTIME as u32,
    __owner: 0xfffffffb,
};

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __u: _NTO_SYNC_NONRECURSIVE,
    __owner: _NTO_SYNC_MUTEX_FREE,
};

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __count: 0,
    __owner: 0,
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
  
    pub fn ptsname_r(fd: ::c_int, buf: *mut ::c_char, buflen: ::size_t) -> ::c_int;
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
    // Software Development Platform 8.0: Known Issues:  2935914
    //pub fn inotify_rm_watch(fd: ::c_int, wd: ::c_int) -> ::c_int;
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
        vlen: ::size_t,
        flags: ::c_int,
    ) -> ::ssize_t;
    pub fn recvmmsg(
        sockfd: ::c_int,
        msgvec: *mut ::mmsghdr,
        vlen: ::size_t,
        flags: ::c_int,
        timeout: *const ::timespec,
    ) -> ::ssize_t;

    pub fn mallopt(param: ::c_int, value: i64) -> ::c_int;
    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;

    pub fn ctermid(s: *mut ::c_char) -> *mut ::c_char;
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;

    pub fn mallinfo() -> ::mallinfo;
    pub fn getpwent_r(
        pwd: *mut ::passwd,
        buf: *mut ::c_char,
        __bufsize: ::size_t,
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
                info: *const dl_phdr_info,
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
    ) -> rcvid_t;
    pub fn MsgReceive_r(
        __chid: ::c_int,
        __msg: *mut ::c_void,
        __bytes: usize,
        __info: *mut _msg_info64,
    ) -> rcvid_t;
    pub fn MsgReceivev(
        __chid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __info: *mut _msg_info64,
    ) -> rcvid_t;
    pub fn MsgReceivev_r(
        __chid: ::c_int,
        __iov: *const ::iovec,
        __parts: usize,
        __info: *mut _msg_info64,
    ) -> rcvid_t;
    pub fn MsgReply(
        __rcvid: rcvid_t,
        __status: ::c_long,
        __msg: *const ::c_void,
        __bytes: usize,
    ) -> ::c_int;
    pub fn MsgReply_r(
        __rcvid: rcvid_t,
        __status: ::c_long,
        __msg: *const ::c_void,
        __bytes: usize,
    ) -> ::c_int;
    pub fn MsgReplyv(
        __rcvid: rcvid_t,
        __status: ::c_long,
        __iov: *const ::iovec,
        __parts: usize,
    ) -> ::c_int;
    pub fn MsgReplyv_r(
        __rcvid: rcvid_t,
        __status: ::c_long,
        __iov: *const ::iovec,
        __parts: usize,
    ) -> ::c_int;
    pub fn MsgRead(
        __rcvid: rcvid_t,
        __msg: *mut ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgRead_r(
        __rcvid: rcvid_t,
        __msg: *mut ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgReadv(
        __rcvid: rcvid_t,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgReadv_r(
        __rcvid: rcvid_t,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWrite(
        __rcvid: rcvid_t,
        __msg: *const ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWrite_r(
        __rcvid: rcvid_t,
        __msg: *const ::c_void,
        __bytes: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWritev(
        __rcvid: rcvid_t,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgWritev_r(
        __rcvid: rcvid_t,
        __iov: *const ::iovec,
        __parts: usize,
        __offset: usize,
    ) -> isize;
    pub fn MsgInfo(__rcvid: rcvid_t, __info: *mut _msg_info64) -> ::c_int;
    pub fn MsgInfo_r(__rcvid: rcvid_t, __info: *mut _msg_info64) -> ::c_int;
    pub fn MsgCurrent(__rcvid: rcvid_t) -> ::c_int;
    pub fn MsgCurrent_r(__rcvid: rcvid_t) -> ::c_int;
    pub fn MsgPause(__rcvid: rcvid_t, __cookie: ::c_uint) -> ::c_int;
    pub fn MsgPause_r(__rcvid: rcvid_t, __cookie: ::c_uint) -> ::c_int;
    pub fn MsgError(__rcvid: rcvid_t, __err: ::c_int) -> ::c_int;
    pub fn MsgError_r(__rcvid: rcvid_t, __err: ::c_int) -> ::c_int;
    pub fn MsgSendPulse(
        __coid: ::c_int,
        __priority: ::c_int,
        __code: ::c_int,
        __value: ::c_long,
    ) -> ::c_int;
    pub fn MsgSendPulse_r(
        __coid: ::c_int,
        __priority: ::c_int,
        __code: ::c_int,
        __value: ::c_long,
    ) -> ::c_int;
    pub fn MsgDeliverEvent(__rcvid: rcvid_t, __event: *const ::sigevent) -> ::c_int;
    pub fn MsgDeliverEvent_r(__rcvid: rcvid_t, __event: *const ::sigevent) -> ::c_int;
    pub fn MsgVerifyEvent(__rcvid: rcvid_t, __event: *const ::sigevent) -> ::c_int;
    pub fn MsgVerifyEvent_r(__rcvid: rcvid_t, __event: *const ::sigevent) -> ::c_int;
    pub fn ChannelCtl(
        __chid: ::c_int,
        __cmd: ::c_int,
        data: *mut ::c_void
    ) -> ::c_int;
    pub fn ChannelCtl_r(
        __chid: ::c_int,
        __cmd: ::c_int,
        data: *mut ::c_void
    ) -> ::c_int;
    pub fn TimerTimeout(
        __id: ::clockid_t,
        __flags: u32,
        __notify: *const ::sigevent,
        __ntime: *const u64,
        __otime: *mut u64,
    ) -> ::c_int;
    pub fn TimerTimeout_r(
        __id: ::clockid_t,
        __flags: u32,
        __notify: *const ::sigevent,
        __ntime: *const u64,
        __otime: *mut u64,
    ) -> ::c_int;
    pub fn TimerDelegate(
        __action: u32,
        owner_cpu: u32,
        delegate_cpu: u32,
    ) -> ::c_int;
    pub fn TimerDelegate_r(
        __action: u32,
        owner_cpu: u32,
        delegate_cpu: u32,
    ) -> ::c_int;

}