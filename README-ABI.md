# hello-wasm-abi

>本文在 `wasm` 合约中使用 `ABI` 我们假设您已经阅读了 [README.md](https://github.com/PDXbaap/ewasm-rust-demo/blob/master/README.md) 并掌握了 `hello-wasm` 例子工程<br>
>接下来我们会在此基础上加以修改，具体代码放在 `hello-wasm-abi` 目录中<br>

## 定义 ABI

>在 `hello-wasm-abi/src/abi.rs` 中定义了 Contract 对象，包含了 `hello-wasm` 样例中的 <br>
>`put/get/getcounter` 三个方法的 `ABI` 描述，注意，我们还不能直接用 `JSON` 来描述 `ABI`<br>
>必须使用 `pdxabi::Contract` 来定义声明；

建议通过以下三步来生成 ABI : 

1. 使用 `solidity` 编写 `contract interface`;
1. 使用 `remix` 编译 `contract interface` 得到对应的 `ABI` 描述；
1. 参照 `ABI` 描述文件编写 `pdxabi::Contract`；

部署 wasm 合约后可以使用合约地址和 contract interface 在 remix 里对合约进行实例化，方便测试

### Solidity Contract Interface

在 [Remix IDE](http://remix.ethereum.org/#optimize=false&version=soljson-v0.5.3+commit.10d17f24.js&evmVersion=null&appVersion=0.7.7) 中编写合约接口，并编译

```solidity
pragma solidity ^0.5.3;

contract hello_wasm_abi {
    function getcounter() public view returns(uint256);
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
}               
```

### JSON ABI

编译合约接口可以得到对应的 `ABI JSON` 描述，提供合约地址和此 `JSON ABI` 文档，
`DAPP` 开发者即可实例化 `hello_wasm_abi` 合约，并使用其中的三个函数

```json
[
	{
		"constant": false,
		"inputs": [
			{
				"name": "key",
				"type": "string"
			},
			{
				"name": "val",
				"type": "string"
			}
		],
		"name": "put",
		"outputs": [],
		"payable": true,
		"stateMutability": "payable",
		"type": "function"
	},
	{
		"constant": true,
		"inputs": [
			{
				"name": "key",
				"type": "string"
			}
		],
		"name": "get",
		"outputs": [
			{
				"name": "",
				"type": "string"
			}
		],
		"payable": false,
		"stateMutability": "view",
		"type": "function"
	},
	{
		"constant": true,
		"inputs": [],
		"name": "getcounter",
		"outputs": [
			{
				"name": "",
				"type": "uint256"
			}
		],
		"payable": false,
		"stateMutability": "view",
		"type": "function"
	}
]
```

### pdxabi::Contract

根据 `JSON ABI` 描述实例化 `pdxabi::Contract` 对象，用来对合约的 `input/output` 进行序列化和反序列化

```rust
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
    functions.insert(fn_put.clone().name, fn_put.clone());
    functions.insert(fn_get.clone().name, fn_get.clone());
    functions.insert(fn_getcounter.clone().name, fn_getcounter.clone());
    pdxabi::Contract {
        constructor: None,
        functions: functions,
        events: HashMap::default(),
        fallback: false,
        signers: HashMap::default(),
    }
}
```
## 使用 `ABI`

>在 hello-wasm-abi 合约中

```rust
extern crate wasm_bindgen;
extern crate ewasm_api;

use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::pdx::utils::*;

// 倒入处理 abi 的开发库
use ewasm_api::pdxabi;
// pdxabi::Contract 定义的对象放在 abi 模块中
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

#[wasm_bindgen]
pub fn main() {
    inc_counter();
    let input = ewasm_api::calldata_acquire();
    if !input.is_empty() {
    	// 获取 pdxabi::Contract 对象，这个函数写在 abi 模块中
        let mut contract = abi::get_contract_abi();
	// 从 input 获取方法签名，按照 ABI 规范，input 的前 4 个 byte 为方法签名
	let fn_sig = &Vec::from(&input[..4]);
	// 根据方法签名获取 function 对象
        let function = contract.function_by_sig(fn_sig).expect("error_fn_sig");
	// 通过 function.name 来匹配相应的 handler
	match function.name.as_str() {
            "getcounter" => { // function getcounter() public view returns(uint256);
	    	// 调用 get_counter 得到返回值，转换成 uint
                let rtn = ewasm_api::pdx::utils::bytes_to_uint(get_counter().as_slice());
		// 此方法没有输入值，只有输出，通过 function.encode_output 序列化输出 
		let data = function.encode_output(&[pdxabi::Token::Uint(rtn.into())]).unwrap();
		// 将结果返回给合约调用者
		ewasm_api::finish_data(data.as_slice());
            }
            "get" => { // function get(string memory key) public view returns(string memory);
		// 此方法有定义输入 string key , 先用 function.decode_input 解析 input, 得到输入列表	    
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
		// 接口中 input 只定义了一个参数，所以 key = tokens[0]
                let key = tokens.get(0).expect("error_put_key");
		// 调用 get_data(key) 函数，得到 val 的字节数组
                let val = get_data(key.clone().to_string().unwrap());
		// 接口描述输出值为 string，所以要将 val 转换为 string
		let rtn = String::from_utf8(val).expect("error_get_val");
		// 使用 function.encode_output 对返回值进行序列化
                let data = function.encode_output(&[pdxabi::Token::String(rtn)]).expect("error_get_output");
		// 将结果返回给合约调用者
                ewasm_api::finish_data(data.as_slice());
            }
            "put" => { // function put(string memory key,string memory val) public payable;
	    	// 此方法有定义输入 [string key,string val] , 先用 function.decode_input 解析 input, 得到输入列表
                let tokens = function.decode_input(input.as_slice()).expect("error_put_input");
		// 接口中定义了两个参数，分别对应 key = tokens[0] , val = tokens[1]
                let key = tokens.get(0).expect("error_put_key");
                let val = tokens.get(1).expect("error_put_val");
		// 调用 put_data(key,val)
                put_data(key.clone().to_string().unwrap(), val.clone().to_string().unwrap());
		// 结束调用，此方法没有返回值
		ewasm_api::finish()
            }
            _ => ewasm_api::finish() // 如果方法匹配失败，则直接返回不做任何处理
        }
    }
}
```
## 部署与使用

* 部署合约方式与 `hello-wasm` 样例相同，可以参照 [README.md](https://github.com/PDXbaap/ewasm-rust-demo/blob/master/README.md) 中关于`部署`的描述;

* 调用合约：部署成功后会得到 `Contract Address` ，如果使用 `web3` 系列 `SDK` 可以使用 `JSON ABI` + `Contract Address` 来实例化合约，并进行调用，如果使用 `remix IDE` 进行测试调用，可以使用 `Solidity Contract Interface` + `Contract Address` 来实例化合约并调用

关于 web3 提供的 SDK 和 remix IDE 的详细资料请参阅 web3 基金会的相关资料


## Solidity 调用 Wasm 合约

用 `sol` 合约来调用 `wasm` 合约，与 `sol` 调用 `sol` 方式相同，
假设已经部署过 `hello-wasm-abi` 这个合约，并得到合约地址 `0xda3ce11d916ffba4a1289cef66a7f142ec5a0f74`,
通过 `hello-wasm-abi` 合约接口和地址，即可实例化这个合约，之后用法与 `sol` 调用 `sol` 一致，
例如：

```solidity
pragma solidity ^0.5.3;

// hello-wasm-abi 合约接口
contract hello_wasm_abi {
    function getcounter() public view returns(uint256);
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
}        

// 使用 hello-wasm-abi 合约的 solidity 合约
contract foobar {
    
    function fetch(address addr,string memory key) public view returns(string memory) {
    	// 第一个参数 addr 为 wasm 合约地址，通过接口和地址实例化合约对象
        hello_wasm_abi hello = hello_wasm_abi(addr);
	// 调用 wasm 合约方法
        return hello.get(key);
    }
    
    function set(address addr,string memory key,string memory val) public payable {
        hello_wasm_abi hello = hello_wasm_abi(addr);
        hello.put(key,val);
    }

}
```

部署 `foobar` 合约后，使用 `hello-wasm-abi` 的合约地址 `0xda3ce11d916ffba4a1289cef66a7f142ec5a0f74` 作为第一个参数分别
调用 `fetch` 和 `set` 方法，完成对 `hello-wasm-abi` 合约的 `get` 和 `put` 的调用。 



