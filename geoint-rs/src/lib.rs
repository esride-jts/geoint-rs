mod traits;

pub struct GeointTool {

}

impl traits::GpTool<bool> for GeointTool {

    fn execute(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::traits::*;

    #[test]
    fn it_works() {
        let tool = GeointTool {};
        let executed = tool.execute();
        assert_eq!(true, executed);
    }
}
