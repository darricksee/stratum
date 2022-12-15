use crate::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum State {
    Shutdown(Error),
    Healthy(String),
}

#[derive(Debug)]
pub enum Component {
    Bridge,
    Upstream,
    Downstream,
}

#[derive(Debug)]
pub struct Status {
    pub state: State,
    pub component: Component,
}
