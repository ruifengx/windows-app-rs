#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AddPagesEventArgs(pub ::windows::core::IInspectable);
impl AddPagesEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AddPagesEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::Printing::PrintTaskOptions>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AddPagesEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Printing.AddPagesEventArgs;{a69f3cb3-6b74-5ee8-b034-188098a98c5d})",
    );
}
unsafe impl ::windows::core::Interface for AddPagesEventArgs {
    type Vtable = IAddPagesEventArgsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa69f3cb3_6b74_5ee8_b034_188098a98c5d);
}
impl ::windows::core::RuntimeName for AddPagesEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.AddPagesEventArgs";
}
impl ::core::convert::From<AddPagesEventArgs> for ::windows::core::IUnknown {
    fn from(value: AddPagesEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AddPagesEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AddPagesEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AddPagesEventArgs> for ::windows::core::IInspectable {
    fn from(value: AddPagesEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AddPagesEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AddPagesEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AddPagesEventArgs {}
unsafe impl ::core::marker::Sync for AddPagesEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AddPagesEventHandler(::windows::core::IUnknown);
impl AddPagesEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<AddPagesEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AddPagesEventHandler_box::<F> {
            vtable: &AddPagesEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, AddPagesEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AddPagesEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({ed77566a-bd03-5118-b7c3-d9cea64307dd})",
    );
}
unsafe impl ::windows::core::Interface for AddPagesEventHandler {
    type Vtable = AddPagesEventHandlerVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xed77566a_bd03_5118_b7c3_d9cea64307dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct AddPagesEventHandlerVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct AddPagesEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<AddPagesEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const AddPagesEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<AddPagesEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > AddPagesEventHandler_box<F>
{
    const VTABLE: AddPagesEventHandlerVtbl = AddPagesEventHandlerVtbl(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AddPagesEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < AddPagesEventArgs as :: windows :: core :: Abi > :: Abi as * const < AddPagesEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct GetPreviewPageEventArgs(pub ::windows::core::IInspectable);
impl GetPreviewPageEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            GetPreviewPageEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GetPreviewPageEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Printing.GetPreviewPageEventArgs;{a68fbe17-f577-597f-b3ab-b379447149e6})" ) ;
}
unsafe impl ::windows::core::Interface for GetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa68fbe17_f577_597f_b3ab_b379447149e6);
}
impl ::windows::core::RuntimeName for GetPreviewPageEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.GetPreviewPageEventArgs";
}
impl ::core::convert::From<GetPreviewPageEventArgs> for ::windows::core::IUnknown {
    fn from(value: GetPreviewPageEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GetPreviewPageEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GetPreviewPageEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GetPreviewPageEventArgs> for ::windows::core::IInspectable {
    fn from(value: GetPreviewPageEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GetPreviewPageEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GetPreviewPageEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a GetPreviewPageEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GetPreviewPageEventArgs {}
unsafe impl ::core::marker::Sync for GetPreviewPageEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct GetPreviewPageEventHandler(::windows::core::IUnknown);
impl GetPreviewPageEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<GetPreviewPageEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = GetPreviewPageEventHandler_box::<F> {
            vtable: &GetPreviewPageEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, GetPreviewPageEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GetPreviewPageEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({1c801689-a018-5574-9109-bcef62176da2})",
    );
}
unsafe impl ::windows::core::Interface for GetPreviewPageEventHandler {
    type Vtable = GetPreviewPageEventHandlerVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1c801689_a018_5574_9109_bcef62176da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct GetPreviewPageEventHandlerVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct GetPreviewPageEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<GetPreviewPageEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const GetPreviewPageEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<GetPreviewPageEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > GetPreviewPageEventHandler_box<F>
{
    const VTABLE: GetPreviewPageEventHandlerVtbl = GetPreviewPageEventHandlerVtbl(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<GetPreviewPageEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < GetPreviewPageEventArgs as :: windows :: core :: Abi > :: Abi as * const < GetPreviewPageEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAddPagesEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAddPagesEventArgs {
    type Vtable = IAddPagesEventArgsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa69f3cb3_6b74_5ee8_b034_188098a98c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPagesEventArgsVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGetPreviewPageEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa68fbe17_f577_597f_b3ab_b379447149e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetPreviewPageEventArgsVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPaginateEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPaginateEventArgs {
    type Vtable = IPaginateEventArgsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6499c196_11a9_5ef8_91cb_52fb963bf172);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaginateEventArgsVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintDocument(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintDocument {
    type Vtable = IPrintDocumentVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1e40f1fc_5d33_5fe9_ba3e_954c0d161524);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pagevisual: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: i32,
        r#type: PreviewPageCountType,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pagenumber: i32,
        pagevisual: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintDocumentFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintDocumentFactory {
    type Vtable = IPrintDocumentFactoryVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc4c1bc12_84d1_539c_b416_d7e54c45ab58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentFactoryVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintDocumentStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintDocumentStatics {
    type Vtable = IPrintDocumentStaticsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8975e4bc_8fc8_5e8f_a55a_bf71b9a827b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentStaticsVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PaginateEventArgs(pub ::windows::core::IInspectable);
impl PaginateEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PaginateEventArgs,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::Printing::PrintTaskOptions>(result__)
        }
    }
    pub fn CurrentPreviewPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PaginateEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Printing.PaginateEventArgs;{6499c196-11a9-5ef8-91cb-52fb963bf172})",
    );
}
unsafe impl ::windows::core::Interface for PaginateEventArgs {
    type Vtable = IPaginateEventArgsVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6499c196_11a9_5ef8_91cb_52fb963bf172);
}
impl ::windows::core::RuntimeName for PaginateEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.PaginateEventArgs";
}
impl ::core::convert::From<PaginateEventArgs> for ::windows::core::IUnknown {
    fn from(value: PaginateEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PaginateEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PaginateEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PaginateEventArgs> for ::windows::core::IInspectable {
    fn from(value: PaginateEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PaginateEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PaginateEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PaginateEventArgs {}
unsafe impl ::core::marker::Sync for PaginateEventArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PaginateEventHandler(::windows::core::IUnknown);
impl PaginateEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PaginateEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PaginateEventHandler_box::<F> {
            vtable: &PaginateEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, PaginateEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PaginateEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"delegate({c291876c-343a-5b9b-a36c-8415ba4cd9dd})",
    );
}
unsafe impl ::windows::core::Interface for PaginateEventHandler {
    type Vtable = PaginateEventHandlerVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc291876c_343a_5b9b_a36c_8415ba4cd9dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct PaginateEventHandlerVtbl(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct PaginateEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<PaginateEventArgs>,
        ) -> ::windows::core::Result<()>
        + 'static,
> {
    vtable: *const PaginateEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PaginateEventArgs>,
            ) -> ::windows::core::Result<()>
            + 'static,
    > PaginateEventHandler_box<F>
{
    const VTABLE: PaginateEventHandlerVtbl = PaginateEventHandlerVtbl(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PaginateEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: core :: IInspectable as :: windows :: core :: Abi > :: Abi as * const < :: windows :: core :: IInspectable as :: windows :: core :: DefaultType > :: DefaultType ) , & * ( & e as * const < PaginateEventArgs as :: windows :: core :: Abi > :: Abi as * const < PaginateEventArgs as :: windows :: core :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: PreviewPageCountType = PreviewPageCountType(0i32);
    pub const Intermediate: PreviewPageCountType = PreviewPageCountType(1i32);
}
impl ::core::convert::From<i32> for PreviewPageCountType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PreviewPageCountType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PreviewPageCountType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Printing.PreviewPageCountType;i4)",
    );
}
impl ::windows::core::DefaultType for PreviewPageCountType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PrintDocument(pub ::windows::core::IInspectable);
impl PrintDocument {
    pub fn DocumentSource(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::Printing::IPrintDocumentSource>(result__)
        }
    }
    pub fn Paginate<'a, Param0: ::windows::core::IntoParam<'a, PaginateEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePaginate<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetPreviewPage<
        'a,
        Param0: ::windows::core::IntoParam<'a, GetPreviewPageEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGetPreviewPage<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddPages<'a, Param0: ::windows::core::IntoParam<'a, AddPagesEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAddPages<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        pagevisual: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                pagevisual.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddPagesComplete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn SetPreviewPageCount(
        &self,
        count: i32,
        r#type: PreviewPageCountType,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                count,
                r#type,
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetPreviewPage<'a, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(
        &self,
        pagenumber: i32,
        pagevisual: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                pagenumber,
                pagevisual.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidatePreview(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DocumentSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPrintDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn new() -> ::windows::core::Result<PrintDocument> {
        Self::IPrintDocumentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<PrintDocument>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IPrintDocumentStatics<
        R,
        F: FnOnce(&IPrintDocumentStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintDocument, IPrintDocumentStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPrintDocumentFactory<
        R,
        F: FnOnce(&IPrintDocumentFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintDocument, IPrintDocumentFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Printing.PrintDocument;{1e40f1fc-5d33-5fe9-ba3e-954c0d161524})",
    );
}
unsafe impl ::windows::core::Interface for PrintDocument {
    type Vtable = IPrintDocumentVtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1e40f1fc_5d33_5fe9_ba3e_954c0d161524);
}
impl ::windows::core::RuntimeName for PrintDocument {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.PrintDocument";
}
impl ::core::convert::From<PrintDocument> for ::windows::core::IUnknown {
    fn from(value: PrintDocument) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintDocument> for ::windows::core::IUnknown {
    fn from(value: &PrintDocument) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintDocument> for ::windows::core::IInspectable {
    fn from(value: PrintDocument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintDocument> for ::windows::core::IInspectable {
    fn from(value: &PrintDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintDocument> for super::DependencyObject {
    fn from(value: PrintDocument) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PrintDocument> for super::DependencyObject {
    fn from(value: &PrintDocument) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for PrintDocument {}
unsafe impl ::core::marker::Sync for PrintDocument {}
