## PDX Utopia Ewasm Specification

The ewasm specification was defined by the ethereum project, which the PDX Utopia blockchain strictly follows:

### Data Types

No floating point numbers and compatible with the data types as defined by EVM, e.g.

* bytes : variable length byte array
* address : 160-bit digit，stored in memory as 20-byte little endian unsigned integer
* u128 : 128-bit digit，stored in memory as 16-byte little endian unsigned integer
* u256 : 256-bit digit，stored in memory as 32-byte little endian unsigned integer

### d-App Format

Each d-App must be stored as wasm byte array

### Imported Modules

The allowed importable modules are limited to what EEI provided. The only module out of the ethereum namespace is debug, which is forbidden in production environment. 

### Exported Functions 

Each d-App must export two and only two functions

* memory :  Shared memory that EEI writes to 
* main :  The entrypoint function with no input parameter and no return, to be executed by the VM

The `wasm`  `start function` must be closed，because enabling it would hinder `ewasm` accquiring the memory address of the d-App before starting it


### About ABI

>Note that the spec on exported functions is different from what the `ABI` for `solidity` defines.<br>
>`solidity` d-App's function is called according to the `ABI` generated from the function signature，<br>
>which appears impossible with `ewasm` , because only one `main` function is exported， <br>
>How to use functions other than `main`? Natually we use the `input` of the d-App to  define <br>
>destination function and input parameters, which in fact is what `solidity` does, just we return this flexibility <br>
>to the developers to implement. Use the unform `main` function as entrypoint and encapsulate the `input` <br>
>In the examples later we can see more flexible options. <br>

## Development Environment

PDX Utopia supports using Rust as ewasm d-App programming language and uses Rust toolchain to compile a `ewasm` de-App. 

1. Install Rustup

```
curl https://sh.rustup.rs -sSf | sh
```

>Note that `nightly` channel must be selected on installer execution. Otherwise, installation of follow-up tools will fail. 
>If there's path issues during installation, please manually set envoirnment : `source $HOME/.cargo/env`

2. Install Rust standard library

```
rustup component add rust-src
```

3. Install wasm-pack compiler tools

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

4. Install wasm Post-processing tools

```bash
$> git clone https://github.com/PDXbaap/wasm-chisel.git
$> cd wasm-chisel
$> cargo build --release
```

If compiled successfully, executable `chisel` will be at `target/release` directory. Please make sure to add it to `$PATH`


__For more info about Rust, please refer to：__

https://www.rust-lang.org/learn/get-started


## PDX WASM Example d-App

Please make sure the development environment has been set up, as we are going to use `cargo` to create a d-App, use `wasm-pack` to compile it, and use `chisel`  to prost-process it.

### Create hello-wasm d-App

Assuming the working directory is `/app/rusthome` ，enter this directory on a terminal:

```bash
$> cargo new --lib hello-wasm
$> cd hello-wasm
$> touch chisel.yml
```

Edit the `chisel.yml` file and fill the folloing content. Here, the `file` attribute is the binary executable that `hello-wasm` will be compiled into.

```yml
hello:
  file: "pkg/hello_wasm_bg.wasm"
  remapimports:
    preset: "ewasm"
  trimexports:
    preset: "ewasm"
  verifyimports:
    preset: "ewasm"
  verifyexports:
    preset: "ewasm"
  repack:
    preset: "ewasm"
``` 

### Add Dependencies

A `wasm` d-App must include at least two development packages, `ewasm-rust-api` and `wasm-bindgen` ，
the former provides `api` to interact with `PDX Utopia` and the latter is responsible to compile Rust into wasm ; 

`PDX Utopia` extends the `EEI`，so the `ewasm-rust-api` provided by `PDX` must be used instead.

Edit the `hello-wasm/Cargo.toml` file，adding the dependencies to the `dependencies` section and configure `profile.release`  to optimize the compilation result.

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["liangchuan <cc14514@icloud.com>"]
edition = "2018"
publish = false

[dependencies]
wasm-bindgen = "0.2"
ewasm_api = { git = "https://github.com/PDXbaap/ewasm-rust-api", tag = "0.9" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = 'z'
debug = false
rpath = false
debug-assertions = false
codegen-units = 1
lto = true
```

Use `cargo check` to check and download the dependencies:

```bash
$> cargo check
    Updating crates.io index
   Compiling proc-macro2 v0.4.30
   Compiling unicode-xid v0.1.0
   Compiling syn v0.15.42
   Compiling wasm-bindgen-shared v0.2.48
   Compiling log v0.4.8
   Compiling cfg-if v0.1.9
   Compiling lazy_static v1.3.0
   Compiling libc v0.2.60
   Compiling bumpalo v2.5.0
    Checking void v1.0.2
   Compiling wee_alloc v0.4.4
   Compiling wasm-bindgen v0.2.48
    Checking memory_units v0.4.0
    Checking unreachable v1.0.0
    Checking ewasm_api v0.9.0 (/app/rusthome/ewasm-rust-api)
   Compiling quote v0.6.13
   Compiling wasm-bindgen-backend v0.2.48
   Compiling wasm-bindgen-macro-support v0.2.48
   Compiling wasm-bindgen-macro v0.2.48
    Checking hello-wasm v0.1.0 (/private/tmp/hello-wasm)
    Finished dev [unoptimized + debuginfo] target(s) in 29.62s
```

### Code the `ewasm` d-App

The prep work to develop a `ewasm` d-App is done. Now we are going to implement a `put/get` function set in the d-App, a simple counter to demonstrate how to use the d-App to store `k/v` and uses `k` to get `v`, and how to use `contract.input` to call specific functions. 

Edit the `hello-wasm/src/lib.rs` file to add real code.

```rust
extern crate wasm_bindgen;
extern crate ewasm_api;

use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::pdx::utils::*;

// Define a 32-byte counter key 
const COUNTER_KEY: Bytes32 = Bytes32 { bytes: [255; 32] };

// increment counter on each call. The count is recorded on the blockchain to test the state write op
// The EEI callback functions storageLoad / storageStore are demonstrated.
fn inc_counter() {
    // storage_load is a function provided by EEI，which limits the k/v to 32 bytes each. 
    // Get the value of type Bytes32 corresponding to key
    let old_v = ewasm_api::storage_load(&COUNTER_KEY);
    // this function is provided by the ewasm_api::pdx::utils namespace
    // which converts a 32-byte array to the corresponding integer
    let old_i = bytes_to_uint(&old_v.bytes[..]);
    let new_i = old_i + 1;
    // this function is provided by the ewasm_api::pdx::utils namespace
    // which concers a uint32 to 32-byte array
    let val = u32_to_bytes32(new_i as u32);
    let value = Bytes32 { bytes: val };
    // storage_store is provided by EEI , which limit the k/v to  32 bytes each
    // which is used to store the k/v to the current state db of the d-App
    ewasm_api::storage_store(&COUNTER_KEY, &value);
}


// The EEI callback function storageLoad is demonstrated.
fn get_counter() {
    let v = ewasm_api::storage_load(&COUNTER_KEY);
    // To return the execution result to the caller, use the finis_data function provided by EEI.
    ewasm_api::finish_data(&v.bytes[..]);
}

fn put_data() {
    // input format: "put:key,value"
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    // Split the input into ["put","key,value"]
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        // Split the "key,value" into ["key","value"]
        let sp: Vec<&str> = sd[1].split(",").collect();
        if sp.len() > 1 {
            let k = sp[0].trim();
            let v = sp[1].trim();
            // storage_store is a PDX-extended function
            // which stores arbituary sized key / value into the d-App state db
            // Note that the gas consumption is based on the size of the data
            // and for  evey 32-byte data the gas consumption is the same as ewasm_api::storage_store
            ewasm_api::pdx::storage_store(k.as_bytes(), v.as_bytes());
        }
    }
}

fn get_data() {
    // input format: "get:key"
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    // Split the input into ["get","key"]
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        let k = sd[1].trim();
        // storage_load is a PDX-extended function
        // which loads arbiturary sized value corresponding to a key
        // Note that the gas consumption is based on the size of the data
        // and for  evey 32-byte data the gas consumption is the same as ewasm_api::storage_load
        let v: Vec<u8> = ewasm_api::pdx::storage_load(k.as_bytes());
        // Return the execution result to the caller
        ewasm_api::finish_data(&v[..]);
    }
}


//fn constructor() {}

// Same as the Solidity anonymous function，called each time the d-App receives a balance transfer. 
// Implement it if special processing is desired on receiving a balance transfer 
fn anonymous() {
    // TODO No need to return anything  
}

// Entrypoint : must use #[wasm_bindgen] annotation to declare and export main function
#[wasm_bindgen]
pub fn main() {
    // Increment the counter whenever the d-App is called via a TX.
    inc_counter();
    // Acquire the contract.input of this TX
    let input = ewasm_api::calldata_acquire();
    // The input is empty when the d-App is first created
    // The input is also empty when sending normal balance transfer TXs
    if !input.is_empty() {
        // This demo uses simple text schema to serialize the  input
        // The format is: : "function:parameter-1,parameter-2,..."
        // Resolve it
        let data = match String::from_utf8(input) {
            Ok(s) => s,
            Err(e) => e.to_string(),// OK to terminate execution here.
        };
        // Split the input into ["function","parameter-1,parameter-2,..., parameter-n"]
        let sd: Vec<&str> = data.split(":").collect();
        // From this matching we can see this d-App exposes three functions and the names are case-insensitive.：
        //      GETCOUNTER : via eth_call to retrieve the counter result 
        //      PUT : Via a TX to add a k/v pair to the d-App state, using "put:k,v" format
        //      GET : Via eth_call to retrive the v corresponding to k, using "get:k" format
        // Where there's no match, "METHOD_NOT_FOUND"  is returned.
        match sd[0].trim().to_uppercase().as_str() {
            "GETCOUNTER" => get_counter(),
            "PUT" => put_data(),
            "GET" => get_data(),
            _ => ewasm_api::finish_data(String::from("METHOD_NOT_FOUND").as_bytes()),
        }
    } else {
        // When input is empty, call the anonymous function
        anonymous();
    }
}
```


### Compile the `ewasm` d-App

On a terminal, enter the `hello-wasm` project directory，compile and post-process the d-App.

```bash
$> wasm-pack build
$> chisel run
```

The above steps will create the `hello_wasm_bg.wasm` file in `pkg` directory，now we are deploying the `ewasm` d-App to a `PDX Utopia` blockchain.


### Deploy & execute the d-App

We can use `web3.js` to deploy and use a `ewasm` d-App on `PDX Utopia`. Here is a simple demo on deploying `hello-wasm` and using the three functions it exposed. 

First download the demo code from [test-wasm-js](https://github.com/PDXbaap/ewasm-rust-demo/tree/master/test-wasm-js) .

The code depends on `nodejs` & `npm`，after download them do `npm install` to install dependencies and modify `config.js` to fill in the correct parameters to deploy and call the d-App via the following steps.

#### config.js

Configuration：

* ethereumUri :  The RPC entry point of a `PDX Utopia` blockchain
* chainId : Use admin.nodeInfo to view the chainId, must be correctly filled
* gasLimit : gas limit
* gasPrice : gas price in wei
* keyStore : The json keystore 
* password : The password used to protect the private key
* wasm_path : the filepath of the `ewasm` d-App
* methods : The functions the d-App exposes, similar to the ABI definitions

For example：

```javascript
const config = {
    'ethereumUri': 'http://127.0.0.1:8545', 
    'chainId': 738,
    'gasLimit': 15000000,
    'gasPrice': 18000000000,
    'keyStore': '{"address":"86082fa9d3c14d00a8627af13cfa893e80b39101","crypto":{"cipher":"aes-128-ctr","ciphertext":"71932cbcfdb4484433393044c0114aec0e737e7eeac908ec5edb23051c1e6e90","cipherparams":{"iv":"42424805dfad0ae0d8f08af898b56a03"},"kdf":"scrypt","kdfparams":{"dklen":32,"n":262144,"p":1,"r":8,"salt":"5946638ccdf2e18f206ffbc86f7d1ffe8d91f4be904c07dae716c58cf5789802"},"mac":"d419b9583c16dd04fff155a1b946b6ec749954459cc745c70ce59742ac332809"},"id":"900ab389-4085-44a0-baa7-e14ab929e5fd","version":3}',
    'password': '123456',
    'wasm_path': '/Users/buyanping/Desktop/hello_wasm_bg.wasm',
    'methods': {'put': 'put:{},{}', 'get': 'get:{}', 'GetCounter': 'GetCounter'}
}
module.exports = config
```


#### index.js 

The entrypoint to the test program, use `npm test` to execute this script after correctly configured it

```javascript

function test_put(...params) {
    let put_method = config.methods['put']
    put_method = format(put_method, params[0], params[1])
    let data = web3.utils.toHex(put_method)
    contract.runWriteMethod(data)
}

function test_get(key) {
    let get_method = config.methods['get']
    get_method = format(get_method, key)
    let data = web3.utils.toHex(get_method)
    contract.runReadMethod(data).then(value => {
        console.log(`value==>${web3.utils.hexToString(value)}`)
        test_GetCounter()
    })
}

function test_GetCounter() {
    let getcounter_method = config.methods['GetCounter']
    let data = web3.utils.toHex(getcounter_method)
    contract.runReadMethod(data).then(counter => {
        console.log(`counter==>${web3.utils.hexToNumber(counter)}`)
    })
}

//Deploy and call the three functions of the d-App
//This test deploys hello-wasm on each call. 
//If multiple executions is desired, just modify it
function test() {
    // 2: Triggered after successful deployment, via put(key,val) method to put state
    contract.once('contract_address', contract_address => {
        console.log(`contract_address==>${contract_address}`)
        contract.contract_address = contract_address
        test_put('foo', 'bar')
    })

    // 3: Triggered after successful put call，via get(key) function to retrive the state just put
    // And retrieve the state change conter via the GetCounter function
    contract.once('runWriteMethod_success', ()=>{
        test_get('foo')
    })
    // 1: Deploy the d-App
    contract.pub()
}

//Entrypoint of the script
test()
```
