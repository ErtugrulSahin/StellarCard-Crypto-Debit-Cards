use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype};

pub struct CryptoCard;

#[contracttype]
pub struct Card {
    pub owner: Address,
    pub balance: i128,
    pub currency: Symbol,
}

#[contractimpl]
impl CryptoCard {
    fn cards<'a>(env: &'a Env) -> Vec<'a, Card> {
        env.storage().instance().get::<Vec<Card>>(Symbol::short("cards")).unwrap_or(Vec::new(&env))
    }

    pub fn issue_card(env: Env, currency: Symbol) {
        let owner = env.invoker();
        let mut cards = Self::cards(&env);
        cards.push_back(Card { owner, balance: 0, currency });
        env.storage().instance().set(Symbol::short("cards"), &cards);
    }

    pub fn top_up(env: Env, index: u32, amount: i128) {
        let owner = env.invoker();
        let mut cards = Self::cards(&env);
        let card = &mut cards[index as usize];
        assert_eq!(card.owner, owner, "Only card owner can top up");
        card.balance += amount;
        env.storage().instance().set(Symbol::short("cards"), &cards);
    }
}
