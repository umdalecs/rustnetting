use std::env;

pub struct Config {
    pub ip_add: String,
    pub requirements: Vec<u32>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ip_add = args[1].clone();

        let requirements: Vec<u32> = args[2..].into_iter().map(|e| e.parse().unwrap()).collect();

        return Ok(Config {
            ip_add,
            requirements,
        });
    }
}

pub fn u32_to_ipv4(numeric: u32) -> String {
    let numeric = format!("{:0>32b}", numeric);

    return format!(
        "{}.{}.{}.{}",
        u8::from_str_radix(&numeric[..8], 2).unwrap(),
        u8::from_str_radix(&numeric[8..16], 2).unwrap(),
        u8::from_str_radix(&numeric[16..24], 2).unwrap(),
        u8::from_str_radix(&numeric[24..], 2).unwrap()
    );
}

pub fn next_add(ip_add: &str, slash: u32) -> (String, String) {
    let step = 2_u32.pow(32 - slash);

    let octets: Vec<&str> = ip_add.split('.').collect();

    let mut bin_ip_add: String = String::new();

    for oct in octets {
        let dec_oct: u32 = oct.parse().unwrap();
        let bin_oct = format!("{:0>8b}", dec_oct);
        bin_ip_add.push_str(&bin_oct);
    }

    let dec_ip_add = u32::from_str_radix(&bin_ip_add, 2).unwrap();

    let broadcast = u32_to_ipv4(dec_ip_add + step - 1);
    let next = u32_to_ipv4(dec_ip_add + step);

    return (next, broadcast);
}
