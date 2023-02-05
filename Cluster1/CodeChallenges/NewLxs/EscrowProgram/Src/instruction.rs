use crate::error::EscrowError::InvalidInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::{convert::TryInto, mem::size_of};

pub enum EscrowInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64,
    },
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person taking the trade
    /// 1. `[writable]` The taker's token account for the token they send
    /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close
    /// 4. `[writable]` The initializer's main account to send their rent fees to
    /// 5. `[writable]` The initializer's token account that will receive tokens
    /// 6. `[writable]` The escrow account holding the escrow info
    /// 7. `[]` The token program
    Exchange {
        /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
        amount: u64,
    },
    /// Accounts expected:
    ///
    /// 0. `[signer]` The initializer that is cancelling the escrow
    /// 1. `[writable]` The PDA's temp token account to get tokens from and eventually close the account
    /// 2. `[]` The initializer's token account that will receive tokens
    /// 3. `[writable]` The escrow account holding the escrow info
    /// 4. `[]` The token program
    Cancel {},
    /// Accounts expected:
    ///
    /// 0. `[signer]` The initializer that is resetting the escrow
    /// 1. `[writable]` The escrow account holding the escrow info
    ResetTimeLock {},
}

impl EscrowInstruction {
    /// unpack a byte buffer into a [EscrowInstruction]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?,
            },
            2 => Self::Cancel {},
            3 => Self::ResetTimeLock {},
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    fn pack(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::InitEscrow { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Exchange { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Cancel {} => {
                buf.push(2);
            }
            Self::ResetTimeLock {} => {
                buf.push(3);
            }
        }
        buf
    }
}

pub fn init_escrow(
    program_id: &Pubkey,
    initiator: &Pubkey,
    temp_token_account: &Pubkey,
    initializer_token_account: &Pubkey,
    escrow_account: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::InitEscrow { amount }.pack();
    let accounts = vec![
        AccountMeta::new(*initiator, true),
        AccountMeta::new(*temp_token_account, false),
        AccountMeta::new_readonly(*initializer_token_account, false),
        AccountMeta::new(*escrow_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn exchange(
    program_id: &Pubkey,
    tacker: &Pubkey,
    tacker_token_account: &Pubkey,
    tacker_token_account2: &Pubkey,
    initiator: &Pubkey,
    temp_token_account: &Pubkey,
    initializer_token_account: &Pubkey,
    escrow_account: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::Exchange { amount }.pack();
    let accounts = vec![
        AccountMeta::new(*tacker, true),
        AccountMeta::new(*tacker_token_account, false),
        AccountMeta::new(*tacker_token_account2, false),
        AccountMeta::new(*initiator, false),
        AccountMeta::new(*temp_token_account, false),
        AccountMeta::new(*initializer_token_account, false),
        AccountMeta::new(*escrow_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn cancel(
    program_id: &Pubkey,
    initiator: &Pubkey,
    temp_token_account: &Pubkey,
    initializer_token_account: &Pubkey,
    escrow_account: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::Cancel {}.pack();
    let accounts = vec![
        AccountMeta::new(*initiator, true),
        AccountMeta::new(*temp_token_account, false),
        AccountMeta::new_readonly(*initializer_token_account, false),
        AccountMeta::new(*escrow_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn reset_time_lock(
    program_id: &Pubkey,
    initiator: &Pubkey,
    escrow_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::Cancel {}.pack();
    let accounts = vec![
        AccountMeta::new(*initiator, true),
        AccountMeta::new(*escrow_account, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
