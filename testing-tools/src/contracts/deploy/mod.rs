use alloy::primitives::{address, fixed_bytes, keccak256, uint, Address, Bytes, B256, U160, U256};

// use super::environment::{ANGSTROM_ADDRESS, ANGSTROM_ADDRESS_SALT};

pub mod angstrom;
pub mod mockreward;
pub mod tokens;
pub mod uniswap_flags;

use uniswap_flags::UniswapFlags;

const DEFAULT_CREATE2_FACTORY: Address = address!("4e59b44847b379578588920cA78FbF26c0B4956C");

/// Attempt to find a target address that includes the appropriate flags
/// Returns the address found and the salt needed to pad the initcode to
/// deploy to that address
pub fn mine_address(
    deployer: Address,
    flags: U160,
    mask: U160,
    initcode: &Bytes
) -> (Address, U256) {
    // this one DOES NOT works with create3
    mine_address_with_factory_create3(deployer, DEFAULT_CREATE2_FACTORY, flags, mask);

    // this one works with create2
    mine_address_with_factory_create2(DEFAULT_CREATE2_FACTORY, flags, mask, initcode)
}

pub fn mine_address_with_factory_create3(
    deployer: Address,
    factory: Address,
    flags: U160,
    mask: U160
) -> (Address, U256) {
    let mut salt = U256::ZERO;
    let mut counter: u128 = 0;
    loop {
        // i tried both variants of these
        let target_address: Address = sanity_calculate(deployer, salt.to_le_bytes());
        // let target_address: Address = sanity_calculate(factory, salt.to_le_bytes());
        let u_address: U160 = target_address.into();
        if (u_address & mask) == flags {
            break;
        }
        salt += U256::from(1_u8);
        counter += 1;
        if counter > 100_000 {
            panic!("We tried this too many times!")
        }
    }
    // i tried both variants of these
    let final_address = sanity_calculate(deployer, salt.to_le_bytes());
    // let final_address: Address = sanity_calculate(factory, salt.to_le_bytes());

    (final_address, salt)
}

fn sanity_calculate(deployer: Address, salt: [u8; 32]) -> Address {
    // i tried both variants of these
    calc_addr(&**deployer, &salt).into()
    // calc_addr_with_bytes(&**deployer, &salt).into()
}

pub fn mine_address_with_factory_create2(
    factory: Address,
    flags: U160,
    mask: U160,
    initcode: &Bytes
) -> (Address, U256) {
    let init_code_hash = keccak256(initcode);
    let mut salt = U256::ZERO;
    let mut counter: u128 = 0;
    loop {
        let target_address: Address = factory.create2(B256::from(salt), init_code_hash);
        let u_address: U160 = target_address.into();
        if (u_address & mask) == flags {
            break;
        }
        salt += uint!(1U256);
        counter += 1;
        if counter > 100_000 {
            panic!("We tried this too many times!")
        }
    }
    let final_address = factory.create2(B256::from(salt), init_code_hash);
    (final_address, salt)
}

#[cfg(test)]
mod tests {
    use super::uniswap_flags::UniswapFlags;

    #[test]
    fn test_deploy_addresses() {
        let flags = UniswapFlags::BeforeSwap
            | UniswapFlags::BeforeInitialize
            | UniswapFlags::BeforeAddLiquidity
            | UniswapFlags::BeforeRemoveLiquidity;
    }
}

pub fn mine_create3_address(owner: Address) -> (Address, U256, u8) {
    let mut salt = U256::from(Into::<U160>::into(owner));
    let nonce = 0u8;
    salt = salt << 96;
    let mut addr;
    loop {
        addr = sub_zero_create3(salt.into(), nonce);
        if angstrom_addr_valid(addr) {
            break;
        }
        salt += uint!(1U256);
    }
    (addr, salt, nonce)
}

pub const SUB_ZERO_FACTORY: Address = address!("000000000000b361194cfe6312ee3210d53c15aa");
const DEPLOY_PROXY_INITHASH: B256 =
    fixed_bytes!("1decbcf04b355d500cbc3bd83c892545b4df34bd5b2c9d91b9f7f8165e2095c3");

fn angstrom_addr_valid(addr: Address) -> bool {
    use UniswapFlags::*;
    if !has_any_permission(addr, BeforeInitialize | AfterInitialize) {
        return false;
    }
    if !has_permissions(addr, BeforeAddLiquidity | BeforeRemoveLiquidity) {
        return false;
    }
    if has_any_permission(addr, AfterAddLiquidity | AfterRemoveLiquidity) {
        return false;
    }
    if !has_any_permission(addr, BeforeSwap | AfterSwap) {
        return false;
    }

    hook_addr_valid(addr)
}

/// Assumes hook with fee of **0**.
fn hook_addr_valid(addr: Address) -> bool {
    use UniswapFlags::*;
    if !has_permission(addr, BeforeSwap) && has_permission(addr, BeforeSwapReturnsDelta) {
        return false;
    }
    if !has_permission(addr, AfterSwap) && has_permission(addr, AfterSwapReturnsDelta) {
        return false;
    }
    if !has_permission(addr, AfterRemoveLiquidity)
        && has_permission(addr, AfterRemoveLiquidityReturnsDelta)
    {
        return false;
    }
    if !has_permission(addr, AfterAddLiquidity)
        && has_permission(addr, AfterAddLiquidityReturnsDelta)
    {
        return false;
    }

    let bits: U160 = addr.into();

    // Has at least some flag
    bits & UniswapFlags::mask() > U160::from(0)
}

fn has_permission(addr: Address, f: UniswapFlags) -> bool {
    let bits: U160 = addr.into();
    bits & f.flag() == f.flag()
}

fn has_permissions(addr: Address, flags: U160) -> bool {
    let bits: U160 = addr.into();
    bits & flags == flags
}

fn has_any_permission(addr: Address, flags: U160) -> bool {
    let bits: U160 = addr.into();
    bits & flags != U160::from(0)
}

fn sub_zero_create3(salt: B256, nonce: u8) -> Address {
    let deploy_proxy = SUB_ZERO_FACTORY.create2(salt, DEPLOY_PROXY_INITHASH);
    deploy_proxy.create((nonce as u64).wrapping_add(1))
}
