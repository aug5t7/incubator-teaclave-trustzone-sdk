use optee_teec::{
    Context, Operation, Uuid, ParamValue, ParamType, ParamNone,
};
use proto::{UUID, Command};

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let p0 = ParamValue::new(4, 6, ParamType::ValueInput);


    let r0 = ParamValue::new(0, 0, ParamType::ValueOutput);
    let mut operation = Operation::new(0, p0, r0, ParamNone, ParamNone);

    session.invoke_command(Command::Add as u32, &mut operation)?;


    println!("Success! {} + {} = {}", 4, 6, operation.parameters().1.a() as i32);

    Ok(())
}
