use ewasm_api::pdxabi;
use std::collections::HashMap;
pub static HELLO_WASM_ABI: &'static str = r#"
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
"#;
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
