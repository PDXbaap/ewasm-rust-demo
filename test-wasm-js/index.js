const contract = require('./Contract')
const format = require('string-format')
const Web3 = require('web3')
const web3 = new Web3()
const config = require('./config')

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

function test() {
    contract.once('contract_address', contract_address => {
        console.log(`contract_address==>${contract_address}`)
        contract.contract_address = contract_address
        test_put('foo', 'bar')
    })
    contract.once('runWriteMethod_success', ()=>{
        test_get('foo')
    })
    contract.pub()
}

test()