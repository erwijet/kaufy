#[macro_export]
macro_rules! impl_std_ops {
    ($t:ty) => {
        impl $t {
            pub fn find_by_id(id: i32) -> Select<$t> {
                Self::find().filter(Column::Id.eq(id))
            }

            pub fn find_by_name(name: &str) -> Select<$t> {
                Self::find().filter(Column::Name.eq(name))
            }

            pub fn delete_by_id(id: i32) -> DeleteMany<$t> {
                Self::delete_many().filter(Column::Id.eq(id))
            }
        }
    };
}