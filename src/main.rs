fn main() {

    let mut database: Vec<Account> = Vec::new();

    let mut cat0 = Cat{
        voice:"Miyav".to_string(),
        walk:"Yavaş".to_string(),
        colour:"Beyaz".to_string()
    };
    let mut cat1 = Cat{
        voice:"Miyav".to_string(),
        walk:"Hızlı".to_string(),
        colour:"Sarı".to_string()
    };

    let mut database1: Vec<Cat> = Vec::new();


    database1.push(cat1.clone());
    database1.push(cat0.clone());

    cat1.walk();
    cat0.colour();

    
   


    let acount = Account{
        tc:121124314,
        iban:2123441142,
        name:"İbrahim".to_string(),
        hesap_no:"müşteri".to_string(),
        amount:1000.0
    };

    

    database.push(acount.clone());
    database[0].deposit(500.00);
    database[0].withdraw(1000.00);
    database[0].show_amount();




}

trait Animal {
    fn voice(&mut self) ;
    fn walk(&mut self) ;
    fn colour(&mut self) ;
    
}
impl Animal for Dog {
    fn colour(&mut self) {
        println!("Köpeğin rengi {}",self.colour)
    }
    fn walk(&mut self) {
        println!("Köpeğin yürüyüşü {}",self.walk)

    }
    fn voice(&mut self) {
        println!("Köpeğin sesi {}",self.voice)

    }

}

impl Animal for Cat {
    fn colour(&mut self) {
        println!("Kedinin  rengi {}",self.colour)
    }
    fn walk(&mut self) {
        println!("Kedinin yürüyüşü {}",self.walk)

    }
    fn voice(&mut self) {
        println!("Kedinin sesi {}",self.voice)

        
    }
    
}

#[derive(Debug,Clone)]
struct Dog{
    voice:String,
    walk:String,
    colour:String
}

#[derive(Debug,Clone)]
struct Cat{
    voice:String,
    walk:String,
    colour:String
}


#[derive(Debug,Clone)]
struct Account {
    tc:u32,
    iban:u32,
    name:String,
    hesap_no:String,
    amount:f32
}

struct Profile {
    name:String,
    lastname:String

}

trait Profile_methods {
    fn register(&mut self);
}



trait Account_functions{
    fn deposit(&mut self,amount:f32) ;
    fn withdraw(&mut self,amount:f32) ;
    fn show_amount(&mut self) ;
}

impl Account_functions for  Account {

    fn deposit(&mut self,amount:f32) {
        self.amount = self.amount+amount;

        println!("Başarıyla yükleme yapıldı yeni bakiyeniz {}",self.amount)
    }

    fn withdraw(&mut self,amount:f32) {
        if amount >=self.amount {
            println!("Hesap bakiyeniz yetersiz hesabınızdaki para {}",self.amount)
            
        }
        else {
            self.amount= self.amount- amount;
            println!("Para başarılı bir şekilde çekildi yeni bakiyeniz {} ",self.amount)
        }
        
    }

    fn show_amount(&mut self) {
        println!("Hesap bakiyeniz {}",self.amount)
        
    }
    
}





