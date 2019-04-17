use optee_teec::{
    Context, Operation, ParamNone, ParamTmpRef, ParamType, ParamValue, Session, Uuid,
};
use proto::*;

const AES_TEST_BUFFER_SIZE: usize = 4096;
const AES_TEST_KEY_SIZE: usize = 16;
const AES_BLOCK_SIZE: usize = 16;

const DECODE: i8 = 0;
const ENCODE: i8 = 1;

fn prepare_aes(session: &mut Session, encode: i8) -> optee_teec::Result<()> {
    let p2_value = if encode == ENCODE {
        Mode::Encode as u32
    } else {
        Mode::Decode as u32
    };
    let p0 = ParamValue::new(Algo::CTR as u32, 0, ParamType::ValueInput);
    let p1 = ParamValue::new(TA_AES_SIZE_128BIT, 0, ParamType::ValueInput);
    let p2 = ParamValue::new(p2_value, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, p1, p2, ParamNone);

    session.invoke_command(Command::Prepare as u32, &mut operation)?;

    Ok(())
}

fn set_key(session: &mut Session, key: &mut [u8]) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(key, ParamType::MemrefTempInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::SetKey as u32, &mut operation)?;

    Ok(())
}

fn set_iv(session: &mut Session, iv: &mut [u8]) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(iv, ParamType::MemrefTempInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    session.invoke_command(Command::SetIV as u32, &mut operation)?;

    Ok(())
}

fn cipher_buffer(
    session: &mut Session,
    intext: &mut [u8],
    outtext: &mut [u8],
) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new(intext, ParamType::MemrefTempInput);
    let p1 = ParamTmpRef::new(outtext, ParamType::MemrefTempOutput);
    let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::Cipher as u32, &mut operation)?;

    Ok(())
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid =
        Uuid::parse_str(&include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../uuid.txt")).trim())
            .unwrap();
    let mut session = ctx.open_session(uuid)?;

    let mut key = [0xa5u8; AES_TEST_KEY_SIZE];
    let mut iv = [0x00u8; AES_BLOCK_SIZE];
    let mut clear = [0x5au8; AES_TEST_BUFFER_SIZE];
    let mut ciph = [0x00u8; AES_TEST_BUFFER_SIZE];
    let mut tmp = [0x00u8; AES_TEST_BUFFER_SIZE];

    println!("Prepare encode operation");
    prepare_aes(&mut session, ENCODE)?;

    println!("Load key in TA");
    set_key(&mut session, &mut key)?;

    println!("Reset ciphering operation in TA (provides the initial vector)");
    set_iv(&mut session, &mut iv)?;

    println!("Encode buffer from TA");
    cipher_buffer(&mut session, &mut clear, &mut ciph)?;

    println!("Prepare decode operation");
    prepare_aes(&mut session, DECODE)?;

    let mut key = [0xa5u8; AES_TEST_KEY_SIZE];
    println!("Load key in TA");
    set_key(&mut session, &mut key)?;

    let mut iv = [0x00u8; AES_BLOCK_SIZE];
    println!("Reset ciphering operation in TA (provides the initial vector)");
    set_iv(&mut session, &mut iv)?;

    println!("Decode buffer from TA");
    cipher_buffer(&mut session, &mut ciph, &mut tmp)?;

    if clear.iter().zip(tmp.iter()).all(|(a, b)| a == b) {
        println!("Clear text and decoded text match");
    } else {
        println!("Clear text and decoded text differ => ERROR");
    }
    Ok(())
}
