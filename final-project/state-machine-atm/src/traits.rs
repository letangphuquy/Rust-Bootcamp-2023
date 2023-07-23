use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


/// A state machine - Generic over the transition type
pub trait StateMachine {
    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(&self, transition: &Self::Transition) -> Self::State;
}


pub const MOD: u64 = 2004010501;
// Simple helper to do some hashing.
pub fn hash<T: Clone>(arr: &Vec<T>) -> u64 where String: From<T> {
    if  arr.is_empty() { 0 }
    else {
        //parse number by a modulo, then use default Hasher
        let s = String::from(arr[0].to_owned());
        let mut num: u64 = 0;
        for c in s.chars() {
            num = 10 * num + c.to_digit(10).unwrap() as u64;
            num %= MOD;
        }
        let mut hasher = DefaultHasher::new();
        num.hash(&mut hasher);
        hasher.finish()
    }
}

// Test for hash function 
#[test]
fn test_hash_enum_vec() {
    //I need to modify the test here as well :(
    #[derive(Clone)]
    enum KeyTest{
        One,
        Two,
        Three,
        Four
    }

    impl From<KeyTest> for String {
        fn from(value: KeyTest) -> Self {
            match value {
                KeyTest::One => "1",
                KeyTest::Two => "2",
                KeyTest::Three => "3",
                KeyTest::Four => "4"
            }.to_string()
        }
    }

    let input: Vec<KeyTest> = vec![KeyTest::One, KeyTest::Two, KeyTest::Three, KeyTest::Four];

    let hash1 = hash(&input);
    let hash2 = hash(&input);

    assert_eq!(hash1, hash2);
}
