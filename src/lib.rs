pub trait DefaultPort {
    fn default_port(&self) -> Option<usize>;
}
