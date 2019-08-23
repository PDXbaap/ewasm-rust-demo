extern crate wasm_bindgen;
extern crate ewasm_api;

use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::pdx::utils::*;

pub mod abi;

use ewasm_api::ethabi;
use crate::abi::get_contract_abi;

// 为 counter 定义一个 32位的 key
const COUNTER_KEY: Bytes32 = Bytes32 { bytes: [255; 32] };

/* 合约接口
contract hello_wasm_abi {
    function getcounter() public view returns(uint256);
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
}
*/
fn inc_counter() {
    let old_v = ewasm_api::storage_load(&COUNTER_KEY);
    let old_i = bytes_to_uint(&old_v.bytes[..]);
    let new_i = old_i + 1;
    let val = u32_to_bytes32(new_i as u32);
    let value = Bytes32 { bytes: val };
    ewasm_api::storage_store(&COUNTER_KEY, &value);
}

fn get_counter() -> Vec<u8> {
    let v = ewasm_api::storage_load(&COUNTER_KEY);
    Vec::from(&v.bytes[..])
}

fn put_data(k: String, v: String) {
    ewasm_api::pdx::storage_store(k.as_bytes(), v.as_bytes());
}

fn get_data(k: String) -> Vec<u8> {
    ewasm_api::pdx::storage_load(k.as_bytes())
}


#[wasm_bindgen]
pub fn main() {
    inc_counter();
    let input = ewasm_api::calldata_acquire();
    if !input.is_empty() {
        // 用 ethabi 解析 input
        //let mut contract = ethabi::Contract::load(abi::HELLO_WASM_ABI.as_bytes()).expect("error_abi");
        let mut contract = get_contract_abi();
        let fn_sig = &Vec::from(&input[..4]);
        let function = contract.function_by_sig(fn_sig).expect("error_fn_sig");
        match function.name.as_str() {
            "getcounter" => {
                let rtn = ewasm_api::pdx::utils::bytes_to_uint(get_counter().as_slice());
                let data = function.encode_output(&[ethabi::Token::Uint(rtn.into())]).unwrap();
                ewasm_api::finish_data(data.as_slice());
            }
            "get" => {
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
                let key = tokens.get(0).expect("error_put_key");
                let val = get_data(key.clone().to_string().unwrap());
                let rtn = String::from_utf8(val).expect("error_get_val");
                let data = function.encode_output(&[ethabi::Token::String(rtn)]).expect("error_get_output");
                ewasm_api::finish_data(data.as_slice());
            }
            "put" => {
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
                let key = tokens.get(0).expect("error_put_key");
                let val = tokens.get(1).expect("error_put_val");
                put_data(key.clone().to_string().unwrap(), val.clone().to_string().unwrap());
                ewasm_api::finish()
            }
            _ => ewasm_api::finish()
        }
    }
}