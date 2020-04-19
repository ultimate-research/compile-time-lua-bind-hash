use compile_time_lua_bind_hash::lua_bind_hash;

#[test]
fn main() {
    assert_eq!(lua_bind_hash!("password"), 0xe58325ff3537c13a);
    assert_eq!(lua_bind_hash!("password1234"), 0xa26a67b576fe1e83);
    assert_eq!(lua_bind_hash!("FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X"), 0xa4d50a730e36970e);
}