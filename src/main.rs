
use std::collections::HashMap;

struct MyContract {
    // Champs du contrat
    owner: String, // Adresse du propriétaire du contrat
    balances: HashMap<String, u64>, // Mappage des adresses utilisateurs à leurs soldes
}
impl MyContract {
    // Fonction de création du contrat
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            balances: HashMap::new(),
        }
    }

    // Fonction pour vérifier le solde d'un utilisateur
    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    // Fonction pour déposer des fonds sur le contrat
    pub fn deposit(&mut self, address: &str, amount: u64) {
        let  balance = self.balances.entry(address.to_string()).or_insert(0);
        *balance += amount;
    }

    // Fonction pour retirer des fonds du contrat
    pub fn withdraw(&mut self, address: &str, amount: u64) {
        let  balance = self.balances.entry(address.to_string()).or_insert(0);
        if *balance >= amount {
            *balance -= amount;
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), &'static str> {
        // Vérifier le solde de l'expéditeur
        let from_balance = self.balances.get(from).unwrap_or(&0);
        if *from_balance < amount {
            return Err("Solde insuffisant pour l'expéditeur");
        }
    
        // Débiter le solde de l'expéditeur
        let mut from_balance = self.balances.entry(from.to_string()).or_insert(0);
        *from_balance -= amount;
    
        // Vérifier si le destinataire existe
        if !self.balances.contains_key(to) {
            return Err("Destinataire inexistant");
        }
    
        // Créditer le solde du destinataire
        let mut to_balance = self.balances.entry(to.to_string()).or_insert(0);
        *to_balance += amount;
    
        Ok(()) // Transaction réussie
    }
    
   
}


fn main() {

}

