extern crate nix;
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType, InetAddr, connect, IpAddr, SockAddr, recv, MsgFlags};
use std::io::{self, Write};

fn main() {

    play_star_wars();
}


fn play_star_wars() -> Result<(), Box<std::error::Error>>{
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None)?;
    let ip_addr = IpAddr::new_v4(94, 142, 241, 111);
    let port = 23;
    let sockaddr = SockAddr::new_inet(InetAddr::new(ip_addr, port));
    connect(sock, &sockaddr)?;
    let mut buf = [0u8; 1024];
    loop {
        let len_of_content_in_buffer = recv(sock, &mut buf, MsgFlags::empty())?;
        if len_of_content_in_buffer == 0 {
            println!("Done!");
            break;
        }
        let new_bytes = &buf[..len_of_content_in_buffer];
        io::stdout().write(new_bytes)?;
    }
    return Ok(());
}