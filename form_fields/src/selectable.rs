pub trait Selectable: Clone {
    type Key: PartialEq + ToString + std::str::FromStr;
    type DisplayValue: maud::Render;
    fn key(&self) -> Self::Key;
    fn display_value(&self) -> Self::DisplayValue;
}

macro_rules! declare_selectable {
    ($type:ty) => {
        impl Selectable for $type {
            type Key = $type;
            type DisplayValue = $type;

            fn key(&self) -> Self::Key {
                self.clone()
            }

            fn display_value(&self) -> Self::DisplayValue {
                self.clone()
            }
        }
    };
}

declare_selectable!(u8);
declare_selectable!(u16);
declare_selectable!(u32);
declare_selectable!(u64);
declare_selectable!(usize);
declare_selectable!(i8);
declare_selectable!(i16);
declare_selectable!(i32);
declare_selectable!(i64);
declare_selectable!(f32);
declare_selectable!(f64);
declare_selectable!(isize);
declare_selectable!(String);
declare_selectable!(char);
