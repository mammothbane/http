#[cfg(feature = "alloc")]
macro_rules! if_downcast_into {
    ($in_ty:ty, $out_ty:ty, $val:ident, $body:expr) => {{
        if ::core::any::TypeId::of::<$in_ty>() == ::core::any::TypeId::of::<$out_ty>() {
            // Store the value in an `Option` so we can `take`
            // it after casting to `&mut dyn Any`.
            let mut slot = Some($val);
            // Re-write the `$val` ident with the downcasted value.
            let $val = (&mut slot as &mut dyn ::core::any::Any)
                .downcast_mut::<Option<$out_ty>>()
                .unwrap()
                .take()
                .unwrap();
            // Run the $body in scope of the replaced val.
            $body
        }
    }};
}
