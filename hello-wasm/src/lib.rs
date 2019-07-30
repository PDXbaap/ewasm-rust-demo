extern crate wasm_bindgen;
extern crate ewasm_api;

use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::pdx::utils::*;

// 为 counter 定义一个 32位的 key
const COUNTER_KEY: Bytes32 = Bytes32 { bytes: [255; 32] };

// 每次方法被调用时都会执行一个 counter++ 操作，在链上记录执行次数，以测试状态写入操作
// EEI 函数回调 storageLoad / storageStore
fn inc_counter() {
    // storage_load 为 eei 中提供的函数，约束 k/v 均为 32byte
    // 此处将获取 key 对应的 Bytes32 类型的 value
    let old_v = ewasm_api::storage_load(&COUNTER_KEY);
    // 此方法由 ewasm_api::pdx::utils 名称空间所提供
    // 用来将 32 byte 字节数组转换为对应的整型
    let old_i = bytes_to_uint(&old_v.bytes[..]);
    let new_i = old_i + 1;
    // 此方法由 ewasm_api::pdx::utils 名称空间所提供
    // 用来将 uint32 转换为 32 byte 数组
    let val = u32_to_bytes32(new_i as u32);
    let value = Bytes32 { bytes: val };
    // storage_store 为 eei 中提供的函数，约束 k/v 均为 32byte
    // 用来保存 k/v 到当前合约的状态库
    ewasm_api::storage_store(&COUNTER_KEY, &value);
}


// EEI 函数回调 storageLoad
fn get_counter() {
    let v = ewasm_api::storage_load(&COUNTER_KEY);
    // 如果向将合约的执行结果返回给调用方，不需要使用 return 也无需在方法签名中指明
    // 必须使用 eei 中规定的 finish_data 函数
    ewasm_api::finish_data(&v.bytes[..]);
}

fn put_data() {
    // input 格式为 "put:key,value"
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    // 将 input 分割为 ["put","key,value"]
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        // 将 "key,value" 分割为 ["key","value"]
        let sp: Vec<&str> = sd[1].split(",").collect();
        if sp.len() > 1 {
            let k = sp[0].trim();
            let v = sp[1].trim();
            // storage_store2 为 pdx 提供的扩展函数
            // 用来将不限制大小的 key / value 保存在合约状态中
            // 值得注意的是此方法的 gas 是以数据大小来计算的
            // 每 32byte 数据所使用的 gas 与 storage_store 相同
            ewasm_api::pdx::storage_store2(k.as_bytes(), v.as_bytes());
        }
    }
}

fn get_data() {
    // input 格式为 "get:key"
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    // 将 input 分割为 ["get","key"]
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        let k = sd[1].trim();
        // storage_load2 为 pdx 提供的扩展函数
        // 用来获取 key 对应的不限制大小的 value
        // 值得注意的是此方法的 gas 是以数据大小来计算的
        // 每 32byte 数据所使用的 gas 与 storage_store 相同
        let v: Vec<u8> = ewasm_api::pdx::storage_load2(k.as_bytes());
        // 将合约执行结果返回给调用端
        ewasm_api::finish_data(&v[..]);
    }
}


//fn constructor() {}

// 同 solidity 中的匿名函数，每次给这个合约转账时都会回调这个函数
// 如果需要使用匿名函数在收到转账时做特殊处理，则可实现这个函数
fn anonymous() {
    // TODO 不需要返回任何值
}

// 合约入口 : 必须使用 #[wasm_bindgen] 注解来声明导出 main 函数
#[wasm_bindgen]
pub fn main() {
    // 当合约通过 tx 调用时表示需要改变状态，此时计数器会加一，否则无效
    inc_counter();
    // 获取本次合约调用的 contract.input
    let input = ewasm_api::calldata_acquire();
    // 当 create 合约时 input 始终为空
    // 当给合约发送普通转账交易时，input 也应为空
    if !input.is_empty() {
        // 本 demo 使用了文本协议来序列化 input
        // 格式为: "目标函数:参数1,参数2,参数n"
        // 解析
        let data = match String::from_utf8(input) {
            Ok(s) => s,
            Err(e) => e.to_string(),// 也可以在此处终止合约
        };
        // 将 input 分割为 ["目标函数","参数1,参数2,参数n"]
        let sd: Vec<&str> = data.split(":").collect();
        // 通过这个匹配可以看出我们这个合约对外暴漏 3 个函数，函数名称不区分大小写：
        //      GETCOUNTER : 通过 eth_call 调用，用来获取计数器结果
        //      PUT : 通过 tx 向合约中添加一个 k/v 对，具体参数格式为 "put:k,v"
        //      GET : 通过 eth_call 调用，获取 k 对应的 v，具体参数格式为 "get:k"
        // 当方法名得不到匹配时，会返回 "METHOD_NOT_FOUND" 标识
        match sd[0].trim().to_uppercase().as_str() {
            "GETCOUNTER" => get_counter(),
            "PUT" => put_data(),
            "GET" => get_data(),
            _ => ewasm_api::finish_data(String::from("METHOD_NOT_FOUND").as_bytes()),
        }
    } else {
        // 当 input 为空时，调度匿名函数
        anonymous();
    }
}