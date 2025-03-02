use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u128,
    pub amount: u64,
    pub slot: u64,
    pub roll: u8,
    pub bump: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&[self.roll, self.bump]);

        s
    }
}

// | Field  | Type | Size (Bytes) | Conversion |
// |--------|------|-------------|------------|
// | `player` | `Pubkey` | 32 | `to_bytes()` |
// | `seed` | `u128` | 16 | `to_le_bytes()` |
// | `amount` | `u64` | 8 | `to_le_bytes()` |
// | `slot` | `u64` | 8 | `to_le_bytes()` |
// | `bump` | `u8` | 1 | Direct append |
// | `roll` | `u8` | 1 | Direct append |
// | **Total** |  | **66 bytes** |  |