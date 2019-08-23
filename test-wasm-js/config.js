const config = {
		'ethereumUri': 'http://127.0.0.1:8545',
		'chainId': 738,
		'gasLimit': 15000000,
		'gasPrice': 18000000000,
		'keyStore': '{"address":"86082fa9d3c14d00a8627af13cfa893e80b39101","crypto":{"cipher":"aes-128-ctr","ciphertext":"71932cbcfdb4484433393044c0114aec0e737e7eeac908ec5edb23051c1e6e90","cipherparams":{"iv":"42424805dfad0ae0d8f08af898b56a03"},"kdf":"scrypt","kdfparams":{"dklen":32,"n":262144,"p":1,"r":8,"salt":"5946638ccdf2e18f206ffbc86f7d1ffe8d91f4be904c07dae716c58cf5789802"},"mac":"d419b9583c16dd04fff155a1b946b6ec749954459cc745c70ce59742ac332809"},"id":"900ab389-4085-44a0-baa7-e14ab929e5fd","version":3}',
		'password': '123456',
		'wasm_path': '/app/rusthome/hello-wasm/pkg/hello_wasm_bg.wasm',
		'methods': {'put': 'put:{},{}', 'get': 'get:{}', 'GetCounter': 'GetCounter'}
}

module.exports = config
