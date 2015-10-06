// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Windows NT Setup and Device Installer services
pub const LINE_LEN: usize = 256;
pub const MAX_INF_STRING_LENGTH: usize = 4096;
pub const MAX_INF_SECTION_NAME_LENGTH: usize = 255;
pub const MAX_TITLE_LEN: usize = 60;
pub const MAX_INSTRUCTION_LEN: usize = 256;
pub const MAX_LABEL_LEN: usize = 30;
pub const MAX_SERVICE_NAME_LEN: usize = 256;
pub const MAX_SUBTITLE_LEN: usize = 256;
pub const SP_MAX_MACHINENAME_LENGTH: usize = ::MAX_PATH + 3;
pub type HINF = ::PVOID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INFCONTEXT {
    pub Inf: ::PVOID,
    pub CurrentInf: ::PVOID,
    pub Section: ::UINT,
    pub Line: ::UINT,
}
pub type PINFCONTEXT = *mut INFCONTEXT;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_INF_INFORMATION {
    pub InfStyle: ::DWORD,
    pub InfCount: ::DWORD,
    pub VersionData: [::BYTE; ::ANYSIZE_ARRAY],
}
pub type PSP_INF_INFORMATION = *mut SP_INF_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: ::DWORD,
    pub Platform: ::DWORD,
    pub MajorVersion: ::DWORD,
    pub MinorVersion: ::DWORD,
    pub ProcessorArchitecture: ::WORD,
    pub Reserved: ::WORD,
    pub FirstValidatedMajorVersion: ::DWORD,
    pub FirstValidatedMinorVersion: ::DWORD,
}
UNION!(SP_ALTPLATFORM_INFO_V2, Reserved, Flags, Flags_mut, ::WORD);
pub type PSP_ALTPLATFORM_INFO_V2 = *mut SP_ALTPLATFORM_INFO_V2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: ::DWORD,
    pub Platform: ::DWORD,
    pub MajorVersion: ::DWORD,
    pub MinorVersion: ::DWORD,
    pub ProcessorArchitecture: ::WORD,
    pub Reserved: ::WORD,
}
pub type PSP_ALTPLATFORM_INFO_V1 = *mut SP_ALTPLATFORM_INFO_V1;
pub type SP_ALTPLATFORM_INFO = SP_ALTPLATFORM_INFO_V2;
pub type PSP_ALTPLATFORM_INFO = PSP_ALTPLATFORM_INFO_V2;
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: ::WORD = 0x0001;
#[repr(C)] #[derive(Copy)]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: ::DWORD,
    pub OriginalInfName: [::CHAR; ::MAX_PATH],
    pub OriginalCatalogName: [::CHAR; ::MAX_PATH],
}
impl Clone for SP_ORIGINAL_FILE_INFO_A { fn clone(&self) -> SP_ORIGINAL_FILE_INFO_A { *self } }
pub type PSP_ORIGINAL_FILE_INFO_A = *mut SP_ORIGINAL_FILE_INFO_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: ::DWORD,
    pub OriginalInfName: [::WCHAR; ::MAX_PATH],
    pub OriginalCatalogName: [::WCHAR; ::MAX_PATH],
}
impl Clone for SP_ORIGINAL_FILE_INFO_W { fn clone(&self) -> SP_ORIGINAL_FILE_INFO_W { *self } }
pub type PSP_ORIGINAL_FILE_INFO_W = *mut SP_ORIGINAL_FILE_INFO_W;
pub const INF_STYLE_NONE: ::DWORD = 0x00000000;
pub const INF_STYLE_OLDNT: ::DWORD = 0x00000001;
pub const INF_STYLE_WIN4: ::DWORD = 0x00000002;
pub const INF_STYLE_CACHE_ENABLE: ::DWORD = 0x00000010;
pub const INF_STYLE_CACHE_DISABLE: ::DWORD = 0x00000020;
pub const INF_STYLE_CACHE_IGNORE: ::DWORD = 0x00000040;
pub const DIRID_ABSOLUTE: ::DWORD = 0 - 1;
pub const DIRID_ABSOLUTE_16BIT: ::DWORD = 0xffff;
pub const DIRID_NULL: ::DWORD = 0;
pub const DIRID_SRCPATH: ::DWORD = 1;
pub const DIRID_WINDOWS: ::DWORD = 10;
pub const DIRID_SYSTEM: ::DWORD = 11;
pub const DIRID_DRIVERS: ::DWORD = 12;
pub const DIRID_IOSUBSYS: ::DWORD = DIRID_DRIVERS;
pub const DIRID_DRIVER_STORE: ::DWORD = 13;
pub const DIRID_INF: ::DWORD = 17;
pub const DIRID_HELP: ::DWORD = 18;
pub const DIRID_FONTS: ::DWORD = 20;
pub const DIRID_VIEWERS: ::DWORD = 21;
pub const DIRID_COLOR: ::DWORD = 23;
pub const DIRID_APPS: ::DWORD = 24;
pub const DIRID_SHARED: ::DWORD = 25;
pub const DIRID_BOOT: ::DWORD = 30;
pub const DIRID_SYSTEM16: ::DWORD = 50;
pub const DIRID_SPOOL: ::DWORD = 51;
pub const DIRID_SPOOLDRIVERS: ::DWORD = 52;
pub const DIRID_USERPROFILE: ::DWORD = 53;
pub const DIRID_LOADER: ::DWORD = 54;
pub const DIRID_PRINTPROCESSOR: ::DWORD = 55;
pub const DIRID_DEFAULT: ::DWORD = DIRID_SYSTEM;
pub const DIRID_COMMON_STARTMENU: ::DWORD = 16406;
pub const DIRID_COMMON_PROGRAMS: ::DWORD = 16407;
pub const DIRID_COMMON_STARTUP: ::DWORD = 16408;
pub const DIRID_COMMON_DESKTOPDIRECTORY: ::DWORD = 16409;
pub const DIRID_COMMON_FAVORITES: ::DWORD = 16415;
pub const DIRID_COMMON_APPDATA: ::DWORD = 16419;
pub const DIRID_PROGRAM_FILES: ::DWORD = 16422;
pub const DIRID_SYSTEM_X86: ::DWORD = 16425;
pub const DIRID_PROGRAM_FILES_X86: ::DWORD = 16426;
pub const DIRID_PROGRAM_FILES_COMMON: ::DWORD = 16427;
pub const DIRID_PROGRAM_FILES_COMMONX86: ::DWORD = 16428;
pub const DIRID_COMMON_TEMPLATES: ::DWORD = 16429;
pub const DIRID_COMMON_DOCUMENTS: ::DWORD = 16430;
pub const DIRID_USER: ::DWORD = 0x8000;
pub type PSP_FILE_CALLBACK_A = Option<unsafe extern "system" fn(
    Context: ::PVOID, Notification: ::UINT, Param1: ::UINT_PTR, Param2: ::UINT_PTR,
) -> ::UINT>;
pub type PSP_FILE_CALLBACK_W = Option<unsafe extern "system" fn(
    Context: ::PVOID, Notification: ::UINT, Param1: ::UINT_PTR, Param2: ::UINT_PTR,
) -> ::UINT>;
pub const SPFILENOTIFY_STARTQUEUE: ::UINT = 0x00000001;
pub const SPFILENOTIFY_ENDQUEUE: ::UINT = 0x00000002;
pub const SPFILENOTIFY_STARTSUBQUEUE: ::UINT = 0x00000003;
pub const SPFILENOTIFY_ENDSUBQUEUE: ::UINT = 0x00000004;
pub const SPFILENOTIFY_STARTDELETE: ::UINT = 0x00000005;
pub const SPFILENOTIFY_ENDDELETE: ::UINT = 0x00000006;
pub const SPFILENOTIFY_DELETEERROR: ::UINT = 0x00000007;
pub const SPFILENOTIFY_STARTRENAME: ::UINT = 0x00000008;
pub const SPFILENOTIFY_ENDRENAME: ::UINT = 0x00000009;
pub const SPFILENOTIFY_RENAMEERROR: ::UINT = 0x0000000a;
pub const SPFILENOTIFY_STARTCOPY: ::UINT = 0x0000000b;
pub const SPFILENOTIFY_ENDCOPY: ::UINT = 0x0000000c;
pub const SPFILENOTIFY_COPYERROR: ::UINT = 0x0000000d;
pub const SPFILENOTIFY_NEEDMEDIA: ::UINT = 0x0000000e;
pub const SPFILENOTIFY_QUEUESCAN: ::UINT = 0x0000000f;
pub const SPFILENOTIFY_CABINETINFO: ::UINT = 0x00000010;
pub const SPFILENOTIFY_FILEINCABINET: ::UINT = 0x00000011;
pub const SPFILENOTIFY_NEEDNEWCABINET: ::UINT = 0x00000012;
pub const SPFILENOTIFY_FILEEXTRACTED: ::UINT = 0x00000013;
pub const SPFILENOTIFY_FILEOPDELAYED: ::UINT = 0x00000014;
pub const SPFILENOTIFY_STARTBACKUP: ::UINT = 0x00000015;
pub const SPFILENOTIFY_BACKUPERROR: ::UINT = 0x00000016;
pub const SPFILENOTIFY_ENDBACKUP: ::UINT = 0x00000017;
pub const SPFILENOTIFY_QUEUESCAN_EX: ::UINT = 0x00000018;
pub const SPFILENOTIFY_STARTREGISTRATION: ::UINT = 0x00000019;
pub const SPFILENOTIFY_ENDREGISTRATION: ::UINT = 0x00000020;
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: ::UINT = 0x00000040;
pub const SPFILENOTIFY_LANGMISMATCH: ::UINT = 0x00010000;
pub const SPFILENOTIFY_TARGETEXISTS: ::UINT = 0x00020000;
pub const SPFILENOTIFY_TARGETNEWER: ::UINT = 0x00040000;
pub const FILEOP_COPY: ::UINT = 0;
pub const FILEOP_RENAME: ::UINT = 1;
pub const FILEOP_DELETE: ::UINT = 2;
pub const FILEOP_BACKUP: ::UINT = 3;
pub const FILEOP_ABORT: ::UINT = 0;
pub const FILEOP_DOIT: ::UINT = 1;
pub const FILEOP_SKIP: ::UINT = 2;
pub const FILEOP_RETRY: ::UINT = FILEOP_DOIT;
pub const FILEOP_NEWPATH: ::UINT = 4;
pub const COPYFLG_WARN_IF_SKIP: ::UINT = 0x00000001;
pub const COPYFLG_NOSKIP: ::UINT = 0x00000002;
pub const COPYFLG_NOVERSIONCHECK: ::UINT = 0x00000004;
pub const COPYFLG_FORCE_FILE_IN_USE: ::UINT = 0x00000008;
pub const COPYFLG_NO_OVERWRITE: ::UINT = 0x00000010;
pub const COPYFLG_NO_VERSION_DIALOG: ::UINT = 0x00000020;
pub const COPYFLG_OVERWRITE_OLDER_ONLY: ::UINT = 0x00000040;
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: ::UINT = 0x00000100;
pub const COPYFLG_REPLACEONLY: ::UINT = 0x00000400;
pub const COPYFLG_NODECOMP: ::UINT = 0x00000800;
pub const COPYFLG_REPLACE_BOOT_FILE: ::UINT = 0x00001000;
pub const COPYFLG_NOPRUNE: ::UINT = 0x00002000;
pub const COPYFLG_IN_USE_TRY_RENAME: ::UINT = 0x00004000;
pub const DELFLG_IN_USE: ::UINT = 0x00000001;
pub const DELFLG_IN_USE1: ::UINT = 0x00010000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILEPATHS_A {
    pub Target: ::PCSTR,
    pub Source: ::PCSTR,
    pub Win32Error: ::UINT,
    pub Flags: ::DWORD,
}
pub type PFILEPATHS_A = *mut FILEPATHS_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILEPATHS_W {
    pub Target: ::PCWSTR,
    pub Source: ::PCWSTR,
    pub Win32Error: ::UINT,
    pub Flags: ::DWORD,
}
pub type PFILEPATHS_W = *mut FILEPATHS_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: ::PCSTR,
    pub Source: ::PCSTR,
    pub Win32Error: ::UINT,
    pub Flags: ::DWORD,
    pub DigitalSigner: ::PCSTR,
    pub Version: ::PCSTR,
    pub CatalogFile: ::PCSTR
}
pub type PFILEPATHS_SIGNERINFO_A = *mut FILEPATHS_SIGNERINFO_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: ::PCWSTR,
    pub Source: ::PCWSTR,
    pub Win32Error: ::UINT,
    pub Flags: ::DWORD,
    pub DigitalSigner: ::PCWSTR,
    pub Version: ::PCWSTR,
    pub CatalogFile: ::PCWSTR
}
pub type PFILEPATHS_SIGNERINFO_W = *mut FILEPATHS_SIGNERINFO_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: ::PCSTR,
    pub Tagfile: ::PCSTR,
    pub Description: ::PCSTR,
    pub SourcePath: ::PCSTR,
    pub SourceFile: ::PCSTR,
    pub Flags: ::DWORD,
}
pub type PSOURCE_MEDIA_A = *mut SOURCE_MEDIA_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: ::PCWSTR,
    pub Tagfile: ::PCWSTR,
    pub Description: ::PCWSTR,
    pub SourcePath: ::PCWSTR,
    pub SourceFile: ::PCWSTR,
    pub Flags: ::DWORD,
}
pub type PSOURCE_MEDIA_W = *mut SOURCE_MEDIA_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CABINET_INFO_A {
    pub CabinetPath: ::PCSTR,
    pub CabinetFile: ::PCSTR,
    pub DiskName: ::PCSTR,
    pub SetId: ::USHORT,
    pub CabinetNumber: ::USHORT,
}
pub type PCABINET_INFO_A = *mut CABINET_INFO_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CABINET_INFO_W {
    pub CabinetPath: ::PCWSTR,
    pub CabinetFile: ::PCWSTR,
    pub DiskName: ::PCWSTR,
    pub SetId: ::USHORT,
    pub CabinetNumber: ::USHORT,
}
pub type PCABINET_INFO_W = *mut CABINET_INFO_W;
#[repr(C)] #[derive(Copy)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: ::PCSTR,
    pub FileSize: ::DWORD,
    pub Win32Error: ::DWORD,
    pub DosDate: ::WORD,
    pub DosTime: ::WORD,
    pub DosAttribs: ::WORD,
    pub FullTargetName: [::CHAR; ::MAX_PATH],
}
impl Clone for FILE_IN_CABINET_INFO_A { fn clone(&self) -> FILE_IN_CABINET_INFO_A { *self } }
pub type PFILE_IN_CABINET_INFO_A = *mut FILE_IN_CABINET_INFO_A;
#[repr(C)] #[derive(Copy)]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: ::PCWSTR,
    pub FileSize: ::DWORD,
    pub Win32Error: ::DWORD,
    pub DosDate: ::WORD,
    pub DosTime: ::WORD,
    pub DosAttribs: ::WORD,
    pub FullTargetName: [::WCHAR; ::MAX_PATH],
}
impl Clone for FILE_IN_CABINET_INFO_W { fn clone(&self) -> FILE_IN_CABINET_INFO_W { *self } }
pub type PFILE_IN_CABINET_INFO_W = *mut FILE_IN_CABINET_INFO_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: ::DWORD,
    pub FileName: ::PCSTR,
    pub Win32Error: ::DWORD,
    pub FailureCode: ::DWORD,
}
pub type PSP_REGISTER_CONTROL_STATUSA = *mut SP_REGISTER_CONTROL_STATUSA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: ::DWORD,
    pub FileName: ::PCWSTR,
    pub Win32Error: ::DWORD,
    pub FailureCode: ::DWORD,
}
pub type PSP_REGISTER_CONTROL_STATUSW = *mut SP_REGISTER_CONTROL_STATUSW;
pub const SPREG_SUCCESS: ::DWORD = 0x00000000;
pub const SPREG_LOADLIBRARY: ::DWORD = 0x00000001;
pub const SPREG_GETPROCADDR: ::DWORD = 0x00000002;
pub const SPREG_REGSVR: ::DWORD = 0x00000003;
pub const SPREG_DLLINSTALL: ::DWORD = 0x00000004;
pub const SPREG_TIMEOUT: ::DWORD = 0x00000005;
pub const SPREG_UNKNOWN: ::DWORD = 0xFFFFFFFF;
pub type HSPFILEQ = ::PVOID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: ::DWORD,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: ::PCSTR,
    pub SourcePath: ::PCSTR,
    pub SourceFilename: ::PCSTR,
    pub SourceDescription: ::PCSTR,
    pub SourceTagfile: ::PCSTR,
    pub TargetDirectory: ::PCSTR,
    pub TargetFilename: ::PCSTR,
    pub CopyStyle: ::DWORD,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: ::PCSTR,
}
pub type PSP_FILE_COPY_PARAMS_A = *mut SP_FILE_COPY_PARAMS_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: ::DWORD,
    pub QueueHandle: HSPFILEQ,
    pub SourceRootPath: ::PCWSTR,
    pub SourcePath: ::PCWSTR,
    pub SourceFilename: ::PCWSTR,
    pub SourceDescription: ::PCWSTR,
    pub SourceTagfile: ::PCWSTR,
    pub TargetDirectory: ::PCWSTR,
    pub TargetFilename: ::PCWSTR,
    pub CopyStyle: ::DWORD,
    pub LayoutInf: HINF,
    pub SecurityDescriptor: ::PCWSTR,
}
pub type PSP_FILE_COPY_PARAMS_W = *mut SP_FILE_COPY_PARAMS_W;
pub type HDSKSPC = ::PVOID;
pub type HDEVINFO = ::PVOID;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: ::DWORD,
    pub ClassGuid: ::GUID,
    pub DevInst: ::DWORD,
    pub Reserved: ::ULONG_PTR,
}
pub type PSP_DEVINFO_DATA = *mut SP_DEVINFO_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: ::DWORD,
    pub InterfaceClassGuid: ::GUID,
    pub Flags: ::DWORD,
    pub Reserved: ::ULONG_PTR,
}
pub type PSP_DEVICE_INTERFACE_DATA = *mut SP_DEVICE_INTERFACE_DATA;
pub const SPINT_ACTIVE: ::DWORD = 0x00000001;
pub const SPINT_DEFAULT: ::DWORD = 0x00000002;
pub const SPINT_REMOVED: ::DWORD = 0x00000004;
pub type SP_INTERFACE_DEVICE_DATA = SP_DEVICE_INTERFACE_DATA;
pub type PSP_INTERFACE_DEVICE_DATA = PSP_DEVICE_INTERFACE_DATA;
pub const SPID_ACTIVE: ::DWORD = SPINT_ACTIVE;
pub const SPID_DEFAULT: ::DWORD = SPINT_DEFAULT;
pub const SPID_REMOVED: ::DWORD = SPINT_REMOVED;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: ::DWORD,
    pub DevicePath: [::CHAR; ::ANYSIZE_ARRAY],
}
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA_A = *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: ::DWORD,
    pub DevicePath: [::WCHAR; ::ANYSIZE_ARRAY],
}
pub type PSP_DEVICE_INTERFACE_DETAIL_DATA_W = *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: ::DWORD,
    pub ClassGuid: ::GUID,
    pub RemoteMachineHandle: ::HANDLE,
    pub RemoteMachineName: [::CHAR; SP_MAX_MACHINENAME_LENGTH],
}
impl Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> SP_DEVINFO_LIST_DETAIL_DATA_A { *self }
}
pub type PSP_DEVINFO_LIST_DETAIL_DATA_A = *mut SP_DEVINFO_LIST_DETAIL_DATA_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: ::DWORD,
    pub ClassGuid: ::GUID,
    pub RemoteMachineHandle: ::HANDLE,
    pub RemoteMachineName: [::WCHAR; SP_MAX_MACHINENAME_LENGTH],
}
impl Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> SP_DEVINFO_LIST_DETAIL_DATA_W { *self }
}
pub type PSP_DEVINFO_LIST_DETAIL_DATA_W = *mut SP_DEVINFO_LIST_DETAIL_DATA_W;
pub const DIF_SELECTDEVICE: DI_FUNCTION = 0x00000001;
pub const DIF_INSTALLDEVICE: DI_FUNCTION = 0x00000002;
pub const DIF_ASSIGNRESOURCES: DI_FUNCTION = 0x00000003;
pub const DIF_PROPERTIES: DI_FUNCTION = 0x00000004;
pub const DIF_REMOVE: DI_FUNCTION = 0x00000005;
pub const DIF_FIRSTTIMESETUP: DI_FUNCTION = 0x00000006;
pub const DIF_FOUNDDEVICE: DI_FUNCTION = 0x00000007;
pub const DIF_SELECTCLASSDRIVERS: DI_FUNCTION = 0x00000008;
pub const DIF_VALIDATECLASSDRIVERS: DI_FUNCTION = 0x00000009;
pub const DIF_INSTALLCLASSDRIVERS: DI_FUNCTION = 0x0000000A;
pub const DIF_CALCDISKSPACE: DI_FUNCTION = 0x0000000B;
pub const DIF_DESTROYPRIVATEDATA: DI_FUNCTION = 0x0000000C;
pub const DIF_VALIDATEDRIVER: DI_FUNCTION = 0x0000000D;
pub const DIF_DETECT: DI_FUNCTION = 0x0000000F;
pub const DIF_INSTALLWIZARD: DI_FUNCTION = 0x00000010;
pub const DIF_DESTROYWIZARDDATA: DI_FUNCTION = 0x00000011;
pub const DIF_PROPERTYCHANGE: DI_FUNCTION = 0x00000012;
pub const DIF_ENABLECLASS: DI_FUNCTION = 0x00000013;
pub const DIF_DETECTVERIFY: DI_FUNCTION = 0x00000014;
pub const DIF_INSTALLDEVICEFILES: DI_FUNCTION = 0x00000015;
pub const DIF_UNREMOVE: DI_FUNCTION = 0x00000016;
pub const DIF_SELECTBESTCOMPATDRV: DI_FUNCTION = 0x00000017;
pub const DIF_ALLOW_INSTALL: DI_FUNCTION = 0x00000018;
pub const DIF_REGISTERDEVICE: DI_FUNCTION = 0x00000019;
pub const DIF_NEWDEVICEWIZARD_PRESELECT: DI_FUNCTION = 0x0000001A;
pub const DIF_NEWDEVICEWIZARD_SELECT: DI_FUNCTION = 0x0000001B;
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: DI_FUNCTION = 0x0000001C;
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: DI_FUNCTION = 0x0000001D;
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: DI_FUNCTION = 0x0000001E;
pub const DIF_UNUSED1: DI_FUNCTION = 0x0000001F;
pub const DIF_INSTALLINTERFACES: DI_FUNCTION = 0x00000020;
pub const DIF_DETECTCANCEL: DI_FUNCTION = 0x00000021;
pub const DIF_REGISTER_COINSTALLERS: DI_FUNCTION = 0x00000022;
pub const DIF_ADDPROPERTYPAGE_ADVANCED: DI_FUNCTION = 0x00000023;
pub const DIF_ADDPROPERTYPAGE_BASIC: DI_FUNCTION = 0x00000024;
pub const DIF_RESERVED1: DI_FUNCTION = 0x00000025;
pub const DIF_TROUBLESHOOTER: DI_FUNCTION = 0x00000026;
pub const DIF_POWERMESSAGEWAKE: DI_FUNCTION = 0x00000027;
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: DI_FUNCTION = 0x00000028;
pub const DIF_UPDATEDRIVER_UI: DI_FUNCTION = 0x00000029;
pub const DIF_FINISHINSTALL_ACTION: DI_FUNCTION = 0x0000002A;
pub const DIF_RESERVED2: DI_FUNCTION = 0x00000030;
pub const DIF_MOVEDEVICE: DI_FUNCTION = 0x0000000E;
pub type DI_FUNCTION = ::UINT;
#[repr(C)] #[derive(Copy)]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: ::DWORD,
    pub Flags: ::DWORD,
    pub FlagsEx: ::DWORD,
    pub hwndParent: ::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: ::PVOID,
    pub FileQueue: HSPFILEQ,
    pub ClassInstallReserved: ::ULONG_PTR,
    pub Reserved: ::DWORD,
    pub DriverPath: [::CHAR; ::MAX_PATH],
}
impl Clone for SP_DEVINSTALL_PARAMS_A { fn clone(&self) -> SP_DEVINSTALL_PARAMS_A { *self } }
pub type PSP_DEVINSTALL_PARAMS_A = *mut SP_DEVINSTALL_PARAMS_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: ::DWORD,
    pub Flags: ::DWORD,
    pub FlagsEx: ::DWORD,
    pub hwndParent: ::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_W,
    pub InstallMsgHandlerContext: ::PVOID,
    pub FileQueue: HSPFILEQ,
    pub ClassInstallReserved: ::ULONG_PTR,
    pub Reserved: ::DWORD,
    pub DriverPath: [::WCHAR; ::MAX_PATH],
}
impl Clone for SP_DEVINSTALL_PARAMS_W { fn clone(&self) -> SP_DEVINSTALL_PARAMS_W { *self } }
pub type PSP_DEVINSTALL_PARAMS_W = *mut SP_DEVINSTALL_PARAMS_W;
pub const DI_SHOWOEM: ::DWORD = 0x00000001;
pub const DI_SHOWCOMPAT: ::DWORD = 0x00000002;
pub const DI_SHOWCLASS: ::DWORD = 0x00000004;
pub const DI_SHOWALL: ::DWORD = 0x00000007;
pub const DI_NOVCP: ::DWORD = 0x00000008;
pub const DI_DIDCOMPAT: ::DWORD = 0x00000010;
pub const DI_DIDCLASS: ::DWORD = 0x00000020;
pub const DI_AUTOASSIGNRES: ::DWORD = 0x00000040;
pub const DI_NEEDRESTART: ::DWORD = 0x00000080;
pub const DI_NEEDREBOOT: ::DWORD = 0x00000100;
pub const DI_NOBROWSE: ::DWORD = 0x00000200;
pub const DI_MULTMFGS: ::DWORD = 0x00000400;
pub const DI_DISABLED: ::DWORD = 0x00000800;
pub const DI_GENERALPAGE_ADDED: ::DWORD = 0x00001000;
pub const DI_RESOURCEPAGE_ADDED: ::DWORD = 0x00002000;
pub const DI_PROPERTIES_CHANGE: ::DWORD = 0x00004000;
pub const DI_INF_IS_SORTED: ::DWORD = 0x00008000;
pub const DI_ENUMSINGLEINF: ::DWORD = 0x00010000;
pub const DI_DONOTCALLCONFIGMG: ::DWORD = 0x00020000;
pub const DI_INSTALLDISABLED: ::DWORD = 0x00040000;
pub const DI_COMPAT_FROM_CLASS: ::DWORD = 0x00080000;
pub const DI_CLASSINSTALLPARAMS: ::DWORD = 0x00100000;
pub const DI_NODI_DEFAULTACTION: ::DWORD = 0x00200000;
pub const DI_QUIETINSTALL: ::DWORD = 0x00800000;
pub const DI_NOFILECOPY: ::DWORD = 0x01000000;
pub const DI_FORCECOPY: ::DWORD = 0x02000000;
pub const DI_DRIVERPAGE_ADDED: ::DWORD = 0x04000000;
pub const DI_USECI_SELECTSTRINGS: ::DWORD = 0x08000000;
pub const DI_OVERRIDE_INFFLAGS: ::DWORD = 0x10000000;
pub const DI_PROPS_NOCHANGEUSAGE: ::DWORD = 0x20000000;
pub const DI_NOSELECTICONS: ::DWORD = 0x40000000;
pub const DI_NOWRITE_IDS: ::DWORD = 0x80000000;
pub const DI_FLAGSEX_RESERVED2: ::DWORD = 0x00000001;
pub const DI_FLAGSEX_RESERVED3: ::DWORD = 0x00000002;
pub const DI_FLAGSEX_CI_FAILED: ::DWORD = 0x00000004;
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: ::DWORD = 0x00000008;
pub const DI_FLAGSEX_DIDINFOLIST: ::DWORD = 0x00000010;
pub const DI_FLAGSEX_DIDCOMPATINFO: ::DWORD = 0x00000020;
pub const DI_FLAGSEX_FILTERCLASSES: ::DWORD = 0x00000040;
pub const DI_FLAGSEX_SETFAILEDINSTALL: ::DWORD = 0x00000080;
pub const DI_FLAGSEX_DEVICECHANGE: ::DWORD = 0x00000100;
pub const DI_FLAGSEX_ALWAYSWRITEIDS: ::DWORD = 0x00000200;
pub const DI_FLAGSEX_PROPCHANGE_PENDING: ::DWORD = 0x00000400;
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: ::DWORD = 0x00000800;
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: ::DWORD = 0x00001000;
pub const DI_FLAGSEX_USECLASSFORCOMPAT: ::DWORD = 0x00002000;
pub const DI_FLAGSEX_RESERVED4: ::DWORD = 0x00004000;
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: ::DWORD = 0x00008000;
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: ::DWORD = 0x00010000;
pub const DI_FLAGSEX_INET_DRIVER: ::DWORD = 0x00020000;
pub const DI_FLAGSEX_APPENDDRIVERLIST: ::DWORD = 0x00040000;
pub const DI_FLAGSEX_PREINSTALLBACKUP: ::DWORD = 0x00080000;
pub const DI_FLAGSEX_BACKUPONREPLACE: ::DWORD = 0x00100000;
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: ::DWORD = 0x00200000;
pub const DI_FLAGSEX_RESERVED1: ::DWORD = 0x00400000;
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: ::DWORD = 0x00800000;
pub const DI_FLAGSEX_POWERPAGE_ADDED: ::DWORD = 0x01000000;
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: ::DWORD = 0x02000000;
pub const DI_FLAGSEX_INSTALLEDDRIVER: ::DWORD = 0x04000000;
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: ::DWORD = 0x08000000;
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: ::DWORD = 0x10000000;
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: ::DWORD = 0x20000000;
pub const DI_FLAGSEX_RECURSIVESEARCH: ::DWORD = 0x40000000;
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: ::DWORD = 0x80000000;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: ::DWORD,
    pub InstallFunction: DI_FUNCTION,
}
pub type PSP_CLASSINSTALL_HEADER = *mut SP_CLASSINSTALL_HEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::GUID,
    pub EnableMessage: ::DWORD,
}
pub type PSP_ENABLECLASS_PARAMS = *mut SP_ENABLECLASS_PARAMS;
pub const ENABLECLASS_QUERY: ::DWORD = 0;
pub const ENABLECLASS_SUCCESS: ::DWORD = 1;
pub const ENABLECLASS_FAILURE: ::DWORD = 2;
pub const DICS_ENABLE: ::DWORD = 0x00000001;
pub const DICS_DISABLE: ::DWORD = 0x00000002;
pub const DICS_PROPCHANGE: ::DWORD = 0x00000003;
pub const DICS_START: ::DWORD = 0x00000004;
pub const DICS_STOP: ::DWORD = 0x00000005;
pub const DICS_FLAG_GLOBAL: ::DWORD = 0x00000001;
pub const DICS_FLAG_CONFIGSPECIFIC: ::DWORD = 0x00000002;
pub const DICS_FLAG_CONFIGGENERAL: ::DWORD = 0x00000004;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: ::DWORD,
    pub Scope: ::DWORD,
    pub HwProfile: ::DWORD,
}
pub type PSP_PROPCHANGE_PARAMS = *mut SP_PROPCHANGE_PARAMS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: ::DWORD,
    pub HwProfile: ::DWORD,
}
pub type PSP_REMOVEDEVICE_PARAMS = *mut SP_REMOVEDEVICE_PARAMS;
pub const DI_REMOVEDEVICE_GLOBAL: ::DWORD = 0x00000001;
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: ::DWORD = 0x00000002;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: ::DWORD,
    pub HwProfile: ::DWORD,
}
pub type PSP_UNREMOVEDEVICE_PARAMS = *mut SP_UNREMOVEDEVICE_PARAMS;
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: ::DWORD = 0x00000002;
#[repr(C)] #[derive(Copy)]
pub struct SP_SELECTDEVICE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [::CHAR; MAX_TITLE_LEN],
    pub Instructions: [::CHAR; MAX_INSTRUCTION_LEN],
    pub ListLabel: [::CHAR; MAX_LABEL_LEN],
    pub SubTitle: [::CHAR; MAX_SUBTITLE_LEN],
    pub Reserved: [::BYTE; 2],
}
impl Clone for SP_SELECTDEVICE_PARAMS_A { fn clone(&self) -> SP_SELECTDEVICE_PARAMS_A { *self } }
pub type PSP_SELECTDEVICE_PARAMS_A = *mut SP_SELECTDEVICE_PARAMS_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [::WCHAR; MAX_TITLE_LEN],
    pub Instructions: [::WCHAR; MAX_INSTRUCTION_LEN],
    pub ListLabel: [::WCHAR; MAX_LABEL_LEN],
    pub SubTitle: [::WCHAR; MAX_SUBTITLE_LEN],
}
impl Clone for SP_SELECTDEVICE_PARAMS_W { fn clone(&self) -> SP_SELECTDEVICE_PARAMS_W { *self } }
pub type PSP_SELECTDEVICE_PARAMS_W = *mut SP_SELECTDEVICE_PARAMS_W;
pub type PDETECT_PROGRESS_NOTIFY = Option<unsafe extern "system" fn(
    ProgressNotifyParam: ::PVOID, DetectComplete: ::DWORD,
) -> ::BOOL>;
#[repr(C)] #[derive(Copy)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: ::PVOID,
}
impl Clone for SP_DETECTDEVICE_PARAMS { fn clone(&self) -> SP_DETECTDEVICE_PARAMS { *self } }
pub type PSP_DETECTDEVICE_PARAMS = *mut SP_DETECTDEVICE_PARAMS;
pub const MAX_INSTALLWIZARD_DYNAPAGES: usize = 20;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: ::DWORD,
    pub DynamicPages: [::HPROPSHEETPAGE; MAX_INSTALLWIZARD_DYNAPAGES],
    pub NumDynamicPages: ::DWORD,
    pub DynamicPageFlags: ::DWORD,
    pub PrivateFlags: ::DWORD,
    pub PrivateData: ::LPARAM,
    pub hwndWizardDlg: ::HWND,
}
pub type PSP_INSTALLWIZARD_DATA = *mut SP_INSTALLWIZARD_DATA;
pub const NDW_INSTALLFLAG_DIDFACTDEFS: ::DWORD = 0x00000001;
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: ::DWORD = 0x00000002;
pub const NDW_INSTALLFLAG_NEEDRESTART: ::DWORD = DI_NEEDRESTART;
pub const NDW_INSTALLFLAG_NEEDREBOOT: ::DWORD = DI_NEEDREBOOT;
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: ::DWORD = 0x00000200;
pub const NDW_INSTALLFLAG_EXPRESSINTRO: ::DWORD = 0x00000400;
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: ::DWORD = 0x00000800;
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: ::DWORD = 0x00001000;
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: ::DWORD = 0x00002000;
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: ::DWORD = 0x00004000;
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: ::DWORD = 0x00008000;
pub const NDW_INSTALLFLAG_PCMCIAMODE: ::DWORD = 0x00010000;
pub const NDW_INSTALLFLAG_PCMCIADEVICE: ::DWORD = 0x00020000;
pub const NDW_INSTALLFLAG_USERCANCEL: ::DWORD = 0x00040000;
pub const NDW_INSTALLFLAG_KNOWNCLASS: ::DWORD = 0x00080000;
pub const DYNAWIZ_FLAG_PAGESADDED: ::DWORD = 0x00000001;
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: ::DWORD = 0x00000008;
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: ::DWORD = 0x00000002;
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: ::DWORD = 0x00000004;
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: ::c_int = 10000;
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: ::c_int = 11000;
pub const IDD_DYNAWIZ_FIRSTPAGE: ::c_int = 10000;
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: ::c_int = 10001;
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: ::c_int = 10002;
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: ::c_int = 10003;
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: ::c_int = 10004;
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: ::c_int = 10009;
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: ::c_int = 10010;
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: ::c_int = 10011;
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: ::c_int = 10012;
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: ::c_int = 10006;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: ::c_int = 10007;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: ::c_int = 10008;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: ::DWORD,
    pub DynamicPages: [::HPROPSHEETPAGE; MAX_INSTALLWIZARD_DYNAPAGES],
    pub NumDynamicPages: ::DWORD,
    pub hwndWizardDlg: ::HWND,
}
pub type PSP_NEWDEVICEWIZARD_DATA = *mut SP_NEWDEVICEWIZARD_DATA;
pub type SP_ADDPROPERTYPAGE_DATA = SP_NEWDEVICEWIZARD_DATA;
pub type PSP_ADDPROPERTYPAGE_DATA = PSP_NEWDEVICEWIZARD_DATA;
#[repr(C)] #[derive(Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [::CHAR; ::MAX_PATH],
    pub HtmlTroubleShooter: [::CHAR; ::MAX_PATH],
}
impl Clone for SP_TROUBLESHOOTER_PARAMS_A {
    fn clone(&self) -> SP_TROUBLESHOOTER_PARAMS_A { *self }
}
pub type PSP_TROUBLESHOOTER_PARAMS_A = *mut SP_TROUBLESHOOTER_PARAMS_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [::WCHAR; ::MAX_PATH],
    pub HtmlTroubleShooter: [::WCHAR; ::MAX_PATH],
}
impl Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> SP_TROUBLESHOOTER_PARAMS_W { *self }
}
pub type PSP_TROUBLESHOOTER_PARAMS_W = *mut SP_TROUBLESHOOTER_PARAMS_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [::CHAR; LINE_LEN * 2],
}
impl Clone for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn clone(&self) -> SP_POWERMESSAGEWAKE_PARAMS_A { *self }
}
pub type PSP_POWERMESSAGEWAKE_PARAMS_A = *mut SP_POWERMESSAGEWAKE_PARAMS_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [::WCHAR; LINE_LEN * 2],
}
impl Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> SP_POWERMESSAGEWAKE_PARAMS_W { *self }
}
pub type PSP_POWERMESSAGEWAKE_PARAMS_W = *mut SP_POWERMESSAGEWAKE_PARAMS_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: ::DWORD,
    pub DriverType: ::DWORD,
    pub Reserved: ::ULONG_PTR,
    pub Description: [::CHAR; LINE_LEN],
    pub MfgName: [::CHAR; LINE_LEN],
    pub ProviderName: [::CHAR; LINE_LEN],
    pub DriverDate: ::FILETIME,
    pub DriverVersion: ::DWORDLONG,
}
impl Clone for SP_DRVINFO_DATA_V2_A { fn clone(&self) -> SP_DRVINFO_DATA_V2_A { *self } }
pub type PSP_DRVINFO_DATA_V2_A = *mut SP_DRVINFO_DATA_V2_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: ::DWORD,
    pub DriverType: ::DWORD,
    pub Reserved: ::ULONG_PTR,
    pub Description: [::WCHAR; LINE_LEN],
    pub MfgName: [::WCHAR; LINE_LEN],
    pub ProviderName: [::WCHAR; LINE_LEN],
    pub DriverDate: ::FILETIME,
    pub DriverVersion: ::DWORDLONG,
}
impl Clone for SP_DRVINFO_DATA_V2_W { fn clone(&self) -> SP_DRVINFO_DATA_V2_W { *self } }
pub type PSP_DRVINFO_DATA_V2_W = *mut SP_DRVINFO_DATA_V2_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: ::DWORD,
    pub DriverType: ::DWORD,
    pub Reserved: ::ULONG_PTR,
    pub Description: [::CHAR; LINE_LEN],
    pub MfgName: [::CHAR; LINE_LEN],
    pub ProviderName: [::CHAR; LINE_LEN],
}
impl Clone for SP_DRVINFO_DATA_V1_A { fn clone(&self) -> SP_DRVINFO_DATA_V1_A { *self } }
pub type PSP_DRVINFO_DATA_V1_A = *mut SP_DRVINFO_DATA_V1_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: ::DWORD,
    pub DriverType: ::DWORD,
    pub Reserved: ::ULONG_PTR,
    pub Description: [::WCHAR; LINE_LEN],
    pub MfgName: [::WCHAR; LINE_LEN],
    pub ProviderName: [::WCHAR; LINE_LEN],
}
impl Clone for SP_DRVINFO_DATA_V1_W { fn clone(&self) -> SP_DRVINFO_DATA_V1_W { *self } }
pub type PSP_DRVINFO_DATA_V1_W = *mut SP_DRVINFO_DATA_V1_W;
pub type SP_DRVINFO_DATA_A = SP_DRVINFO_DATA_V2_A;
pub type PSP_DRVINFO_DATA_A = PSP_DRVINFO_DATA_V2_A;
pub type SP_DRVINFO_DATA_W = SP_DRVINFO_DATA_V2_W;
pub type PSP_DRVINFO_DATA_W = PSP_DRVINFO_DATA_V2_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: ::DWORD,
    pub InfDate: ::FILETIME,
    pub CompatIDsOffset: ::DWORD,
    pub CompatIDsLength: ::DWORD,
    pub Reserved: ::ULONG_PTR,
    pub SectionName: [::CHAR; LINE_LEN],
    pub InfFileName: [::CHAR; ::MAX_PATH],
    pub DrvDescription: [::CHAR; LINE_LEN],
    pub HardwareID: [::CHAR; ::ANYSIZE_ARRAY],
}
impl Clone for SP_DRVINFO_DETAIL_DATA_A { fn clone(&self) -> SP_DRVINFO_DETAIL_DATA_A { *self } }
pub type PSP_DRVINFO_DETAIL_DATA_A = *mut SP_DRVINFO_DETAIL_DATA_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: ::DWORD,
    pub InfDate: ::FILETIME,
    pub CompatIDsOffset: ::DWORD,
    pub CompatIDsLength: ::DWORD,
    pub Reserved: ::ULONG_PTR,
    pub SectionName: [::WCHAR; LINE_LEN],
    pub InfFileName: [::WCHAR; ::MAX_PATH],
    pub DrvDescription: [::WCHAR; LINE_LEN],
    pub HardwareID: [::WCHAR; ::ANYSIZE_ARRAY],
}
impl Clone for SP_DRVINFO_DETAIL_DATA_W { fn clone(&self) -> SP_DRVINFO_DETAIL_DATA_W { *self } }
pub type PSP_DRVINFO_DETAIL_DATA_W = *mut SP_DRVINFO_DETAIL_DATA_W;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: ::DWORD,
    pub Rank: ::DWORD,
    pub Flags: ::DWORD,
    pub PrivateData: ::DWORD_PTR,
    pub Reserved: ::DWORD,
}
pub type PSP_DRVINSTALL_PARAMS = *mut SP_DRVINSTALL_PARAMS;
pub const DNF_DUPDESC: ::DWORD = 0x00000001;
pub const DNF_OLDDRIVER: ::DWORD = 0x00000002;
pub const DNF_EXCLUDEFROMLIST: ::DWORD = 0x00000004;
pub const DNF_NODRIVER: ::DWORD = 0x00000008;
pub const DNF_LEGACYINF: ::DWORD = 0x00000010;
pub const DNF_CLASS_DRIVER: ::DWORD = 0x00000020;
pub const DNF_COMPATIBLE_DRIVER: ::DWORD = 0x00000040;
pub const DNF_INET_DRIVER: ::DWORD = 0x00000080;
pub const DNF_UNUSED1: ::DWORD = 0x00000100;
pub const DNF_UNUSED2: ::DWORD = 0x00000200;
pub const DNF_OLD_INET_DRIVER: ::DWORD = 0x00000400;
pub const DNF_BAD_DRIVER: ::DWORD = 0x00000800;
pub const DNF_DUPPROVIDER: ::DWORD = 0x00001000;
pub const DNF_INF_IS_SIGNED: ::DWORD = 0x00002000;
pub const DNF_OEM_F6_INF: ::DWORD = 0x00004000;
pub const DNF_DUPDRIVERVER: ::DWORD = 0x00008000;
pub const DNF_BASIC_DRIVER: ::DWORD = 0x00010000;
pub const DNF_AUTHENTICODE_SIGNED: ::DWORD = 0x00020000;
pub const DNF_INSTALLEDDRIVER: ::DWORD = 0x00040000;
pub const DNF_ALWAYSEXCLUDEFROMLIST: ::DWORD = 0x00080000;
pub const DNF_INBOX_DRIVER: ::DWORD = 0x00100000;
pub const DNF_REQUESTADDITIONALSOFTWARE: ::DWORD = 0x00200000;
pub const DNF_UNUSED_22: ::DWORD = 0x00400000;
pub const DNF_UNUSED_23: ::DWORD = 0x00800000;
pub const DNF_UNUSED_24: ::DWORD = 0x01000000;
pub const DNF_UNUSED_25: ::DWORD = 0x02000000;
pub const DNF_UNUSED_26: ::DWORD = 0x04000000;
pub const DNF_UNUSED_27: ::DWORD = 0x08000000;
pub const DNF_UNUSED_28: ::DWORD = 0x10000000;
pub const DNF_UNUSED_29: ::DWORD = 0x20000000;
pub const DNF_UNUSED_30: ::DWORD = 0x40000000;
pub const DNF_UNUSED_31: ::DWORD = 0x80000000;
pub type PSP_DETSIG_CMPPROC = Option<unsafe extern "system" fn(
    DeviceInfoSet: HDEVINFO, NewDeviceData: PSP_DEVINFO_DATA, ExistingDeviceData: PSP_DEVINFO_DATA,
    CompareContext: ::PVOID,
) -> ::DWORD>;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: ::BOOL,
    pub InstallResult: ::DWORD,
    pub PrivateData: ::PVOID,
}
pub type PCOINSTALLER_CONTEXT_DATA = *mut COINSTALLER_CONTEXT_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: ::DWORD,
    pub ImageList: ::HIMAGELIST,
    pub Reserved: ::ULONG_PTR,
}
pub type PSP_CLASSIMAGELIST_DATA = *mut SP_CLASSIMAGELIST_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: ::DWORD,
    pub PageRequested: ::DWORD,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: PSP_DEVINFO_DATA,
}
pub type PSP_PROPSHEETPAGE_REQUEST = *mut SP_PROPSHEETPAGE_REQUEST;
pub const SPPSR_SELECT_DEVICE_RESOURCES: ::DWORD = 1;
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: ::DWORD = 2;
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: ::DWORD = 3;
#[repr(C)] #[derive(Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: ::DWORD,
    pub FullInfPath: [::CHAR; ::MAX_PATH],
    pub FilenameOffset: ::INT,
    pub ReinstallInstance: [::CHAR; ::MAX_PATH],
}
impl Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> SP_BACKUP_QUEUE_PARAMS_V2_A { *self }
}
pub type PSP_BACKUP_QUEUE_PARAMS_V2_A = *mut SP_BACKUP_QUEUE_PARAMS_V2_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: ::DWORD,
    pub FullInfPath: [::WCHAR; ::MAX_PATH],
    pub FilenameOffset: ::INT,
    pub ReinstallInstance: [::WCHAR; ::MAX_PATH],
}
impl Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> SP_BACKUP_QUEUE_PARAMS_V2_W { *self }
}
pub type PSP_BACKUP_QUEUE_PARAMS_V2_W = *mut SP_BACKUP_QUEUE_PARAMS_V2_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: ::DWORD,
    pub FullInfPath: [::CHAR; ::MAX_PATH],
    pub FilenameOffset: ::INT,
}
impl Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> SP_BACKUP_QUEUE_PARAMS_V1_A { *self }
}
pub type PSP_BACKUP_QUEUE_PARAMS_V1_A = *mut SP_BACKUP_QUEUE_PARAMS_V1_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: ::DWORD,
    pub FullInfPath: [::WCHAR; ::MAX_PATH],
    pub FilenameOffset: ::INT,
}
impl Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> SP_BACKUP_QUEUE_PARAMS_V1_W { *self }
}
pub type PSP_BACKUP_QUEUE_PARAMS_V1_W = *mut SP_BACKUP_QUEUE_PARAMS_V1_W;
pub type SP_BACKUP_QUEUE_PARAMS_A = SP_BACKUP_QUEUE_PARAMS_V2_A;
pub type PSP_BACKUP_QUEUE_PARAMS_A = PSP_BACKUP_QUEUE_PARAMS_V2_A;
pub type SP_BACKUP_QUEUE_PARAMS_W = SP_BACKUP_QUEUE_PARAMS_V2_W;
pub type PSP_BACKUP_QUEUE_PARAMS_W = PSP_BACKUP_QUEUE_PARAMS_V2_W;
pub const ERROR_EXPECTED_SECTION_NAME: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0;
pub const ERROR_BAD_SECTION_NAME_LINE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 1;
pub const ERROR_SECTION_NAME_TOO_LONG: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 2;
pub const ERROR_GENERAL_SYNTAX: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 3;
pub const ERROR_WRONG_INF_STYLE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x100;
pub const ERROR_SECTION_NOT_FOUND: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x101;
pub const ERROR_LINE_NOT_FOUND: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x102;
pub const ERROR_NO_BACKUP: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x103;
pub const ERROR_NO_ASSOCIATED_CLASS: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x200;
pub const ERROR_CLASS_MISMATCH: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x201;
pub const ERROR_DUPLICATE_FOUND: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x202;
pub const ERROR_NO_DRIVER_SELECTED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x203;
pub const ERROR_KEY_DOES_NOT_EXIST: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x204;
pub const ERROR_INVALID_DEVINST_NAME: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x205;
pub const ERROR_INVALID_CLASS: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x206;
pub const ERROR_DEVINST_ALREADY_EXISTS: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x207;
pub const ERROR_DEVINFO_NOT_REGISTERED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x208;
pub const ERROR_INVALID_REG_PROPERTY: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x209;
pub const ERROR_NO_INF: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x20A;
pub const ERROR_NO_SUCH_DEVINST: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x20B;
pub const ERROR_CANT_LOAD_CLASS_ICON: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x20C;
pub const ERROR_INVALID_CLASS_INSTALLER: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x20D;
pub const ERROR_DI_DO_DEFAULT: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x20E;
pub const ERROR_DI_NOFILECOPY: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x20F;
pub const ERROR_INVALID_HWPROFILE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x210;
pub const ERROR_NO_DEVICE_SELECTED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x211;
pub const ERROR_DEVINFO_LIST_LOCKED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x212;
pub const ERROR_DEVINFO_DATA_LOCKED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x213;
pub const ERROR_DI_BAD_PATH: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x214;
pub const ERROR_NO_CLASSINSTALL_PARAMS: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x215;
pub const ERROR_FILEQUEUE_LOCKED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x216;
pub const ERROR_BAD_SERVICE_INSTALLSECT: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x217;
pub const ERROR_NO_CLASS_DRIVER_LIST: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x218;
pub const ERROR_NO_ASSOCIATED_SERVICE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x219;
pub const ERROR_NO_DEFAULT_DEVICE_INTERFACE: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x21A;
pub const ERROR_DEVICE_INTERFACE_ACTIVE: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x21B;
pub const ERROR_DEVICE_INTERFACE_REMOVED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x21C;
pub const ERROR_BAD_INTERFACE_INSTALLSECT: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x21D;
pub const ERROR_NO_SUCH_INTERFACE_CLASS: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x21E;
pub const ERROR_INVALID_REFERENCE_STRING: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x21F;
pub const ERROR_INVALID_MACHINENAME: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x220;
pub const ERROR_REMOTE_COMM_FAILURE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x221;
pub const ERROR_MACHINE_UNAVAILABLE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x222;
pub const ERROR_NO_CONFIGMGR_SERVICES: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x223;
pub const ERROR_INVALID_PROPPAGE_PROVIDER: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x224;
pub const ERROR_NO_SUCH_DEVICE_INTERFACE: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x225;
pub const ERROR_DI_POSTPROCESSING_REQUIRED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x226;
pub const ERROR_INVALID_COINSTALLER: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x227;
pub const ERROR_NO_COMPAT_DRIVERS: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x228;
pub const ERROR_NO_DEVICE_ICON: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x229;
pub const ERROR_INVALID_INF_LOGCONFIG: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x22A;
pub const ERROR_DI_DONT_INSTALL: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x22B;
pub const ERROR_INVALID_FILTER_DRIVER: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x22C;
pub const ERROR_NON_WINDOWS_NT_DRIVER: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x22D;
pub const ERROR_NON_WINDOWS_DRIVER: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x22E;
pub const ERROR_NO_CATALOG_FOR_OEM_INF: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x22F;
pub const ERROR_DEVINSTALL_QUEUE_NONNATIVE: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x230;
pub const ERROR_NOT_DISABLEABLE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x231;
pub const ERROR_CANT_REMOVE_DEVINST: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x232;
pub const ERROR_INVALID_TARGET: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x233;
pub const ERROR_DRIVER_NONNATIVE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x234;
pub const ERROR_IN_WOW64: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x235;
pub const ERROR_SET_SYSTEM_RESTORE_POINT: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x236;
pub const ERROR_SCE_DISABLED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR | 0x238;
pub const ERROR_UNKNOWN_EXCEPTION: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x239;
pub const ERROR_PNP_REGISTRY_ERROR: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x23A;
pub const ERROR_REMOTE_REQUEST_UNSUPPORTED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x23B;
pub const ERROR_NOT_AN_INSTALLED_OEM_INF: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x23C;
pub const ERROR_INF_IN_USE_BY_DEVICES: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x23D;
pub const ERROR_DI_FUNCTION_OBSOLETE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x23E;
pub const ERROR_NO_AUTHENTICODE_CATALOG: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x23F;
pub const ERROR_AUTHENTICODE_DISALLOWED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x240;
pub const ERROR_AUTHENTICODE_TRUSTED_PUBLISHER: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x241;
pub const ERROR_AUTHENTICODE_TRUST_NOT_ESTABLISHED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x242;
pub const ERROR_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x243;
pub const ERROR_SIGNATURE_OSATTRIBUTE_MISMATCH: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x244;
pub const ERROR_ONLY_VALIDATE_VIA_AUTHENTICODE: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x245;
pub const ERROR_DEVICE_INSTALLER_NOT_READY: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x246;
pub const ERROR_DRIVER_STORE_ADD_FAILED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x247;
pub const ERROR_DEVICE_INSTALL_BLOCKED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x248;
pub const ERROR_DRIVER_INSTALL_BLOCKED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x249;
pub const ERROR_WRONG_INF_TYPE: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x24A;
pub const ERROR_FILE_HASH_NOT_IN_CATALOG: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x24B;
pub const ERROR_DRIVER_STORE_DELETE_FAILED: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x24C;
pub const ERROR_UNRECOVERABLE_STACK_OVERFLOW: ::DWORD = ::APPLICATION_ERROR_MASK
    | ::ERROR_SEVERITY_ERROR | 0x300;
pub const EXCEPTION_SPAPI_UNRECOVERABLE_STACK_OVERFLOW: ::DWORD =
    ERROR_UNRECOVERABLE_STACK_OVERFLOW;
pub const ERROR_NO_DEFAULT_INTERFACE_DEVICE: ::DWORD = ERROR_NO_DEFAULT_DEVICE_INTERFACE;
pub const ERROR_INTERFACE_DEVICE_ACTIVE: ::DWORD = ERROR_DEVICE_INTERFACE_ACTIVE;
pub const ERROR_INTERFACE_DEVICE_REMOVED: ::DWORD = ERROR_DEVICE_INTERFACE_REMOVED;
pub const ERROR_NO_SUCH_INTERFACE_DEVICE: ::DWORD = ERROR_NO_SUCH_DEVICE_INTERFACE;
pub const ERROR_NOT_INSTALLED: ::DWORD = ::APPLICATION_ERROR_MASK | ::ERROR_SEVERITY_ERROR
    | 0x1000;
pub const INFINFO_INF_SPEC_IS_HINF: ::DWORD = 1;
pub const INFINFO_INF_NAME_IS_ABSOLUTE: ::DWORD = 2;
pub const INFINFO_DEFAULT_SEARCH: ::DWORD = 3;
pub const INFINFO_REVERSE_DEFAULT_SEARCH: ::DWORD = 4;
pub const INFINFO_INF_PATH_LIST_SEARCH: ::DWORD = 5;
pub const FILE_COMPRESSION_NONE: ::UINT = 0;
pub const FILE_COMPRESSION_WINLZA: ::UINT = 1;
pub const FILE_COMPRESSION_MSZIP: ::UINT = 2;
pub const FILE_COMPRESSION_NTCAB: ::UINT = 3;
pub const SRCLIST_TEMPORARY: ::DWORD = 0x00000001;
pub const SRCLIST_NOBROWSE: ::DWORD = 0x00000002;
pub const SRCLIST_SYSTEM: ::DWORD = 0x00000010;
pub const SRCLIST_USER: ::DWORD = 0x00000020;
pub const SRCLIST_SYSIFADMIN: ::DWORD = 0x00000040;
pub const SRCLIST_SUBDIRS: ::DWORD = 0x00000100;
pub const SRCLIST_APPEND: ::DWORD = 0x00000200;
pub const SRCLIST_NOSTRIPPLATFORM: ::DWORD = 0x00000400;
pub const IDF_NOBROWSE: ::DWORD = 0x00000001;
pub const IDF_NOSKIP: ::DWORD = 0x00000002;
pub const IDF_NODETAILS: ::DWORD = 0x00000004;
pub const IDF_NOCOMPRESSED: ::DWORD = 0x00000008;
pub const IDF_CHECKFIRST: ::DWORD = 0x00000100;
pub const IDF_NOBEEP: ::DWORD = 0x00000200;
pub const IDF_NOFOREGROUND: ::DWORD = 0x00000400;
pub const IDF_WARNIFSKIP: ::DWORD = 0x00000800;
pub const IDF_NOREMOVABLEMEDIAPROMPT: ::DWORD = 0x00001000;
pub const IDF_USEDISKNAMEASPROMPT: ::DWORD = 0x00002000;
pub const IDF_OEMDISK: ::DWORD = 0x80000000;
pub const DPROMPT_SUCCESS: ::UINT = 0;
pub const DPROMPT_CANCEL: ::UINT = 1;
pub const DPROMPT_SKIPFILE: ::UINT = 2;
pub const DPROMPT_BUFFERTOOSMALL: ::UINT = 3;
pub const DPROMPT_OUTOFMEMORY: ::UINT = 4;
pub const SETDIRID_NOT_FULL_PATH: ::DWORD = 0x00000001;
pub const SRCINFO_PATH: ::UINT = 1;
pub const SRCINFO_TAGFILE: ::UINT = 2;
pub const SRCINFO_DESCRIPTION: ::UINT = 3;
pub const SRCINFO_FLAGS: ::UINT = 4;
pub const SRCINFO_TAGFILE2: ::UINT = 4;
pub const SRC_FLAGS_CABFILE: ::UINT = 0x0010;
pub const SP_COPY_DELETESOURCE: ::DWORD = 0x0000001;
pub const SP_COPY_REPLACEONLY: ::DWORD = 0x0000002;
pub const SP_COPY_NEWER: ::DWORD = 0x0000004;
pub const SP_COPY_NEWER_OR_SAME: ::DWORD = SP_COPY_NEWER;
pub const SP_COPY_NOOVERWRITE: ::DWORD = 0x0000008;
pub const SP_COPY_NODECOMP: ::DWORD = 0x0000010;
pub const SP_COPY_LANGUAGEAWARE: ::DWORD = 0x0000020;
pub const SP_COPY_SOURCE_ABSOLUTE: ::DWORD = 0x0000040;
pub const SP_COPY_SOURCEPATH_ABSOLUTE: ::DWORD = 0x0000080;
pub const SP_COPY_IN_USE_NEEDS_REBOOT: ::DWORD = 0x0000100;
pub const SP_COPY_FORCE_IN_USE: ::DWORD = 0x0000200;
pub const SP_COPY_NOSKIP: ::DWORD = 0x0000400;
pub const SP_FLAG_CABINETCONTINUATION: ::DWORD = 0x0000800;
pub const SP_COPY_FORCE_NOOVERWRITE: ::DWORD = 0x0001000;
pub const SP_COPY_FORCE_NEWER: ::DWORD = 0x0002000;
pub const SP_COPY_WARNIFSKIP: ::DWORD = 0x0004000;
pub const SP_COPY_NOBROWSE: ::DWORD = 0x0008000;
pub const SP_COPY_NEWER_ONLY: ::DWORD = 0x0010000;
pub const SP_COPY_RESERVED: ::DWORD = 0x0020000;
pub const SP_COPY_OEMINF_CATALOG_ONLY: ::DWORD = 0x0040000;
pub const SP_COPY_REPLACE_BOOT_FILE: ::DWORD = 0x0080000;
pub const SP_COPY_NOPRUNE: ::DWORD = 0x0100000;
pub const SP_COPY_OEM_F6_INF: ::DWORD = 0x0200000;
pub const SP_COPY_ALREADYDECOMP: ::DWORD = 0x0400000;
pub const SP_COPY_WINDOWS_SIGNED: ::DWORD = 0x1000000;
pub const SP_COPY_PNPLOCKED: ::DWORD = 0x2000000;
pub const SP_COPY_IN_USE_TRY_RENAME: ::DWORD = 0x4000000;
pub const SP_COPY_INBOX_INF: ::DWORD = 0x8000000;
pub const SP_COPY_HARDLINK: ::DWORD = 0x10000000;
pub const SP_BACKUP_BACKUPPASS: ::DWORD = 0x00000001;
pub const SP_BACKUP_DEMANDPASS: ::DWORD = 0x00000002;
pub const SP_BACKUP_SPECIAL: ::DWORD = 0x00000004;
pub const SP_BACKUP_BOOTFILE: ::DWORD = 0x00000008;
pub const SPQ_SCAN_FILE_PRESENCE: ::DWORD = 0x00000001;
pub const SPQ_SCAN_FILE_VALIDITY: ::DWORD = 0x00000002;
pub const SPQ_SCAN_USE_CALLBACK: ::DWORD = 0x00000004;
pub const SPQ_SCAN_USE_CALLBACKEX: ::DWORD = 0x00000008;
pub const SPQ_SCAN_INFORM_USER: ::DWORD = 0x00000010;
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: ::DWORD = 0x00000020;
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: ::DWORD = 0x00000040;
pub const SPQ_SCAN_PRUNE_DELREN: ::DWORD = 0x00000080;
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: ::DWORD = 0x00000100;
pub const SPQ_SCAN_FILE_COMPARISON: ::DWORD = 0x00000200;
pub const SPQ_SCAN_ACTIVATE_DRP: ::DWORD = 0x00000400;
pub const SPQ_DELAYED_COPY: ::DWORD = 0x00000001;
pub const SPQ_FLAG_BACKUP_AWARE: ::DWORD = 0x00000001;
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: ::DWORD = 0x00000002;
pub const SPQ_FLAG_FILES_MODIFIED: ::DWORD = 0x00000004;
pub const SPQ_FLAG_DO_SHUFFLEMOVE: ::DWORD = 0x00000008;
pub const SPQ_FLAG_VALID: ::DWORD = 0x0000000F;
pub const SPOST_NONE: ::DWORD = 0;
pub const SPOST_PATH: ::DWORD = 1;
pub const SPOST_URL: ::DWORD = 2;
pub const SPOST_MAX: ::DWORD = 3;
pub const SUOI_FORCEDELETE: ::DWORD = 0x00000001;
pub const SUOI_INTERNAL1: ::DWORD = 0x00000002;
pub const SPDSL_IGNORE_DISK: ::UINT = 0x00000001;
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: ::UINT = 0x00000002;
pub const SPFILEQ_FILE_IN_USE: ::INT = 0x00000001;
pub const SPFILEQ_REBOOT_RECOMMENDED: ::INT = 0x00000002;
pub const SPFILEQ_REBOOT_IN_PROGRESS: ::INT = 0x00000004;
pub const FLG_ADDREG_DELREG_BIT: ::DWORD = 0x00008000;
pub const FLG_ADDREG_BINVALUETYPE: ::DWORD = 0x00000001;
pub const FLG_ADDREG_NOCLOBBER: ::DWORD = 0x00000002;
pub const FLG_ADDREG_DELVAL: ::DWORD = 0x00000004;
pub const FLG_ADDREG_APPEND: ::DWORD = 0x00000008;
pub const FLG_ADDREG_KEYONLY: ::DWORD = 0x00000010;
pub const FLG_ADDREG_OVERWRITEONLY: ::DWORD = 0x00000020;
pub const FLG_ADDREG_64BITKEY: ::DWORD = 0x00001000;
pub const FLG_ADDREG_KEYONLY_COMMON: ::DWORD = 0x00002000;
pub const FLG_ADDREG_32BITKEY: ::DWORD = 0x00004000;
pub const FLG_ADDREG_TYPE_MASK: ::DWORD = 0xFFFF0000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_ADDREG_TYPE_SZ: ::DWORD = 0x00000000;
pub const FLG_ADDREG_TYPE_MULTI_SZ: ::DWORD = 0x00010000;
pub const FLG_ADDREG_TYPE_EXPAND_SZ: ::DWORD = 0x00020000;
pub const FLG_ADDREG_TYPE_BINARY: ::DWORD = 0x00000000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_ADDREG_TYPE_DWORD: ::DWORD = 0x00010000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_ADDREG_TYPE_NONE: ::DWORD = 0x00020000 | FLG_ADDREG_BINVALUETYPE;
pub const FLG_DELREG_VALUE: ::DWORD = 0x00000000;
pub const FLG_DELREG_TYPE_MASK: ::DWORD = FLG_ADDREG_TYPE_MASK;
pub const FLG_DELREG_TYPE_SZ: ::DWORD = FLG_ADDREG_TYPE_SZ;
pub const FLG_DELREG_TYPE_MULTI_SZ: ::DWORD = FLG_ADDREG_TYPE_MULTI_SZ;
pub const FLG_DELREG_TYPE_EXPAND_SZ: ::DWORD = FLG_ADDREG_TYPE_EXPAND_SZ;
pub const FLG_DELREG_TYPE_BINARY: ::DWORD = FLG_ADDREG_TYPE_BINARY;
pub const FLG_DELREG_TYPE_DWORD: ::DWORD = FLG_ADDREG_TYPE_DWORD;
pub const FLG_DELREG_TYPE_NONE: ::DWORD = FLG_ADDREG_TYPE_NONE;
pub const FLG_DELREG_64BITKEY: ::DWORD = FLG_ADDREG_64BITKEY;
pub const FLG_DELREG_KEYONLY_COMMON: ::DWORD = FLG_ADDREG_KEYONLY_COMMON;
pub const FLG_DELREG_32BITKEY: ::DWORD = FLG_ADDREG_32BITKEY;
pub const FLG_DELREG_OPERATION_MASK: ::DWORD = 0x000000FE;
pub const FLG_DELREG_MULTI_SZ_DELSTRING: ::DWORD = FLG_DELREG_TYPE_MULTI_SZ | FLG_ADDREG_DELREG_BIT
    | 0x00000002;
pub const FLG_BITREG_CLEARBITS: ::DWORD = 0x00000000;
pub const FLG_BITREG_SETBITS: ::DWORD = 0x00000001;
pub const FLG_BITREG_64BITKEY: ::DWORD = 0x00001000;
pub const FLG_BITREG_32BITKEY: ::DWORD = 0x00004000;
pub const FLG_INI2REG_64BITKEY: ::DWORD = 0x00001000;
pub const FLG_INI2REG_32BITKEY: ::DWORD = 0x00004000;
pub const FLG_REGSVR_DLLREGISTER: ::DWORD = 0x00000001;
pub const FLG_REGSVR_DLLINSTALL: ::DWORD = 0x00000002;
pub const FLG_PROFITEM_CURRENTUSER: ::DWORD = 0x00000001;
pub const FLG_PROFITEM_DELETE: ::DWORD = 0x00000002;
pub const FLG_PROFITEM_GROUP: ::DWORD = 0x00000004;
pub const FLG_PROFITEM_CSIDL: ::DWORD = 0x00000008;
pub const FLG_ADDPROPERTY_NOCLOBBER: ::DWORD = 0x00000001;
pub const FLG_ADDPROPERTY_OVERWRITEONLY: ::DWORD = 0x00000002;
pub const FLG_ADDPROPERTY_APPEND: ::DWORD = 0x00000004;
pub const FLG_ADDPROPERTY_OR: ::DWORD = 0x00000008;
pub const FLG_ADDPROPERTY_AND: ::DWORD = 0x00000010;
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: ::DWORD = 0x00000001;
pub const SPINST_LOGCONFIG: ::UINT = 0x00000001;
pub const SPINST_INIFILES: ::UINT = 0x00000002;
pub const SPINST_REGISTRY: ::UINT = 0x00000004;
pub const SPINST_INI2REG: ::UINT = 0x00000008;
pub const SPINST_FILES: ::UINT = 0x00000010;
pub const SPINST_BITREG: ::UINT = 0x00000020;
pub const SPINST_REGSVR: ::UINT = 0x00000040;
pub const SPINST_UNREGSVR: ::UINT = 0x00000080;
pub const SPINST_PROFILEITEMS: ::UINT = 0x00000100;
pub const SPINST_COPYINF: ::UINT = 0x00000200;
pub const SPINST_PROPERTIES: ::UINT = 0x00000400;
pub const SPINST_ALL: ::UINT = 0x000007ff;
pub const SPINST_SINGLESECTION: ::UINT = 0x00010000;
pub const SPINST_LOGCONFIG_IS_FORCED: ::UINT = 0x00020000;
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: ::UINT = 0x00040000;
pub const SPINST_REGISTERCALLBACKAWARE: ::UINT = 0x00080000;
pub const SPINST_DEVICEINSTALL: ::UINT = 0x00100000;
pub const SPSVCINST_TAGTOFRONT: ::DWORD = 0x00000001;
pub const SPSVCINST_ASSOCSERVICE: ::DWORD = 0x00000002;
pub const SPSVCINST_DELETEEVENTLOGENTRY: ::DWORD = 0x00000004;
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: ::DWORD = 0x00000008;
pub const SPSVCINST_NOCLOBBER_STARTTYPE: ::DWORD = 0x00000010;
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: ::DWORD = 0x00000020;
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: ::DWORD = 0x00000040;
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: ::DWORD = 0x00000080;
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: ::DWORD = 0x00000100;
pub const SPSVCINST_STOPSERVICE: ::DWORD = 0x00000200;
pub const SPSVCINST_CLOBBER_SECURITY: ::DWORD = 0x00000400;
pub const SPSVCINST_STARTSERVICE: ::DWORD = 0x00000800;
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: ::DWORD = 0x00001000;
pub type HSPFILELOG = ::PVOID;
pub const SPFILELOG_SYSTEMLOG: ::DWORD = 0x00000001;
pub const SPFILELOG_FORCENEW: ::DWORD = 0x00000002;
pub const SPFILELOG_QUERYONLY: ::DWORD = 0x00000004;
pub const SPFILELOG_OEMFILE: ::DWORD = 0x00000001;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum SetupFileLogInfo {
    SetupFileLogSourceFilename,
    SetupFileLogChecksum,
    SetupFileLogDiskTagfile,
    SetupFileLogDiskDescription,
    SetupFileLogOtherInfo,
    SetupFileLogMax,
}
pub type LogSeverity = ::DWORD;
pub const LogSevInformation: LogSeverity = 0x00000000;
pub const LogSevWarning: LogSeverity = 0x00000001;
pub const LogSevError: LogSeverity = 0x00000002;
pub const LogSevFatalError: LogSeverity = 0x00000003;
pub const LogSevMaximum: LogSeverity = 0x00000004;
pub const DICD_GENERATE_ID: ::DWORD = 0x00000001;
pub const DICD_INHERIT_CLASSDRVS: ::DWORD = 0x00000002;
pub const DIOD_INHERIT_CLASSDRVS: ::DWORD = 0x00000002;
pub const DIOD_CANCEL_REMOVE: ::DWORD = 0x00000004;
pub const DIODI_NO_ADD: ::DWORD = 0x00000001;
pub const SPRDI_FIND_DUPS: ::DWORD = 0x00000001;
pub const SPDIT_NODRIVER: ::DWORD = 0x00000000;
pub const SPDIT_CLASSDRIVER: ::DWORD = 0x00000001;
pub const SPDIT_COMPATDRIVER: ::DWORD = 0x00000002;
pub const DIGCF_DEFAULT: ::DWORD = 0x00000001;
pub const DIGCF_PRESENT: ::DWORD = 0x00000002;
pub const DIGCF_ALLCLASSES: ::DWORD = 0x00000004;
pub const DIGCF_PROFILE: ::DWORD = 0x00000008;
pub const DIGCF_DEVICEINTERFACE: ::DWORD = 0x00000010;
pub const DIBCI_NOINSTALLCLASS: ::DWORD = 0x00000001;
pub const DIBCI_NODISPLAYCLASS: ::DWORD = 0x00000002;
pub const DIOCR_INSTALLER: ::DWORD = 0x00000001;
pub const DIOCR_INTERFACE: ::DWORD = 0x00000002;
pub const DIREG_DEV: ::DWORD = 0x00000001;
pub const DIREG_DRV: ::DWORD = 0x00000002;
pub const DIREG_BOTH: ::DWORD = 0x00000004;
pub const DICLASSPROP_INSTALLER: ::DWORD = 0x00000001;
pub const DICLASSPROP_INTERFACE: ::DWORD = 0x00000002;
pub const SPDRP_DEVICEDESC: ::DWORD = 0x00000000;
pub const SPDRP_HARDWAREID: ::DWORD = 0x00000001;
pub const SPDRP_COMPATIBLEIDS: ::DWORD = 0x00000002;
pub const SPDRP_UNUSED0: ::DWORD = 0x00000003;
pub const SPDRP_SERVICE: ::DWORD = 0x00000004;
pub const SPDRP_UNUSED1: ::DWORD = 0x00000005;
pub const SPDRP_UNUSED2: ::DWORD = 0x00000006;
pub const SPDRP_CLASS: ::DWORD = 0x00000007;
pub const SPDRP_CLASSGUID: ::DWORD = 0x00000008;
pub const SPDRP_DRIVER: ::DWORD = 0x00000009;
pub const SPDRP_CONFIGFLAGS: ::DWORD = 0x0000000A;
pub const SPDRP_MFG: ::DWORD = 0x0000000B;
pub const SPDRP_FRIENDLYNAME: ::DWORD = 0x0000000C;
pub const SPDRP_LOCATION_INFORMATION: ::DWORD = 0x0000000D;
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: ::DWORD = 0x0000000E;
pub const SPDRP_CAPABILITIES: ::DWORD = 0x0000000F;
pub const SPDRP_UI_NUMBER: ::DWORD = 0x00000010;
pub const SPDRP_UPPERFILTERS: ::DWORD = 0x00000011;
pub const SPDRP_LOWERFILTERS: ::DWORD = 0x00000012;
pub const SPDRP_BUSTYPEGUID: ::DWORD = 0x00000013;
pub const SPDRP_LEGACYBUSTYPE: ::DWORD = 0x00000014;
pub const SPDRP_BUSNUMBER: ::DWORD = 0x00000015;
pub const SPDRP_ENUMERATOR_NAME: ::DWORD = 0x00000016;
pub const SPDRP_SECURITY: ::DWORD = 0x00000017;
pub const SPDRP_SECURITY_SDS: ::DWORD = 0x00000018;
pub const SPDRP_DEVTYPE: ::DWORD = 0x00000019;
pub const SPDRP_EXCLUSIVE: ::DWORD = 0x0000001A;
pub const SPDRP_CHARACTERISTICS: ::DWORD = 0x0000001B;
pub const SPDRP_ADDRESS: ::DWORD = 0x0000001C;
pub const SPDRP_UI_NUMBER_DESC_FORMAT: ::DWORD = 0x0000001D;
pub const SPDRP_DEVICE_POWER_DATA: ::DWORD = 0x0000001E;
pub const SPDRP_REMOVAL_POLICY: ::DWORD = 0x0000001F;
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: ::DWORD = 0x00000020;
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: ::DWORD = 0x00000021;
pub const SPDRP_INSTALL_STATE: ::DWORD = 0x00000022;
pub const SPDRP_LOCATION_PATHS: ::DWORD = 0x00000023;
pub const SPDRP_BASE_CONTAINERID: ::DWORD = 0x00000024;
pub const SPDRP_MAXIMUM_PROPERTY: ::DWORD = 0x00000025;
pub const SPCRP_UPPERFILTERS: ::DWORD = 0x00000011;
pub const SPCRP_LOWERFILTERS: ::DWORD = 0x00000012;
pub const SPCRP_SECURITY: ::DWORD = 0x00000017;
pub const SPCRP_SECURITY_SDS: ::DWORD = 0x00000018;
pub const SPCRP_DEVTYPE: ::DWORD = 0x00000019;
pub const SPCRP_EXCLUSIVE: ::DWORD = 0x0000001A;
pub const SPCRP_CHARACTERISTICS: ::DWORD = 0x0000001B;
pub const SPCRP_MAXIMUM_PROPERTY: ::DWORD = 0x0000001C;
pub const DMI_MASK: ::DWORD = 0x00000001;
pub const DMI_BKCOLOR: ::DWORD = 0x00000002;
pub const DMI_USERECT: ::DWORD = 0x00000004;
pub const DIGCDP_FLAG_BASIC: ::DWORD = 0x00000001;
pub const DIGCDP_FLAG_ADVANCED: ::DWORD = 0x00000002;
pub const DIGCDP_FLAG_REMOTE_BASIC: ::DWORD = 0x00000003;
pub const DIGCDP_FLAG_REMOTE_ADVANCED: ::DWORD = 0x00000004;
pub const IDI_RESOURCEFIRST: ::c_int = 159;
pub const IDI_RESOURCE: ::c_int = 159;
pub const IDI_RESOURCELAST: ::c_int = 161;
pub const IDI_RESOURCEOVERLAYFIRST: ::c_int = 161;
pub const IDI_RESOURCEOVERLAYLAST: ::c_int = 161;
pub const IDI_CONFLICT: ::c_int = 161;
pub const IDI_CLASSICON_OVERLAYFIRST: ::c_int = 500;
pub const IDI_CLASSICON_OVERLAYLAST: ::c_int = 502;
pub const IDI_PROBLEM_OVL: ::c_int = 500;
pub const IDI_DISABLED_OVL: ::c_int = 501;
pub const IDI_FORCED_OVL: ::c_int = 502;
pub const SPWPT_SELECTDEVICE: ::DWORD = 0x00000001;
pub const SPWP_USE_DEVINFO_DATA: ::DWORD = 0x00000001;
#[repr(C)] #[derive(Copy)]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: ::DWORD,
    pub CatalogFile: [::CHAR; ::MAX_PATH],
    pub DigitalSigner: [::CHAR; ::MAX_PATH],
    pub DigitalSignerVersion: [::CHAR; ::MAX_PATH],
}
impl Clone for SP_INF_SIGNER_INFO_V1_A { fn clone(&self) -> SP_INF_SIGNER_INFO_V1_A { *self } }
pub type PSP_INF_SIGNER_INFO_V1_A = *mut SP_INF_SIGNER_INFO_V1_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: ::DWORD,
    pub CatalogFile: [::WCHAR; ::MAX_PATH],
    pub DigitalSigner: [::WCHAR; ::MAX_PATH],
    pub DigitalSignerVersion: [::WCHAR; ::MAX_PATH],
}
impl Clone for SP_INF_SIGNER_INFO_V1_W { fn clone(&self) -> SP_INF_SIGNER_INFO_V1_W { *self } }
pub type PSP_INF_SIGNER_INFO_V1_W = *mut SP_INF_SIGNER_INFO_V1_W;
#[repr(C)] #[derive(Copy)]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: ::DWORD,
    pub CatalogFile: [::CHAR; ::MAX_PATH],
    pub DigitalSigner: [::CHAR; ::MAX_PATH],
    pub DigitalSignerVersion: [::CHAR; ::MAX_PATH],
    pub SignerScore: ::DWORD,
}
impl Clone for SP_INF_SIGNER_INFO_V2_A { fn clone(&self) -> SP_INF_SIGNER_INFO_V2_A { *self } }
pub type PSP_INF_SIGNER_INFO_V2_A = *mut SP_INF_SIGNER_INFO_V2_A;
#[repr(C)] #[derive(Copy)]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: ::DWORD,
    pub CatalogFile: [::WCHAR; ::MAX_PATH],
    pub DigitalSigner: [::WCHAR; ::MAX_PATH],
    pub DigitalSignerVersion: [::WCHAR; ::MAX_PATH],
    pub SignerScore: ::DWORD,
}
impl Clone for SP_INF_SIGNER_INFO_V2_W { fn clone(&self) -> SP_INF_SIGNER_INFO_V2_W { *self } }
pub type PSP_INF_SIGNER_INFO_V2_W = *mut SP_INF_SIGNER_INFO_V2_W;
pub type SP_INF_SIGNER_INFO_A = SP_INF_SIGNER_INFO_V2_A;
pub type PSP_INF_SIGNER_INFO_A = PSP_INF_SIGNER_INFO_V2_A;
pub type SP_INF_SIGNER_INFO_W = SP_INF_SIGNER_INFO_V2_W;
pub type PSP_INF_SIGNER_INFO_W = PSP_INF_SIGNER_INFO_V2_W;
pub const SIGNERSCORE_UNKNOWN: ::DWORD = 0xFF000000;
pub const SIGNERSCORE_W9X_SUSPECT: ::DWORD = 0xC0000000;
pub const SIGNERSCORE_UNSIGNED: ::DWORD = 0x80000000;
pub const SIGNERSCORE_AUTHENTICODE: ::DWORD = 0x0F000000;
pub const SIGNERSCORE_WHQL: ::DWORD = 0x0D000005;
pub const SIGNERSCORE_UNCLASSIFIED: ::DWORD = 0x0D000004;
pub const SIGNERSCORE_INBOX: ::DWORD = 0x0D000003;
pub const SIGNERSCORE_LOGO_STANDARD: ::DWORD = 0x0D000002;
pub const SIGNERSCORE_LOGO_PREMIUM: ::DWORD = 0x0D000001;
pub const SIGNERSCORE_MASK: ::DWORD = 0xFF000000;
pub const SIGNERSCORE_SIGNED_MASK: ::DWORD = 0xF0000000;
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: ::DWORD = 0x00000001;
pub const SCWMI_CLOBBER_SECURITY: ::DWORD = 0x00000001;
