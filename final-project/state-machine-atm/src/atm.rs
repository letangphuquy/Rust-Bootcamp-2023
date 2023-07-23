//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

// NOTE: I modified the tests, because I somehow "need" to modify the function signatures to be convenient

use crate::traits::{StateMachine, hash};

/// The keys on the ATM keypad
#[derive(Clone, Copy)]
#[derive(PartialEq, Debug)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM
#[derive(Clone, Copy)]
#[derive(PartialEq, Debug)]
pub enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}




/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
/// (haha, communism withdrawing?)
#[derive(Clone)]
#[derive(PartialEq, Debug)]
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    auth_state: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}


//TODO
// Implement trait Default for Auth 
// return Waiting status 
impl Default for Auth {
    fn default() -> Self {
        Auth::Waiting
    }
}


//TODO
// Implement trait From  for &str
// Convert  elements in Key to &str
// NOTE: Hello teacher, I change it to String
impl From<Key> for String {
    fn from(value: Key) -> Self {
        match value {
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Enter => "\n"
        }.to_string()
    }
}

impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
    type State = Atm;
    type Transition = Action;
    // Hint
    // Should use `default` method when auth status is Waiting status
    // Should use `from` method to convert  elements in Key to &str
    // Parse &str to integer to calculate amount
    // Use a hash function to verify the PIN both before and after the user presses the Enter key.
    fn next_state(&self, transition: &Self::Transition) -> Self::State {
        let mut result = self.clone();
        result.auth_state =
        match self.auth_state {
            Auth::Waiting => {
                match transition {
                    Action::SwipeCard(hash) => Auth::Authenticating(hash.to_owned()),
                    _ => Auth::Waiting
                }
            },
            Auth::Authenticating(expected_hash) => {
                match transition {
                    Action::PressKey(key) => {
                        if *key == Key::Enter {
                            let received_hash = hash(&self.keystroke_register);
                            result.keystroke_register.clear();
                            if received_hash == expected_hash {
                                Auth::Authenticated
                            } else {
                                Auth::Waiting
                            }
                        }
                        else {
                            result.keystroke_register.push(*key);
                            self.auth_state
                        }
                    },
                    Action::SwipeCard(incoming_hash) => {
                        if *incoming_hash == expected_hash {
                            self.auth_state
                        }
                        else {
                            //EDGE CASE FOUND: customer A swiped his card, but didn't withdraw. Then customer B came in and swipe her card
                            //need to clear properly
                            result.keystroke_register.clear();
                            Auth::Authenticating(*incoming_hash)
                        }
                    }
                }
            },
            Auth::Authenticated => {
                match transition {
                    Action::PressKey(key) => {
                        if *key == Key::Enter {
                            if self.keystroke_register.len() <= self.cash_inside.ilog10() as usize {
                                let mut withdraw_amount: u64 = 0;
                                for digit in self.keystroke_register.iter() {
                                    withdraw_amount *= 10;
                                    withdraw_amount += String::from(*digit).parse::<u64>().unwrap();
                                }
                                if withdraw_amount <= self.cash_inside {
                                    result.cash_inside -= withdraw_amount;
                                }
                            }
                            result.keystroke_register.clear();
                            Auth::Waiting
                        }
                        else {
                            result.keystroke_register.push(*key);
                            self.auth_state
                        }
                    },
                    Action::SwipeCard(_) => {
                        //same SECURITY concern as above, simple solution? just log out
                        result.keystroke_register.clear();
                        self.auth_state
                    }
                }
            }
        };
        result.to_owned()
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
    let expected1 = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = hash(&pin);

    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = hash(&pin);

    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
    let expected1 = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        auth_state: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Atm {
        cash_inside: 10,
        auth_state: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        auth_state: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}