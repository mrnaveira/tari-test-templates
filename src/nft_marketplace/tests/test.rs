use tari_template_lib::args;
use tari_template_lib::models::{
    Amount, Bucket, ComponentAddress, NonFungibleAddress, ResourceAddress,
};
use tari_template_test_tooling::crypto::RistrettoSecretKey;
use tari_template_test_tooling::TemplateTest;
use tari_transaction::Transaction;

#[test]
fn it_compiles() {
    let TestSetup {
        mut test,
        marketplace_component,
        admin_account,
        admin_proof,
        admin_key,
        ..
    } = setup();

    /*
    let result = test.execute_expect_success(
        Transaction::builder()
            .call_method(marketplace_component, "take_free_coins", args![])
            .put_last_instruction_output_on_workspace("bucket")
            .call_method(admin_account, "deposit", args![Workspace("bucket")])
            .call_method(marketplace_component, "total_supply", args![])
            .sign(&admin_key)
            .build(),
        vec![admin_proof.clone()],
    );

    let total_supply = result.finalize.execution_results[3]
        .decode::<Amount>()
        .unwrap();

    assert_eq!(total_supply, Amount(1_000_000_000));
    */
}

struct TestSetup {
    test: TemplateTest,
    marketplace_component: ComponentAddress,
    admin_account: ComponentAddress,
    admin_proof: NonFungibleAddress,
    admin_key: RistrettoSecretKey,
    seller_badge_resource: ResourceAddress,
}

fn setup() -> TestSetup {
    let mut test = TemplateTest::new(["./"]);
    let (admin_account, admin_proof, admin_key) = test.create_owned_account();
    let template = test.get_template_address("NftMarketplace");

    let result = test.execute_expect_success(
        Transaction::builder()
            .call_function(template, "new", args![])
            .sign(&admin_key)
            .build(),
        vec![admin_proof.clone()],
    );

    let marketplace_component = result.finalize.execution_results[0]
        .decode::<ComponentAddress>()
        .unwrap();

    let indexed = test
        .read_only_state_store()
        .inspect_component(marketplace_component)
        .unwrap();

    let seller_badge_resource = indexed
        .get_value("$.seller_badge_resource")
        .unwrap()
        .expect("seller_badge_resource not found");

    TestSetup {
        test,
        marketplace_component,
        admin_account,
        admin_proof,
        admin_key,
        seller_badge_resource,
    }
}