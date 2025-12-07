use bank::SavingAccount;
mod utils;

#[test]
fn should_have_starting_balance_0() {
    utils::common_setup();
    let account = SavingAccount::new();
    assert_eq!(account.get_balance(), 0);
}
