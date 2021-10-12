#![forbid(unsafe_code)]
#[derive(Debug)]
pub struct MyExt {
  pub a: i32,
  pub b: String,
}
pub(crate) mod mangled { pub use super::MyExt as _7_android_4_aidl_5_tests_9_extension_5_MyExt; }
impl Default for MyExt {
  fn default() -> Self {
    Self {
      a: 0,
      b: Default::default(),
    }
  }
}
impl binder::parcel::Parcelable for MyExt {
  fn write_to_parcel(&self, parcel: &mut binder::parcel::Parcel) -> binder::Result<()> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.a)?;
      subparcel.write(&self.b)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::parcel::Parcel) -> binder::Result<()> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.a = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.b = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(MyExt);
binder::impl_deserialize_for_parcelable!(MyExt);
impl binder::parcel::ParcelableMetadata for MyExt {
  fn get_descriptor() -> &'static str { "android.aidl.tests.extension.MyExt" }
}