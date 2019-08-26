# hello-wasm-sol

* 阅读本章前请先阅读并掌握 [hello-wasm](https://github.com/PDXbaap/ewasm-rust-demo/blob/master/README.md) 和 [hello-wasm-abi](https://github.com/PDXbaap/ewasm-rust-demo/blob/master/README-ABI.md) 两章的内容；
* 本章主要演示 wasm 合约与 solidity 合约互相调用；

# wasm 与 solidity 合约互相调用

在 hello-wasm-abi 合约接口的基础上增加两个方法，用来演示如何调用 `solidity` 合约，完整的合约接口定义如下：

```solidity

// wasm 合约接口
contract hello_wasm_abi {
    function getcounter() public view returns(uint256);
    
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
    // 调用 solidity 合约的 get 方法 
    function solget(address addr, string memory key) public view returns(string memory);
    // 调用 solidity 合约的 put 方法 
    function solput(address addr, string memory key,string memory val) public payable;
}        

// solidity 合约接口
contract hello_sol {
    
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
    // 调用 wasm 合约的 get 方法
    function wasmget(address addr, string memory key) public view returns(string memory);
    // 调用 wasm 合约的 put 方法
    function wasmput(address addr, string memory key,string memory val) public payable;
}
```

## hello_wasm_abi 接口实现

样例程序 `hello-wasm-sol` 是在 `hello-wasm-abi` 的基础上增加两个方法用来调用 `solidity` 合约，并且增加一个用来描述 `solidity` 合约接口的 `pdxabi::Contract` 对象，具体如下：

### abi.rs

使用 `pdxabi::Contract` 描述完整的 `hello_wasm_abi` 接口，因为我们要实现这个接口，对于 `hello_sol` 的描述只关注我们要使用的方法 `get/put` 即可

```rust
//src/abi.rs
use ewasm_api::pdxabi;
use std::collections::HashMap;

/// 目标 solidity 合约的接口描述，因为我们只调用 get/put 两个方法，所以这里只描述了两个方法
pub fn get_sol_contract_abi() -> pdxabi::Contract {
    let mut functions: HashMap<String, pdxabi::Function> = HashMap::new();
    let fn_put = pdxabi::Function {
        constant: false,
        name: String::from("put"),
        inputs: Vec::from(vec![
            pdxabi::Param { name: String::from("key"), kind: pdxabi::param_type::ParamType::String },
            pdxabi::Param { name: String::from("val"), kind: pdxabi::param_type::ParamType::String },
        ]),
        outputs: Vec::default(),
    };
    let fn_get = pdxabi::Function {
        constant: true,
        name: String::from("get"),
        inputs: Vec::from(vec![
            pdxabi::Param { name: String::from("key"), kind: pdxabi::param_type::ParamType::String },
        ]),
        outputs: Vec::from(vec![
            pdxabi::Param { name: String::default(), kind: pdxabi::param_type::ParamType::String },
        ]),
    };
    functions.insert(fn_put.clone().name, fn_put.clone());
    functions.insert(fn_get.clone().name, fn_get.clone());
    pdxabi::Contract {
        constructor: None,
        functions: functions,
        events: HashMap::default(),
        fallback: false,
        signers: HashMap::default(),
    }
}

pub fn get_contract_abi() -> pdxabi::Contract {
    let mut functions: HashMap<String, pdxabi::Function> = HashMap::new();
    let fn_put = pdxabi::Function {
        constant: false,
        name: String::from("put"),
        inputs: Vec::from(vec![
            pdxabi::Param { name: String::from("key"), kind: pdxabi::param_type::ParamType::String },
            pdxabi::Param { name: String::from("val"), kind: pdxabi::param_type::ParamType::String },
        ]),
        outputs: Vec::default(),
    };
    let fn_get = pdxabi::Function {
        constant: true,
        name: String::from("get"),
        inputs: Vec::from(vec![
            pdxabi::Param { name: String::from("key"), kind: pdxabi::param_type::ParamType::String },
        ]),
        outputs: Vec::from(vec![
            pdxabi::Param { name: String::default(), kind: pdxabi::param_type::ParamType::String },
        ]),
    };
    let fn_getcounter = pdxabi::Function {
        constant: true,
        name: String::from("getcounter"),
        inputs: Vec::default(),
        outputs: Vec::from(vec![
            pdxabi::Param { name: String::default(), kind: pdxabi::param_type::ParamType::Uint(256) },
        ]),
    };

    // 用来调用 solidity 合约 >>>>>>>>>>
    let fn_solput = pdxabi::Function {
        constant: false,
        name: String::from("solput"),
        inputs: Vec::from(vec![
            pdxabi::Param { name: String::from("addr"), kind: pdxabi::param_type::ParamType::Address },
            pdxabi::Param { name: String::from("key"), kind: pdxabi::param_type::ParamType::String },
            pdxabi::Param { name: String::from("val"), kind: pdxabi::param_type::ParamType::String },
        ]),
        outputs: Vec::default(),
    };
    let fn_solget = pdxabi::Function {
        constant: true,
        name: String::from("solget"),
        inputs: Vec::from(vec![
            pdxabi::Param { name: String::from("addr"), kind: pdxabi::param_type::ParamType::Address },
            pdxabi::Param { name: String::from("key"), kind: pdxabi::param_type::ParamType::String },
        ]),
        outputs: Vec::from(vec![
            pdxabi::Param { name: String::default(), kind: pdxabi::param_type::ParamType::String },
        ]),
    };
    // 用来调用 solidity 合约 <<<<<<<<<<

    functions.insert(fn_put.clone().name, fn_put.clone());
    functions.insert(fn_get.clone().name, fn_get.clone());
    functions.insert(fn_getcounter.clone().name, fn_getcounter.clone());
    functions.insert(fn_solput.clone().name, fn_solput.clone());
    functions.insert(fn_solget.clone().name, fn_solget.clone());
    pdxabi::Contract {
        constructor: None,
        functions: functions,
        events: HashMap::default(),
        fallback: false,
        signers: HashMap::default(),
    }
}
```

### lib.rs

实现部分只列出了与 `hello-wasm-abi` 不同的地方

```rust
......
fn callsol_put_data(a: &ewasm_api::pdxabi::Token, k: &ewasm_api::pdxabi::Token, v: &ewasm_api::pdxabi::Token) {
    let addr = a.clone().to_address().expect("error_address");
    let gas = ewasm_api::gas_left();
    let sol = abi::get_sol_contract_abi();
    let fn_put = sol.function("put").unwrap();
    let input_data = fn_put.encode_input(&[k.clone(), v.clone()]).expect("error_input");
    let value = &ewasm_api::types::EtherValue { bytes: [0; 16] };
    ewasm_api::call_mutable(gas, &ewasm_api::types::Bytes20 { bytes: addr.0 }, value, input_data.as_slice());
}

fn callsol_get_data(a: &ewasm_api::pdxabi::Token, k: &ewasm_api::pdxabi::Token) -> Vec<u8> {
    let addr = a.clone().to_address().expect("error_address");
    let gas = ewasm_api::gas_left();
    let sol = abi::get_sol_contract_abi();
    let fn_get = sol.function("get").unwrap();
    let input_data = fn_get.encode_input(&[k.clone()]).expect("error_input");
    let value = &ewasm_api::types::EtherValue { bytes: [0; 16] };
    let ret = ewasm_api::call_mutable(gas, &ewasm_api::types::Bytes20 { bytes: addr.0 }, value, input_data.as_slice());
    let mut output: Vec<u8> = Vec::new();
    // 获取返回值，只有 CallResult 为 Successful 时才会有返回值
    match ret {
        ewasm_api::CallResult::Successful => output = ewasm_api::returndata_acquire(),
        _ => ()
    }
    output
}

#[wasm_bindgen]
pub fn main() {
        ......
        match function.name.as_str() {
            ......
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
```

