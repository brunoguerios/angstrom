use angstrom_types::contract_payloads::angstrom::AngstromBundle;
use testing_tools::type_generator::consensus::proposal::ProposalBuilder;

#[test]
fn build_and_ship_random_bundle() {
    let proposal = ProposalBuilder::new()
        .for_random_pools(3)
        .for_block(10)
        .order_count(50)
        .preproposal_count(5)
        .build();
    let _bundle = AngstromBundle::from_proposal(&proposal, gas_details, pools).unwrap();
    Contr
}
