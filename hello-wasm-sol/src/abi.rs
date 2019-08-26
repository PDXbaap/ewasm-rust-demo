use ewasm_api::pdxabi;
use std::collections::HashMap;

/*
contract hello_sol {
    function get(string memory key) public view returns(string memory);
    function put(string memory key,string memory val) public payable;
}
*/
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
