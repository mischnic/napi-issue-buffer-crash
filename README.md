1. Run `yarn`
2. Run `yarn build`
3. Run `node test.js`


It crashes with
```
thread '<unnamed>' panicked at library/core/src/panicking.rs:156:5:
unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_nounwind_fmt::runtime
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:110:18
   2: core::panicking::panic_nounwind_fmt
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:123:9
   3: core::panicking::panic_nounwind
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:156:5
   4: core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs:2799:21
   5: core::ptr::non_null::NonNull<T>::new_unchecked
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/non_null.rs:220:13
   6: core::ptr::unique::Unique<T>::new_unchecked
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/unique.rs:89:36
   7: alloc::raw_vec::RawVec<T,A>::from_raw_parts_in
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/raw_vec.rs:250:30
   8: alloc::vec::Vec<T,A>::from_raw_parts_in
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:820:29
   9: alloc::vec::Vec<T>::from_raw_parts
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:604:18
  10: napi::js_values::buffer::JsBufferValue::from_raw
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.4/src/js_values/buffer.rs:74:45
  11: <&mut napi::js_values::de::De as serde::de::Deserializer>::deserialize_byte_buf
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.4/src/js_values/de.rs:89:26
  12: <serde_bytes::bytebuf::ByteBuf as serde::de::Deserialize>::deserialize
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_bytes-0.11.14/src/bytebuf.rs:251:9
  13: <serde_bytes::bytebuf::ByteBuf as serde_bytes::de::Deserialize>::deserialize
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_bytes-0.11.14/src/de.rs:113:9
  14: <alloc::vec::Vec<u8> as serde_bytes::de::Deserialize>::deserialize
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_bytes-0.11.14/src/de.rs:52:9
  15: serde_bytes::deserialize
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_bytes-0.11.14/src/lib.rs:121:5
  16: <<napi_buffer_crash::_::<impl serde::de::Deserialize for napi_buffer_crash::Config>::deserialize::__Visitor as serde::de::Visitor>::visit_map::__DeserializeWith as serde::de::Deserialize>::deserialize
             at ./src/lib.rs:6:28
  17: <core::marker::PhantomData<T> as serde::de::DeserializeSeed>::deserialize
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs:794:9
  18: <napi::js_values::de::JsObjectAccess as serde::de::MapAccess>::next_value_seed
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.4/src/js_values/de.rs:365:15
  19: serde::de::MapAccess::next_value
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs:1867:9
  20: <&mut A as serde::de::MapAccess>::next_value
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs:1948:9
  21: <napi_buffer_crash::_::<impl serde::de::Deserialize for napi_buffer_crash::Config>::deserialize::__Visitor as serde::de::Visitor>::visit_map
             at ./src/lib.rs:6:28
  22: <&mut napi::js_values::de::De as serde::de::Deserializer>::deserialize_any
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.4/src/js_values/de.rs:53:11
  23: <&mut napi::js_values::de::De as serde::de::Deserializer>::deserialize_struct
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs:133:13
  24: napi_buffer_crash::_::<impl serde::de::Deserialize for napi_buffer_crash::Config>::deserialize
             at ./src/lib.rs:6:28
  25: napi::env::Env::from_js_value
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.4/src/env.rs:1336:5
  26: napi_buffer_crash::transform
             at ./src/lib.rs:15:26
  27: napi_buffer_crash::__napi__transform::{{closure}}::{{closure}}
             at ./src/lib.rs:13:1
  28: napi::bindgen_prelude::within_runtime_if_available
             at /Users/niklas/.cargo/registry/src/index.crates.io-6f17d22bba15001f/napi-2.16.4/src/lib.rs:162:5
  29: napi_buffer_crash::__napi__transform::{{closure}}
             at ./src/lib.rs:13:1
  30: core::result::Result<T,E>::and_then
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/result.rs:1321:22
  31: napi_buffer_crash::__napi__transform
             at ./src/lib.rs:13:1
  32: __ZN6v8impl12_GLOBAL__N_123FunctionCallbackWrapper6InvokeERKN2v820FunctionCallbackInfoINS2_5ValueEEE
  33: __ZN2v88internal25FunctionCallbackArguments4CallENS0_15CallHandlerInfoE
  34: __ZN2v88internal12_GLOBAL__N_119HandleApiCallHelperILb0EEENS0_11MaybeHandleINS0_6ObjectEEEPNS0_7IsolateENS0_6HandleINS0_10HeapObjectEEENS8_INS0_20FunctionTemplateInfoEEENS8_IS4_EEPmi
  35: __ZN2v88internal21Builtin_HandleApiCallEiPmPNS0_7IsolateE
```