extern crate wasm_bindgen;
extern crate ewasm_api;

use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::pdx::utils::*;

use ewasm_api::pdxabi;
use crate::abi::get_sol_contract_abi;

use ewasm_api::prelude::debug;
use ewasm_api::debug::print_mem_hex;

pub mod abi;

const COUNTER_KEY: Bytes32 = Bytes32 { bytes: [255; 32] };

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

fn callsol_put_data(a: &ewasm_api::pdxabi::Token, k: &ewasm_api::pdxabi::Token, v: &ewasm_api::pdxabi::Token) {
    let addr = a.clone().to_address().expect("error_address");
    let gas = ewasm_api::gas_left();
    let sol = get_sol_contract_abi();
    let fn_put = sol.function("put").unwrap();
    let input_data = fn_put.encode_input(&[k.clone(), v.clone()]).expect("error_input");
    let value = &ewasm_api::types::EtherValue { bytes: [0; 16] };
    ewasm_api::call_mutable(gas, &ewasm_api::types::Bytes20 { bytes: addr.0 }, value, input_data.as_slice());
}

fn callsol_get_data(a: &ewasm_api::pdxabi::Token, k: &ewasm_api::pdxabi::Token) -> Vec<u8> {
    let addr = a.clone().to_address().expect("error_address");
    let gas = ewasm_api::gas_left();
    let sol = get_sol_contract_abi();
    let fn_get = sol.function("get").unwrap();
    let input_data = fn_get.encode_input(&[k.clone()]).expect("error_input");
    let value = &ewasm_api::types::EtherValue { bytes: [0; 16] };
    print_mem_hex(&addr.0[..]);
    print_mem_hex(&value.bytes[..]);
    let ret = ewasm_api::call_mutable(gas, &ewasm_api::types::Bytes20 { bytes: addr.0 }, value, input_data.as_slice());
    let mut output: Vec<u8> = Vec::new();
    match ret {
        ewasm_api::CallResult::Successful => output = ewasm_api::returndata_acquire(),
        _ => ()
    }
    output
}

#[wasm_bindgen]
pub fn main() {
    inc_counter();
    let input = ewasm_api::calldata_acquire();
    if !input.is_empty() {
        let mut contract = abi::get_contract_abi();
        let fn_sig = &Vec::from(&input[..4]);
        let function = contract.function_by_sig(fn_sig).expect("error_fn_sig");
        match function.name.as_str() {
            "getcounter" => {
                let rtn = ewasm_api::pdx::utils::bytes_to_uint(get_counter().as_slice());
                let data = function.encode_output(&[pdxabi::Token::Uint(rtn.into())]).unwrap();
                ewasm_api::finish_data(data.as_slice());
            }
            "get" => {
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
                let key = tokens.get(0).expect("error_put_key");
                let val = get_data(key.clone().to_string().unwrap());
                let rtn = String::from_utf8(val).expect("error_get_val");
                let data = function.encode_output(&[pdxabi::Token::String(rtn)]).expect("error_get_output");
                ewasm_api::finish_data(data.as_slice());
            }
            "put" => {
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
                let key = tokens.get(0).expect("error_put_key");
                let val = tokens.get(1).expect("error_put_val");
                put_data(key.clone().to_string().unwrap(), val.clone().to_string().unwrap());
                ewasm_api::finish();
            }
            "solput" => {
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
                let addrToken = tokens.get(0).unwrap();
                let keyToken = tokens.get(1).unwrap();
                let valToken = tokens.get(2).unwrap();
                callsol_put_data(&addrToken, &keyToken, &valToken);
                ewasm_api::finish();
            }
            "solget" => {
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
                let addrToken = tokens.get(0).unwrap();
                let keyToken = tokens.get(1).unwrap();
                let output = callsol_get_data(&addrToken, &keyToken);
                ewasm_api::finish_data(output.as_slice());
            }
            _ => ewasm_api::finish()
        }
    }
}