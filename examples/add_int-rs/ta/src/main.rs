#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command,
    ta_open_session, trace_println,
    Parameters, Result,
    Error, ErrorKind,
};

use proto::Command;
use std::{io::Write, mem::transmute};

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

fn add(a0: i32, a1: i32) -> Result<i32> {
    let res = a0 + a1;
    trace_println!("{} + {} = {}", a0, a1, res);
    Ok(res)
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    let n0 = unsafe { params.0.as_value().unwrap() };
    let mut r0_ref = unsafe { params.1.as_memref().unwrap() };

    match Command::from(cmd_id) {
        Command::Add => {
            let res = add(n0.a() as i32, n0.b() as i32).unwrap();
            r0_ref.buffer().write(& unsafe{ transmute::<i32, [u8; 4]>(res) });
            Ok(())
        },
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }

}

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 32 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"Example of TA.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Big int TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
