use borsh::{BorshDeserialize, BorshSerialize}; //Used for (de)serialization of data
use mpl_token_metadata::instruction::create_metadata_accounts; //Library from Metaplex for handling token metadata.
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
}; //Core library for Solana programs, providing necessary utilities.

// Define a structure to hold the input data for minting the NFT
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintNFTInstruction {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

// The program's entry point
entrypoint!(process_instruction);

// The main function to process instructions
fn process_instruction(
    program_id: &Pubkey, //Public key of the program.
    accounts: &[AccountInfo], //List of accounts involved in the transaction.
    instruction_data: &[u8], //Serialized instruction data to be processed.
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Parse the accounts needed
    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;

    // Deserialize the instruction data
    let instruction_data = MintNFTInstruction::try_from_slice(instruction_data)?;
    msg!("Minting NFT: {:?}", instruction_data);

    // Invoke the Metaplex create_metadata_accounts instruction
    let ix = create_metadata_accounts(
        *token_metadata_program.key,
        *metadata_account.key,
        *mint.key,
        *mint.key,
        *payer.key,
        *payer.key,
        instruction_data.name,
        instruction_data.symbol,
        instruction_data.uri,
        None,
        1,
        false,
        false,
    );

    invoke(
        &ix,
        &[
            metadata_account.clone(),
            mint.clone(),
            payer.clone(),
            token_account.clone(),
            rent.clone(),
            token_program.clone(),
            system_program.clone(),
            token_metadata_program.clone(),
        ],
    )?; //Executes the instruction to create the metadata account, effectively minting the NFT.

    Ok(())
}
