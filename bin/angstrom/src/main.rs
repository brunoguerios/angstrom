// We use jemalloc for performance reasons
#[cfg(all(feature = "jemalloc", unix))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    if let Err(err) = angstrom::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use alloy::signers::SignerSync;
    use alloy::signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner};
    use alloy_primitives::{Bytes, U256, address, fixed_bytes};
    use angstrom_types::primitive::{ANGSTROM_DOMAIN, AngstromSigner};
    use angstrom_types::sol_bindings::RawPoolOrder;
    use angstrom_types::sol_bindings::ext::grouped_orders::{AllOrders, FlashVariants};
    use angstrom_types::sol_bindings::rpc_orders::{ExactFlashOrder, OmitOrderMeta, OrderMeta};
    use pade::PadeEncode;
    use serde_json::json;

    #[test]
    fn test_nina_sig() {
        let order_json = json!(
        {"Flash":{"Exact":{"ref_id":0,"exact_in":false,"amount":1000000000000000000u128,"max_extra_fee_asset0":0,"min_price":0,"use_internal":false,"asset_in":"0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238","asset_out":"0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984","recipient":"0x201f23a6197c396Acf7D6981287B18eba6A5078b","hook_data":"0x9D0ce8B3DF426008c4a4E74E7845B1bffF346a90","valid_for_block":7980190,"meta":{"isEcdsa":true,"from":"0x201f23a6197c396Acf7D6981287B18eba6A5078b","signature":"0xbe9b3cd490b318c543c7abb0545974d3562087585ebd2bcf00e7293586c12d447617e3078853c6b2f4ad4170ee6dc7aaded716dc558877bf9c75ced178f299031b"}}}}
                );
        let order: AllOrders = serde_json::from_value(order_json).unwrap();
        // assert!(order.is_valid_signature());
        let order = match order {
            AllOrders::Flash(FlashVariants::Exact(o)) => o,
            _ => unreachable!(),
        };
        let sk: PrivateKeySigner =
            "3dc39e5072e5c707efda706d39afadae72d12c9282b11a7ba6fbddb21d571092"
                .parse()
                .unwrap();
        let angstrom_signer = AngstromSigner::new(sk);

        println!("{ANGSTROM_DOMAIN:#?}");
        let order_hash = order.no_meta_eip712_signing_hash(&ANGSTROM_DOMAIN);
        let sig = angstrom_signer.sign_hash_sync(&order_hash).unwrap();
        let cur_decode = order.order_signature().unwrap();
        assert_eq!(sig, cur_decode, "signed {:#?}\n\n decoded {:#?}", sig, cur_decode);

        // let encoded: Bytes = sig.pade_encode().into();
        //
        // println!("{:#?}\n\n {encoded:x}", order);
    }
}
