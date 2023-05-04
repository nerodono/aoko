use std::{
    io,
    net::{IpAddr, UdpSocket},
};

use bytes::BufMut;

use super::controller::Controller;

pub struct Connection {
    sock: UdpSocket,
    buffer: [u8; 108],

    ip: IpAddr,
    port: u16,
}

impl Connection {
    pub const MAGIC: u16 = 0x3276;

    pub fn update(&mut self, gamepads: &[Option<Controller>; 4]) {
        let gamepads_no = gamepads.iter().filter(|v| v.is_some()).count() as u16;
        let mut buf_ref: &mut [u8] = &mut self.buffer;

        buf_ref.put_u16_le(Self::MAGIC);
        buf_ref.put_u16_le(gamepads_no);

        for gamepad in gamepads {
            let Some(ref pad) = gamepad else {
                buf_ref.put_u16(0);
                buf_ref.put_u64_le(0);
                buf_ref.put_i32_le(0);
                buf_ref.put_i32_le(0);
                buf_ref.put_i32_le(0);
                buf_ref.put_i32_le(0);

                continue;
            };

            buf_ref.put_u16_le(pad.type_ as u16);
            buf_ref.put_u64_le(pad.keys.bits());

            buf_ref.put_i32_le(pad.joy_left.0);
            buf_ref.put_i32_le(pad.joy_left.1);
            buf_ref.put_i32_le(pad.joy_right.0);
            buf_ref.put_i32_le(pad.joy_right.1);
        }
    }

    pub fn send(&self) -> io::Result<usize> {
        self.sock.send(&self.buffer)
    }

    pub fn reconnect(&mut self) -> io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect((self.ip, self.port))?;
        self.sock = socket;

        Ok(())
    }

    pub fn new(ip: IpAddr, port: u16) -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect((ip, port))?;

        Ok(Self {
            sock: socket,
            ip,
            port,
            buffer: [0; 108],
        })
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.update(&[None, None, None, None]);
        _ = self.send();
    }
}
