#![allow(unused_variables)]
#![allow(unused_assignments)]

pub mod convert {
    use std::net::{SocketAddr, ToSocketAddrs};

    pub fn to_ip(input: String) -> SocketAddr {
        return input.to_socket_addrs().unwrap().as_slice()[0];
    }
}

pub mod client_input {
    use regex::Regex;

    pub enum InputType {
        Ip,
        Name,
    }

    pub fn read_input(mut message: String, input_type: InputType) -> String {
        use std::io::{stdin, stdout, Write};
        let mut input = String::new();
        let mut res = String::new();
        loop {
            input = String::new();
            res = String::new();
            print!("{}", message);
            let _ = stdout().flush();
            stdin().read_line(&mut input).expect("Did not enter a correct string");
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            res = input.trim().to_string();
            match input_type {
                InputType::Ip => {
                    if validate_ip(res.clone()) { break; }
                    message = "Entered IP is incorrect. Try again: ".to_string();
                }
                InputType::Name => {
                    if validate_user_name(res.clone()) { break; }
                    message = "Entered name is too short. Try again: ".to_string();
                }
            }
        }
        return res;
    }

    fn validate_user_name(name: String) -> bool {
        return name.len() > 1;
    }

    fn validate_ip(input: String) -> bool {
        let ip_re = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d+$").unwrap();
        return ip_re.is_match(input.as_str());
    }
}
