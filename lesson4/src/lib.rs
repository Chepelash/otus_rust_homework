/// Умная розетка умеет:
///     Предоставлять текстовое описание.
///     Включаться и выключаться.
///     Предоставлять данные о текущей потребляемой мощности.
pub mod socket {
    /// Type for describing socket state
    #[derive(Debug, Default)]
    pub enum WorkingState {
        /// Socket is On
        On,
        /// Socket is Off
        #[default]
        Off,
    }

    /// Struct for socket
    #[derive(Debug, Default)]
    pub struct SmartSocket {
        /// User description
        pub description: Option<String>,
        /// State: On or Off. Can be accessed by methods
        state: WorkingState,
        /// Power value. Valid if `state = WorkingState::On`
        power: u32,
    }

    impl SmartSocket {
        /// Constructs new socket with description. By default state is Off and power is 0.
        /// Can use `SmartSocket::default()` construct without description
        ///
        /// # Examples
        ///
        /// ```rust
        /// use lesson4::socket::SmartSocket;
        /// let socket = SmartSocket::default();
        /// assert_eq!(None, socket.description);
        /// ```
        pub fn new(description: &str) -> Self {
            Self {
                description: Some(description.to_string()),
                state: WorkingState::Off,
                power: 0,
            }
        }
        
        /// Returns power value if state is WorkingState::On, returns None otherwise
        pub fn get_power(&mut self) -> Option<u32> {
            match self.state {
                WorkingState::On => {
                    self.power = self.calculate_power();
                    Some(self.power)
                }
                WorkingState::Off => None,
            }
        }

        /// Turning on socket
        pub fn turn_on(&mut self) {
            self.state = WorkingState::On;
        }

        /// Turning off socket
        pub fn turn_off(&mut self) {
            self.state = WorkingState::Off;
        }

        /// Returns socket state
        pub fn get_state(&self) -> &WorkingState {
            &self.state
        }

        /// Calculates current power
        fn calculate_power(&self) -> u32 {
            10
        }
    }
}

///     Термометр умеет:
///     Выдавать данные о текущей температуре.
pub mod thermometer {

    /// Struct for thermometer
    /// Should be instantiated via `Thermometer::default()`
    #[derive(Debug, Default)]
    pub struct Thermometer {
        /// Temperature value
        temperature: i32,
    }

    impl Thermometer {
        /// Returns currently calculated temperature
        pub fn calculate_temperature(&mut self) -> i32 {
            self.temperature = self.internal_calculate_temperature();
            self.temperature
        }

        /// Internal method for temperature calculation
        /// Returns current temperature
        fn internal_calculate_temperature(&self) -> i32 {
            22
        }
    }
}
