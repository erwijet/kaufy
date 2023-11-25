use async_graphql::Enum;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive, Eq, PartialEq, Enum)]
pub enum AclRole {
    Barista = 1,
    Admin = 2,
}

#[derive(Debug)]
pub struct Roleset(pub Vec<AclRole>);

impl Roleset {
    pub fn new() -> Self {
        Roleset(vec![])
    }
}

impl From<Vec<AclRole>> for Roleset {
    fn from(value: Vec<AclRole>) -> Self {
        Roleset(value)
    }
}

impl std::ops::DerefMut for Roleset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Roleset> for u32 {
    fn from(value: Roleset) -> Self {
        value.0.into_iter().fold(0x0, |v, cur| v | (1 << cur as u8))
    }
}

impl From<u32> for Roleset {
    fn from(value: u32) -> Self {
        let mut i = 0;
        let mut roleset = Roleset::new();

        while let Some(role) = {
            i += 1;
            num::FromPrimitive::from_i32(i) as Option<AclRole>
        } {
            if (0x1 << i) & value != 0x0 {
                roleset.push(role)
            }
        }

        roleset
    }
}

impl std::ops::Deref for Roleset {
    type Target = Vec<AclRole>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}