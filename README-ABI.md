# hello-wasm-abi

>本文在 `wasm` 合约中使用 `ABI` 我们假设您已经阅读了 `README.md` 并掌握了 `hello-wasm` 例子工程<br>
>接下来我们会在此基础上加以修改，具体代码放在 `hello-wasm-abi` 目录中<br>

## 定义 ABI

>在 `hello-wasm-abi/src/abi.rs` 中定义了 Contract 对象，包含了 `hello-wasm` 样例中的 <br>
>`put/get/getcounter` 三个方法的 `ABI` 描述，注意，我们还不能直接用 `JSON` 来描述 `ABI`<br>
>必须使用 `ethabi::Contract` 来定义声明；

建议通过以下三步来生成 ABI : 

1. 使用 `solidity` 编写 `contract interface`;
1. 使用 `remix` 编译 `contract interface` 得到对应的 `ABI` 描述；
1. 参照 `ABI` 描述文件编写 `ethabi::Contract`；

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

### ethabi::Contract

根据 `JSON ABI` 描述实例化 `ethabi::Contract` 对象，用来对合约的 `input/output` 进行序列化和反序列化

```rust
pub fn get_contract_abi() -> ethabi::Contract {
    let mut functions: HashMap<String, ethabi::Function> = HashMap::new();
    let fn_put = ethabi::Function {
        constant: false,
        name: String::from("put"),
        inputs: Vec::from(vec![
            ethabi::Param { name: String::from("key"), kind: ethabi::param_type::ParamType::String },
            ethabi::Param { name: String::from("val"), kind: ethabi::param_type::ParamType::String },
        ]),
        outputs: Vec::default(),
    };
    let fn_get = ethabi::Function {
        constant: true,
        name: String::from("get"),
        inputs: Vec::from(vec![
            ethabi::Param { name: String::from("key"), kind: ethabi::param_type::ParamType::String },
        ]),
        outputs: Vec::from(vec![
            ethabi::Param { name: String::default(), kind: ethabi::param_type::ParamType::String },
        ]),
    };
    let fn_getcounter = ethabi::Function {
        constant: true,
        name: String::from("getcounter"),
        inputs: Vec::default(),
        outputs: Vec::from(vec![
            ethabi::Param { name: String::default(), kind: ethabi::param_type::ParamType::Uint(256) },
        ]),
    };
    functions.insert(fn_put.clone().name, fn_put.clone());
    functions.insert(fn_get.clone().name, fn_get.clone());
    functions.insert(fn_getcounter.clone().name, fn_getcounter.clone());
    ethabi::Contract {
        constructor: None,
        functions: functions,
        events: HashMap::default(),
        fallback: false,
        signers: HashMap::default(),
    }
}
```





