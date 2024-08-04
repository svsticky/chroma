use std::process::{ExitCode, Termination};

use tracing::error;

pub enum Exit {
    Ok,
    Err(anyhow::Error),
}

impl Termination for Exit {
    fn report(self) -> ExitCode {
        match self {
            Exit::Ok => ExitCode::from(0),
            Exit::Err(err) => {
                error!("{}", err);
                err.chain()
                    .skip(1)
                    .for_each(|cause| error!("because: {}", cause));

                ExitCode::from(1)
            }
        }
    }
}
