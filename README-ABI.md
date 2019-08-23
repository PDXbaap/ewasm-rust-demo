# hello-wasm-abi

>本文在 `wasm` 合约中使用 `ABI` 我们假设您已经阅读了 `README.md` 并掌握了 `hello-wasm` 例子工程<br>
>接下来我们会在此基础上加以修改，具体代码放在 `hello-wasm-abi` 目录中<br>

## 定义 ABI

在 `hello-wasm-abi/src/abi.rs` 中定义了 Contract 对象，包含了 `put/get/getcounter` 三个方法的 `ABI` 描述，
注意，我们还不能用 `JSON` 来描述 `ABI`，必须按照如下方式定义 `ABI`


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

与此对应的 JSON ABI 如下：

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


