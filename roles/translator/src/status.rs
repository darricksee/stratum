use crate::error::Error;
use tracing::error;

#[derive(Debug)]
pub enum State<'a> {
    Shutdown(Error<'a>),
    Healthy(String),
}

#[derive(Debug)]
pub struct Status<'a> {
    pub state: State<'a>,
}


/// first parameter is a result (Result<T, crate::error::Error)
/// second parameter is a crate::status::Component enum variant
#[macro_export]
macro_rules! handle_result {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                // send status
                let _ = $crate::status::Status {
                    state: $crate::status::State::Shutdown(e)
                };
                panic!("Error");
            },
        }
    };
}
