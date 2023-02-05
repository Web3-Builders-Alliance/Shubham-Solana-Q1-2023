use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
};


entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Attempt to serialize the BPF format to our struct
    //  using Borsh
    //
    let instruction_data_object = InstructionData::try_from_slice(&instruction_data)?;

    msg!("Welcome to the park, {}!", instruction_data_object.name);
    if instruction_data_object.height > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    name: String,
    height: u32,
}
/*

There is an entrypoint process_instruction which takes a Pubkey, a slice of AccountInfo, and a slice of u8 as arguments, and returns a ProgramResult.
The instruction data received in the instruction_data argument is deserialized into an InstructionData struct using Borsh's try_from_slice method.
If the deserialization is successful, a message is printed to the logs, indicating the name of the person and if they are tall enough to ride a ride based on their height.
If the deserialization fails, an error with the message ProgramError::InvalidInstructionData is returned.

*/
