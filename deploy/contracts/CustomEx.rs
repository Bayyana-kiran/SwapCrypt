use std::collections::HashMap;

struct CustomToken {
    name: String,
    symbol: String,
    balance: HashMap<String, u64>,
    total_supply: u64,
}

impl CustomToken {
    fn new(name: &str, symbol: &str, initial_balance: u64) -> Self {
        let mut balance = HashMap::new();
        balance.insert(String::from("msg.sender"), initial_balance);
        CustomToken {
            name: String::from(name),
            symbol: String::from(symbol),
            balance,
            total_supply: initial_balance,
        }
    }

    fn transfer(&mut self, recipient: &str, amount: u64) -> bool {
        let sender_balance = match self.balance.get(&String::from("msg.sender")) {
            Some(balance) => balance,
            None => return false,
        };
        if *sender_balance < amount {
            return false;
        }
        let recipient_balance = self.balance.entry(String::from(recipient)).or_insert(0);
        *recipient_balance += amount;
        self.balance.insert(String::from("msg.sender"), sender_balance - amount);
        true
    }

    fn balance_of(&self, owner: &str) -> Option<u64> {
        self.balance.get(owner).copied()
    }

    fn total_supply(&self) -> u64 {
        self.total_supply
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct CustomDex {
    tokens: Vec<String>,
    token_instance_map: HashMap<String, CustomToken>,
    eth_value: u64,
    history_index: u64,
    historys: HashMap<u64, History>,
}

impl CustomDex {
    fn new() -> Self {
        CustomDex {
            tokens: vec![
                String::from("Tether USD"),
                String::from("BNB"),
                String::from("USD Coin"),
                String::from("stETH"),
                String::from("TRON"),
                String::from("Matic Token"),
                String::from("SHIBA INU"),
                String::from("Uniswap"),
            ],
            token_instance_map: HashMap::new(),
            eth_value: 100_000_000_000_000,
            history_index: 0,
            historys: HashMap::new(),
        }
    }

    fn initialize_tokens(&mut self) {
        for token_name in &self.tokens {
            let custom_token = CustomToken::new(token_name, token_name, 10_000_000 * 10u64.pow(18));
            self.token_instance_map.insert(token_name.clone(), custom_token);
        }
    }

    fn get_balance(&self, token_name: &str, _address: &str) -> Option<u64> {
        match self.token_instance_map.get(token_name) {
            Some(token) => token.balance_of(_address),
            None => None,
        }
    }

    fn get_total_supply(&self, token_name: &str) -> Option<u64> {
        match self.token_instance_map.get(token_name) {
            Some(token) => Some(token.total_supply()),
            None => None,
        }
    }

    fn get_name(&self, token_name: &str) -> Option<&str> {
        match self.token_instance_map.get(token_name) {
            Some(token) => Some(token.name()),
            None => None,
        }
    }

    fn get_token_address(&self, token_name: &str) -> Option<&CustomToken> {
        self.token_instance_map.get(token_name)
    }

    fn get_eth_balance(&self) -> u64 {
        0 // Need to implement Ethereum balance tracking
    }

    fn transaction_history(&mut self, token_name: &str, ether_token: &str, input_value: u64, output_value: u64) {
        self.history_index += 1;
        let history_id = self.history_index;
        let history = History {
            history_id,
            user_address: String::from("msg.sender"),
            token_a: String::from(token_name),
            token_b: String::from(ether_token),
            input_value,
            output_value,
        };
        self.historys.insert(history_id, history);
    }

    fn swap_eth_to_token(&mut self, token_name: &str, input_value: u64) -> Option<u64> {
        let output_value = (input_value / self.eth_value) * 10u64.pow(18);

        match self.token_instance_map.get_mut(token_name) {
            Some(token) => {
                if token.transfer("msg.sender", output_value) {
                    let ether_value = "Ether";
                    self.transaction_history(token_name, ether_value, input_value, output_value);
                    Some(output_value)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn swap_token_to_eth(&mut self, token_name: &str, amount: u64) -> Option<u64> {
        let exact_amount = amount / 10u64.pow(18);
        let eth_to_be_transfer = exact_amount * self.eth_value;

        match self.token_instance_map.get_mut(token_name) {
            Some(token) => {
                if let Some(sender_balance) = token.balance_of("msg.sender") {
                    if *sender_balance >= amount {
                        // Assume balance checking
                        // Need to implement Ethereum balance checking
                        let ether_value = "Ether";
                        self.transaction_history(token_name, ether_value, exact_amount, eth_to_be_transfer);
                        Some(eth_to_be_transfer)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn swap_token_to_token(&mut self, src_token: &str, des_token: &str, amount: u64) -> bool {
        match self.token_instance_map.get_mut(src_token) {
            Some(src_token_instance) => {
                if let Some(sender_balance) = src_token_instance.balance_of("msg.sender") {
                    if *sender_balance >= amount {
                        match self.token_instance_map.get_mut(des_token) {
                            Some(des_token_instance) => {
                                if src_token_instance.transfer(des_token, amount) {
                                    let _ = des_token_instance.transfer("msg.sender", amount);
                                    self.transaction_history(src_token, des_token, amount, amount);
                                    return true;
                                }
                            }
                            None => return false,
                        }
                    }
                }
                false
            }
            None => false,
        }
    }

    fn get_all_history(&self) -> Vec<History> {
        self.historys.values().cloned().collect()
    }
}

struct History {
    history_id: u64,
    user_address: String,
    token_a: String,
    token_b: String,
    input_value: u64,
    output_value: u64,
}

fn main() {
    let mut custom_dex = CustomDex::new();
    custom_dex.initialize_tokens();

    // Test your functions here
}
