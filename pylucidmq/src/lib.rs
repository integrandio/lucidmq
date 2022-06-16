use pyo3::{prelude::*};

mod consumer;
mod producer;
mod message;
mod lucidmq;

use lucidmq::LucidMQ;
use consumer::Consumer;
use producer::Producer;
use message::Message;

/// A Python module implemented in Rust.
#[pymodule]
fn pylucidmq(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Message>()?;
    m.add_class::<Producer>()?;
    m.add_class::<Consumer>()?;
    m.add_class::<LucidMQ>()?;
    Ok(())
}