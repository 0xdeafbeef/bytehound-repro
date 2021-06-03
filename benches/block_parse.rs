use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ton_abi::function::Function;
use ton_block::{Block, Deserializable};

fn prepare() -> [Function; 1] {
    use ton_block::{Block, Deserializable};

    const abi: &str = r#"{
	"ABI version": 2,
	"header": ["pubkey", "time", "expire"],
	"functions": [
		{
			"name": "constructor",
			"inputs": [
				{"name":"owners","type":"uint256[]"},
				{"name":"reqConfirms","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "acceptTransfer",
			"inputs": [
				{"name":"payload","type":"bytes"}
			],
			"outputs": [
			]
		},
		{
			"name": "sendTransaction",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"value","type":"uint128"},
				{"name":"bounce","type":"bool"},
				{"name":"flags","type":"uint8"},
				{"name":"payload","type":"cell"}
			],
			"outputs": [
			]
		},
		{
			"name": "submitTransaction",
			"inputs": [
				{"name":"dest","type":"address"},
				{"name":"value","type":"uint128"},
				{"name":"bounce","type":"bool"},
				{"name":"allBalance","type":"bool"},
				{"name":"payload","type":"cell"}
			],
			"outputs": [
				{"name":"transId","type":"uint64"}
			]
		},
		{
			"name": "confirmTransaction",
			"inputs": [
				{"name":"transactionId","type":"uint64"}
			],
			"outputs": [
			]
		},
		{
			"name": "isConfirmed",
			"inputs": [
				{"name":"mask","type":"uint32"},
				{"name":"index","type":"uint8"}
			],
			"outputs": [
				{"name":"confirmed","type":"bool"}
			]
		},
		{
			"name": "getParameters",
			"inputs": [
			],
			"outputs": [
				{"name":"maxQueuedTransactions","type":"uint8"},
				{"name":"maxCustodianCount","type":"uint8"},
				{"name":"expirationTime","type":"uint64"},
				{"name":"minValue","type":"uint128"},
				{"name":"requiredTxnConfirms","type":"uint8"}
			]
		},
		{
			"name": "getTransaction",
			"inputs": [
				{"name":"transactionId","type":"uint64"}
			],
			"outputs": [
				{"components":[{"name":"id","type":"uint64"},{"name":"confirmationsMask","type":"uint32"},{"name":"signsRequired","type":"uint8"},{"name":"signsReceived","type":"uint8"},{"name":"creator","type":"uint256"},{"name":"index","type":"uint8"},{"name":"dest","type":"address"},{"name":"value","type":"uint128"},{"name":"sendFlags","type":"uint16"},{"name":"payload","type":"cell"},{"name":"bounce","type":"bool"}],"name":"trans","type":"tuple"}
			]
		},
		{
			"name": "getTransactions",
			"inputs": [
			],
			"outputs": [
				{"components":[{"name":"id","type":"uint64"},{"name":"confirmationsMask","type":"uint32"},{"name":"signsRequired","type":"uint8"},{"name":"signsReceived","type":"uint8"},{"name":"creator","type":"uint256"},{"name":"index","type":"uint8"},{"name":"dest","type":"address"},{"name":"value","type":"uint128"},{"name":"sendFlags","type":"uint16"},{"name":"payload","type":"cell"},{"name":"bounce","type":"bool"}],"name":"transactions","type":"tuple[]"}
			]
		},
		{
			"name": "getTransactionIds",
			"inputs": [
			],
			"outputs": [
				{"name":"ids","type":"uint64[]"}
			]
		},
		{
			"name": "getCustodians",
			"inputs": [
			],
			"outputs": [
				{"components":[{"name":"index","type":"uint8"},{"name":"pubkey","type":"uint256"}],"name":"custodians","type":"tuple[]"}
			]
		}
	],
	"data": [
	],
	"events": [
		{
			"name": "TransferAccepted",
			"inputs": [
				{"name":"payload","type":"bytes"}
			],
			"outputs": [
			]
		}
	]
}"#;

    let fns = [ton_abi::Contract::load(std::io::Cursor::new(abi))
        .unwrap()
        .function("submitTransaction")
        .unwrap()
        .clone()];
    fns
}

fn criterion_benchmark(c: &mut Criterion) {
    const block_boc :&str="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";
    let fns = prepare();
    let block = Block::construct_from_base64(block_boc).unwrap();

    c.bench_function("block_parse_one_fn", |b| {
        b.iter(|| indexer_lib::parse_block(black_box(&fns), black_box(&block)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);