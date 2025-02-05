
trait Account {
    fn withdraw(&self);
    fn get_account_number(&self);
}

struct BankAccount;
impl BankAccount {
    fn new() -> Self {
        Self
    }
}
impl Account for BankAccount {
    fn get_account_number(&self) {
        println!("Get account number directly from bank...");
    }
    fn withdraw(&self) {
        println!("Withdraw money directly from the bank...");
    }
}

struct ATMProxy;
impl ATMProxy {
    fn new() -> Self {
        Self
    }
}
impl Account for ATMProxy {
    fn get_account_number(&self) {
        println!("Get account number from ATM machine....");
    }
    fn withdraw(&self) {
        println!("First go to ATM...");
        let bank_acc = BankAccount::new();
        bank_acc.withdraw();
        println!("Come back from ATM..");
    }
}

fn main() {
    let atm = ATMProxy;
    atm.withdraw();
}
