use crate::error::Error;

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
