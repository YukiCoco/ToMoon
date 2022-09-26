mod tests {

    use crate::control;
    use std::{thread, time::Duration};

    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 4);
    }

    #[test]
    fn run_clash()
    {
        let mut clash = control::Clash::default();
        println!("{}",std::env::current_dir().unwrap().to_str().unwrap());
        clash.run().unwrap();
        thread::sleep(Duration::from_secs(5));
        println!("disable clash");
        clash.stop();
        thread::sleep(Duration::from_secs(10));
    }
}