// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(clippy::missing_errors_doc, warnings)]

mod consts;
mod error;
pub use error::{Error, Result};
use futures::{executor::block_on, future::join};

#[allow(clippy::empty_enum)]
pub enum Never {}

struct Game {}

struct LocalInputs {}

struct Inputs {
    local: LocalInputs,
    remote: RemoteInputs,
}

impl From<(LocalInputs, RemoteInputs)> for Inputs {
    fn from(inputs: (LocalInputs, RemoteInputs)) -> Self {
        Inputs {
            local: inputs.0,
            remote: inputs.1,
        }
    }
}

struct RemoteInputs {}

impl Game {
    pub const fn new() -> Self {
        Self {}
    }

    fn display(&self) -> Result<()> {
        unimplemented!()
    }

    fn game_loop(&mut self) -> Result<Never> {
        loop {
            block_on(self.update_state())?;
            self.display();
        }
    }

    async fn read_inputs(&self) -> Result<Inputs> {
        // TODO: Ensure `join` runs futures concurrently
        let (local, remote) = join(self.read_local_inputs(), self.read_remote_inputs()).await;
        Ok((local?, remote?).into())
    }

    async fn read_local_inputs(&self) -> Result<LocalInputs> {
        unimplemented!()
    }

    async fn read_remote_inputs(&self) -> Result<RemoteInputs> {
        unimplemented!()
    }

    async fn update_state(&mut self) -> Result<()> {
        // write local // !! potentially expensive, non-deterministic !!
        // write remote
        let inputs = self.read_inputs().await?;
        self.write_inputs(&inputs).await?;

        Ok(())
    }

    async fn write_inputs(&mut self) -> Result<&mut Self> {
        join!(write_remote(&inputs), write_local(&inputs)).await?;
        Ok(self)
    }
}

pub fn lib_main() -> Result<Never> {
    Game::new().game_loop()
}
