use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    //let s = savon::gen::gen_write("../assets/example.wsdl", env!("OUT_DIR")).unwrap();
    //let s = savon::gen::gen_write("../assets/example.wsdl", &out_dir).unwrap();
    let s = savon::gen::gen_write("https://www.onvif.org/ver10/device/wsdl/devicemgmt.wsdl", &out_dir).unwrap();
}
