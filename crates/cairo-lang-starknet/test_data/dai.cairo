/// Whishlist
/// - Add Address type to the language
/// - Add MAX values to types

#[contract]
mod Dai {
    use starknet::get_caller_address;

    struct Storage {
        _total_supply: u256,
        _wards: LegacyMap::<felt, bool>,
        _balances: LegacyMap::<felt, u256>,
        _allowances: LegacyMap::<(felt, felt), u256>,
    }

    #[event]
    fn Rely(usr: felt) {}

    #[event]
    fn Deny(usr: felt) {}

    #[event]
    fn Transfer(from: felt, to: felt, value: u256) {}

    #[event]
    fn Approval(owner: felt, spender: felt, value: u256) {}

    #[constructor]
    fn constructor(ward: felt) {
        _wards::write(ward, true);
        Rely(ward);
    }

    fn name() -> felt {
        'Dai Stablecoin'
    }

    #[view]
    fn totalSupply() -> u256 {
        _total_supply::read()
    }

    #[view]
    fn balanceOf(user: felt) -> u256 {
        _balances::read(user)
    }

    #[view]
    fn allowance(owner: felt, spender: felt) -> u256 {
        _allowances::read((owner, spender))
    }

    #[view]
    fn wards(user: felt) -> bool {
        _wards::read(user)
    }

    fn auth() {
        assert(_wards::read(get_caller_address()), 'Dai/not-authorized');
    }

    #[external]
    fn mint(account: felt, amount: u256) {
        auth();
        assert(account!= 0, 'dai/invalid-recipient');
        /// TODO: assert contract != address

        /// Update balance
        _balances::write(account, _balances::read(account) + amount);

        /// Update total supply
        _total_supply::write(_total_supply::read() + amount);

        Transfer(0, account, amount);
    }

    #[external]
    fn burn(account: felt, amount: u256) {
        let balance = _balances::read(account);
        assert(balance >= amount, 'dai/insufficient-balance');

        _balances::write(account, balance - amount);

        _total_supply::write(_total_supply::read() - amount);

        Transfer(account, 0, amount);

        let caller = get_caller_address();
        if caller != account {
            let allowance = _allowances::read((account, caller));
            let MAX = u256{low: 0xffffffffffffffffffffffffffffffff_u128, high: 0xffffffffffffffffffffffffffffffff_u128};
            if allowance != MAX {
                assert(allowance >= amount, 'dai/insufficient-allowance');
                _allowances::write((account, caller), allowance - amount);
            }
        }
    }

    #[external]
    fn rely(usr: felt) {
        auth();
        _wards::write(usr, true);
        Rely(usr);
    }

    #[external]
    fn deny(usr: felt) {
        auth();
        _wards::write(usr, false);
        Deny(usr);
    }

    #[external]
    fn transfer(recipient: felt, amount: u256) -> bool {
        _transfer(get_caller_address(), recipient, amount);
        true
    }

    #[external]
    fn transferFrom(sender: felt, recipient: felt, amount: u256) -> bool {
        _transfer(sender, recipient, amount);
        let caller = get_caller_address();
        if caller != sender {
            let allowance = _allowances::read((sender, caller));
            let MAX = u256{low: 0xffffffffffffffffffffffffffffffff_u128, high: 0xffffffffffffffffffffffffffffffff_u128};
            if allowance != MAX {
                assert(allowance >= amount, 'dai/insufficient-allowance');
                _allowances::write((sender, caller), allowance - amount);
            }
        }
        true
    }

    #[external]
    fn approve(spender: felt, amount: u256) -> bool {
        _approve(get_caller_address(), spender, amount);
        true
    }

    #[external]
    fn increaseAllowance(spender: felt, amount: u256) -> bool {
        let caller = get_caller_address();
        _approve(caller, spender, _allowances::read((caller, spender)) + amount);
        true
    }

    #[external]
    fn decreaseAllowance(spender: felt, amount: u256) -> bool {
        let caller = get_caller_address();
        _approve(caller, spender, _allowances::read((caller, spender)) - amount);
        true
    }

    fn _transfer(sender: felt, recipient: felt, amount: u256) {
        assert(recipient != 0, 'dai/invalid-recipient');

        let sender_balance = _balances::read(sender);
        assert(sender_balance >= amount, 'dai/insufficient-balance');
        _balances::write(sender, sender_balance - amount);

        _balances::write(recipient, _balances::read(recipient) + amount);

        Transfer(sender, recipient, amount);
    }

    fn _approve(caller: felt, spender: felt, amount: u256) {
        assert(spender != 0, 'dai/invalid-recipient'); /// TODO: check if this is needed
        _allowances::write((caller, spender), amount);
        Approval(caller, spender, amount);
    }
}