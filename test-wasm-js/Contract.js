const fs = require('fs')
const Web3 = require('web3')
const co = require('co')
const thunk = require('thunkify')
const Tx = require('ethereumjs-tx')
const EventEmitter = require('events').EventEmitter
const config = require('./config.js')

class Contract extends EventEmitter {
    constructor() {
        super()
        this.web3 = new Web3(new Web3.providers.HttpProvider(config.ethereumUri))
        this.getTransactionCount = thunk(this.web3.eth.getTransactionCount)
    }

    pub() {
        let _this = this
        let rs = fs.createReadStream(config.wasm_path)
        rs.on('data', function (data) {
            data = data.toString('hex')
            _this._sendTransaction('0x' + data)
        })
    }

    runReadMethod(data) {
        let _this = this
        return this.web3.eth.call({
            to: _this.contract_address,
            data: data
        })
    }

    runWriteMethod(data) {
        this._sendTransaction(data)
    }

    _sendTransaction(data) {
        let decryptObj = this.web3.eth.accounts.decrypt(config.keyStore, config.password)
        let address = decryptObj.address
        let privateKey = decryptObj.privateKey
        privateKey = Buffer.from(privateKey.substring(2), 'hex')
        let _this = this
        co(function* () {
            let nonce = yield _this.getTransactionCount(address, 'pending')
            let rawTransaction = {
                "from": address,
                "to": _this.contract_address ? _this.contract_address : '',
                "nonce": "0x" + nonce.toString(16),
                "gasPrice": _this.web3.utils.toHex(config.gasPrice),
                "gasLimit": _this.web3.utils.toHex(config.gasLimit),
                "data": data,
                "chainId": _this.web3.utils.toHex(config.chainId)
            }
            let tx = new Tx(rawTransaction)
            tx.sign(privateKey)
            let serializedTx = tx.serialize()
            let sign = '0x' + serializedTx.toString('hex')
            try {
                let date = new Date();
                let transaction = _this.web3.eth.sendSignedTransaction(sign)
                transaction.on('transactionHash', hash => {
                    console.log(`[${date.toLocaleString()}] hash ${hash}`);
                });
                transaction.on('receipt', receipt => {
                    if (receipt.contractAddress) {
                        _this.emit('contract_address', receipt.contractAddress)
                    } else {
                        _this.emit('runWriteMethod_success')
                    }
                });
            } catch (error) {
                console.log('sendSignedTransaction err:', error);
            }
        })
    }
}

module.exports = new Contract()