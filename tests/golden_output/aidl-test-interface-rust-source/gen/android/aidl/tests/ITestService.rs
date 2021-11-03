#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  ITestService["android.aidl.tests.ITestService"] {
    native: BnTestService(on_transact),
    proxy: BpTestService {
    },
    async: ITestServiceAsync,
  }
}
pub trait ITestService: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.aidl.tests.ITestService" }
  fn UnimplementedMethod(&self, _arg_arg: i32) -> binder::public_api::Result<i32>;
  #[deprecated = "to make sure we have something in system/tools/aidl which does a compile check of deprecated and make sure this is reflected in goldens"]
  fn Deprecated(&self) -> binder::public_api::Result<()>;
  fn TestOneway(&self) -> binder::public_api::Result<()>;
  fn RepeatBoolean(&self, _arg_token: bool) -> binder::public_api::Result<bool>;
  fn RepeatByte(&self, _arg_token: i8) -> binder::public_api::Result<i8>;
  fn RepeatChar(&self, _arg_token: u16) -> binder::public_api::Result<u16>;
  fn RepeatInt(&self, _arg_token: i32) -> binder::public_api::Result<i32>;
  fn RepeatLong(&self, _arg_token: i64) -> binder::public_api::Result<i64>;
  fn RepeatFloat(&self, _arg_token: f32) -> binder::public_api::Result<f32>;
  fn RepeatDouble(&self, _arg_token: f64) -> binder::public_api::Result<f64>;
  fn RepeatString(&self, _arg_token: &str) -> binder::public_api::Result<String>;
  fn RepeatByteEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>;
  fn RepeatIntEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>;
  fn RepeatLongEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>;
  fn ReverseBoolean(&self, _arg_input: &[bool], _arg_repeated: &mut Vec<bool>) -> binder::public_api::Result<Vec<bool>>;
  fn ReverseByte(&self, _arg_input: &[u8], _arg_repeated: &mut Vec<u8>) -> binder::public_api::Result<Vec<u8>>;
  fn ReverseChar(&self, _arg_input: &[u16], _arg_repeated: &mut Vec<u16>) -> binder::public_api::Result<Vec<u16>>;
  fn ReverseInt(&self, _arg_input: &[i32], _arg_repeated: &mut Vec<i32>) -> binder::public_api::Result<Vec<i32>>;
  fn ReverseLong(&self, _arg_input: &[i64], _arg_repeated: &mut Vec<i64>) -> binder::public_api::Result<Vec<i64>>;
  fn ReverseFloat(&self, _arg_input: &[f32], _arg_repeated: &mut Vec<f32>) -> binder::public_api::Result<Vec<f32>>;
  fn ReverseDouble(&self, _arg_input: &[f64], _arg_repeated: &mut Vec<f64>) -> binder::public_api::Result<Vec<f64>>;
  fn ReverseString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>>;
  fn ReverseByteEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>;
  fn ReverseIntEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>;
  fn ReverseLongEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>;
  fn GetOtherTestService(&self, _arg_name: &str) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>;
  fn VerifyName(&self, _arg_service: &binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &str) -> binder::public_api::Result<bool>;
  fn ReverseStringList(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>>;
  fn RepeatParcelFileDescriptor(&self, _arg_read: &binder::parcel::ParcelFileDescriptor) -> binder::public_api::Result<binder::parcel::ParcelFileDescriptor>;
  fn ReverseParcelFileDescriptorArray(&self, _arg_input: &[binder::parcel::ParcelFileDescriptor], _arg_repeated: &mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>>;
  fn ThrowServiceException(&self, _arg_code: i32) -> binder::public_api::Result<()>;
  fn RepeatNullableIntArray(&self, _arg_input: Option<&[i32]>) -> binder::public_api::Result<Option<Vec<i32>>>;
  fn RepeatNullableByteEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>>;
  fn RepeatNullableIntEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>>;
  fn RepeatNullableLongEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>>;
  fn RepeatNullableString(&self, _arg_input: Option<&str>) -> binder::public_api::Result<Option<String>>;
  fn RepeatNullableStringList(&self, _arg_input: Option<&[Option<String>]>) -> binder::public_api::Result<Option<Vec<Option<String>>>>;
  fn RepeatNullableParcelable(&self, _arg_input: Option<&crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>;
  fn RepeatNullableParcelableArray(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>>;
  fn RepeatNullableParcelableList(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>>;
  fn TakesAnIBinder(&self, _arg_input: &binder::SpIBinder) -> binder::public_api::Result<()>;
  fn TakesANullableIBinder(&self, _arg_input: Option<&binder::SpIBinder>) -> binder::public_api::Result<()>;
  fn TakesAnIBinderList(&self, _arg_input: &[binder::SpIBinder]) -> binder::public_api::Result<()>;
  fn TakesANullableIBinderList(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>) -> binder::public_api::Result<()>;
  fn RepeatUtf8CppString(&self, _arg_token: &str) -> binder::public_api::Result<String>;
  fn RepeatNullableUtf8CppString(&self, _arg_token: Option<&str>) -> binder::public_api::Result<Option<String>>;
  fn ReverseUtf8CppString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>>;
  fn ReverseNullableUtf8CppString(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>>;
  fn ReverseUtf8CppStringList(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>>;
  fn GetCallback(&self, _arg_return_null: bool) -> binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>>;
  fn FillOutStructuredParcelable(&self, _arg_parcel: &mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::public_api::Result<()>;
  fn RepeatExtendableParcelable(&self, _arg_ep: &crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::public_api::Result<()>;
  fn ReverseList(&self, _arg_list: &crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList>;
  fn ReverseIBinderArray(&self, _arg_input: &[binder::SpIBinder], _arg_repeated: &mut Vec<Option<binder::SpIBinder>>) -> binder::public_api::Result<Vec<binder::SpIBinder>>;
  fn ReverseNullableIBinderArray(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _arg_repeated: &mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>>;
  fn GetOldNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>>;
  fn GetNewNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>>;
  fn GetCppJavaTests(&self) -> binder::public_api::Result<Option<binder::SpIBinder>>;
  fn getBackendType(&self) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType>;
  fn getDefaultImpl() -> ITestServiceDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: ITestServiceDefaultRef) -> ITestServiceDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait ITestServiceAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.aidl.tests.ITestService" }
  fn UnimplementedMethod<'a>(&'a self, _arg_arg: i32) -> binder::BoxFuture<'a, binder::public_api::Result<i32>>;
  #[deprecated = "to make sure we have something in system/tools/aidl which does a compile check of deprecated and make sure this is reflected in goldens"]
  fn Deprecated<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn TestOneway(&self) -> std::future::Ready<binder::public_api::Result<()>>;
  fn RepeatBoolean<'a>(&'a self, _arg_token: bool) -> binder::BoxFuture<'a, binder::public_api::Result<bool>>;
  fn RepeatByte<'a>(&'a self, _arg_token: i8) -> binder::BoxFuture<'a, binder::public_api::Result<i8>>;
  fn RepeatChar<'a>(&'a self, _arg_token: u16) -> binder::BoxFuture<'a, binder::public_api::Result<u16>>;
  fn RepeatInt<'a>(&'a self, _arg_token: i32) -> binder::BoxFuture<'a, binder::public_api::Result<i32>>;
  fn RepeatLong<'a>(&'a self, _arg_token: i64) -> binder::BoxFuture<'a, binder::public_api::Result<i64>>;
  fn RepeatFloat<'a>(&'a self, _arg_token: f32) -> binder::BoxFuture<'a, binder::public_api::Result<f32>>;
  fn RepeatDouble<'a>(&'a self, _arg_token: f64) -> binder::BoxFuture<'a, binder::public_api::Result<f64>>;
  fn RepeatString<'a>(&'a self, _arg_token: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<String>>;
  fn RepeatByteEnum<'a>(&'a self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>;
  fn RepeatIntEnum<'a>(&'a self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>;
  fn RepeatLongEnum<'a>(&'a self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>;
  fn ReverseBoolean<'a>(&'a self, _arg_input: &'a [bool], _arg_repeated: &'a mut Vec<bool>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<bool>>>;
  fn ReverseByte<'a>(&'a self, _arg_input: &'a [u8], _arg_repeated: &'a mut Vec<u8>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<u8>>>;
  fn ReverseChar<'a>(&'a self, _arg_input: &'a [u16], _arg_repeated: &'a mut Vec<u16>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<u16>>>;
  fn ReverseInt<'a>(&'a self, _arg_input: &'a [i32], _arg_repeated: &'a mut Vec<i32>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<i32>>>;
  fn ReverseLong<'a>(&'a self, _arg_input: &'a [i64], _arg_repeated: &'a mut Vec<i64>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<i64>>>;
  fn ReverseFloat<'a>(&'a self, _arg_input: &'a [f32], _arg_repeated: &'a mut Vec<f32>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<f32>>>;
  fn ReverseDouble<'a>(&'a self, _arg_input: &'a [f64], _arg_repeated: &'a mut Vec<f64>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<f64>>>;
  fn ReverseString<'a>(&'a self, _arg_input: &'a [String], _arg_repeated: &'a mut Vec<String>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<String>>>;
  fn ReverseByteEnum<'a>(&'a self, _arg_input: &'a [crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &'a mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>>;
  fn ReverseIntEnum<'a>(&'a self, _arg_input: &'a [crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &'a mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>>;
  fn ReverseLongEnum<'a>(&'a self, _arg_input: &'a [crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &'a mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>>;
  fn GetOtherTestService<'a>(&'a self, _arg_name: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>>;
  fn VerifyName<'a>(&'a self, _arg_service: &'a binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<bool>>;
  fn ReverseStringList<'a>(&'a self, _arg_input: &'a [String], _arg_repeated: &'a mut Vec<String>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<String>>>;
  fn RepeatParcelFileDescriptor<'a>(&'a self, _arg_read: &'a binder::parcel::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::public_api::Result<binder::parcel::ParcelFileDescriptor>>;
  fn ReverseParcelFileDescriptorArray<'a>(&'a self, _arg_input: &'a [binder::parcel::ParcelFileDescriptor], _arg_repeated: &'a mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>>>;
  fn ThrowServiceException<'a>(&'a self, _arg_code: i32) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn RepeatNullableIntArray<'a>(&'a self, _arg_input: Option<&'a [i32]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<i32>>>>;
  fn RepeatNullableByteEnumArray<'a>(&'a self, _arg_input: Option<&'a [crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>>>;
  fn RepeatNullableIntEnumArray<'a>(&'a self, _arg_input: Option<&'a [crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>>>;
  fn RepeatNullableLongEnumArray<'a>(&'a self, _arg_input: Option<&'a [crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>>>;
  fn RepeatNullableString<'a>(&'a self, _arg_input: Option<&'a str>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<String>>>;
  fn RepeatNullableStringList<'a>(&'a self, _arg_input: Option<&'a [Option<String>]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<String>>>>>;
  fn RepeatNullableParcelable<'a>(&'a self, _arg_input: Option<&'a crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>;
  fn RepeatNullableParcelableArray<'a>(&'a self, _arg_input: Option<&'a [Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>>>;
  fn RepeatNullableParcelableList<'a>(&'a self, _arg_input: Option<&'a [Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>>>;
  fn TakesAnIBinder<'a>(&'a self, _arg_input: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn TakesANullableIBinder<'a>(&'a self, _arg_input: Option<&'a binder::SpIBinder>) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn TakesAnIBinderList<'a>(&'a self, _arg_input: &'a [binder::SpIBinder]) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn TakesANullableIBinderList<'a>(&'a self, _arg_input: Option<&'a [Option<binder::SpIBinder>]>) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn RepeatUtf8CppString<'a>(&'a self, _arg_token: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<String>>;
  fn RepeatNullableUtf8CppString<'a>(&'a self, _arg_token: Option<&'a str>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<String>>>;
  fn ReverseUtf8CppString<'a>(&'a self, _arg_input: &'a [String], _arg_repeated: &'a mut Vec<String>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<String>>>;
  fn ReverseNullableUtf8CppString<'a>(&'a self, _arg_input: Option<&'a [Option<String>]>, _arg_repeated: &'a mut Option<Vec<Option<String>>>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<String>>>>>;
  fn ReverseUtf8CppStringList<'a>(&'a self, _arg_input: Option<&'a [Option<String>]>, _arg_repeated: &'a mut Option<Vec<Option<String>>>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<String>>>>>;
  fn GetCallback<'a>(&'a self, _arg_return_null: bool) -> binder::BoxFuture<'a, binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>>>;
  fn FillOutStructuredParcelable<'a>(&'a self, _arg_parcel: &'a mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn RepeatExtendableParcelable<'a>(&'a self, _arg_ep: &'a crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &'a mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::BoxFuture<'a, binder::public_api::Result<()>>;
  fn ReverseList<'a>(&'a self, _arg_list: &'a crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList>>;
  fn ReverseIBinderArray<'a>(&'a self, _arg_input: &'a [binder::SpIBinder], _arg_repeated: &'a mut Vec<Option<binder::SpIBinder>>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<binder::SpIBinder>>>;
  fn ReverseNullableIBinderArray<'a>(&'a self, _arg_input: Option<&'a [Option<binder::SpIBinder>]>, _arg_repeated: &'a mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>>>;
  fn GetOldNameInterface<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>>>;
  fn GetNewNameInterface<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>>>;
  fn GetCppJavaTests<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<Option<binder::SpIBinder>>>;
  fn getBackendType<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType>>;
}
pub trait ITestServiceDefault: Send + Sync {
  fn UnimplementedMethod(&self, _arg_arg: i32) -> binder::public_api::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn Deprecated(&self) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn TestOneway(&self) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatBoolean(&self, _arg_token: bool) -> binder::public_api::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatByte(&self, _arg_token: i8) -> binder::public_api::Result<i8> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatChar(&self, _arg_token: u16) -> binder::public_api::Result<u16> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatInt(&self, _arg_token: i32) -> binder::public_api::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatLong(&self, _arg_token: i64) -> binder::public_api::Result<i64> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatFloat(&self, _arg_token: f32) -> binder::public_api::Result<f32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatDouble(&self, _arg_token: f64) -> binder::public_api::Result<f64> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatString(&self, _arg_token: &str) -> binder::public_api::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatByteEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatIntEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatLongEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseBoolean(&self, _arg_input: &[bool], _arg_repeated: &mut Vec<bool>) -> binder::public_api::Result<Vec<bool>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseByte(&self, _arg_input: &[u8], _arg_repeated: &mut Vec<u8>) -> binder::public_api::Result<Vec<u8>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseChar(&self, _arg_input: &[u16], _arg_repeated: &mut Vec<u16>) -> binder::public_api::Result<Vec<u16>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseInt(&self, _arg_input: &[i32], _arg_repeated: &mut Vec<i32>) -> binder::public_api::Result<Vec<i32>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseLong(&self, _arg_input: &[i64], _arg_repeated: &mut Vec<i64>) -> binder::public_api::Result<Vec<i64>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseFloat(&self, _arg_input: &[f32], _arg_repeated: &mut Vec<f32>) -> binder::public_api::Result<Vec<f32>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseDouble(&self, _arg_input: &[f64], _arg_repeated: &mut Vec<f64>) -> binder::public_api::Result<Vec<f64>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseByteEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseIntEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseLongEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn GetOtherTestService(&self, _arg_name: &str) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn VerifyName(&self, _arg_service: &binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &str) -> binder::public_api::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseStringList(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatParcelFileDescriptor(&self, _arg_read: &binder::parcel::ParcelFileDescriptor) -> binder::public_api::Result<binder::parcel::ParcelFileDescriptor> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseParcelFileDescriptorArray(&self, _arg_input: &[binder::parcel::ParcelFileDescriptor], _arg_repeated: &mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ThrowServiceException(&self, _arg_code: i32) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableIntArray(&self, _arg_input: Option<&[i32]>) -> binder::public_api::Result<Option<Vec<i32>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableByteEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableIntEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableLongEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableString(&self, _arg_input: Option<&str>) -> binder::public_api::Result<Option<String>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableStringList(&self, _arg_input: Option<&[Option<String>]>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableParcelable(&self, _arg_input: Option<&crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableParcelableArray(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableParcelableList(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn TakesAnIBinder(&self, _arg_input: &binder::SpIBinder) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn TakesANullableIBinder(&self, _arg_input: Option<&binder::SpIBinder>) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn TakesAnIBinderList(&self, _arg_input: &[binder::SpIBinder]) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn TakesANullableIBinderList(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatUtf8CppString(&self, _arg_token: &str) -> binder::public_api::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatNullableUtf8CppString(&self, _arg_token: Option<&str>) -> binder::public_api::Result<Option<String>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseUtf8CppString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseNullableUtf8CppString(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseUtf8CppStringList(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn GetCallback(&self, _arg_return_null: bool) -> binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn FillOutStructuredParcelable(&self, _arg_parcel: &mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn RepeatExtendableParcelable(&self, _arg_ep: &crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::public_api::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseList(&self, _arg_list: &crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseIBinderArray(&self, _arg_input: &[binder::SpIBinder], _arg_repeated: &mut Vec<Option<binder::SpIBinder>>) -> binder::public_api::Result<Vec<binder::SpIBinder>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn ReverseNullableIBinderArray(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _arg_repeated: &mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn GetOldNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn GetNewNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn GetCppJavaTests(&self) -> binder::public_api::Result<Option<binder::SpIBinder>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn getBackendType(&self) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const UnimplementedMethod: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 0;
  pub const Deprecated: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 1;
  pub const TestOneway: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 2;
  pub const RepeatBoolean: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 3;
  pub const RepeatByte: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 4;
  pub const RepeatChar: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 5;
  pub const RepeatInt: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 6;
  pub const RepeatLong: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 7;
  pub const RepeatFloat: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 8;
  pub const RepeatDouble: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 9;
  pub const RepeatString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 10;
  pub const RepeatByteEnum: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 11;
  pub const RepeatIntEnum: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 12;
  pub const RepeatLongEnum: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 13;
  pub const ReverseBoolean: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 14;
  pub const ReverseByte: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 15;
  pub const ReverseChar: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 16;
  pub const ReverseInt: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 17;
  pub const ReverseLong: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 18;
  pub const ReverseFloat: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 19;
  pub const ReverseDouble: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 20;
  pub const ReverseString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 21;
  pub const ReverseByteEnum: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 22;
  pub const ReverseIntEnum: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 23;
  pub const ReverseLongEnum: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 24;
  pub const GetOtherTestService: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 25;
  pub const VerifyName: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 26;
  pub const ReverseStringList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 27;
  pub const RepeatParcelFileDescriptor: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 28;
  pub const ReverseParcelFileDescriptorArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 29;
  pub const ThrowServiceException: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 30;
  pub const RepeatNullableIntArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 31;
  pub const RepeatNullableByteEnumArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 32;
  pub const RepeatNullableIntEnumArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 33;
  pub const RepeatNullableLongEnumArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 34;
  pub const RepeatNullableString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 35;
  pub const RepeatNullableStringList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 36;
  pub const RepeatNullableParcelable: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 37;
  pub const RepeatNullableParcelableArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 38;
  pub const RepeatNullableParcelableList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 39;
  pub const TakesAnIBinder: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 40;
  pub const TakesANullableIBinder: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 41;
  pub const TakesAnIBinderList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 42;
  pub const TakesANullableIBinderList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 43;
  pub const RepeatUtf8CppString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 44;
  pub const RepeatNullableUtf8CppString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 45;
  pub const ReverseUtf8CppString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 46;
  pub const ReverseNullableUtf8CppString: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 47;
  pub const ReverseUtf8CppStringList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 48;
  pub const GetCallback: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 49;
  pub const FillOutStructuredParcelable: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 50;
  pub const RepeatExtendableParcelable: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 51;
  pub const ReverseList: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 52;
  pub const ReverseIBinderArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 53;
  pub const ReverseNullableIBinderArray: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 54;
  pub const GetOldNameInterface: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 55;
  pub const GetNewNameInterface: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 56;
  pub const GetCppJavaTests: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 57;
  pub const getBackendType: binder::TransactionCode = binder::FIRST_CALL_TRANSACTION + 58;
}
pub type ITestServiceDefaultRef = Option<std::sync::Arc<dyn ITestServiceDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<ITestServiceDefaultRef> = std::sync::Mutex::new(None);
}
pub const TEST_CONSTANT: i32 = 42;
pub const TEST_CONSTANT2: i32 = -42;
pub const TEST_CONSTANT3: i32 = 42;
pub const TEST_CONSTANT4: i32 = 4;
pub const TEST_CONSTANT5: i32 = -4;
pub const TEST_CONSTANT6: i32 = 0;
pub const TEST_CONSTANT7: i32 = 0;
pub const TEST_CONSTANT8: i32 = 0;
pub const TEST_CONSTANT9: i32 = 86;
pub const TEST_CONSTANT10: i32 = 165;
pub const TEST_CONSTANT11: i32 = 250;
pub const TEST_CONSTANT12: i32 = -1;
pub const BYTE_TEST_CONSTANT: i8 = 17;
pub const LONG_TEST_CONSTANT: i64 = 1099511627776;
pub const STRING_TEST_CONSTANT: &str = "foo";
pub const STRING_TEST_CONSTANT2: &str = "bar";
pub const STRING_TEST_CONSTANT_UTF8: &str = "baz";
pub const A1: i32 = 1;
pub const A2: i32 = 1;
pub const A3: i32 = 1;
pub const A4: i32 = 1;
pub const A5: i32 = 1;
pub const A6: i32 = 1;
pub const A7: i32 = 1;
pub const A8: i32 = 1;
pub const A9: i32 = 1;
pub const A10: i32 = 1;
pub const A11: i32 = 1;
pub const A12: i32 = 1;
pub const A13: i32 = 1;
pub const A14: i32 = 1;
pub const A15: i32 = 1;
pub const A16: i32 = 1;
pub const A17: i32 = 1;
pub const A18: i32 = 1;
pub const A19: i32 = 1;
pub const A20: i32 = 1;
pub const A21: i32 = 1;
pub const A22: i32 = 1;
pub const A23: i32 = 1;
pub const A24: i32 = 1;
pub const A25: i32 = 1;
pub const A26: i32 = 1;
pub const A27: i32 = 1;
pub const A28: i32 = 1;
pub const A29: i32 = 1;
pub const A30: i32 = 1;
pub const A31: i32 = 1;
pub const A32: i32 = 1;
pub const A33: i32 = 1;
pub const A34: i32 = 1;
pub const A35: i32 = 1;
pub const A36: i32 = 1;
pub const A37: i32 = 1;
pub const A38: i32 = 1;
pub const A39: i32 = 1;
pub const A40: i32 = 1;
pub const A41: i32 = 1;
pub const A42: i32 = 1;
pub const A43: i32 = 1;
pub const A44: i32 = 1;
pub const A45: i32 = 1;
pub const A46: i32 = 1;
pub const A47: i32 = 1;
pub const A48: i32 = 1;
pub const A49: i32 = 1;
pub const A50: i32 = 1;
pub const A51: i32 = 1;
pub const A52: i32 = 1;
pub const A53: i32 = 1;
pub const A54: i32 = 1;
pub const A55: i32 = 1;
pub const A56: i32 = 1;
pub const A57: i32 = 1;
impl BpTestService {
  fn build_parcel_UnimplementedMethod(&self, _arg_arg: i32) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_arg)?;
    Ok(aidl_data_owned)
  }
  fn read_response_UnimplementedMethod(&self, _arg_arg: i32, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.UnimplementedMethod(_arg_arg);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_Deprecated(&self) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    Ok(aidl_data_owned)
  }
  fn read_response_Deprecated(&self, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.Deprecated();
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_TestOneway(&self) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    Ok(aidl_data_owned)
  }
  fn read_response_TestOneway(&self, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.TestOneway();
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    Ok(())
  }
  fn build_parcel_RepeatBoolean(&self, _arg_token: bool) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatBoolean(&self, _arg_token: bool, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatBoolean(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatByte(&self, _arg_token: i8) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatByte(&self, _arg_token: i8, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<i8> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatByte(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i8 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatChar(&self, _arg_token: u16) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatChar(&self, _arg_token: u16, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<u16> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatChar(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: u16 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatInt(&self, _arg_token: i32) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatInt(&self, _arg_token: i32, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatInt(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatLong(&self, _arg_token: i64) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatLong(&self, _arg_token: i64, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<i64> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatLong(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i64 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatFloat(&self, _arg_token: f32) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatFloat(&self, _arg_token: f32, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<f32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatFloat(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: f32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatDouble(&self, _arg_token: f64) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatDouble(&self, _arg_token: f64, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<f64> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatDouble(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: f64 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatString(&self, _arg_token: &str) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatString(&self, _arg_token: &str, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatString(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatByteEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatByteEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatByteEnum(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatIntEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatIntEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatIntEnum(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatLongEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatLongEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatLongEnum(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseBoolean(&self, _arg_input: &[bool], _arg_repeated: &mut Vec<bool>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseBoolean(&self, _arg_input: &[bool], _arg_repeated: &mut Vec<bool>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<bool>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseBoolean(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<bool> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseByte(&self, _arg_input: &[u8], _arg_repeated: &mut Vec<u8>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseByte(&self, _arg_input: &[u8], _arg_repeated: &mut Vec<u8>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<u8>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseByte(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<u8> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseChar(&self, _arg_input: &[u16], _arg_repeated: &mut Vec<u16>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseChar(&self, _arg_input: &[u16], _arg_repeated: &mut Vec<u16>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<u16>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseChar(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<u16> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseInt(&self, _arg_input: &[i32], _arg_repeated: &mut Vec<i32>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseInt(&self, _arg_input: &[i32], _arg_repeated: &mut Vec<i32>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<i32>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseInt(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<i32> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseLong(&self, _arg_input: &[i64], _arg_repeated: &mut Vec<i64>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseLong(&self, _arg_input: &[i64], _arg_repeated: &mut Vec<i64>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<i64>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseLong(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<i64> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseFloat(&self, _arg_input: &[f32], _arg_repeated: &mut Vec<f32>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseFloat(&self, _arg_input: &[f32], _arg_repeated: &mut Vec<f32>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<f32>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseFloat(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<f32> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseDouble(&self, _arg_input: &[f64], _arg_repeated: &mut Vec<f64>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseDouble(&self, _arg_input: &[f64], _arg_repeated: &mut Vec<f64>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<f64>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseDouble(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<f64> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<String>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseString(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<String> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseByteEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseByteEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseByteEnum(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseIntEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseIntEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseIntEnum(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseLongEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseLongEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseLongEnum(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_GetOtherTestService(&self, _arg_name: &str) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_name)?;
    Ok(aidl_data_owned)
  }
  fn read_response_GetOtherTestService(&self, _arg_name: &str, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.GetOtherTestService(_arg_name);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_VerifyName(&self, _arg_service: &binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &str) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_service)?;
    aidl_data.write(_arg_name)?;
    Ok(aidl_data_owned)
  }
  fn read_response_VerifyName(&self, _arg_service: &binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &str, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.VerifyName(_arg_service, _arg_name);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseStringList(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseStringList(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<String>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseStringList(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<String> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatParcelFileDescriptor(&self, _arg_read: &binder::parcel::ParcelFileDescriptor) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_read)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatParcelFileDescriptor(&self, _arg_read: &binder::parcel::ParcelFileDescriptor, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<binder::parcel::ParcelFileDescriptor> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatParcelFileDescriptor(_arg_read);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::parcel::ParcelFileDescriptor = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseParcelFileDescriptorArray(&self, _arg_input: &[binder::parcel::ParcelFileDescriptor], _arg_repeated: &mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseParcelFileDescriptorArray(&self, _arg_input: &[binder::parcel::ParcelFileDescriptor], _arg_repeated: &mut Vec<Option<binder::parcel::ParcelFileDescriptor>>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseParcelFileDescriptorArray(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<binder::parcel::ParcelFileDescriptor> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ThrowServiceException(&self, _arg_code: i32) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_code)?;
    Ok(aidl_data_owned)
  }
  fn read_response_ThrowServiceException(&self, _arg_code: i32, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ThrowServiceException(_arg_code);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_RepeatNullableIntArray(&self, _arg_input: Option<&[i32]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableIntArray(&self, _arg_input: Option<&[i32]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<i32>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableIntArray(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<i32>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableByteEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableByteEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableByteEnumArray(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableIntEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableIntEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableIntEnumArray(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableLongEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableLongEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableLongEnumArray(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableString(&self, _arg_input: Option<&str>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableString(&self, _arg_input: Option<&str>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<String>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableString(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<String> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableStringList(&self, _arg_input: Option<&[Option<String>]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableStringList(&self, _arg_input: Option<&[Option<String>]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableStringList(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<String>>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableParcelable(&self, _arg_input: Option<&crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableParcelable(&self, _arg_input: Option<&crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableParcelable(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableParcelableArray(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableParcelableArray(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableParcelableArray(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableParcelableList(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableParcelableList(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableParcelableList(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_TakesAnIBinder(&self, _arg_input: &binder::SpIBinder) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_TakesAnIBinder(&self, _arg_input: &binder::SpIBinder, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.TakesAnIBinder(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_TakesANullableIBinder(&self, _arg_input: Option<&binder::SpIBinder>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_TakesANullableIBinder(&self, _arg_input: Option<&binder::SpIBinder>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.TakesANullableIBinder(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_TakesAnIBinderList(&self, _arg_input: &[binder::SpIBinder]) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_TakesAnIBinderList(&self, _arg_input: &[binder::SpIBinder], _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.TakesAnIBinderList(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_TakesANullableIBinderList(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_TakesANullableIBinderList(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.TakesANullableIBinderList(_arg_input);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_RepeatUtf8CppString(&self, _arg_token: &str) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatUtf8CppString(&self, _arg_token: &str, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatUtf8CppString(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_RepeatNullableUtf8CppString(&self, _arg_token: Option<&str>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_token)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatNullableUtf8CppString(&self, _arg_token: Option<&str>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<String>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatNullableUtf8CppString(_arg_token);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<String> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseUtf8CppString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseUtf8CppString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<String>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseUtf8CppString(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<String> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseNullableUtf8CppString(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    aidl_data.write_slice_size(_arg_repeated.as_deref())?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseNullableUtf8CppString(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseNullableUtf8CppString(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<String>>> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseUtf8CppStringList(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseUtf8CppStringList(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseUtf8CppStringList(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<String>>> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_GetCallback(&self, _arg_return_null: bool) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_return_null)?;
    Ok(aidl_data_owned)
  }
  fn read_response_GetCallback(&self, _arg_return_null: bool, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.GetCallback(_arg_return_null);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_FillOutStructuredParcelable(&self, _arg_parcel: &mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_parcel)?;
    Ok(aidl_data_owned)
  }
  fn read_response_FillOutStructuredParcelable(&self, _arg_parcel: &mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.FillOutStructuredParcelable(_arg_parcel);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    _aidl_reply.read_onto(_arg_parcel)?;
    Ok(())
  }
  fn build_parcel_RepeatExtendableParcelable(&self, _arg_ep: &crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_ep)?;
    Ok(aidl_data_owned)
  }
  fn read_response_RepeatExtendableParcelable(&self, _arg_ep: &crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.RepeatExtendableParcelable(_arg_ep, _arg_ep2);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    _aidl_reply.read_onto(_arg_ep2)?;
    Ok(())
  }
  fn build_parcel_ReverseList(&self, _arg_list: &crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_list)?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseList(&self, _arg_list: &crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseList(_arg_list);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseIBinderArray(&self, _arg_input: &[binder::SpIBinder], _arg_repeated: &mut Vec<Option<binder::SpIBinder>>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(_arg_input)?;
    aidl_data.write_slice_size(Some(_arg_repeated))?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseIBinderArray(&self, _arg_input: &[binder::SpIBinder], _arg_repeated: &mut Vec<Option<binder::SpIBinder>>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Vec<binder::SpIBinder>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseIBinderArray(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<binder::SpIBinder> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_ReverseNullableIBinderArray(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _arg_repeated: &mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    aidl_data.write(&_arg_input)?;
    aidl_data.write_slice_size(_arg_repeated.as_deref())?;
    Ok(aidl_data_owned)
  }
  fn read_response_ReverseNullableIBinderArray(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _arg_repeated: &mut Option<Vec<Option<binder::SpIBinder>>>, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.ReverseNullableIBinderArray(_arg_input, _arg_repeated);
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<Vec<Option<binder::SpIBinder>>> = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_repeated)?;
    Ok(_aidl_return)
  }
  fn build_parcel_GetOldNameInterface(&self) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    Ok(aidl_data_owned)
  }
  fn read_response_GetOldNameInterface(&self, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.GetOldNameInterface();
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_GetNewNameInterface(&self) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    Ok(aidl_data_owned)
  }
  fn read_response_GetNewNameInterface(&self, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.GetNewNameInterface();
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_GetCppJavaTests(&self) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    Ok(aidl_data_owned)
  }
  fn read_response_GetCppJavaTests(&self, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<Option<binder::SpIBinder>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.GetCppJavaTests();
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<binder::SpIBinder> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBackendType(&self) -> binder::public_api::Result<binder::OwnedParcel> {
    let mut aidl_data_owned = self.binder.prepare_transact()?;
    let mut aidl_data = aidl_data_owned.borrowed();
    aidl_data.mark_sensitive();
    Ok(aidl_data_owned)
  }
  fn read_response_getBackendType(&self, _aidl_reply: binder::Result<binder::OwnedParcel>) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as ITestService>::getDefaultImpl() {
        return _aidl_default_impl.getBackendType();
      }
    }
    let _aidl_reply = _aidl_reply?.into_parcel();
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_4_aidl_5_tests_11_BackendType = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl ITestService for BpTestService {
  fn UnimplementedMethod(&self, _arg_arg: i32) -> binder::public_api::Result<i32> {
    let _aidl_data = self.build_parcel_UnimplementedMethod(_arg_arg)?;
    let _aidl_reply = self.binder.submit_transact(transactions::UnimplementedMethod, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_UnimplementedMethod(_arg_arg, _aidl_reply)
  }
  fn Deprecated(&self) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_Deprecated()?;
    let _aidl_reply = self.binder.submit_transact(transactions::Deprecated, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_Deprecated(_aidl_reply)
  }
  fn TestOneway(&self) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_TestOneway()?;
    let _aidl_reply = self.binder.submit_transact(transactions::TestOneway, _aidl_data, binder::FLAG_ONEWAY | binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_TestOneway(_aidl_reply)
  }
  fn RepeatBoolean(&self, _arg_token: bool) -> binder::public_api::Result<bool> {
    let _aidl_data = self.build_parcel_RepeatBoolean(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatBoolean, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatBoolean(_arg_token, _aidl_reply)
  }
  fn RepeatByte(&self, _arg_token: i8) -> binder::public_api::Result<i8> {
    let _aidl_data = self.build_parcel_RepeatByte(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatByte, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatByte(_arg_token, _aidl_reply)
  }
  fn RepeatChar(&self, _arg_token: u16) -> binder::public_api::Result<u16> {
    let _aidl_data = self.build_parcel_RepeatChar(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatChar, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatChar(_arg_token, _aidl_reply)
  }
  fn RepeatInt(&self, _arg_token: i32) -> binder::public_api::Result<i32> {
    let _aidl_data = self.build_parcel_RepeatInt(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatInt, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatInt(_arg_token, _aidl_reply)
  }
  fn RepeatLong(&self, _arg_token: i64) -> binder::public_api::Result<i64> {
    let _aidl_data = self.build_parcel_RepeatLong(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatLong, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatLong(_arg_token, _aidl_reply)
  }
  fn RepeatFloat(&self, _arg_token: f32) -> binder::public_api::Result<f32> {
    let _aidl_data = self.build_parcel_RepeatFloat(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatFloat, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatFloat(_arg_token, _aidl_reply)
  }
  fn RepeatDouble(&self, _arg_token: f64) -> binder::public_api::Result<f64> {
    let _aidl_data = self.build_parcel_RepeatDouble(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatDouble, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatDouble(_arg_token, _aidl_reply)
  }
  fn RepeatString(&self, _arg_token: &str) -> binder::public_api::Result<String> {
    let _aidl_data = self.build_parcel_RepeatString(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatString(_arg_token, _aidl_reply)
  }
  fn RepeatByteEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> {
    let _aidl_data = self.build_parcel_RepeatByteEnum(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatByteEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatByteEnum(_arg_token, _aidl_reply)
  }
  fn RepeatIntEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> {
    let _aidl_data = self.build_parcel_RepeatIntEnum(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatIntEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatIntEnum(_arg_token, _aidl_reply)
  }
  fn RepeatLongEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> {
    let _aidl_data = self.build_parcel_RepeatLongEnum(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatLongEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatLongEnum(_arg_token, _aidl_reply)
  }
  fn ReverseBoolean(&self, _arg_input: &[bool], _arg_repeated: &mut Vec<bool>) -> binder::public_api::Result<Vec<bool>> {
    let _aidl_data = self.build_parcel_ReverseBoolean(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseBoolean, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseBoolean(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseByte(&self, _arg_input: &[u8], _arg_repeated: &mut Vec<u8>) -> binder::public_api::Result<Vec<u8>> {
    let _aidl_data = self.build_parcel_ReverseByte(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseByte, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseByte(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseChar(&self, _arg_input: &[u16], _arg_repeated: &mut Vec<u16>) -> binder::public_api::Result<Vec<u16>> {
    let _aidl_data = self.build_parcel_ReverseChar(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseChar, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseChar(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseInt(&self, _arg_input: &[i32], _arg_repeated: &mut Vec<i32>) -> binder::public_api::Result<Vec<i32>> {
    let _aidl_data = self.build_parcel_ReverseInt(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseInt, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseInt(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseLong(&self, _arg_input: &[i64], _arg_repeated: &mut Vec<i64>) -> binder::public_api::Result<Vec<i64>> {
    let _aidl_data = self.build_parcel_ReverseLong(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseLong, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseLong(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseFloat(&self, _arg_input: &[f32], _arg_repeated: &mut Vec<f32>) -> binder::public_api::Result<Vec<f32>> {
    let _aidl_data = self.build_parcel_ReverseFloat(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseFloat, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseFloat(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseDouble(&self, _arg_input: &[f64], _arg_repeated: &mut Vec<f64>) -> binder::public_api::Result<Vec<f64>> {
    let _aidl_data = self.build_parcel_ReverseDouble(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseDouble, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseDouble(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> {
    let _aidl_data = self.build_parcel_ReverseString(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseString(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseByteEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> {
    let _aidl_data = self.build_parcel_ReverseByteEnum(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseByteEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseByteEnum(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseIntEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> {
    let _aidl_data = self.build_parcel_ReverseIntEnum(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseIntEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseIntEnum(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseLongEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> {
    let _aidl_data = self.build_parcel_ReverseLongEnum(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseLongEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseLongEnum(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn GetOtherTestService(&self, _arg_name: &str) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>> {
    let _aidl_data = self.build_parcel_GetOtherTestService(_arg_name)?;
    let _aidl_reply = self.binder.submit_transact(transactions::GetOtherTestService, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_GetOtherTestService(_arg_name, _aidl_reply)
  }
  fn VerifyName(&self, _arg_service: &binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &str) -> binder::public_api::Result<bool> {
    let _aidl_data = self.build_parcel_VerifyName(_arg_service, _arg_name)?;
    let _aidl_reply = self.binder.submit_transact(transactions::VerifyName, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_VerifyName(_arg_service, _arg_name, _aidl_reply)
  }
  fn ReverseStringList(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> {
    let _aidl_data = self.build_parcel_ReverseStringList(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseStringList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseStringList(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn RepeatParcelFileDescriptor(&self, _arg_read: &binder::parcel::ParcelFileDescriptor) -> binder::public_api::Result<binder::parcel::ParcelFileDescriptor> {
    let _aidl_data = self.build_parcel_RepeatParcelFileDescriptor(_arg_read)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatParcelFileDescriptor, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatParcelFileDescriptor(_arg_read, _aidl_reply)
  }
  fn ReverseParcelFileDescriptorArray(&self, _arg_input: &[binder::parcel::ParcelFileDescriptor], _arg_repeated: &mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>> {
    let _aidl_data = self.build_parcel_ReverseParcelFileDescriptorArray(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseParcelFileDescriptorArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseParcelFileDescriptorArray(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ThrowServiceException(&self, _arg_code: i32) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_ThrowServiceException(_arg_code)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ThrowServiceException, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ThrowServiceException(_arg_code, _aidl_reply)
  }
  fn RepeatNullableIntArray(&self, _arg_input: Option<&[i32]>) -> binder::public_api::Result<Option<Vec<i32>>> {
    let _aidl_data = self.build_parcel_RepeatNullableIntArray(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableIntArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableIntArray(_arg_input, _aidl_reply)
  }
  fn RepeatNullableByteEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>> {
    let _aidl_data = self.build_parcel_RepeatNullableByteEnumArray(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableByteEnumArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableByteEnumArray(_arg_input, _aidl_reply)
  }
  fn RepeatNullableIntEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>> {
    let _aidl_data = self.build_parcel_RepeatNullableIntEnumArray(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableIntEnumArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableIntEnumArray(_arg_input, _aidl_reply)
  }
  fn RepeatNullableLongEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>> {
    let _aidl_data = self.build_parcel_RepeatNullableLongEnumArray(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableLongEnumArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableLongEnumArray(_arg_input, _aidl_reply)
  }
  fn RepeatNullableString(&self, _arg_input: Option<&str>) -> binder::public_api::Result<Option<String>> {
    let _aidl_data = self.build_parcel_RepeatNullableString(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableString(_arg_input, _aidl_reply)
  }
  fn RepeatNullableStringList(&self, _arg_input: Option<&[Option<String>]>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    let _aidl_data = self.build_parcel_RepeatNullableStringList(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableStringList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableStringList(_arg_input, _aidl_reply)
  }
  fn RepeatNullableParcelable(&self, _arg_input: Option<&crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>> {
    let _aidl_data = self.build_parcel_RepeatNullableParcelable(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableParcelable, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableParcelable(_arg_input, _aidl_reply)
  }
  fn RepeatNullableParcelableArray(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> {
    let _aidl_data = self.build_parcel_RepeatNullableParcelableArray(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableParcelableArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableParcelableArray(_arg_input, _aidl_reply)
  }
  fn RepeatNullableParcelableList(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> {
    let _aidl_data = self.build_parcel_RepeatNullableParcelableList(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableParcelableList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableParcelableList(_arg_input, _aidl_reply)
  }
  fn TakesAnIBinder(&self, _arg_input: &binder::SpIBinder) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_TakesAnIBinder(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::TakesAnIBinder, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_TakesAnIBinder(_arg_input, _aidl_reply)
  }
  fn TakesANullableIBinder(&self, _arg_input: Option<&binder::SpIBinder>) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_TakesANullableIBinder(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::TakesANullableIBinder, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_TakesANullableIBinder(_arg_input, _aidl_reply)
  }
  fn TakesAnIBinderList(&self, _arg_input: &[binder::SpIBinder]) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_TakesAnIBinderList(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::TakesAnIBinderList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_TakesAnIBinderList(_arg_input, _aidl_reply)
  }
  fn TakesANullableIBinderList(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_TakesANullableIBinderList(_arg_input)?;
    let _aidl_reply = self.binder.submit_transact(transactions::TakesANullableIBinderList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_TakesANullableIBinderList(_arg_input, _aidl_reply)
  }
  fn RepeatUtf8CppString(&self, _arg_token: &str) -> binder::public_api::Result<String> {
    let _aidl_data = self.build_parcel_RepeatUtf8CppString(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatUtf8CppString(_arg_token, _aidl_reply)
  }
  fn RepeatNullableUtf8CppString(&self, _arg_token: Option<&str>) -> binder::public_api::Result<Option<String>> {
    let _aidl_data = self.build_parcel_RepeatNullableUtf8CppString(_arg_token)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatNullableUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatNullableUtf8CppString(_arg_token, _aidl_reply)
  }
  fn ReverseUtf8CppString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> {
    let _aidl_data = self.build_parcel_ReverseUtf8CppString(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseUtf8CppString(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseNullableUtf8CppString(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    let _aidl_data = self.build_parcel_ReverseNullableUtf8CppString(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseNullableUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseNullableUtf8CppString(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseUtf8CppStringList(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>> {
    let _aidl_data = self.build_parcel_ReverseUtf8CppStringList(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseUtf8CppStringList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseUtf8CppStringList(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn GetCallback(&self, _arg_return_null: bool) -> binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>> {
    let _aidl_data = self.build_parcel_GetCallback(_arg_return_null)?;
    let _aidl_reply = self.binder.submit_transact(transactions::GetCallback, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_GetCallback(_arg_return_null, _aidl_reply)
  }
  fn FillOutStructuredParcelable(&self, _arg_parcel: &mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_FillOutStructuredParcelable(_arg_parcel)?;
    let _aidl_reply = self.binder.submit_transact(transactions::FillOutStructuredParcelable, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_FillOutStructuredParcelable(_arg_parcel, _aidl_reply)
  }
  fn RepeatExtendableParcelable(&self, _arg_ep: &crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::public_api::Result<()> {
    let _aidl_data = self.build_parcel_RepeatExtendableParcelable(_arg_ep, _arg_ep2)?;
    let _aidl_reply = self.binder.submit_transact(transactions::RepeatExtendableParcelable, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_RepeatExtendableParcelable(_arg_ep, _arg_ep2, _aidl_reply)
  }
  fn ReverseList(&self, _arg_list: &crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList> {
    let _aidl_data = self.build_parcel_ReverseList(_arg_list)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseList(_arg_list, _aidl_reply)
  }
  fn ReverseIBinderArray(&self, _arg_input: &[binder::SpIBinder], _arg_repeated: &mut Vec<Option<binder::SpIBinder>>) -> binder::public_api::Result<Vec<binder::SpIBinder>> {
    let _aidl_data = self.build_parcel_ReverseIBinderArray(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseIBinderArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseIBinderArray(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn ReverseNullableIBinderArray(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _arg_repeated: &mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>> {
    let _aidl_data = self.build_parcel_ReverseNullableIBinderArray(_arg_input, _arg_repeated)?;
    let _aidl_reply = self.binder.submit_transact(transactions::ReverseNullableIBinderArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_ReverseNullableIBinderArray(_arg_input, _arg_repeated, _aidl_reply)
  }
  fn GetOldNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>> {
    let _aidl_data = self.build_parcel_GetOldNameInterface()?;
    let _aidl_reply = self.binder.submit_transact(transactions::GetOldNameInterface, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_GetOldNameInterface(_aidl_reply)
  }
  fn GetNewNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>> {
    let _aidl_data = self.build_parcel_GetNewNameInterface()?;
    let _aidl_reply = self.binder.submit_transact(transactions::GetNewNameInterface, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_GetNewNameInterface(_aidl_reply)
  }
  fn GetCppJavaTests(&self) -> binder::public_api::Result<Option<binder::SpIBinder>> {
    let _aidl_data = self.build_parcel_GetCppJavaTests()?;
    let _aidl_reply = self.binder.submit_transact(transactions::GetCppJavaTests, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_GetCppJavaTests(_aidl_reply)
  }
  fn getBackendType(&self) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType> {
    let _aidl_data = self.build_parcel_getBackendType()?;
    let _aidl_reply = self.binder.submit_transact(transactions::getBackendType, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    self.read_response_getBackendType(_aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> ITestServiceAsync<P> for BpTestService {
  fn UnimplementedMethod<'a>(&'a self, _arg_arg: i32) -> binder::BoxFuture<'a, binder::public_api::Result<i32>> {
    let _aidl_data = match self.build_parcel_UnimplementedMethod(_arg_arg) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::UnimplementedMethod, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_UnimplementedMethod(_arg_arg, _aidl_reply)
      }
    )
  }
  fn Deprecated<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_Deprecated() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::Deprecated, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_Deprecated(_aidl_reply)
      }
    )
  }
  fn TestOneway(&self) -> std::future::Ready<binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_TestOneway() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::TestOneway, _aidl_data, binder::FLAG_ONEWAY | binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_TestOneway(_aidl_reply))
  }
  fn RepeatBoolean<'a>(&'a self, _arg_token: bool) -> binder::BoxFuture<'a, binder::public_api::Result<bool>> {
    let _aidl_data = match self.build_parcel_RepeatBoolean(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatBoolean, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatBoolean(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatByte<'a>(&'a self, _arg_token: i8) -> binder::BoxFuture<'a, binder::public_api::Result<i8>> {
    let _aidl_data = match self.build_parcel_RepeatByte(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatByte, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatByte(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatChar<'a>(&'a self, _arg_token: u16) -> binder::BoxFuture<'a, binder::public_api::Result<u16>> {
    let _aidl_data = match self.build_parcel_RepeatChar(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatChar, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatChar(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatInt<'a>(&'a self, _arg_token: i32) -> binder::BoxFuture<'a, binder::public_api::Result<i32>> {
    let _aidl_data = match self.build_parcel_RepeatInt(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatInt, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatInt(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatLong<'a>(&'a self, _arg_token: i64) -> binder::BoxFuture<'a, binder::public_api::Result<i64>> {
    let _aidl_data = match self.build_parcel_RepeatLong(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatLong, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatLong(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatFloat<'a>(&'a self, _arg_token: f32) -> binder::BoxFuture<'a, binder::public_api::Result<f32>> {
    let _aidl_data = match self.build_parcel_RepeatFloat(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatFloat, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatFloat(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatDouble<'a>(&'a self, _arg_token: f64) -> binder::BoxFuture<'a, binder::public_api::Result<f64>> {
    let _aidl_data = match self.build_parcel_RepeatDouble(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatDouble, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatDouble(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatString<'a>(&'a self, _arg_token: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<String>> {
    let _aidl_data = match self.build_parcel_RepeatString(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatString(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatByteEnum<'a>(&'a self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> {
    let _aidl_data = match self.build_parcel_RepeatByteEnum(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatByteEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatByteEnum(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatIntEnum<'a>(&'a self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> {
    let _aidl_data = match self.build_parcel_RepeatIntEnum(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatIntEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatIntEnum(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatLongEnum<'a>(&'a self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> {
    let _aidl_data = match self.build_parcel_RepeatLongEnum(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatLongEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatLongEnum(_arg_token, _aidl_reply)
      }
    )
  }
  fn ReverseBoolean<'a>(&'a self, _arg_input: &'a [bool], _arg_repeated: &'a mut Vec<bool>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<bool>>> {
    let _aidl_data = match self.build_parcel_ReverseBoolean(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseBoolean, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseBoolean(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseByte<'a>(&'a self, _arg_input: &'a [u8], _arg_repeated: &'a mut Vec<u8>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<u8>>> {
    let _aidl_data = match self.build_parcel_ReverseByte(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseByte, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseByte(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseChar<'a>(&'a self, _arg_input: &'a [u16], _arg_repeated: &'a mut Vec<u16>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<u16>>> {
    let _aidl_data = match self.build_parcel_ReverseChar(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseChar, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseChar(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseInt<'a>(&'a self, _arg_input: &'a [i32], _arg_repeated: &'a mut Vec<i32>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<i32>>> {
    let _aidl_data = match self.build_parcel_ReverseInt(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseInt, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseInt(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseLong<'a>(&'a self, _arg_input: &'a [i64], _arg_repeated: &'a mut Vec<i64>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<i64>>> {
    let _aidl_data = match self.build_parcel_ReverseLong(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseLong, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseLong(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseFloat<'a>(&'a self, _arg_input: &'a [f32], _arg_repeated: &'a mut Vec<f32>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<f32>>> {
    let _aidl_data = match self.build_parcel_ReverseFloat(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseFloat, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseFloat(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseDouble<'a>(&'a self, _arg_input: &'a [f64], _arg_repeated: &'a mut Vec<f64>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<f64>>> {
    let _aidl_data = match self.build_parcel_ReverseDouble(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseDouble, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseDouble(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseString<'a>(&'a self, _arg_input: &'a [String], _arg_repeated: &'a mut Vec<String>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<String>>> {
    let _aidl_data = match self.build_parcel_ReverseString(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseString(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseByteEnum<'a>(&'a self, _arg_input: &'a [crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &'a mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>> {
    let _aidl_data = match self.build_parcel_ReverseByteEnum(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseByteEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseByteEnum(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseIntEnum<'a>(&'a self, _arg_input: &'a [crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &'a mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>> {
    let _aidl_data = match self.build_parcel_ReverseIntEnum(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseIntEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseIntEnum(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseLongEnum<'a>(&'a self, _arg_input: &'a [crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &'a mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>> {
    let _aidl_data = match self.build_parcel_ReverseLongEnum(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseLongEnum, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseLongEnum(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn GetOtherTestService<'a>(&'a self, _arg_name: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>> {
    let _aidl_data = match self.build_parcel_GetOtherTestService(_arg_name) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::GetOtherTestService, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_GetOtherTestService(_arg_name, _aidl_reply)
      }
    )
  }
  fn VerifyName<'a>(&'a self, _arg_service: &'a binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<bool>> {
    let _aidl_data = match self.build_parcel_VerifyName(_arg_service, _arg_name) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::VerifyName, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_VerifyName(_arg_service, _arg_name, _aidl_reply)
      }
    )
  }
  fn ReverseStringList<'a>(&'a self, _arg_input: &'a [String], _arg_repeated: &'a mut Vec<String>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<String>>> {
    let _aidl_data = match self.build_parcel_ReverseStringList(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseStringList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseStringList(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn RepeatParcelFileDescriptor<'a>(&'a self, _arg_read: &'a binder::parcel::ParcelFileDescriptor) -> binder::BoxFuture<'a, binder::public_api::Result<binder::parcel::ParcelFileDescriptor>> {
    let _aidl_data = match self.build_parcel_RepeatParcelFileDescriptor(_arg_read) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatParcelFileDescriptor, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatParcelFileDescriptor(_arg_read, _aidl_reply)
      }
    )
  }
  fn ReverseParcelFileDescriptorArray<'a>(&'a self, _arg_input: &'a [binder::parcel::ParcelFileDescriptor], _arg_repeated: &'a mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>>> {
    let _aidl_data = match self.build_parcel_ReverseParcelFileDescriptorArray(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseParcelFileDescriptorArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseParcelFileDescriptorArray(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ThrowServiceException<'a>(&'a self, _arg_code: i32) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_ThrowServiceException(_arg_code) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ThrowServiceException, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ThrowServiceException(_arg_code, _aidl_reply)
      }
    )
  }
  fn RepeatNullableIntArray<'a>(&'a self, _arg_input: Option<&'a [i32]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<i32>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableIntArray(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableIntArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableIntArray(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableByteEnumArray<'a>(&'a self, _arg_input: Option<&'a [crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableByteEnumArray(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableByteEnumArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableByteEnumArray(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableIntEnumArray<'a>(&'a self, _arg_input: Option<&'a [crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableIntEnumArray(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableIntEnumArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableIntEnumArray(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableLongEnumArray<'a>(&'a self, _arg_input: Option<&'a [crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableLongEnumArray(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableLongEnumArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableLongEnumArray(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableString<'a>(&'a self, _arg_input: Option<&'a str>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<String>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableString(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableString(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableStringList<'a>(&'a self, _arg_input: Option<&'a [Option<String>]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<String>>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableStringList(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableStringList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableStringList(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableParcelable<'a>(&'a self, _arg_input: Option<&'a crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableParcelable(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableParcelable, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableParcelable(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableParcelableArray<'a>(&'a self, _arg_input: Option<&'a [Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableParcelableArray(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableParcelableArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableParcelableArray(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatNullableParcelableList<'a>(&'a self, _arg_input: Option<&'a [Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableParcelableList(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableParcelableList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableParcelableList(_arg_input, _aidl_reply)
      }
    )
  }
  fn TakesAnIBinder<'a>(&'a self, _arg_input: &'a binder::SpIBinder) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_TakesAnIBinder(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::TakesAnIBinder, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_TakesAnIBinder(_arg_input, _aidl_reply)
      }
    )
  }
  fn TakesANullableIBinder<'a>(&'a self, _arg_input: Option<&'a binder::SpIBinder>) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_TakesANullableIBinder(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::TakesANullableIBinder, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_TakesANullableIBinder(_arg_input, _aidl_reply)
      }
    )
  }
  fn TakesAnIBinderList<'a>(&'a self, _arg_input: &'a [binder::SpIBinder]) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_TakesAnIBinderList(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::TakesAnIBinderList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_TakesAnIBinderList(_arg_input, _aidl_reply)
      }
    )
  }
  fn TakesANullableIBinderList<'a>(&'a self, _arg_input: Option<&'a [Option<binder::SpIBinder>]>) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_TakesANullableIBinderList(_arg_input) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::TakesANullableIBinderList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_TakesANullableIBinderList(_arg_input, _aidl_reply)
      }
    )
  }
  fn RepeatUtf8CppString<'a>(&'a self, _arg_token: &'a str) -> binder::BoxFuture<'a, binder::public_api::Result<String>> {
    let _aidl_data = match self.build_parcel_RepeatUtf8CppString(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatUtf8CppString(_arg_token, _aidl_reply)
      }
    )
  }
  fn RepeatNullableUtf8CppString<'a>(&'a self, _arg_token: Option<&'a str>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<String>>> {
    let _aidl_data = match self.build_parcel_RepeatNullableUtf8CppString(_arg_token) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatNullableUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatNullableUtf8CppString(_arg_token, _aidl_reply)
      }
    )
  }
  fn ReverseUtf8CppString<'a>(&'a self, _arg_input: &'a [String], _arg_repeated: &'a mut Vec<String>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<String>>> {
    let _aidl_data = match self.build_parcel_ReverseUtf8CppString(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseUtf8CppString(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseNullableUtf8CppString<'a>(&'a self, _arg_input: Option<&'a [Option<String>]>, _arg_repeated: &'a mut Option<Vec<Option<String>>>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<String>>>>> {
    let _aidl_data = match self.build_parcel_ReverseNullableUtf8CppString(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseNullableUtf8CppString, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseNullableUtf8CppString(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseUtf8CppStringList<'a>(&'a self, _arg_input: Option<&'a [Option<String>]>, _arg_repeated: &'a mut Option<Vec<Option<String>>>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<String>>>>> {
    let _aidl_data = match self.build_parcel_ReverseUtf8CppStringList(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseUtf8CppStringList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseUtf8CppStringList(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn GetCallback<'a>(&'a self, _arg_return_null: bool) -> binder::BoxFuture<'a, binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>>> {
    let _aidl_data = match self.build_parcel_GetCallback(_arg_return_null) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::GetCallback, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_GetCallback(_arg_return_null, _aidl_reply)
      }
    )
  }
  fn FillOutStructuredParcelable<'a>(&'a self, _arg_parcel: &'a mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_FillOutStructuredParcelable(_arg_parcel) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::FillOutStructuredParcelable, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_FillOutStructuredParcelable(_arg_parcel, _aidl_reply)
      }
    )
  }
  fn RepeatExtendableParcelable<'a>(&'a self, _arg_ep: &'a crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &'a mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::BoxFuture<'a, binder::public_api::Result<()>> {
    let _aidl_data = match self.build_parcel_RepeatExtendableParcelable(_arg_ep, _arg_ep2) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::RepeatExtendableParcelable, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_RepeatExtendableParcelable(_arg_ep, _arg_ep2, _aidl_reply)
      }
    )
  }
  fn ReverseList<'a>(&'a self, _arg_list: &'a crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList>> {
    let _aidl_data = match self.build_parcel_ReverseList(_arg_list) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseList, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseList(_arg_list, _aidl_reply)
      }
    )
  }
  fn ReverseIBinderArray<'a>(&'a self, _arg_input: &'a [binder::SpIBinder], _arg_repeated: &'a mut Vec<Option<binder::SpIBinder>>) -> binder::BoxFuture<'a, binder::public_api::Result<Vec<binder::SpIBinder>>> {
    let _aidl_data = match self.build_parcel_ReverseIBinderArray(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseIBinderArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseIBinderArray(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn ReverseNullableIBinderArray<'a>(&'a self, _arg_input: Option<&'a [Option<binder::SpIBinder>]>, _arg_repeated: &'a mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::BoxFuture<'a, binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>>> {
    let _aidl_data = match self.build_parcel_ReverseNullableIBinderArray(_arg_input, _arg_repeated) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::ReverseNullableIBinderArray, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_ReverseNullableIBinderArray(_arg_input, _arg_repeated, _aidl_reply)
      }
    )
  }
  fn GetOldNameInterface<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>>> {
    let _aidl_data = match self.build_parcel_GetOldNameInterface() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::GetOldNameInterface, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_GetOldNameInterface(_aidl_reply)
      }
    )
  }
  fn GetNewNameInterface<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>>> {
    let _aidl_data = match self.build_parcel_GetNewNameInterface() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::GetNewNameInterface, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_GetNewNameInterface(_aidl_reply)
      }
    )
  }
  fn GetCppJavaTests<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<Option<binder::SpIBinder>>> {
    let _aidl_data = match self.build_parcel_GetCppJavaTests() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::GetCppJavaTests, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_GetCppJavaTests(_aidl_reply)
      }
    )
  }
  fn getBackendType<'a>(&'a self) -> binder::BoxFuture<'a, binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType>> {
    let _aidl_data = match self.build_parcel_getBackendType() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getBackendType, _aidl_data, binder::FLAG_CLEAR_BUF | binder::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBackendType(_aidl_reply)
      }
    )
  }
}
impl ITestService for binder::Binder<BnTestService> {
  fn UnimplementedMethod(&self, _arg_arg: i32) -> binder::public_api::Result<i32> { self.0.UnimplementedMethod(_arg_arg) }
  fn Deprecated(&self) -> binder::public_api::Result<()> { self.0.Deprecated() }
  fn TestOneway(&self) -> binder::public_api::Result<()> { self.0.TestOneway() }
  fn RepeatBoolean(&self, _arg_token: bool) -> binder::public_api::Result<bool> { self.0.RepeatBoolean(_arg_token) }
  fn RepeatByte(&self, _arg_token: i8) -> binder::public_api::Result<i8> { self.0.RepeatByte(_arg_token) }
  fn RepeatChar(&self, _arg_token: u16) -> binder::public_api::Result<u16> { self.0.RepeatChar(_arg_token) }
  fn RepeatInt(&self, _arg_token: i32) -> binder::public_api::Result<i32> { self.0.RepeatInt(_arg_token) }
  fn RepeatLong(&self, _arg_token: i64) -> binder::public_api::Result<i64> { self.0.RepeatLong(_arg_token) }
  fn RepeatFloat(&self, _arg_token: f32) -> binder::public_api::Result<f32> { self.0.RepeatFloat(_arg_token) }
  fn RepeatDouble(&self, _arg_token: f64) -> binder::public_api::Result<f64> { self.0.RepeatDouble(_arg_token) }
  fn RepeatString(&self, _arg_token: &str) -> binder::public_api::Result<String> { self.0.RepeatString(_arg_token) }
  fn RepeatByteEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> { self.0.RepeatByteEnum(_arg_token) }
  fn RepeatIntEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> { self.0.RepeatIntEnum(_arg_token) }
  fn RepeatLongEnum(&self, _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> { self.0.RepeatLongEnum(_arg_token) }
  fn ReverseBoolean(&self, _arg_input: &[bool], _arg_repeated: &mut Vec<bool>) -> binder::public_api::Result<Vec<bool>> { self.0.ReverseBoolean(_arg_input, _arg_repeated) }
  fn ReverseByte(&self, _arg_input: &[u8], _arg_repeated: &mut Vec<u8>) -> binder::public_api::Result<Vec<u8>> { self.0.ReverseByte(_arg_input, _arg_repeated) }
  fn ReverseChar(&self, _arg_input: &[u16], _arg_repeated: &mut Vec<u16>) -> binder::public_api::Result<Vec<u16>> { self.0.ReverseChar(_arg_input, _arg_repeated) }
  fn ReverseInt(&self, _arg_input: &[i32], _arg_repeated: &mut Vec<i32>) -> binder::public_api::Result<Vec<i32>> { self.0.ReverseInt(_arg_input, _arg_repeated) }
  fn ReverseLong(&self, _arg_input: &[i64], _arg_repeated: &mut Vec<i64>) -> binder::public_api::Result<Vec<i64>> { self.0.ReverseLong(_arg_input, _arg_repeated) }
  fn ReverseFloat(&self, _arg_input: &[f32], _arg_repeated: &mut Vec<f32>) -> binder::public_api::Result<Vec<f32>> { self.0.ReverseFloat(_arg_input, _arg_repeated) }
  fn ReverseDouble(&self, _arg_input: &[f64], _arg_repeated: &mut Vec<f64>) -> binder::public_api::Result<Vec<f64>> { self.0.ReverseDouble(_arg_input, _arg_repeated) }
  fn ReverseString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> { self.0.ReverseString(_arg_input, _arg_repeated) }
  fn ReverseByteEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> { self.0.ReverseByteEnum(_arg_input, _arg_repeated) }
  fn ReverseIntEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> { self.0.ReverseIntEnum(_arg_input, _arg_repeated) }
  fn ReverseLongEnum(&self, _arg_input: &[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum], _arg_repeated: &mut Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>) -> binder::public_api::Result<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> { self.0.ReverseLongEnum(_arg_input, _arg_repeated) }
  fn GetOtherTestService(&self, _arg_name: &str) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>> { self.0.GetOtherTestService(_arg_name) }
  fn VerifyName(&self, _arg_service: &binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>, _arg_name: &str) -> binder::public_api::Result<bool> { self.0.VerifyName(_arg_service, _arg_name) }
  fn ReverseStringList(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> { self.0.ReverseStringList(_arg_input, _arg_repeated) }
  fn RepeatParcelFileDescriptor(&self, _arg_read: &binder::parcel::ParcelFileDescriptor) -> binder::public_api::Result<binder::parcel::ParcelFileDescriptor> { self.0.RepeatParcelFileDescriptor(_arg_read) }
  fn ReverseParcelFileDescriptorArray(&self, _arg_input: &[binder::parcel::ParcelFileDescriptor], _arg_repeated: &mut Vec<Option<binder::parcel::ParcelFileDescriptor>>) -> binder::public_api::Result<Vec<binder::parcel::ParcelFileDescriptor>> { self.0.ReverseParcelFileDescriptorArray(_arg_input, _arg_repeated) }
  fn ThrowServiceException(&self, _arg_code: i32) -> binder::public_api::Result<()> { self.0.ThrowServiceException(_arg_code) }
  fn RepeatNullableIntArray(&self, _arg_input: Option<&[i32]>) -> binder::public_api::Result<Option<Vec<i32>>> { self.0.RepeatNullableIntArray(_arg_input) }
  fn RepeatNullableByteEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>>> { self.0.RepeatNullableByteEnumArray(_arg_input) }
  fn RepeatNullableIntEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>>> { self.0.RepeatNullableIntEnumArray(_arg_input) }
  fn RepeatNullableLongEnumArray(&self, _arg_input: Option<&[crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum]>) -> binder::public_api::Result<Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>>> { self.0.RepeatNullableLongEnumArray(_arg_input) }
  fn RepeatNullableString(&self, _arg_input: Option<&str>) -> binder::public_api::Result<Option<String>> { self.0.RepeatNullableString(_arg_input) }
  fn RepeatNullableStringList(&self, _arg_input: Option<&[Option<String>]>) -> binder::public_api::Result<Option<Vec<Option<String>>>> { self.0.RepeatNullableStringList(_arg_input) }
  fn RepeatNullableParcelable(&self, _arg_input: Option<&crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>) -> binder::public_api::Result<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>> { self.0.RepeatNullableParcelable(_arg_input) }
  fn RepeatNullableParcelableArray(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> { self.0.RepeatNullableParcelableArray(_arg_input) }
  fn RepeatNullableParcelableList(&self, _arg_input: Option<&[Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>]>) -> binder::public_api::Result<Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>> { self.0.RepeatNullableParcelableList(_arg_input) }
  fn TakesAnIBinder(&self, _arg_input: &binder::SpIBinder) -> binder::public_api::Result<()> { self.0.TakesAnIBinder(_arg_input) }
  fn TakesANullableIBinder(&self, _arg_input: Option<&binder::SpIBinder>) -> binder::public_api::Result<()> { self.0.TakesANullableIBinder(_arg_input) }
  fn TakesAnIBinderList(&self, _arg_input: &[binder::SpIBinder]) -> binder::public_api::Result<()> { self.0.TakesAnIBinderList(_arg_input) }
  fn TakesANullableIBinderList(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>) -> binder::public_api::Result<()> { self.0.TakesANullableIBinderList(_arg_input) }
  fn RepeatUtf8CppString(&self, _arg_token: &str) -> binder::public_api::Result<String> { self.0.RepeatUtf8CppString(_arg_token) }
  fn RepeatNullableUtf8CppString(&self, _arg_token: Option<&str>) -> binder::public_api::Result<Option<String>> { self.0.RepeatNullableUtf8CppString(_arg_token) }
  fn ReverseUtf8CppString(&self, _arg_input: &[String], _arg_repeated: &mut Vec<String>) -> binder::public_api::Result<Vec<String>> { self.0.ReverseUtf8CppString(_arg_input, _arg_repeated) }
  fn ReverseNullableUtf8CppString(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>> { self.0.ReverseNullableUtf8CppString(_arg_input, _arg_repeated) }
  fn ReverseUtf8CppStringList(&self, _arg_input: Option<&[Option<String>]>, _arg_repeated: &mut Option<Vec<Option<String>>>) -> binder::public_api::Result<Option<Vec<Option<String>>>> { self.0.ReverseUtf8CppStringList(_arg_input, _arg_repeated) }
  fn GetCallback(&self, _arg_return_null: bool) -> binder::public_api::Result<Option<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback>>> { self.0.GetCallback(_arg_return_null) }
  fn FillOutStructuredParcelable(&self, _arg_parcel: &mut crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable) -> binder::public_api::Result<()> { self.0.FillOutStructuredParcelable(_arg_parcel) }
  fn RepeatExtendableParcelable(&self, _arg_ep: &crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable, _arg_ep2: &mut crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable) -> binder::public_api::Result<()> { self.0.RepeatExtendableParcelable(_arg_ep, _arg_ep2) }
  fn ReverseList(&self, _arg_list: &crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList> { self.0.ReverseList(_arg_list) }
  fn ReverseIBinderArray(&self, _arg_input: &[binder::SpIBinder], _arg_repeated: &mut Vec<Option<binder::SpIBinder>>) -> binder::public_api::Result<Vec<binder::SpIBinder>> { self.0.ReverseIBinderArray(_arg_input, _arg_repeated) }
  fn ReverseNullableIBinderArray(&self, _arg_input: Option<&[Option<binder::SpIBinder>]>, _arg_repeated: &mut Option<Vec<Option<binder::SpIBinder>>>) -> binder::public_api::Result<Option<Vec<Option<binder::SpIBinder>>>> { self.0.ReverseNullableIBinderArray(_arg_input, _arg_repeated) }
  fn GetOldNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_IOldName>> { self.0.GetOldNameInterface() }
  fn GetNewNameInterface(&self) -> binder::public_api::Result<binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_8_INewName>> { self.0.GetNewNameInterface() }
  fn GetCppJavaTests(&self) -> binder::public_api::Result<Option<binder::SpIBinder>> { self.0.GetCppJavaTests() }
  fn getBackendType(&self) -> binder::public_api::Result<crate::mangled::_7_android_4_aidl_5_tests_11_BackendType> { self.0.getBackendType() }
}
fn on_transact(_aidl_service: &dyn ITestService, _aidl_code: binder::TransactionCode, _aidl_data: &binder::parcel::Parcel, _aidl_reply: &mut binder::parcel::Parcel) -> binder::Result<()> {
  match _aidl_code {
    transactions::UnimplementedMethod => {
      let _arg_arg: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.UnimplementedMethod(_arg_arg);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::Deprecated => {
      let _aidl_return = _aidl_service.Deprecated();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::TestOneway => {
      let _aidl_return = _aidl_service.TestOneway();
      Ok(())
    }
    transactions::RepeatBoolean => {
      let _arg_token: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatBoolean(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatByte => {
      let _arg_token: i8 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatByte(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatChar => {
      let _arg_token: u16 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatChar(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatInt => {
      let _arg_token: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatInt(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatLong => {
      let _arg_token: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatLong(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatFloat => {
      let _arg_token: f32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatFloat(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatDouble => {
      let _arg_token: f64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatDouble(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatString => {
      let _arg_token: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatString(&_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatByteEnum => {
      let _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatByteEnum(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatIntEnum => {
      let _arg_token: crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatIntEnum(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatLongEnum => {
      let _arg_token: crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatLongEnum(_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseBoolean => {
      let _arg_input: Vec<bool> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<bool> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseBoolean(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseByte => {
      let _arg_input: Vec<u8> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<u8> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseByte(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseChar => {
      let _arg_input: Vec<u16> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<u16> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseChar(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseInt => {
      let _arg_input: Vec<i32> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<i32> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseInt(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseLong => {
      let _arg_input: Vec<i64> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<i64> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseLong(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseFloat => {
      let _arg_input: Vec<f32> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<f32> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseFloat(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseDouble => {
      let _arg_input: Vec<f64> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<f64> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseDouble(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseString => {
      let _arg_input: Vec<String> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<String> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseString(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseByteEnum => {
      let _arg_input: Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseByteEnum(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseIntEnum => {
      let _arg_input: Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseIntEnum(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseLongEnum => {
      let _arg_input: Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseLongEnum(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::GetOtherTestService => {
      let _arg_name: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.GetOtherTestService(&_arg_name);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::VerifyName => {
      let _arg_service: binder::Strong<dyn crate::mangled::_7_android_4_aidl_5_tests_14_INamedCallback> = _aidl_data.read()?;
      let _arg_name: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.VerifyName(&_arg_service, &_arg_name);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseStringList => {
      let _arg_input: Vec<String> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<String> = Default::default();
      let _aidl_return = _aidl_service.ReverseStringList(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatParcelFileDescriptor => {
      let _arg_read: binder::parcel::ParcelFileDescriptor = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatParcelFileDescriptor(&_arg_read);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseParcelFileDescriptorArray => {
      let _arg_input: Vec<binder::parcel::ParcelFileDescriptor> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<Option<binder::parcel::ParcelFileDescriptor>> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseParcelFileDescriptorArray(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          if _arg_repeated.iter().any(Option::is_none) { return Err(binder::StatusCode::UNEXPECTED_NULL); }
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ThrowServiceException => {
      let _arg_code: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.ThrowServiceException(_arg_code);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableIntArray => {
      let _arg_input: Option<Vec<i32>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableIntArray(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableByteEnumArray => {
      let _arg_input: Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_ByteEnum>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableByteEnumArray(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableIntEnumArray => {
      let _arg_input: Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_7_IntEnum>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableIntEnumArray(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableLongEnumArray => {
      let _arg_input: Option<Vec<crate::mangled::_7_android_4_aidl_5_tests_8_LongEnum>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableLongEnumArray(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableString => {
      let _arg_input: Option<String> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableString(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableStringList => {
      let _arg_input: Option<Vec<Option<String>>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableStringList(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableParcelable => {
      let _arg_input: Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableParcelable(_arg_input.as_ref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableParcelableArray => {
      let _arg_input: Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableParcelableArray(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableParcelableList => {
      let _arg_input: Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableParcelableList(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::TakesAnIBinder => {
      let _arg_input: binder::SpIBinder = _aidl_data.read()?;
      let _aidl_return = _aidl_service.TakesAnIBinder(&_arg_input);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::TakesANullableIBinder => {
      let _arg_input: Option<binder::SpIBinder> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.TakesANullableIBinder(_arg_input.as_ref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::TakesAnIBinderList => {
      let _arg_input: Vec<binder::SpIBinder> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.TakesAnIBinderList(&_arg_input);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::TakesANullableIBinderList => {
      let _arg_input: Option<Vec<Option<binder::SpIBinder>>> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.TakesANullableIBinderList(_arg_input.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatUtf8CppString => {
      let _arg_token: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatUtf8CppString(&_arg_token);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatNullableUtf8CppString => {
      let _arg_token: Option<String> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.RepeatNullableUtf8CppString(_arg_token.as_deref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseUtf8CppString => {
      let _arg_input: Vec<String> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<String> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseUtf8CppString(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseNullableUtf8CppString => {
      let _arg_input: Option<Vec<Option<String>>> = _aidl_data.read()?;
      let mut _arg_repeated: Option<Vec<Option<String>>> = Default::default();
      _aidl_data.resize_nullable_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseNullableUtf8CppString(_arg_input.as_deref(), &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseUtf8CppStringList => {
      let _arg_input: Option<Vec<Option<String>>> = _aidl_data.read()?;
      let mut _arg_repeated: Option<Vec<Option<String>>> = Default::default();
      let _aidl_return = _aidl_service.ReverseUtf8CppStringList(_arg_input.as_deref(), &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::GetCallback => {
      let _arg_return_null: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.GetCallback(_arg_return_null);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::FillOutStructuredParcelable => {
      let mut _arg_parcel: crate::mangled::_7_android_4_aidl_5_tests_20_StructuredParcelable = _aidl_data.read()?;
      let _aidl_return = _aidl_service.FillOutStructuredParcelable(&mut _arg_parcel);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(&_arg_parcel)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::RepeatExtendableParcelable => {
      let _arg_ep: crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable = _aidl_data.read()?;
      let mut _arg_ep2: crate::mangled::_7_android_4_aidl_5_tests_9_extension_20_ExtendableParcelable = Default::default();
      let _aidl_return = _aidl_service.RepeatExtendableParcelable(&_arg_ep, &mut _arg_ep2);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(&_arg_ep2)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseList => {
      let _arg_list: crate::mangled::_7_android_4_aidl_5_tests_13_RecursiveList = _aidl_data.read()?;
      let _aidl_return = _aidl_service.ReverseList(&_arg_list);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseIBinderArray => {
      let _arg_input: Vec<binder::SpIBinder> = _aidl_data.read()?;
      let mut _arg_repeated: Vec<Option<binder::SpIBinder>> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseIBinderArray(&_arg_input, &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::ReverseNullableIBinderArray => {
      let _arg_input: Option<Vec<Option<binder::SpIBinder>>> = _aidl_data.read()?;
      let mut _arg_repeated: Option<Vec<Option<binder::SpIBinder>>> = Default::default();
      _aidl_data.resize_nullable_out_vec(&mut _arg_repeated)?;
      let _aidl_return = _aidl_service.ReverseNullableIBinderArray(_arg_input.as_deref(), &mut _arg_repeated);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_repeated)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::GetOldNameInterface => {
      let _aidl_return = _aidl_service.GetOldNameInterface();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::GetNewNameInterface => {
      let _aidl_return = _aidl_service.GetNewNameInterface();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::GetCppJavaTests => {
      let _aidl_return = _aidl_service.GetCppJavaTests();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::getBackendType => {
      let _aidl_return = _aidl_service.getBackendType();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub mod Empty {
  #[derive(Debug, Clone, PartialEq)]
  pub struct Empty {
  }
  impl Default for Empty {
    fn default() -> Self {
      Self {
      }
    }
  }
  impl binder::parcel::Parcelable for Empty {
    fn write_to_parcel(&self, parcel: &mut binder::parcel::Parcel) -> binder::Result<()> {
      parcel.sized_write(|subparcel| {
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::parcel::Parcel) -> binder::Result<()> {
      parcel.sized_read(|subparcel| {
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(Empty);
  binder::impl_deserialize_for_parcelable!(Empty);
  impl binder::parcel::ParcelableMetadata for Empty {
    fn get_descriptor() -> &'static str { "android.aidl.tests.ITestService.Empty" }
  }
}
pub mod CompilerChecks {
  #[derive(Debug)]
  pub struct CompilerChecks {
    pub binder: Option<binder::SpIBinder>,
    pub nullable_binder: Option<binder::SpIBinder>,
    pub binder_array: Vec<binder::SpIBinder>,
    pub nullable_binder_array: Option<Vec<Option<binder::SpIBinder>>>,
    pub binder_list: Vec<binder::SpIBinder>,
    pub nullable_binder_list: Option<Vec<Option<binder::SpIBinder>>>,
    pub pfd: Option<binder::parcel::ParcelFileDescriptor>,
    pub nullable_pfd: Option<binder::parcel::ParcelFileDescriptor>,
    pub pfd_array: Vec<binder::parcel::ParcelFileDescriptor>,
    pub nullable_pfd_array: Option<Vec<Option<binder::parcel::ParcelFileDescriptor>>>,
    pub pfd_list: Vec<binder::parcel::ParcelFileDescriptor>,
    pub nullable_pfd_list: Option<Vec<Option<binder::parcel::ParcelFileDescriptor>>>,
    pub parcel: crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty,
    pub nullable_parcel: Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>,
    pub parcel_array: Vec<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>,
    pub nullable_parcel_array: Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>,
    pub parcel_list: Vec<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>,
    pub nullable_parcel_list: Option<Vec<Option<crate::mangled::_7_android_4_aidl_5_tests_12_ITestService_5_Empty>>>,
  }
  impl Default for CompilerChecks {
    fn default() -> Self {
      Self {
        binder: Default::default(),
        nullable_binder: Default::default(),
        binder_array: Default::default(),
        nullable_binder_array: Default::default(),
        binder_list: Default::default(),
        nullable_binder_list: Default::default(),
        pfd: Default::default(),
        nullable_pfd: Default::default(),
        pfd_array: Default::default(),
        nullable_pfd_array: Default::default(),
        pfd_list: Default::default(),
        nullable_pfd_list: Default::default(),
        parcel: Default::default(),
        nullable_parcel: Default::default(),
        parcel_array: Default::default(),
        nullable_parcel_array: Default::default(),
        parcel_list: Default::default(),
        nullable_parcel_list: Default::default(),
      }
    }
  }
  impl binder::parcel::Parcelable for CompilerChecks {
    fn write_to_parcel(&self, parcel: &mut binder::parcel::Parcel) -> binder::Result<()> {
      parcel.sized_write(|subparcel| {
        let __field_ref = self.binder.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
        subparcel.write(__field_ref)?;
        subparcel.write(&self.nullable_binder)?;
        subparcel.write(&self.binder_array)?;
        subparcel.write(&self.nullable_binder_array)?;
        subparcel.write(&self.binder_list)?;
        subparcel.write(&self.nullable_binder_list)?;
        let __field_ref = self.pfd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
        subparcel.write(__field_ref)?;
        subparcel.write(&self.nullable_pfd)?;
        subparcel.write(&self.pfd_array)?;
        subparcel.write(&self.nullable_pfd_array)?;
        subparcel.write(&self.pfd_list)?;
        subparcel.write(&self.nullable_pfd_list)?;
        subparcel.write(&self.parcel)?;
        subparcel.write(&self.nullable_parcel)?;
        subparcel.write(&self.parcel_array)?;
        subparcel.write(&self.nullable_parcel_array)?;
        subparcel.write(&self.parcel_list)?;
        subparcel.write(&self.nullable_parcel_list)?;
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::parcel::Parcel) -> binder::Result<()> {
      parcel.sized_read(|subparcel| {
        if subparcel.has_more_data() {
          self.binder = Some(subparcel.read()?);
        }
        if subparcel.has_more_data() {
          self.nullable_binder = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.binder_array = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_binder_array = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.binder_list = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_binder_list = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.pfd = Some(subparcel.read()?);
        }
        if subparcel.has_more_data() {
          self.nullable_pfd = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.pfd_array = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_pfd_array = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.pfd_list = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_pfd_list = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.parcel = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_parcel = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.parcel_array = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_parcel_array = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.parcel_list = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.nullable_parcel_list = subparcel.read()?;
        }
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(CompilerChecks);
  binder::impl_deserialize_for_parcelable!(CompilerChecks);
  impl binder::parcel::ParcelableMetadata for CompilerChecks {
    fn get_descriptor() -> &'static str { "android.aidl.tests.ITestService.CompilerChecks" }
  }
}
pub(crate) mod mangled {
 pub use super::ITestService as _7_android_4_aidl_5_tests_12_ITestService;
 pub use super::Empty::Empty as _7_android_4_aidl_5_tests_12_ITestService_5_Empty;
 pub use super::CompilerChecks::CompilerChecks as _7_android_4_aidl_5_tests_12_ITestService_14_CompilerChecks;
}
