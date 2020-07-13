pub trait GpTool<TOutput> {

    fn execute(&self) -> TOutput;
}