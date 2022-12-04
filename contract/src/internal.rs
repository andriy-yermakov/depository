use crate::*;

impl Depository {
    pub(crate) fn internal_new(deposit_account: AccountId) -> Self {
        assert!(
            !env::state_exists(),
            "Contract state is already initialize."
        );

        assert!(
            env::is_valid_account_id(deposit_account.as_bytes()),
            "The deposit account ID is invalid"
        );

        Self {
            deposit_register: Vec::new(),
            deposit_account,
            even_attempt: true,
        }
    }

    pub(crate) fn internal_deposit(&mut self) -> bool {
        self.even_attempt = !self.even_attempt;

        let account = env::signer_account_id();
        // let predecessor = env::predecessor_account_id();
        let amount = env::attached_deposit();

        if amount == 0 {
            env::panic_str("Insufficient near deposit.");
        }

        if self.even_attempt {
            let message = format!("@{} deposited {}.", account, amount);

            log!(&message);

            self.deposit_register.push(Deposit { account, amount });
            Promise::new(self.deposit_account.clone()).transfer(amount);
        } else {
            Promise::new(account).transfer(amount);
        }

        self.even_attempt
    }

    pub(crate) fn internal_list(&self) -> Vec<Deposit> {
        self.deposit_register.clone()
    }
}
