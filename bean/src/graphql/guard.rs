// use async_graphql::{Context, Guard, Result};
// pub struct OwnerGuard;

// impl OwnerGuard {
//     fn new() -> Self {
//         Self
//     }
// }

// #[async_trait::async_trait]
// impl Guard for OwnerGuard {
//     async fn check(&self, ctx: &Context<'_>) -> Result<()> {

//         if self.expect != self.actual {
//             Err("Forbidden".into())
//         } else {
//             Ok(())
//         }
//     }
// }
