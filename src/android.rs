// use jni::{InitArgsBuilder, JNIVersion, JavaVM};
use log::info;

pub fn try_jni_call() -> Result<(), jni::errors::Error> {
    info!("mark0!");
    let android_context = ndk_context::android_context();
    info!("mark1!");
    let vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    info!("mark2!");
    let mut env = vm.attach_current_thread()?;
    info!("mark3!");
    // let context = unsafe { jni::objects::JObject::from_raw(android_context.context().cast()) };

    // let class = env.find_class("java/lang/String")?;
    let string = env.new_string("Hello, world!")?;
    info!("mark4!");
    let length = env.call_method(string, "length", "()I", &[])?;
    info!("mark5!");

    info!("{:?}", length);
    Ok(())
}

// pub fn try_jni_call() -> Result<(), jni::errors::Error> {
//     info!("mark0 ROPO 🟠!");
//     let jvm_args = InitArgsBuilder::new()
//         .version(JNIVersion::V4)
//         .option("-Xcheck:jni")
//         // .option(&InitArgsOption::OptionUseBundledJNI, "true")
//         .build()
//         .unwrap();

//     info!("mark1!");
//     let jvm = JavaVM::new(jvm_args).expect("Could not create jvm");

//     info!("mark2!");
//     let mut env = jvm.attach_current_thread()?;
//     info!("mark3!");

//     let string = env.new_string("Hello, world!")?;
//     info!("mark4!");
//     let length = env.call_method(string, "length", "()I", &[])?;
//     info!("mark5!");

//     info!("{:?}", length);
//     Ok(())
// }
