fn main(){
    let mut account:BankAccount=BankAccount{
        name:String::from("RUSTY"),
        balance:200.3
    };
    account.check_balance();
    account.withdraw(20.6);
    account.check_balance();
}

struct BankAccount{
    name:String,
    balance:f64
}

impl BankAccount{
    fn check_balance(&self){
        println!("Account {} has a balance of $ {}",self.name,self.balance);
    }
    fn withdraw(&mut self,amount:f64){
        println!("Withdrawing amount {}",amount);
        self.balance-=amount;
    }
}