use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
/*
BorshDeserialize and BorshSerialize are traits that define methods for serializing and deserializing binary data using the Borsh format.
The entrypoint module provides functions for defining entry points into a Solana program.
The msg module provides a type for a message passed to a Solana program.
The program module provides utility functions for working with Solana programs. The invoke function is used to execute a system instruction, which are defined in the system_instruction module.
The pubkey module provides a type for public keys in Solana.
The rent module provides information about rent charged to accounts on the Solana network.
The sysvar module provides a trait and a type for working with system variables in Solana.
The account_info module provides functions and types for working with account information in Solana. The next_account_info function is used to iterate over the account information passed to the program. The AccountInfo type holds information about an account in Solana, such as its pubkey, data, and lamports.
*/

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

// Generates the entrypoint function process_instruction if the no-entrypoint feature is not enabled.

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    match PowerStatus::try_from_slice(&instruction_data) {
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {},
    }

    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {},
    }

    Err(ProgramError::InvalidInstructionData)
}
/*
The process_instruction function is an entrypoint function of a Solana program. It takes three parameters:

program_id: A Pubkey representing the program ID.
accounts: An array of AccountInfo objects representing the accounts that are being passed to the program.
instruction_data: A byte array representing the data passed in the instruction.
The function first tries to parse the instruction data as a PowerStatus struct. If the parse is successful, it calls the initialize function with the program_id and accounts as arguments and power_status as the parsed struct.

If the parse fails, the function tries to parse the instruction data as a SetPowerStatus struct. If the parse is successful, it calls the switch_power function with accounts and the parsed SetPowerStatus.name as arguments.

If both parses fail, the function returns ProgramError::InvalidInstructionData.
*/

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = (power_status.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            user.clone(), power.clone(), system_program.clone()
        ]
    )?;

    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}
/*
This function is initializing the program with a PowerStatus struct. It takes a program id and a reference to an array of AccountInfo as input.
The function first sets up an iterator over the accounts and gets three accounts: power, user, and system_program.
Then it computes the required lamports (the minimum balance required for a new account) based on the size of the serialized power_status using the Rent system variable and the minimum_balance function.
Finally, it calls the invoke function to create a new user account, with the power account as the owner, the required lamports as the starting balance, the size of the power_status as the account space and the program id as the program id. The power_status is then serialized and stored in the newly created user account's data.

*/
   
pub fn switch_power(
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    
    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    power_status.is_on = !power_status.is_on;
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    msg!("{} is pulling the power switch!", &name);

    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}
/*
The function takes two arguments: accounts is an array of AccountInfo structs, and name is a string representing the name of the person who is switching the power.
The function starts by creating an iterator over the accounts array and calling next_account_info to get the first account. This account is assumed to store the status of the power.
Next, the code deserializes the power status from the data field of the account, toggles the is_on property, serializes it back to the data field, and logs the action of the person switching the power and the current power status.
Finally, the function returns Ok(()), indicating success.
*/

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}
/*
There are two structs: SetPowerStatus and PowerStatus.
SetPowerStatus has a single field name of type String.
PowerStatus has a single field is_on of type bool.

Both structs implement the BorshDeserialize and BorshSerialize traits, which are used for (de)serializing the structs from and to binary data.
The Debug trait is also implemented for both structs, allowing them to be printed as human-readable strings when used with the {:?} format specifier.
*/


/*

#Ques: What are the concepts (borrowing, ownership, vectors etc)?
Ans: 
Borrowing: Borrowing is a mechanism in Rust that allows one piece of code to temporarily hold a reference to another piece of code, without taking ownership of it. In this program, the data in an account is borrowed using the borrow and borrow_mut methods.
Ownership: Ownership is a fundamental concept in Rust that ensures that all values have a unique owner at all times. The owner of a value is the only part of the code that can modify it, and it's automatically dropped when its owner goes out of scope. In this program, the ownership of the account data is transferred from one part of the code to another through the use of references and functions.
Vectors: A vector is a dynamically sized array in Rust. In this program, vectors are used to store and manipulate the data in an account.

#Ques: What is the contract doing? What is the mechanism? 
Ans:
The program provides functionality to initialize and switch the power status of a device. The program defines two structures: SetPowerStatus and PowerStatus. The PowerStatus structure has a single field is_on that stores the current power status of the device, as a boolean value. The SetPowerStatus structure has a single field name which is used to store the name of the user who wants to switch the power.
The process_instruction function is the entry point of the program. It deserializes the incoming instruction data and processes it accordingly. If the instruction data corresponds to PowerStatus, the function calls the initialize function, which creates a new account for the device, stores the PowerStatus data in the newly created account, and returns the result. If the instruction data corresponds to SetPowerStatus, the function calls the switch_power function, which switches the current power status and returns the result.
The initialize function takes the program_id, the accounts, and the PowerStatus as input and creates a new account for the device. It uses the invoke function from the solana_program crate to create the new account. The switch_power function takes the accounts and the SetPowerStatus as input and switches the current power status. It retrieves the PowerStatus data from the corresponding account, updates it with the new power status, and returns the result.

#Ques: How could it be better? More efficient? Safer? 
Ans: There are several ways in which the code can be improved:

Error handling: The code does not handle errors effectively. There could be cases where some of the operations fail and the code does not have provisions to handle these cases.
Namespacing: The code has global definitions and functions which can lead to naming conflicts. A better solution would be to namespace the code in modules or packages.
Naming conventions: The naming conventions for functions, variables, and structures could be improved for readability and maintainability.
Optimization: The code does not take advantage of parallelism and concurrency to perform certain tasks. For example, certain operations like serializing and deserializing can be performed in parallel to improve performance.
Security: The code does not perform any input validation, which can lead to security vulnerabilities. A better solution would be to validate inputs before performing any operations.

*/
