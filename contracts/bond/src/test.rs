// test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{vec, Env, String};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = env.register(MyContract, ());
        let client = MyContractClient::new(&env, &contract_id);

        let words = client.hello(&String::from_str(&env, "Dev"));
        assert_eq!(
            words,
            vec![
                &env,
                String::from_str(&env, "Hello"),
                String::from_str(&env, "Dev"),
            ]
        );
    }
}
