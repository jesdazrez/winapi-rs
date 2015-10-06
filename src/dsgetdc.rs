// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! This file contains structures, function prototypes, and definitions for the DsGetDcName API.
pub const DS_FORCE_REDISCOVERY: ::ULONG = 0x00000001;
pub const DS_DIRECTORY_SERVICE_REQUIRED: ::ULONG = 0x00000010;
pub const DS_DIRECTORY_SERVICE_PREFERRED: ::ULONG = 0x00000020;
pub const DS_GC_SERVER_REQUIRED: ::ULONG = 0x00000040;
pub const DS_PDC_REQUIRED: ::ULONG = 0x00000080;
pub const DS_BACKGROUND_ONLY: ::ULONG = 0x00000100;
pub const DS_IP_REQUIRED: ::ULONG = 0x00000200;
pub const DS_KDC_REQUIRED: ::ULONG = 0x00000400;
pub const DS_TIMESERV_REQUIRED: ::ULONG = 0x00000800;
pub const DS_WRITABLE_REQUIRED: ::ULONG = 0x00001000;
pub const DS_GOOD_TIMESERV_PREFERRED: ::ULONG = 0x00002000;
pub const DS_AVOID_SELF: ::ULONG = 0x00004000;
pub const DS_ONLY_LDAP_NEEDED: ::ULONG = 0x00008000;
pub const DS_IS_FLAT_NAME: ::ULONG = 0x00010000;
pub const DS_IS_DNS_NAME: ::ULONG = 0x00020000;
pub const DS_TRY_NEXTCLOSEST_SITE: ::ULONG = 0x00040000;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: ::ULONG = 0x00080000;
pub const DS_WEB_SERVICE_REQUIRED: ::ULONG = 0x00100000;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: ::ULONG = 0x00200000;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: ::ULONG = 0x00400000;
pub const DS_RETURN_DNS_NAME: ::ULONG = 0x40000000;
pub const DS_RETURN_FLAT_NAME: ::ULONG = 0x80000000;
pub const DSGETDC_VALID_FLAGS: ::ULONG = DS_FORCE_REDISCOVERY | DS_DIRECTORY_SERVICE_REQUIRED
    | DS_DIRECTORY_SERVICE_PREFERRED | DS_GC_SERVER_REQUIRED | DS_PDC_REQUIRED | DS_BACKGROUND_ONLY
    | DS_IP_REQUIRED | DS_KDC_REQUIRED | DS_TIMESERV_REQUIRED | DS_WRITABLE_REQUIRED
    | DS_GOOD_TIMESERV_PREFERRED | DS_AVOID_SELF | DS_ONLY_LDAP_NEEDED | DS_IS_FLAT_NAME
    | DS_IS_DNS_NAME | DS_TRY_NEXTCLOSEST_SITE | DS_DIRECTORY_SERVICE_6_REQUIRED
    | DS_DIRECTORY_SERVICE_8_REQUIRED | DS_DIRECTORY_SERVICE_9_REQUIRED | DS_WEB_SERVICE_REQUIRED
    | DS_RETURN_FLAT_NAME | DS_RETURN_DNS_NAME;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DOMAIN_CONTROLLER_INFOA {
    pub DomainControllerName: ::LPSTR,
    pub DomainControllerAddress: ::LPSTR,
    pub DomainControllerAddressType: ::ULONG,
    pub DomainGuid: ::GUID,
    pub DomainName: ::LPSTR,
    pub DnsForestName: ::LPSTR,
    pub Flags: ::ULONG,
    pub DcSiteName: ::LPSTR,
    pub ClientSiteName: ::LPSTR,
}
pub type PDOMAIN_CONTROLLER_INFOA = *mut DOMAIN_CONTROLLER_INFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DOMAIN_CONTROLLER_INFOW {
    pub DomainControllerName: ::LPWSTR,
    pub DomainControllerAddress: ::LPWSTR,
    pub DomainControllerAddressType: ::ULONG,
    pub DomainGuid: ::GUID,
    pub DomainName: ::LPWSTR,
    pub DnsForestName: ::LPWSTR,
    pub Flags: ::ULONG,
    pub DcSiteName: ::LPWSTR,
    pub ClientSiteName: ::LPWSTR,
}
pub type PDOMAIN_CONTROLLER_INFOW = *mut DOMAIN_CONTROLLER_INFOW;
pub const DS_INET_ADDRESS: ::ULONG = 1;
pub const DS_NETBIOS_ADDRESS: ::ULONG = 2;
pub const DS_PDC_FLAG: ::ULONG = 0x00000001;
pub const DS_GC_FLAG: ::ULONG = 0x00000004;
pub const DS_LDAP_FLAG: ::ULONG = 0x00000008;
pub const DS_DS_FLAG: ::ULONG = 0x00000010;
pub const DS_KDC_FLAG: ::ULONG = 0x00000020;
pub const DS_TIMESERV_FLAG: ::ULONG = 0x00000040;
pub const DS_CLOSEST_FLAG: ::ULONG = 0x00000080;
pub const DS_WRITABLE_FLAG: ::ULONG = 0x00000100;
pub const DS_GOOD_TIMESERV_FLAG: ::ULONG = 0x00000200;
pub const DS_NDNC_FLAG: ::ULONG = 0x00000400;
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: ::ULONG = 0x00000800;
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: ::ULONG = 0x00001000;
pub const DS_WS_FLAG: ::ULONG = 0x00002000;
pub const DS_DS_8_FLAG: ::ULONG = 0x00004000;
pub const DS_DS_9_FLAG: ::ULONG = 0x00008000;
pub const DS_PING_FLAGS: ::ULONG = 0x000FFFFF;
pub const DS_DNS_CONTROLLER_FLAG: ::ULONG = 0x20000000;
pub const DS_DNS_DOMAIN_FLAG: ::ULONG = 0x40000000;
pub const DS_DNS_FOREST_FLAG: ::ULONG = 0x80000000;
pub const DS_DOMAIN_IN_FOREST: ::ULONG = 0x0001;
pub const DS_DOMAIN_DIRECT_OUTBOUND: ::ULONG = 0x0002;
pub const DS_DOMAIN_TREE_ROOT: ::ULONG = 0x0004;
pub const DS_DOMAIN_PRIMARY: ::ULONG = 0x0008;
pub const DS_DOMAIN_NATIVE_MODE: ::ULONG = 0x0010;
pub const DS_DOMAIN_DIRECT_INBOUND: ::ULONG = 0x0020;
pub const DS_DOMAIN_VALID_FLAGS: ::ULONG = DS_DOMAIN_IN_FOREST | DS_DOMAIN_DIRECT_OUTBOUND
    | DS_DOMAIN_TREE_ROOT | DS_DOMAIN_PRIMARY | DS_DOMAIN_NATIVE_MODE | DS_DOMAIN_DIRECT_INBOUND;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DS_DOMAIN_TRUSTSW {
    pub NetbiosDomainName: ::LPWSTR,
    pub DnsDomainName: ::LPWSTR,
    pub Flags: ::ULONG,
    pub ParentIndex: ::ULONG,
    pub TrustType: ::ULONG,
    pub TrustAttributes: ::ULONG,
    pub DomainSid: ::PSID,
    pub DomainGuid: ::GUID,
}
pub type PDS_DOMAIN_TRUSTSW = *mut DS_DOMAIN_TRUSTSW;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DS_DOMAIN_TRUSTSA {
    pub NetbiosDomainName: ::LPSTR,
    pub DnsDomainName: ::LPSTR,
    pub Flags: ::ULONG,
    pub ParentIndex: ::ULONG,
    pub TrustType: ::ULONG,
    pub TrustAttributes: ::ULONG,
    pub DomainSid: ::PSID,
    pub DomainGuid: ::GUID,
}
pub type PDS_DOMAIN_TRUSTSA = *mut DS_DOMAIN_TRUSTSA;
pub const DS_ONLY_DO_SITE_NAME: ::ULONG = 0x01;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: ::ULONG = 0x02;
pub const DS_OPEN_VALID_OPTION_FLAGS: ::ULONG = DS_ONLY_DO_SITE_NAME
    | DS_NOTIFY_AFTER_SITE_RECORDS;
pub const DS_OPEN_VALID_FLAGS: ::ULONG = DS_FORCE_REDISCOVERY | DS_ONLY_LDAP_NEEDED
    | DS_KDC_REQUIRED | DS_PDC_REQUIRED | DS_GC_SERVER_REQUIRED | DS_WRITABLE_REQUIRED;
