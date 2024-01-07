#![no_std]

pub struct BusCommand {
    pub register: u8,
    pub data: [u8; 20],
    pub data_size: usize,
    pub read_direction: bool,
}

pub struct BusStatus {
    last_register: Option<u8>,
    read_direction: bool,
    data: [u8; 20],
    data_index: usize,
    data_size: usize,
    command: Option<BusCommand>,
    stopped: bool,
}

impl BusStatus {
    pub fn new() -> Self {
        Self {
            last_register: None,
            read_direction: false,
            data: [0u8; 20],
            data_index: 0,
            data_size: 0,
            command: None,
            stopped: true,
        }
    }

    pub fn addr(&mut self, read_direction: bool) {
        if !self.stopped {
            //Build a command for the previous operation
            self.build_command();
        }

        self.stopped = false;
        self.read_direction = read_direction;
        self.data_index = 0;

        if !read_direction {
            self.last_register = None;
            self.data_size = 0;
        }
    }

    pub fn is_reading(&self) -> bool {
        self.read_direction
    }

    pub fn write_data(&mut self, data: u8) -> bool {
        if self.last_register.is_none() {
            self.last_register = Some(data);
            true
        } else {
            self.data[self.data_index] = data;
            self.data_index += 1;
            self.data_size += 1;

            true
        }
    }

    pub fn read_data(&mut self) -> u8 {
        //TODO: Decide what to do here.  If the central is reading data we need to return something.
        if self.data_index < self.data_size {
            let result = self.data[self.data_index];
            self.data_index += 1;

            result
        } else {
            0xFF
        }
    }

    pub fn stop(&mut self) {
        self.build_command();
        self.stopped = true;
    }

    fn build_command(&mut self) {
        if let Some(last_register) = self.last_register {
            let result = BusCommand {
                register: last_register,
                data: self.data,
                data_size: self.data_index,
                read_direction: self.read_direction,
            };

            self.command = Some(result);
        }
    }

    pub fn process(&mut self) -> Option<BusCommand> {
        let result = self.command.take();
        self.command = None;

        result
    }

    pub fn provide_data(&mut self, register: u8, data: &[u8; 20], data_size: usize) {
        if Some(register) == self.last_register {
            self.data[..20].copy_from_slice(data);
            self.data_size = data_size;
            self.data_index = 0;
        }
    }
}

mod test {
    #[test]
    fn build_command_before_any_data_results_in_no_command() {
        let mut status = super::BusStatus::new();

        status.build_command();

        assert!(status.command.is_none(), "Expected no command");
    }

    #[test]
    fn write_sets_first_byte_as_register() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(0x12));
        assert_eq!(status.last_register, Some(0x12));
        assert_eq!(status.data_size, 0);
        assert_eq!(status.data_index, 0);
    }

    #[test]
    fn write_sets_second_byte_as_data() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert!(status.write_data(0x12));
        assert!(status.write_data(0xAA));
        assert_eq!(status.data[0], 0xAA);
        assert_eq!(status.data_size, 1);
        assert_eq!(status.data_index, 1);
    }

    #[test]
    fn write_without_stop_does_not_process_to_command() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert!(status.write_data(0x12));
        assert!(status.write_data(0xAA));

        let result = status.process();
        assert!(result.is_none(), "Should not have processed command");
    }

    #[test]
    fn write_with_stop_processes_to_command() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert!(status.write_data(0x12));
        assert!(status.write_data(0xAA));
        status.stop();

        let result = status.process();
        assert!(result.is_some(), "Should have processed command");

        let result = status.process();
        assert!(result.is_none(), "Should have no more commands");
    }

    #[test]
    fn write_with_restart_processes_to_command() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert!(status.write_data(0x12));
        status.addr(true);

        let result = status.process();
        assert!(result.is_some(), "Should have processed command");
        let result = result.unwrap();
        assert_eq!(
            result.read_direction, false,
            "Should have read direction set"
        );
        assert_eq!(result.data_size, 0);
        assert_eq!(result.register, 0x12);

        let result = status.process();
        assert!(result.is_none(), "Should have no more commands");
    }


    #[test]
    fn write_with_stop_then_read_only_gives_one_command() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert!(status.write_data(0x12));

        status.stop();

        let result = status.process();
        assert!(result.is_some(), "Should have processed command");
        let result = result.unwrap();
        assert_eq!(
            result.read_direction, false,
            "Should have read direction set"
        );
        assert_eq!(result.data_size, 0);
        assert_eq!(result.register, 0x12);

        status.addr(true);

        let result = status.process();
        assert!(result.is_none(), "Should have no more commands");
    }

    #[test]
    fn write_with_stop_leaves_last_register_set() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert!(status.write_data(0x12));
        status.stop();

        assert_eq!(status.last_register, Some(0x12));
        assert_eq!(status.is_reading(), false);
    }

    #[test]
    fn is_reading_returns_true_after_read_begins() {
        let mut status = super::BusStatus::new();

        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(0x12));
        status.stop();

        status.addr(true);

        assert_eq!(status.is_reading(), true);
    }

    #[test]
    fn unsatisfied_write_command_followed_by_read_returns_no_data() {
        const REGISTER: u8 = 0x12;

        let mut status = super::BusStatus::new();

        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(REGISTER));

        status.stop();

        let command = status.process();

        assert!(command.is_some(), "Should have processed command");

        let command = command.unwrap();
        assert_eq!(command.register, REGISTER);
        assert_eq!(command.data_size, 0);

        status.addr(true);

        assert_eq!(status.is_reading(), true);

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0xFF, "Should not have data byte");
    }

    #[test]
    fn satisfied_write_command_followed_by_read_returns_correct_data() {
        let mut register_data: [u8; 20] = [0; 20];
        const SRC_DATA: [u8; 5] = [0x12, 0x34, 0x56, 0x78, 0x9A];

        register_data[..5].copy_from_slice(&SRC_DATA);

        let register_data_size = 5;
        let register_data: [u8; 20] = register_data;

        const REGISTER: u8 = 0x12;

        let mut status = super::BusStatus::new();

        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(REGISTER));

        status.stop();

        let command = status.process();

        assert!(command.is_some(), "Should have processed command");

        let command = command.unwrap();
        assert_eq!(command.register, REGISTER);
        assert_eq!(command.data_size, 0);

        status.provide_data(REGISTER, &register_data, register_data_size);

        status.addr(true);

        assert_eq!(status.is_reading(), true);

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0x12, "Should have data byte");

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0x34, "Should have data byte");

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0x56, "Should have data byte");

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0x78, "Should have data byte");

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0x9A, "Should have data byte");

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0xFF, "Should have no more data bytes");
    }

    #[test]
    fn satisfied_data_for_wrong_register_followed_by_read_returns_no_data() {
        let mut register_data: [u8; 20] = [0; 20];
        const SRC_DATA: [u8; 5] = [0x12, 0x34, 0x56, 0x78, 0x9A];

        register_data[..5].copy_from_slice(&SRC_DATA);

        let register_data_size = 5;
        let register_data: [u8; 20] = register_data;

        const REGISTER: u8 = 0x12;

        let mut status = super::BusStatus::new();

        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(REGISTER));

        status.stop();

        let command = status.process();

        assert!(command.is_some(), "Should have processed command");

        let command = command.unwrap();
        assert_eq!(command.register, REGISTER);
        assert_eq!(command.data_size, 0);

        status.provide_data(0xAA, &register_data, register_data_size);

        status.addr(true);

        assert_eq!(status.is_reading(), true);

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0xFF, "Should have no more data bytes");
    }

    #[test]
    fn satisfied_write_command_then_another_write_then_read_returns_no_data() {
        let mut register_data: [u8; 20] = [0; 20];
        const SRC_DATA: [u8; 5] = [0x12, 0x34, 0x56, 0x78, 0x9A];

        register_data[..5].copy_from_slice(&SRC_DATA);

        let register_data_size = 5;
        let register_data: [u8; 20] = register_data;

        const REGISTER: u8 = 0x12;

        let mut status = super::BusStatus::new();

        //Initial Write
        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(REGISTER));

        status.stop();

        //Process command, provide data

        let command = status.process();

        assert!(command.is_some(), "Should have processed command");

        let command = command.unwrap();
        assert_eq!(command.register, REGISTER);
        assert_eq!(command.data_size, 0);

        status.provide_data(REGISTER, &register_data, register_data_size);

        //Second write
        status.addr(false);

        assert_eq!(status.is_reading(), false);

        assert!(status.write_data(REGISTER));

        //Read
        status.addr(true);

        assert_eq!(status.is_reading(), true);

        let data_byte = status.read_data();

        assert_eq!(data_byte, 0xFF, "Should have no more data bytes");
    }
}
