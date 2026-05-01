#[derive(Debug)]
struct BankAccount {
    balance: i32,
}

impl BankAccount {
    // constructor
    fn new(bal: i32) -> Self {
        Self { balance: bal }
    }

    // deposit money (mutates state)
    fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            println!("Amount should be positive");
        } else {
            self.balance += amount;
        }
    }

    // withdraw money (mutates state)
    fn withdraw(&mut self, amount: i32) -> i32 {
        if amount < 0 {
            println!("Invalid amount");
            return self.balance;
        }

        if self.balance < amount {
            println!("Insufficient balance");
            return self.balance;
        }

        self.balance -= amount;
        self.balance
    }
}

fn main() {
    let mut bank_account = BankAccount::new(100);

    bank_account.deposit(50);

    let balance_left = bank_account.withdraw(70);

    println!("Balance left: {}", balance_left);
}