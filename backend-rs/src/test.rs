mod tests {
    use crate::control;

    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 4);
    }

    #[test]
    fn run_clash()
    {
        let clash = control::clash::default();
        let mut clash = clash.run().unwrap();
        clash.kill().unwrap();
        
    }
}