pub struct SavingAccount {
    balance: i32,
}
impl SavingAccount {
    pub fn new() -> SavingAccount {
        SavingAccount { balance: 0 }
    }
    pub fn get_balance(&self) -> i32 {
        self.balance
    }
    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot deposit negative amount");
        }
        self.balance += amount;
    }
    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_have_a_starting_balance_0() {
        let account = SavingAccount::new();
        assert_eq!(account.get_balance(), 0);
    }
    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
    }
    #[test]
    fn should_not_equal() {
        let account = SavingAccount::new();
        assert_ne!(account.get_balance(), 200);
    }
    #[test]
    fn assert_macro() {
        let account = SavingAccount::new();
        assert!(account.get_balance() == 0);
    }
    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingAccount::new();
        account.deposit(200);
        account.transfer(123, 50)?;
        Ok(())
        // Call transfer(...) -> If it returns Ok(value) → continue,If it returns Err(error) → immediately return that error from should_transfer_money
    }
    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingAccount::new();
        account.deposit(-1);
    }
}
