use optee_teec::{
    Context, Operation, Uuid, ParamValue, ParamTmpRef, ParamType, ParamNone,
};
use proto::{UUID, Command};

use std::mem::transmute;

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let p0 = ParamValue::new(4, 6, ParamType::ValueInput);

    let res = &mut [0u8; 4];

    let r0 = ParamTmpRef::new_output(res);
    let mut operation = Operation::new(0, p0, r0, ParamNone, ParamNone);

    session.invoke_command(Command::Add as u32, &mut operation)?;


    println!("Success! {}", unsafe{ transmute::<[u8; 4], i32>(*res) });

    Ok(())
}
