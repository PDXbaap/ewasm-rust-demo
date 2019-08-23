use ewasm_api::ethabi;
use ethabi::token::{Tokenizer, LenientTokenizer};
use ewasm_api::ethabi::hex::{ToHex, FromHex};
use crate::abi::get_contract_abi;
//use hex::{ToHex, FromHex};

/*
合约接口

contract hello_wasm_abi {
    function getcounter() public view returns(uint256);
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
}
*/
pub mod abi;


/// 测试 ABI 编解码
fn main() {
    //let mut contract = ethabi::Contract::load(abi::HELLO_WASM_ABI.as_bytes()).unwrap();
    let mut contract = get_contract_abi();
    println!("contract = {:#?}", contract);
    println!("--------------------- test_put_input_encode --------------------------------------->");
    let put_input_data = test_put_input_encode(&mut contract);
    println!("--------------------- test_put_input_decode --------------------------------------->");
    let val = test_put_input_decode(&mut contract, &put_input_data);
    println!("--------------------- test_get_output_encode -------------------------------------->");
    let get_output_data = test_get_output_encode(&mut contract, &val);
    println!("--------------------- test_getcounter_output_encode ------------------------------->");
    test_getcounter_output_encode(&mut contract);
}

fn test_getcounter_output_encode(contract: &mut ethabi::Contract) {
    let fn_getcounter = contract.function("getcounter").unwrap();
    let output_data: Vec<u8> = vec![255, 255];
    let n = ewasm_api::pdx::utils::bytes_to_uint(output_data.as_slice());
    println!("{:?} --> {}", output_data, n);
    let r = fn_getcounter.encode_output(&[ethabi::Token::Uint(n.into())]).unwrap();
    println!("r={:?}", r);
}

fn test_get_output_encode(contract: &mut ethabi::Contract, val: &Vec<u8>) -> Vec<u8> {
    let fn_get = contract.function("get").unwrap();
    println!("fn_get = {:?}", fn_get);
    let v = String::from_utf8(val.clone()).unwrap();
    println!("val = {:?}", v);

    let values = vec![v];

    let params: Vec<_> = fn_get.outputs.iter()
        .map(|p| p.kind.clone())
        .zip(values.iter().map(|s| s as &str))
        .collect();
    println!("params = {:?}", params);
    let tokens_result: Result<Vec<ethabi::Token>, ethabi::Error> = params.iter()
        .map(|&(ref k, v)| LenientTokenizer::tokenize(k, v))
        .collect::<Result<_, _>>()
        .map_err(From::from);
    let tokens = tokens_result.unwrap();
    println!("tokens = {:?}", tokens);
    let output_data = fn_get.encode_output(tokens.as_slice()).unwrap();
    let output_hex: String = output_data.to_hex();
    println!("output_data = {:?}", output_data);
    println!("output_hex = {:?}", output_hex);
    output_data
}

// 模拟执行合约时，解析 input data 的过程
fn test_put_input_decode(contract: &mut ethabi::Contract, input: &Vec<u8>) -> Vec<u8> {
    let sig = &input[..4];
    let fn_put = contract.function_by_sig(Vec::from(sig).as_ref()).unwrap();
    println!("fn_put = {:?}", fn_put);
    // 我们是清楚 put 函数的输入值的，是 string 类型的 k 和 v
    let values = fn_put.decode_input(input.as_slice()).unwrap();
    let k_token = values.get(0).unwrap();
    let v_token = values.get(1).unwrap();
    let k = k_token.clone().to_string().unwrap();
    let v = v_token.clone().to_string().unwrap();
    println!("k={:?} , v={:?}", k, v);
    println!("k={:?} , v={:?}", k.as_bytes(), v.as_bytes());
    Vec::from(v.as_bytes())
}

fn test_put_input_encode(contract: &mut ethabi::Contract) -> Vec<u8> {
    contract.functions.iter().for_each(|(_, f)| {
        println!("{:?}", f);
    });
    let fn_put = contract.function("put").unwrap();
    let key: String = String::from("foobar");
    let val: String = String::from("world");
    let values = vec![key, val];
    let params: Vec<_> = fn_put.inputs.iter()
        .map(|p| p.kind.clone())
        .zip(values.iter().map(|v| v as &str))
        .collect();
    println!("params = {:?}", params);

    let tokens_result: Result<Vec<ethabi::Token>, ethabi::Error> = params.iter()
        .map(|&(ref k, v)| LenientTokenizer::tokenize(k, v))
        .collect::<Result<_, _>>()
        .map_err(From::from);
    let tokens = tokens_result.unwrap();
    println!("tokens = {:?}", tokens);
    let input_data = fn_put.encode_input(tokens.as_slice()).unwrap();
    println!("input_data = {:?}", input_data);
    let input_hex: String = input_data.to_hex();
    println!("input_hex = {:?}", input_hex);
    return input_data;
}