pub mod traits {
    pub trait IRegister {
        fn get_available_registers(&self) -> Vec<String>;
        fn write_to_v_register(&mut self, register: str, content: u8);
        fn read_v_register(&self, register: str) -> u8;
        fn write_to_i_register(&mut self, content: u16);
        fn read_i_register(&self) -> u16;
    }

    pub trait IStack {
        fn push(&mut self, item: u16);
        fn pop(&mut self) -> u16;
        fn top(&self) -> u16;
        fn is_full(&self) -> bool;
        fn is_empty(&self) -> bool;
    }

    pub trait IStreamingOutputDevice {
        fn write(&mut self, content: u16);
    }

}