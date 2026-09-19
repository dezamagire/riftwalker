use windows::{
    core::{GUID, HRESULT, Interface, Result},
    Win32::{
        Foundation::HWND,
        System::Com::{
            CoCreateInstance,
            CoInitializeEx,
            CLSCTX_ALL,
            COINIT_APARTMENTTHREADED,
        },
    },
};

const CLSID_VIRTUAL_DESKTOP_MANAGER: GUID = GUID::from_u128(
    0xaa509086_5ca9_4c25_8f95_589d3c07b48a,
);

const IID_VIRTUAL_DESKTOP_MANAGER: GUID = GUID::from_u128(
    0xa5cd92ff_29be_454c_8d04_d82879fb3f1b,
);

const IID_VIRTUAL_DESKTOP_MANAGER_INTERNAL: GUID = GUID::from_u128(
    0x53f5ca0b_158f_4124_900c_057158060b27,
);

const IID_VIRTUAL_DESKTOP: GUID = GUID::from_u128(
    0x3f07f4be_b107_441a_af0f_39d82529072c,
);

const CLSID_IMMERSIVE_SHELL: GUID = GUID::from_u128(
    0xc2f03a33_21f5_47fa_b4bb_156362a2f239,
);

const CLSID_VIRTUAL_DESKTOP_MANAGER_INTERNAL: GUID = GUID::from_u128(
    0xc5e0cdca_7b6e_41b2_9fc4_d93975cc467b,
);

const IID_I_OBJECT_ARRAY: GUID = GUID::from_u128(
    0x92ca9dcd_5622_4bba_a805_5e9f541bd8c9,
);

const IID_APPLICATION_VIEW_COLLECTION: GUID = GUID::from_u128(
    0x1841c6d7_4f9d_42c0_af41_8747538f10e5,
);

const IID_APPLICATION_VIEW: GUID = GUID::from_u128(
    0x372e1d3b_38d3_42e4_a15b_8ab2b178f513,
);

const CLSID_VIRTUAL_DESKTOP_PINNED_APPS: GUID = GUID::from_u128(
    0xb5a399e7_1c87_46b8_88e9_fc5747b171bd,
);

const IID_VIRTUAL_DESKTOP_PINNED_APPS: GUID = GUID::from_u128(
    0x4ce81583_1e4c_4632_a621_07a53543148f,
);

// ============================================================
// IVirtualDesktopManager
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IVirtualDesktopManager(windows::core::IUnknown);

unsafe impl Interface for IVirtualDesktopManager {
    type Vtable = IVirtualDesktopManager_Vtbl;
    const IID: GUID = IID_VIRTUAL_DESKTOP_MANAGER;
}

#[repr(C)]
pub struct IVirtualDesktopManager_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub IsWindowOnCurrentVirtualDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            hwnd: HWND,
            result: *mut i32,
        ) -> HRESULT,

    pub GetWindowDesktopId:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            hwnd: HWND,
            desktop_id: *mut GUID,
        ) -> HRESULT,

    pub MoveWindowToDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            hwnd: HWND,
            desktop_id: *const GUID,
        ) -> HRESULT,
}

// ============================================================
// IServiceProvider
// ============================================================

const IID_I_SERVICE_PROVIDER: GUID = GUID::from_u128(
    0x6d5140c1_7436_11ce_8034_00aa006009fa,
);

#[repr(transparent)]
#[derive(Clone)]
pub struct IServiceProvider(windows::core::IUnknown);

unsafe impl Interface for IServiceProvider {
    type Vtable = IServiceProvider_Vtbl;
    const IID: GUID = IID_I_SERVICE_PROVIDER;
}

#[repr(C)]
pub struct IServiceProvider_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub QueryService:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            service: *const GUID,
            riid: *const GUID,
            object: *mut *mut std::ffi::c_void,
        ) -> HRESULT,
}

impl IServiceProvider {
    unsafe fn query_service<T: Interface>(
        &self,
        service: &GUID,
    ) -> Result<T> {
        let mut object: *mut std::ffi::c_void =
            std::ptr::null_mut();

        ((*self.vtable()).QueryService)(
            self.as_raw(),
            service,
            &T::IID,
            &mut object,
        )
        .ok()?;

        if object.is_null() {
            return Err(
                windows::core::Error::from_hresult(
                    HRESULT(0x80004003u32 as i32),
                ),
            );
        }

        Ok(T::from_raw(object))
    }
}

// ============================================================
// IVirtualDesktopManagerInternal
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IVirtualDesktopManagerInternal(
    windows::core::IUnknown,
);

unsafe impl Interface for IVirtualDesktopManagerInternal {
    type Vtable = IVirtualDesktopManagerInternal_Vtbl;
    const IID: GUID = IID_VIRTUAL_DESKTOP_MANAGER_INTERNAL;
}

#[repr(C)]
pub struct IVirtualDesktopManagerInternal_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub GetCount:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
        ) -> i32,

    pub MoveViewToDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            view: *mut std::ffi::c_void,
            desktop: *mut std::ffi::c_void,
        ) -> HRESULT,

    pub CanViewMoveDesktops:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            view: *mut std::ffi::c_void,
        ) -> i32,

    pub GetCurrentDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            desktop: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub GetDesktops:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            desktops: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub GetAdjacentDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            from: *mut std::ffi::c_void,
            direction: i32,
            desktop: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub SwitchDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            desktop: *mut std::ffi::c_void,
        ) -> HRESULT,

    pub CreateDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            desktop: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub RemoveDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            desktop: *mut std::ffi::c_void,
            fallback: *mut std::ffi::c_void,
        ) -> HRESULT,

    pub FindDesktop:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            desktop_id: *const GUID,
            desktop: *mut *mut std::ffi::c_void,
        ) -> HRESULT,
}

// ============================================================
// IObjectArray
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IObjectArray(windows::core::IUnknown);

unsafe impl Interface for IObjectArray {
    type Vtable = IObjectArray_Vtbl;
    const IID: GUID = IID_I_OBJECT_ARRAY;
}

#[repr(C)]
pub struct IObjectArray_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub GetCount:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            count: *mut u32,
        ) -> HRESULT,

    pub GetAt:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            index: u32,
            iid: *const GUID,
            object: *mut *mut std::ffi::c_void,
        ) -> HRESULT,
}

// ============================================================
// IVirtualDesktop
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IVirtualDesktop(windows::core::IUnknown);

unsafe impl Interface for IVirtualDesktop {
    type Vtable = IVirtualDesktop_Vtbl;
    const IID: GUID = IID_VIRTUAL_DESKTOP;
}

#[repr(C)]
pub struct IVirtualDesktop_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub IsViewVisible:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            view: *mut std::ffi::c_void,
        ) -> i32,

    pub GetId:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            id: *mut GUID,
        ) -> HRESULT,
}

// ============================================================
// IApplicationView
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IApplicationView(windows::core::IUnknown);

unsafe impl Interface for IApplicationView {
    type Vtable = IApplicationView_Vtbl;
    const IID: GUID = IID_APPLICATION_VIEW;
}

#[repr(C)]
pub struct IApplicationView_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    // IInspectable
    pub GetIids:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            iid_count: *mut u32,
            iids: *mut *mut GUID,
        ) -> HRESULT,

    pub GetRuntimeClassName:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            class_name: *mut *mut u16,
        ) -> HRESULT,

    pub GetTrustLevel:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            trust_level: *mut i32,
        ) -> HRESULT,

    // IApplicationView
    pub SetFocus:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
        ) -> HRESULT,

    pub SwitchTo:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
        ) -> HRESULT,

    pub TryInvokeBack:
        *const std::ffi::c_void,

    pub GetThumbnailWindow:
        *const std::ffi::c_void,

    pub GetMonitor:
        *const std::ffi::c_void,

    pub GetVisibility:
        *const std::ffi::c_void,

    pub SetCloak:
        *const std::ffi::c_void,

    pub GetPosition:
        *const std::ffi::c_void,

    pub SetPosition:
        *const std::ffi::c_void,

    pub InsertAfterWindow:
        *const std::ffi::c_void,

    pub GetExtendedFramePosition:
        *const std::ffi::c_void,

    pub GetAppUserModelId:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            app_id: *mut *mut u16,
        ) -> HRESULT,

    pub SetAppUserModelId:
        *const std::ffi::c_void,

    pub IsEqualByAppUserModelId:
        *const std::ffi::c_void,

    pub GetViewState:
        *const std::ffi::c_void,

    pub SetViewState:
        *const std::ffi::c_void,

    pub GetNeediness:
        *const std::ffi::c_void,

    pub GetLastActivationTimestamp:
        *const std::ffi::c_void,

    pub SetLastActivationTimestamp:
        *const std::ffi::c_void,

    pub GetVirtualDesktopId:
        *const std::ffi::c_void,

    pub SetVirtualDesktopId:
        *const std::ffi::c_void,

    pub GetShowInSwitchers:
        *const std::ffi::c_void,

    pub SetShowInSwitchers:
        *const std::ffi::c_void,

    pub GetScaleFactor:
        *const std::ffi::c_void,

    pub CanReceiveInput:
        *const std::ffi::c_void,

    pub GetCompatibilityPolicyType:
        *const std::ffi::c_void,

    pub SetCompatibilityPolicyType:
        *const std::ffi::c_void,

    pub GetSizeConstraints:
        *const std::ffi::c_void,

    pub GetSizeConstraintsForDpi:
        *const std::ffi::c_void,

    pub SetSizeConstraintsForDpi:
        *const std::ffi::c_void,

    pub OnMinSizePreferencesUpdated:
        *const std::ffi::c_void,

    pub ApplyOperation:
        *const std::ffi::c_void,

    pub IsTray:
        *const std::ffi::c_void,

    pub IsInHighZOrderBand:
        *const std::ffi::c_void,

    pub IsSplashScreenPresented:
        *const std::ffi::c_void,

    pub Flash:
        *const std::ffi::c_void,

    pub GetRootSwitchableOwner:
        *const std::ffi::c_void,

    pub EnumerateOwnershipTree:
        *const std::ffi::c_void,

    pub GetEnterpriseId:
        *const std::ffi::c_void,

    pub IsMirrored:
        *const std::ffi::c_void,

    pub Unknown1:
        *const std::ffi::c_void,

    pub Unknown2:
        *const std::ffi::c_void,

    pub Unknown3:
        *const std::ffi::c_void,

    pub Unknown4:
        *const std::ffi::c_void,

    pub Unknown5:
        *const std::ffi::c_void,

    pub Unknown6:
        *const std::ffi::c_void,

    pub Unknown7:
        *const std::ffi::c_void,

    pub Unknown8:
        *const std::ffi::c_void,

    pub Unknown9:
        *const std::ffi::c_void,

    pub Unknown10:
        *const std::ffi::c_void,

    pub Unknown11:
        *const std::ffi::c_void,

    pub Unknown12:
        *const std::ffi::c_void,
}

// ============================================================
// IApplicationViewCollection
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IApplicationViewCollection(
    windows::core::IUnknown,
);

unsafe impl Interface for IApplicationViewCollection {
    type Vtable = IApplicationViewCollection_Vtbl;
    const IID: GUID = IID_APPLICATION_VIEW_COLLECTION;
}

#[repr(C)]
pub struct IApplicationViewCollection_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub GetViews:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            array: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub GetViewsByZOrder:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            array: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub GetViewsByAppUserModelId:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            app_id: *const u16,
            array: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub GetViewForHwnd:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            hwnd: HWND,
            view: *mut *mut std::ffi::c_void,
        ) -> HRESULT,

    pub GetViewForApplication:
        *const std::ffi::c_void,

    pub GetViewForAppUserModelId:
        *const std::ffi::c_void,

    pub GetViewInFocus:
        *const std::ffi::c_void,

    pub Unknown1:
        *const std::ffi::c_void,

    pub RefreshCollection:
        *const std::ffi::c_void,

    pub RegisterForApplicationViewChanges:
        *const std::ffi::c_void,

    pub UnregisterForApplicationViewChanges:
        *const std::ffi::c_void,
}

// ============================================================
// IVirtualDesktopPinnedApps
// ============================================================

#[repr(transparent)]
#[derive(Clone)]
pub struct IVirtualDesktopPinnedApps(
    windows::core::IUnknown,
);

unsafe impl Interface for IVirtualDesktopPinnedApps {
    type Vtable = IVirtualDesktopPinnedApps_Vtbl;
    const IID: GUID = IID_VIRTUAL_DESKTOP_PINNED_APPS;
}

#[repr(C)]
pub struct IVirtualDesktopPinnedApps_Vtbl {
    pub base__: windows::core::IUnknown_Vtbl,

    pub IsAppIdPinned:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            app_id: *const u16,
            pinned: *mut i32,
        ) -> HRESULT,

    pub PinAppID:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            app_id: *const u16,
        ) -> HRESULT,

    pub UnpinAppID:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            app_id: *const u16,
        ) -> HRESULT,

    pub IsViewPinned:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            view: *mut std::ffi::c_void,
            pinned: *mut i32,
        ) -> HRESULT,

    pub PinView:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            view: *mut std::ffi::c_void,
        ) -> HRESULT,

    pub UnpinView:
        unsafe extern "system" fn(
            this: *mut std::ffi::c_void,
            view: *mut std::ffi::c_void,
        ) -> HRESULT,
}

// ============================================================
// Initialization
// ============================================================

pub fn init(_hwnd: HWND) -> Result<()> {
    unsafe {
        CoInitializeEx(
            None,
            COINIT_APARTMENTTHREADED,
        )
        .ok()?;

        println!("Virtual desktop COM initialized");

        let manager: IVirtualDesktopManager =
            CoCreateInstance(
                &CLSID_VIRTUAL_DESKTOP_MANAGER,
                None,
                CLSCTX_ALL,
            )?;

        println!("Virtual Desktop Manager created");

        let mut desktop_id = GUID::zeroed();

        ((*manager.vtable()).GetWindowDesktopId)(
            manager.as_raw(),
            _hwnd,
            &mut desktop_id,
        )
        .ok()?;

        println!(
            "Riftwalker desktop ID: {:?}",
            desktop_id
        );

        let mut is_current = 0i32;

        ((*manager.vtable()).IsWindowOnCurrentVirtualDesktop)(
            manager.as_raw(),
            _hwnd,
            &mut is_current,
        )
        .ok()?;

        println!(
            "Riftwalker on current desktop: {}",
            is_current != 0
        );

        println!("STEP 1: creating immersive shell");

        let shell: IServiceProvider =
            CoCreateInstance(
                &CLSID_IMMERSIVE_SHELL,
                None,
                CLSCTX_ALL,
            )?;

        println!("STEP 2: immersive shell created");

        let internal: IVirtualDesktopManagerInternal =
            shell.query_service(
                &CLSID_VIRTUAL_DESKTOP_MANAGER_INTERNAL,
            )?;

        println!("STEP 3: internal manager acquired");

        println!("STEP 4: internal manager acquired successfully");

        /*

        let shell: IServiceProvider =
            CoCreateInstance(
                &CLSID_IMMERSIVE_SHELL,
                None,
                CLSCTX_ALL,
            )?;

        let internal: IVirtualDesktopManagerInternal =
            shell.query_service(
                &CLSID_VIRTUAL_DESKTOP_MANAGER_INTERNAL,
            )?;

        let count =
            ((*internal.vtable()).GetCount)(
                internal.as_raw(),
            );

        println!(
            "Enumerated {} virtual desktops",
            count
        );

        let mut desktops_ptr =
            std::ptr::null_mut();

        ((*internal.vtable()).GetDesktops)(
            internal.as_raw(),
            &mut desktops_ptr,
        )
        .ok()?;

        let desktops =
            IObjectArray::from_raw(desktops_ptr);

        let mut object_count = 0u32;

        ((*desktops.vtable()).GetCount)(
            desktops.as_raw(),
            &mut object_count,
        )
        .ok()?;

        for index in 0..object_count {
            let mut desktop_ptr =
                std::ptr::null_mut();

            ((*desktops.vtable()).GetAt)(
                desktops.as_raw(),
                index,
                &IID_VIRTUAL_DESKTOP,
                &mut desktop_ptr,
            )
            .ok()?;

            let desktop =
                IVirtualDesktop::from_raw(
                    desktop_ptr,
                );

            let mut id = GUID::zeroed();

            ((*desktop.vtable()).GetId)(
                desktop.as_raw(),
                &mut id,
            )
            .ok()?;

            println!(
                "Desktop {}: {:?}",
                index + 1,
                id
            );
        }

        let mut current_ptr =
            std::ptr::null_mut();

        ((*internal.vtable()).GetCurrentDesktop)(
            internal.as_raw(),
            &mut current_ptr,
        )
        .ok()?;

        let current =
            IVirtualDesktop::from_raw(
                current_ptr,
            );

        let mut current_id = GUID::zeroed();

        ((*current.vtable()).GetId)(
            current.as_raw(),
            &mut current_id,
        )
        .ok()?;

        println!(
            "Current desktop: {} ({:?})",
            desktop_index_from_id(
                &internal,
                current_id,
            ) + 1,
            current_id
        );

        */
        println!("Virtual desktop COM init test complete");

        Ok(())
    }
}

fn desktop_index_from_id(
    internal: &IVirtualDesktopManagerInternal,
    target: GUID,
) -> usize {
    unsafe {
        let count =
            ((*internal.vtable()).GetCount)(
                internal.as_raw(),
            );

        let mut desktops_ptr =
            std::ptr::null_mut();

        if ((*internal.vtable()).GetDesktops)(
            internal.as_raw(),
            &mut desktops_ptr,
        )
        .is_err()
        {
            return 0;
        }

        let desktops =
            IObjectArray::from_raw(
                desktops_ptr,
            );

        let mut object_count = 0u32;

        if ((*desktops.vtable()).GetCount)(
            desktops.as_raw(),
            &mut object_count,
        )
        .is_err()
        {
            return 0;
        }

        for index in 0..object_count.min(count as u32) {
            let mut desktop_ptr =
                std::ptr::null_mut();

            if ((*desktops.vtable()).GetAt)(
                desktops.as_raw(),
                index,
                &IID_VIRTUAL_DESKTOP,
                &mut desktop_ptr,
            )
            .is_err()
            {
                continue;
            }

            let desktop =
                IVirtualDesktop::from_raw(
                    desktop_ptr,
                );

            let mut id = GUID::zeroed();

            if ((*desktop.vtable()).GetId)(
                desktop.as_raw(),
                &mut id,
            )
            .is_err()
            {
                continue;
            }

            if id == target {
                return index as usize;
            }
        }

        0
    }
}

// ============================================================
// Switch desktop
// ============================================================
/*
pub fn switch_desktop(index: usize) -> Result<()> {
    unsafe {
        let shell: IServiceProvider =
            CoCreateInstance(
                &CLSID_IMMERSIVE_SHELL,
                None,
                CLSCTX_ALL,
            )?;

        let internal:
            IVirtualDesktopManagerInternal =
            shell.query_service(
                &CLSID_VIRTUAL_DESKTOP_MANAGER_INTERNAL,
            )?;

        let count =
            ((*internal.vtable()).GetCount)(
                internal.as_raw(),
            );

        if index >= count as usize {
            return Err(
                windows::core::Error::new(
                    HRESULT(0x80070057u32 as i32),
                    "Invalid virtual desktop index",
                ),
            );
        }

        let mut desktops_ptr =
            std::ptr::null_mut();

        ((*internal.vtable()).GetDesktops)(
            internal.as_raw(),
            &mut desktops_ptr,
        )
        .ok()?;

        let desktops =
            IObjectArray::from_raw(
                desktops_ptr,
            );

        let mut desktop_ptr =
            std::ptr::null_mut();

        ((*desktops.vtable()).GetAt)(
            desktops.as_raw(),
            index as u32,
            &IID_VIRTUAL_DESKTOP,
            &mut desktop_ptr,
        )
        .ok()?;

        let desktop =
            IVirtualDesktop::from_raw(
                desktop_ptr,
            );

        ((*internal.vtable()).SwitchDesktop)(
            internal.as_raw(),
            desktop.as_raw(),
        )
        .ok()?;

        Ok(())
    }
}
*/
pub fn switch_desktop(index: usize) -> Result<()> {
    unsafe {
        let shell: IServiceProvider =
            CoCreateInstance(
                &CLSID_IMMERSIVE_SHELL,
                None,
                CLSCTX_ALL,
            )?;

        let internal: IVirtualDesktopManagerInternal =
            shell.query_service(
                &CLSID_VIRTUAL_DESKTOP_MANAGER_INTERNAL,
            )?;

        let mut desktops_ptr =
            std::ptr::null_mut();

        ((*internal.vtable()).GetDesktops)(
            internal.as_raw(),
            &mut desktops_ptr,
        )
        .ok()?;

        let desktops =
            IObjectArray::from_raw(desktops_ptr);

        let mut object_count = 0u32;

        ((*desktops.vtable()).GetCount)(
            desktops.as_raw(),
            &mut object_count,
        )
        .ok()?;

        if index >= object_count as usize {
            return Err(
                windows::core::Error::new(
                    HRESULT(0x80070057u32 as i32),
                    "Invalid virtual desktop index",
                ),
            );
        }

        let mut desktop_ptr =
            std::ptr::null_mut();

        ((*desktops.vtable()).GetAt)(
            desktops.as_raw(),
            index as u32,
            &IID_VIRTUAL_DESKTOP,
            &mut desktop_ptr,
        )
        .ok()?;

        let desktop =
            IVirtualDesktop::from_raw(desktop_ptr);

        println!("SWITCH: calling SwitchDesktop({})", index + 1);

        ((*internal.vtable()).SwitchDesktop)(
            internal.as_raw(),
            desktop.as_raw(),
        )
        .ok()?;

        println!("SWITCH: SwitchDesktop succeeded");

        Ok(())
    }
}
// ============================================================
// Pin current window
// ============================================================

pub fn pin_window(hwnd: HWND) -> Result<()> {
    unsafe {
        let shell: IServiceProvider =
            CoCreateInstance(
                &CLSID_IMMERSIVE_SHELL,
                None,
                CLSCTX_ALL,
            )?;

        let collection:
            IApplicationViewCollection =
            shell.query_service(
                &IID_APPLICATION_VIEW_COLLECTION,
            )?;

        let mut view_ptr =
            std::ptr::null_mut();

        ((*collection.vtable()).GetViewForHwnd)(
            collection.as_raw(),
            hwnd,
            &mut view_ptr,
        )
        .ok()?;

        if view_ptr.is_null() {
            return Err(
                windows::core::Error::from_hresult(
                    HRESULT(0x80004003u32 as i32),
                ),
            );
        }

        let view =
            IApplicationView::from_raw(
                view_ptr,
            );

        let pinned_apps:
            IVirtualDesktopPinnedApps =
            shell.query_service(
                &CLSID_VIRTUAL_DESKTOP_PINNED_APPS,
            )?;

        let mut pinned = 0i32;

        ((*pinned_apps.vtable()).IsViewPinned)(
            pinned_apps.as_raw(),
            view.as_raw(),
            &mut pinned,
        )
        .ok()?;

        if pinned != 0 {
            println!(
                "Riftwalker view is already pinned"
            );

            return Ok(());
        }

        ((*pinned_apps.vtable()).PinView)(
            pinned_apps.as_raw(),
            view.as_raw(),
        )
        .ok()?;

        println!("Riftwalker view pinned");

        Ok(())
    }
}

// ============================================================
// Pin application by AppUserModelId
// ============================================================

pub fn pin_app(hwnd: HWND) -> Result<()> {
    unsafe {
        println!(
            "Getting Riftwalker application view..."
        );

        let shell: IServiceProvider =
            CoCreateInstance(
                &CLSID_IMMERSIVE_SHELL,
                None,
                CLSCTX_ALL,
            )?;

        let collection:
            IApplicationViewCollection =
            shell.query_service(
                &IID_APPLICATION_VIEW_COLLECTION,
            )?;

        println!(
            "Application view collection created"
        );

        let mut view_ptr =
            std::ptr::null_mut();

        ((*collection.vtable()).GetViewForHwnd)(
            collection.as_raw(),
            hwnd,
            &mut view_ptr,
        )
        .ok()?;

        if view_ptr.is_null() {
            return Err(
                windows::core::Error::from_hresult(
                    HRESULT(0x80004003u32 as i32),
                ),
            );
        }

        println!(
            "Riftwalker application view found"
        );

        let view =
            IApplicationView::from_raw(
                view_ptr,
            );

        let mut app_id_ptr =
            std::ptr::null_mut();

        println!(
            "Getting Riftwalker AppUserModelId..."
        );

        ((*view.vtable()).GetAppUserModelId)(
            view.as_raw(),
            &mut app_id_ptr,
        )
        .ok()?;

        if app_id_ptr.is_null() {
            return Err(
                windows::core::Error::from_hresult(
                    HRESULT(0x80004003u32 as i32),
                ),
            );
        }

        let mut length = 0usize;

        while *app_id_ptr.add(length) != 0 {
            length += 1;
        }

        let app_id_slice =
            std::slice::from_raw_parts(
                app_id_ptr,
                length,
            );

        let app_id =
            String::from_utf16_lossy(
                app_id_slice,
            );

        println!(
            "Riftwalker AppUserModelId: {}",
            app_id
        );

        let pinned_apps:
            IVirtualDesktopPinnedApps =
            shell.query_service(
                &CLSID_VIRTUAL_DESKTOP_PINNED_APPS,
            )?;

        let app_id_wide: Vec<u16> =
            app_id
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

        let mut pinned = 0i32;

        ((*pinned_apps.vtable()).IsAppIdPinned)(
            pinned_apps.as_raw(),
            app_id_wide.as_ptr(),
            &mut pinned,
        )
        .ok()?;

        if pinned != 0 {
            println!(
                "Riftwalker application is already pinned"
            );

            return Ok(());
        }

        ((*pinned_apps.vtable()).PinAppID)(
            pinned_apps.as_raw(),
            app_id_wide.as_ptr(),
        )
        .ok()?;

        println!(
            "Riftwalker application pinned to all desktops"
        );

        Ok(())
    }
}