#[macro_export]
macro_rules! newtype {
    ($name:ident, $wrap:ty) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub struct $name($wrap);

        impl<T> From<T> for $name
        where T: Into<$wrap>
        {
            fn from(v: T) -> Self {
                Self(v.into())
            }
        }
    };

    ($name:ident, $wrap:ty, $asref:ty) => {
        newtype!($name, $wrap);

        impl AsRef<$asref> for $name {
            fn as_ref(&self) -> &$asref {
                self.0.as_ref()
            }
        }
    };
}
