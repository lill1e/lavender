use ureq::Agent;

pub trait Callable {
    fn agent(&self) -> Agent;
}