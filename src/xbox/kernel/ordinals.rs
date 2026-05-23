#![allow(non_upper_case_globals)]

/// Xbox kernel ordinal constants and argument counts.
/// Ported from XboxKernelList.h and XboxKernelArgCount.h (X-macro tables).
/// Ordinals 1-378 from xboxkrnl.exe.def.
///
/// Arg count = number of 4-byte stdcall args the callee pops.
/// cdecl/variadic functions have arg_count=0 (caller cleans).
/// __fastcall functions have arg_count=0 (args in ECX/EDX, no stack cleanup).
/// Data exports have arg_count=0 and is_data=true.

/// Generate ordinal constants and arg count table from a single source of truth.
macro_rules! define_ordinals {
    ( $( ($ord:expr, $name:ident, $argc:expr, $data:expr) ),* $(,)? ) => {
        // Named constants
        $( pub const $name: u32 = $ord; )*

        /// Max ordinal value (for array sizing).
        pub const MAX_ORDINAL: usize = 400;

        /// Lookup arg count for an ordinal. Returns 0 for unknown.
        pub fn arg_count(ordinal: u32) -> u32 {
            static TABLE: [u8; MAX_ORDINAL] = {
                let mut t = [0u8; MAX_ORDINAL];
                $( if ($ord as usize) < MAX_ORDINAL { t[$ord as usize] = $argc; } )*
                t
            };
            if (ordinal as usize) < MAX_ORDINAL {
                TABLE[ordinal as usize] as u32
            } else {
                0
            }
        }

        /// Check if ordinal is a data export (not a function).
        pub fn is_data(ordinal: u32) -> bool {
            static FLAGS: [bool; MAX_ORDINAL] = {
                let mut t = [false; MAX_ORDINAL];
                $( if ($ord as usize) < MAX_ORDINAL { t[$ord as usize] = $data; } )*
                t
            };
            if (ordinal as usize) < MAX_ORDINAL {
                FLAGS[ordinal as usize]
            } else {
                false
            }
        }

        /// Get ordinal name string for logging.
        pub fn name(ordinal: u32) -> &'static str {
            match ordinal {
                $( $ord => stringify!($name), )*
                _ => "Unknown",
            }
        }
    };
}

define_ordinals! {
    // AV (1-4)
    (1,   AvGetSavedDataAddress,        0, false),
    (2,   AvSendTVEncoderOption,        4, false),
    (3,   AvSetDisplayMode,             6, false),
    (4,   AvSetSavedDataAddress,        1, false),

    // Debug (5-11)
    (5,   DbgBreakPoint,                0, false),
    (6,   DbgBreakPointWithStatus,      1, false),
    (7,   DbgLoadImageSymbols,          3, false),
    (8,   DbgPrint,                     0, false), // cdecl variadic
    (9,   HalReadSMCTrayState,          0, false),
    (10,  DbgPrompt,                    3, false),
    (11,  DbgUnLoadImageSymbols,        3, false),

    // Executive (12-34)
    (12,  ExAcquireReadWriteLockExclusive, 1, false),
    (13,  ExAcquireReadWriteLockShared,    1, false),
    (14,  ExAllocatePool,               1, false),
    (15,  ExAllocatePoolWithTag,        2, false),
    (16,  ExEventObjectType,            0, true),
    (17,  ExFreePool,                   1, false),
    (18,  ExInitializeReadWriteLock,    1, false),
    (19,  ExInterlockedAddLargeInteger, 4, false),
    (20,  ExInterlockedAddLargeStatistic, 2, false),
    (21,  ExInterlockedCompareExchange64, 3, false),
    (22,  ExMutantObjectType,           0, true),
    (23,  ExQueryPoolBlockSize,         1, false),
    (24,  ExQueryNonVolatileSetting,    5, false),
    (25,  ExReadWriteRefurbInfo,        3, false),
    (26,  ExRaiseException,             1, false),
    (27,  ExRaiseStatus,                1, false),
    (28,  ExReleaseReadWriteLock,       1, false),
    (29,  ExSaveNonVolatileSetting,     5, false),
    (30,  ExSemaphoreObjectType,        0, true),
    (31,  ExTimerObjectType,            0, true),
    (32,  ExfInterlockedInsertHeadList, 0, false), // __fastcall
    (33,  ExfInterlockedInsertTailList, 0, false), // __fastcall
    (34,  ExfInterlockedRemoveHeadList, 0, false), // __fastcall

    // Filesystem cache (35-37)
    (35,  FscGetCacheSize,              0, false),
    (36,  FscInvalidateIdleBlocks,      0, false),
    (37,  FscSetCacheSize,              1, false),

    // HAL (38-50)
    (38,  HalClearSoftwareInterrupt,    1, false),
    (39,  HalDisableSystemInterrupt,    2, false),
    (40,  HalDiskCachePartitionCount,   0, true),
    (41,  HalDiskModelNumber,           0, true),
    (42,  HalDiskSerialNumber,          0, true),
    (43,  HalEnableSystemInterrupt,     3, false),
    (44,  HalGetInterruptVector,        2, false),
    (45,  HalReadSMBusValue,            4, false),
    (46,  HalReadWritePCISpace,         6, false),
    (47,  HalRegisterShutdownNotification, 2, false),
    (48,  HalRequestSoftwareInterrupt,  1, false),
    (49,  HalReturnToFirmware,          1, false),
    (50,  HalWriteSMBusValue,           4, false),

    // Interlocked (51-58) — ALL __fastcall
    (51,  InterlockedCompareExchange,   1, false),
    (52,  InterlockedDecrement,         0, false),
    (53,  InterlockedIncrement,         0, false),
    (54,  InterlockedExchange,          0, false),
    (55,  InterlockedExchangeAdd,       0, false),
    (56,  InterlockedFlushSList,        0, false),
    (57,  InterlockedPopEntrySList,     0, false),
    (58,  InterlockedPushEntrySList,    0, false),

    // I/O Manager (59-91)
    (59,  IoAllocateIrp,                1, false),
    (60,  IoBuildAsynchronousFsdRequest, 6, false),
    (61,  IoBuildDeviceIoControlRequest, 9, false),
    (62,  IoBuildSynchronousFsdRequest, 6, false),
    (63,  IoCheckShareAccess,           5, false),
    (64,  IoCompletionObjectType,       0, true),
    (65,  IoCreateDevice,               6, false),
    (66,  IoCreateFile,                10, false),
    (67,  IoCreateSymbolicLink,         2, false),
    (68,  IoDeleteDevice,               1, false),
    (69,  IoDeleteSymbolicLink,         1, false),
    (70,  IoDeviceObjectType,           0, true),
    (71,  IoFileObjectType,             0, true),
    (72,  IoFreeIrp,                    1, false),
    (73,  IoInitializeIrp,              3, false),
    (74,  IoInvalidDeviceRequest,       0, false),
    (75,  IoQueryFileInformation,       4, false),
    (76,  IoQueryVolumeInformation,     4, false),
    (77,  IoQueueThreadIrp,             1, false),
    (78,  IoRemoveShareAccess,          2, false),
    (79,  IoSetIoCompletion,            5, false),
    (80,  IoSetShareAccess,             4, false),
    (81,  IoStartNextPacket,            1, false),
    (82,  IoStartNextPacketByKey,       3, false),
    (83,  IoStartPacket,                4, false),
    (84,  IoSynchronousDeviceIoControlRequest, 8, false),
    (85,  IoSynchronousFsdRequest,      5, false),
    (86,  IofCallDriver,                0, false), // __fastcall
    (87,  IofCompleteRequest,           0, false), // __fastcall
    (88,  KdDebuggerEnabled,            0, true),
    (89,  KdDebuggerNotPresent,         0, true),
    (90,  IoDismountVolume,             1, false),
    (91,  IoDismountVolumeByName,       1, false),

    // Ke (92-159)
    (92,  KeAlertResumeThread,          2, false),
    (93,  KeAlertThread,                2, false),
    (94,  KeBoostPriorityThread,        2, false),
    (95,  KeBugCheck,                   1, false),
    (96,  KeBugCheckEx,                 5, false),
    (97,  KeCancelTimer,                1, false),
    (98,  KeConnectInterrupt,           1, false),
    (99,  KeDelayExecutionThread,       3, false),
    (100, KeDisconnectInterrupt,        1, false),
    (101, KeEnterCriticalRegion,        0, false),
    (102, MmGlobalData,                 0, true),
    (103, KeGetCurrentIrql,             0, false),
    (104, KeGetCurrentThread,           0, false),
    (105, KeInitializeApc,              3, false),
    (106, KeInitializeDeviceQueue,      1, false),
    (107, KeInitializeDpc,              3, false),
    (108, KeInitializeEvent,            3, false),
    (109, KeInitializeInterrupt,        7, false),
    (110, KeInitializeMutant,           2, false),
    (111, KeInitializeQueue,            1, false),
    (112, KeInitializeSemaphore,        2, false),
    (113, KeInitializeTimerEx,          2, false),
    (114, KeInsertByKeyDeviceQueue,     2, false),
    (115, KeInsertDeviceQueue,          1, false),
    (116, KeInsertHeadQueue,            2, false),
    (117, KeInsertQueue,                2, false),
    (118, KeInsertQueueApc,             3, false),
    (119, KeInsertQueueDpc,             3, false),
    (120, KeInterruptTime,              0, true),
    (121, KeIsExecutingDpc,             0, false),
    (122, KeLeaveCriticalRegion,        0, false),
    (123, KePulseEvent,                 2, false),
    (124, KeQueryBasePriorityThread,    1, false),
    (125, KeQueryInterruptTime,         0, false),
    (126, KeQueryPerformanceCounter,    0, false),
    (127, KeQueryPerformanceFrequency,  0, false),
    (128, KeQuerySystemTime,            1, false),
    (129, KeRaiseIrqlToDpcLevel,        0, false),
    (130, KeRaiseIrqlToSynchLevel,      0, false),
    (131, KeReleaseMutant,              3, false),
    (132, KeReleaseSemaphore,           3, false),
    (133, KeRemoveByKeyDeviceQueue,     2, false),
    (134, KeRemoveDeviceQueue,          1, false),
    (135, KeRemoveEntryDeviceQueue,     2, false),
    (136, KeRemoveQueue,                2, false),
    (137, KeRemoveQueueDpc,             1, false),
    (138, KeResetEvent,                 1, false),
    (139, KeRestoreFloatingPointState,  1, false),
    (140, KeResumeThread,               1, false),
    (141, KeRundownQueue,               1, false),
    (142, KeSaveFloatingPointState,     1, false),
    (143, KeSetBasePriorityThread,      2, false),
    (144, KeSetDisableBoostThread,      2, false),
    (145, KeSetEvent,                   3, false),
    (146, KeSetEventBoostPriority,      1, false),
    (147, KeSetPriorityProcess,         2, false),
    (148, KeSetPriorityThread,          2, false),
    (149, KeSetTimer,                   4, false),
    (150, KeSetTimerEx,                 5, false),
    (151, KeStallExecutionProcessor,    1, false),
    (152, KeSuspendThread,              1, false),
    (153, KeSynchronizeExecution,       3, false),
    (154, KeSystemTime,                 0, true),
    (155, KeTestAlertThread,            1, false),
    (156, KeTickCount,                  0, true),
    (157, KeTimeIncrement,              0, true),
    (158, KeWaitForMultipleObjects,     8, false),
    (159, KeWaitForSingleObject,        5, false),

    // IRQL (160-164) — Kf* are __fastcall
    (160, KfRaiseIrql,                  0, false),
    (161, KfLowerIrql,                  0, false),
    (162, KiBugCheckData,               0, true),
    (163, KiUnlockDispatcherDatabase,   0, false), // __fastcall
    (164, LaunchDataPage,               0, true),

    // Memory Manager (165-183)
    (165, MmAllocateContiguousMemory,   1, false),
    (166, MmAllocateContiguousMemoryEx, 5, false),
    (167, MmAllocateSystemMemory,       2, false),
    (168, MmClaimGpuInstanceMemory,     2, false),
    (169, MmCreateKernelStack,          2, false),
    (170, MmDeleteKernelStack,          2, false),
    (171, MmFreeContiguousMemory,       1, false),
    (172, MmFreeSystemMemory,           2, false),
    (173, MmGetPhysicalAddress,         1, false),
    (174, MmIsAddressValid,             1, false),
    (175, MmLockUnlockBufferPages,      3, false),
    (176, MmLockUnlockPhysicalPage,     2, false),
    (177, MmMapIoSpace,                 3, false),
    (178, MmPersistContiguousMemory,    3, false),
    (179, MmQueryAddressProtect,        1, false),
    (180, MmQueryAllocationSize,        1, false),
    (181, MmQueryStatistics,            1, false),
    (182, MmSetAddressProtect,          3, false),
    (183, MmUnmapIoSpace,              2, false),

    // NT Syscalls (184-238)
    (184, NtAllocateVirtualMemory,      5, false),
    (185, NtCancelTimer,                2, false),
    (186, NtClearEvent,                 1, false),
    (187, NtClose,                      1, false),
    (188, NtCreateDirectoryObject,      3, false),
    (189, NtCreateEvent,                4, false),
    (190, NtCreateFile,                 9, false),
    (191, NtCreateIoCompletion,         4, false),
    (192, NtCreateMutant,               3, false),
    (193, NtCreateSemaphore,            4, false),
    (194, NtCreateTimer,                3, false),
    (195, NtDeleteFile,                 1, false),
    (196, NtDeviceIoControlFile,       10, false),
    (197, NtDuplicateObject,            3, false),
    (198, NtFlushBuffersFile,           2, false),
    (199, NtFreeVirtualMemory,          3, false),
    (200, NtFsControlFile,             10, false),
    (201, NtOpenDirectoryObject,        2, false),
    (202, NtOpenFile,                   6, false),
    (203, NtOpenSymbolicLinkObject,     2, false),
    (204, NtProtectVirtualMemory,       4, false),
    (205, NtPulseEvent,                 2, false),
    (206, NtQueueApcThread,             5, false),
    (207, NtQueryDirectoryFile,        10, false),
    (208, NtQueryDirectoryObject,       5, false),
    (209, NtQueryEvent,                 2, false),
    (210, NtQueryFullAttributesFile,    2, false),
    (211, NtQueryInformationFile,       5, false),
    (212, NtQueryIoCompletion,          2, false),
    (213, NtQueryMutant,                2, false),
    (214, NtQuerySemaphore,             2, false),
    (215, NtQuerySymbolicLinkObject,    3, false),
    (216, NtQueryTimer,                 2, false),
    (217, NtQueryVirtualMemory,         5, false),
    (218, NtQueryVolumeInformationFile, 5, false),
    (219, NtReadFile,                   8, false),
    (220, NtReadFileScatter,            8, false),
    (221, NtReleaseMutant,              2, false),
    (222, NtReleaseSemaphore,           3, false),
    (223, NtRemoveIoCompletion,         5, false),
    (224, NtResumeThread,               2, false),
    (225, NtSetEvent,                   2, false),
    (226, NtSetInformationFile,         5, false),
    (227, NtSetIoCompletion,            5, false),
    (228, NtSetSystemTime,              2, false),
    (229, NtSetTimerEx,                 8, false),
    (230, NtSignalAndWaitForSingleObjectEx, 4, false),
    (231, NtSuspendThread,              2, false),
    (232, NtUserIoApcDispatcher,        0, false),
    (233, NtWaitForSingleObject,        3, false),
    (234, NtWaitForSingleObjectEx,      4, false),
    (235, NtWaitForMultipleObjectsEx,   6, false),
    (236, NtWriteFile,                  8, false),
    (237, NtWriteFileGather,            8, false),
    (238, NtYieldExecution,             0, false),

    // Object Manager (239-251)
    (239, ObCreateObject,               4, false),
    (240, ObDirectoryObjectType,        0, true),
    (241, ObInsertObject,               3, false),
    (242, ObMakeTemporaryObject,        1, false),
    (243, ObOpenObjectByName,           4, false),
    (244, ObOpenObjectByPointer,        3, false),
    (245, ObpObjectHandleTable,         0, true),
    (246, ObReferenceObjectByHandle,    4, false),
    (247, ObReferenceObjectByName,      5, false),
    (248, ObReferenceObjectByPointer,   2, false),
    (249, ObSymbolicLinkObjectType,     0, true),
    (250, ObfDereferenceObject,         0, false), // __fastcall
    (251, ObfReferenceObject,           0, false), // __fastcall

    // PHY/PS (252-259)
    // XONLINES PHY thunks used by tested retail titles arrive as no-stack-arg
    // kernel calls here; the link/status return value is supplied by hal.rs.
    (252, PhyGetLinkState,              0, false),
    (253, PhyInitialize,                0, false),
    (254, PsCreateSystemThread,         5, false),
    (255, PsCreateSystemThreadEx,      10, false),
    (256, PsQueryStatistics,            1, false),
    (257, PsSetCreateThreadNotifyRoutine, 1, false),
    (258, PsTerminateSystemThread,      1, false),
    (259, PsThreadObjectType,           0, true),

    // RTL (260-320)
    (260, RtlAnsiStringToUnicodeString, 3, false),
    (261, RtlAppendStringToString,      2, false),
    (262, RtlAppendUnicodeStringToString, 2, false),
    (263, RtlAppendUnicodeToString,     2, false),
    (264, RtlAssert,                    3, false),
    (265, RtlCaptureContext,            1, false),
    (266, RtlCaptureStackBackTrace,     4, false),
    (267, RtlCharToInteger,             3, false),
    (268, RtlCompareMemory,             3, false),
    (269, RtlCompareMemoryUlong,        3, false),
    (270, RtlCompareString,             3, false),
    (271, RtlCompareUnicodeString,      3, false),
    (272, RtlCopyString,                2, false),
    (273, RtlCopyUnicodeString,         2, false),
    (274, RtlCreateUnicodeString,       1, false),
    (275, RtlDowncaseUnicodeChar,       1, false),
    (276, RtlDowncaseUnicodeString,     3, false),
    (277, RtlEnterCriticalSection,      1, false),
    (278, RtlEnterCriticalSectionAndRegion, 1, false),
    (279, RtlEqualString,               3, false),
    (280, RtlEqualUnicodeString,        3, false),
    (281, RtlExtendedIntegerMultiply,   3, false),
    (282, RtlExtendedLargeIntegerDivide, 4, false),
    (283, RtlExtendedMagicDivide,       5, false),
    (284, RtlFillMemory,                3, false),
    (285, RtlFillMemoryUlong,           3, false),
    (286, RtlFreeAnsiString,            1, false),
    (287, RtlFreeUnicodeString,         1, false),
    (288, RtlGetCallersAddress,         2, false),
    (289, RtlInitAnsiString,            2, false),
    (290, RtlInitUnicodeString,         2, false),
    (291, RtlInitializeCriticalSection, 1, false),
    (292, RtlIntegerToChar,             4, false),
    (293, RtlIntegerToUnicodeString,    3, false),
    (294, RtlLeaveCriticalSection,      1, false),
    (295, RtlLeaveCriticalSectionAndRegion, 1, false),
    (296, RtlLowerChar,                 1, false),
    (297, RtlMapGenericMask,            2, false),
    (298, RtlMoveMemory,                3, false),
    (299, RtlMultiByteToUnicodeN,       5, false),
    (300, RtlMultiByteToUnicodeSize,    3, false),
    (301, RtlNtStatusToDosError,        1, false),
    (302, RtlRaiseException,            1, false),
    (303, RtlRaiseStatus,               1, false),
    (304, RtlTimeFieldsToTime,          2, false),
    (305, RtlTimeToTimeFields,          2, false),
    (306, RtlTryEnterCriticalSection,   1, false),
    (307, RtlUlongByteSwap,             1, false),
    (308, RtlUnicodeStringToAnsiString, 3, false),
    (309, RtlUnicodeStringToInteger,    3, false),
    (310, RtlUnicodeToMultiByteN,       5, false),
    (311, RtlUnicodeToMultiByteSize,    3, false),
    (312, RtlUnwind,                    4, false),
    (313, RtlUpcaseUnicodeChar,         1, false),
    (314, RtlUpcaseUnicodeString,       2, false),
    (315, RtlUpcaseUnicodeToMultiByteN, 5, false),
    (316, RtlUpperChar,                 1, false),
    (317, RtlUpperString,               2, false),
    (318, RtlUshortByteSwap,            1, false),
    (319, RtlWalkFrameChain,            3, false),
    (320, RtlZeroMemory,                2, false),

    // Xbox data (321-328)
    (321, XboxEEPROMKey,                0, true),
    (322, XboxHardwareInfo,             0, true),
    (323, XboxHDKey,                    0, true),
    (324, XboxKrnlVersion,              0, true),
    (325, XboxSignatureKey,             0, true),
    (326, XeImageFileName,              0, true),
    (327, XeLoadSection,                1, false),
    (328, XeUnloadSection,              1, false),

    // Port I/O (329-334)
    (329, READ_PORT_BUFFER_UCHAR,       3, false),
    (330, READ_PORT_BUFFER_USHORT,      3, false),
    (331, READ_PORT_BUFFER_ULONG,       3, false),
    (332, WRITE_PORT_BUFFER_UCHAR,      3, false),
    (333, WRITE_PORT_BUFFER_USHORT,     3, false),
    (334, WRITE_PORT_BUFFER_ULONG,      3, false),

    // Crypto (335-351)
    (335, XcSHAInit,                    1, false),
    (336, XcSHAUpdate,                  3, false),
    (337, XcSHAFinal,                   2, false),
    (338, XcRC4Key,                     3, false),
    (339, XcRC4Crypt,                   3, false),
    (340, XcHMAC,                       7, false),
    (341, XcPKEncPublic,                4, false),
    (342, XcPKDecPrivate,               4, false),
    (343, XcPKGetKeyLen,                1, false),
    (344, XcVerifyPKCS1Signature,       4, false),
    (345, XcModExp,                     4, false),
    (346, XcDESKeyParity,               1, false),
    (347, XcKeyTable,                   3, false),
    (348, XcBlockCrypt,                 4, false),
    (349, XcBlockCryptCBC,              5, false),
    (350, XcCryptService,               2, false),
    (351, XcUpdateCrypto,               0, false),

    // Misc (352-366)
    (352, RtlRip,                       3, false),
    (353, XboxLANKey,                   0, true),
    (354, XboxAlternateSignatureKeys,   0, true),
    (355, XePublicKeyData,              0, true),
    (356, HalBootSMCVideoMode,          0, true),
    (357, IdexChannelObject,            0, true),
    (358, HalIsResetOrShutdownPending,  0, false),
    (359, IoMarkIrpMustComplete,        1, false),
    (360, HalInitiateShutdown,          0, false),
    (361, RtlSnprintf,                  0, false), // cdecl variadic
    (362, RtlSprintf,                   0, false), // cdecl variadic
    (363, RtlVsnprintf,                 4, false),
    (364, RtlVsprintf,                  3, false),
    (365, HalEnableSecureTrayEject,     0, false),
    (366, HalWriteSMCScratchRegister,   1, false),

    // Debug (374-378)
    (374, MmDbgAllocateMemory,          2, false),
    (375, MmDbgFreeMemory,             2, false),
    (376, MmDbgQueryAvailablePages,     0, false),
    (377, MmDbgReleaseAddress,          2, false),
    (378, MmDbgWriteCheck,              2, false),
}
