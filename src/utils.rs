// pub fn parse_ip(ip: &str) -> Result<(u8, u8, u8, u8, u16), &str> {
//     let split: Vec<&str> = ip.trim().split('.').collect();
//     if split.len() != 4 {
//         return Err("Invalid IP");
//     }
//     let mut ip_as_int: vec![];
//     for i in 0..3 {
//         if let Some(num) = split[i].trim().parse() {
//             if num < 0 {
//                 return Err("Invalid IP");
//             }
//             result.i = num;
//         } else {
//             return Err("Invalid IP");
//         }
//     }

//     Ok(result)
// }
