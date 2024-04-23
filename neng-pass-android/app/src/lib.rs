use jni::{JNIEnv, objects::{JString, JClass}};

#[no_mangle]
//                     Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_setMasterKey
pub extern "system" fn Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_setMasterKey(mut env: JNIEnv, _p_class: JClass, p_file: JString, p_master_key: JString) {
    let file_name = env.get_string(&p_file).unwrap().to_str().unwrap().to_string();
    let master_key = env.get_string(&p_master_key).unwrap().to_str().unwrap().to_string();
    neng_pass::set_master_key(&file_name, &master_key).unwrap();
}
