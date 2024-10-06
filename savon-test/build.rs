use std::env;

fn main() {
    let _out_dir = env::var("OUT_DIR").unwrap();
    //let s = savon::gen::gen_write("../assets/example.wsdl", env!("OUT_DIR")).unwrap();
    let out_dir = "./";
    // let _s = savon::gen::gen_write("../assets/example.wsdl", &out_dir).unwrap();
    // let _s = savon::gen::gen_write(
    //     "https://www.onvif.org/ver10/device/wsdl/devicemgmt.wsdl",
    //     &out_dir,
    // )
    // .unwrap();
    let _s = savon::gen::gen_write("countrinfoservice.wsdl", &out_dir).unwrap();
}
