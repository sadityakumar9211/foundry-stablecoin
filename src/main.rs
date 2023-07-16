#![allow(unused)] // For beginners only

use crate::prelude::*;

use std::fs::File;
use std::io::Read;
use std::net::Ipv4Addr;
use std::result;

mod error;
mod prelude;
mod utils;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/// A convenient method of manipulating the packets
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    /// A fresh buffer for holding the packets content and a
    /// field for keeping track of where we are.

    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    /// Current position within buffer
    fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;

        Ok(())
    }

    /// Change the buffer position
    fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    fn read(&mut self) -> Result<u8> {
        if self.pos >= 512 {
            return Err("End fo buffer".into());
        }

        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    fn get(&mut self, pos: usize) -> Result<u8> {
        if pos >= 512 {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len as usize])
    }

    /// Read two bytes, stepping two steps forward
    fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(res)
    }

    /// Read a question name
    ///
    /// The tricky part: Reading domain names, taking lables into considerating: each lable is compressed (jump notation) and has a single byte indicating the length of the label
    /// Will take something like [3]www[6]saditya[6]vercel[3]app and append
    /// www.saditya.vercel.app to outstr
    ///
    fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        /// As there might be jumps, we have to keep track of local position apart from the position in the Packet Buffer struct.
        /// This allows us to move the shared position to a point past our current question name (qname), while keeping track of
        /// our progress on the current qname using this variable.
        let mut pos = self.pos();

        // track whether or not we've jumped
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        /// Our delimiter which we append for each label. Since we don't the dot at the beginning of the domain name, we'll leave it empty now and set it to '.' at the end of the first iteration.
        ///
        let mut delim = "";
        loop {
            /// DNS Packets are untrusted data, so we need to be paranoid. Someone can craft a packet with a cycle in the jump instructions.
            /// This guards against such attacks.
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            /// Every label starts with a length byte
            let len = self.get(pos)?;

            /// If len has the two most singnificant bit set, it represents a jump to some other offset in the packet:
            if (len & 0xC0) == 0xC0 {
                /// Update the buffer position to a point past the current label. We don't need to touch it any further.
                if !jumped {
                    self.seek(pos + 2);
                }

                /// Read another byte, calculate offset and perform the jump by updating our local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                // Indicate that a jump was performed
                jumped = true;
                jumps_performed += 1;

                continue;
            }
            // The base scenario, where we're reading a single label and appending it to the output.
            else {
                // Move a single byte forward to move past the length byte.
                pos += 1;

                /// Domain names are terminated by an empty label of length 0, ,
                /// so if the length is zero we're done.
                if len == 0 {
                    break;
                }

                // Append the delimiter to our output buffer first.
                outstr.push_str(delim);

                /// Extract the actual ASCII bytes for this label and append them to the output buffer.
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delim = ".";

                // Move forward the full length of the label.
                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// The enum defining the values of `rescode` field: Result of a DNS query.
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResultCode {
    pub fn from_num(num: u8) -> ResultCode {
        match num {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0 | _ => ResultCode::NOERROR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16, // 16 bits

    pub recursion_desired: bool,    //1 bit
    pub truncated_message: bool,    //1 bit
    pub authoritative_answer: bool, //1 bit
    pub opcode: u8,
    pub response: bool,

    pub rescode: ResultCode,       // 4 bits
    pub checking_disabled: bool,   // 1 bit
    pub authed_data: bool,           // 1bit
    pub z: bool,                   // 1 bit
    pub recursion_available: bool, // 1bit

    pub questions: u16,             //16 bits
    pub answers: u16,               // 16 bits
    pub authoritative_entries: u16, //16 bits
    pub resource_entries: u16,      //16 bits
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,
            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,
            rescode: ResultCode::NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,
            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;
        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.response = (a & (1 << 7)) > 0;

        self.rescode = ResultCode::from_num(b & 0x0F);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        // Return the constant header size
        Ok(())
    }
}


fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
