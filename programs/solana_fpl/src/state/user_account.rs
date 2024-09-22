#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub bet_amount: u64,
    pub is_eligible: bool,
    pub payout_amount: u64,
}