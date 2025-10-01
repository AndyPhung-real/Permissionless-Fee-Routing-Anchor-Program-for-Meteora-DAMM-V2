// BEGIN tests/fee_router_bankrun.rs
use anchor_lang::prelude::*;
use anchor_client::{Client, Program};
use solana_sdk::signature::{Keypair, Signer};
use std::rc::Rc;
use fee_router::state::unix_day;

#[tokio::test]
async fn e2e_quote_distribution() -> anyhow::Result<()> {
    /* ------------ spin up client -------------- */
    let (client, payer) = {
        let kp = Keypair::new();
        let cli = Client::new_with_options(
            anchor_client::Cluster::Localnet,
            Rc::new(kp.clone()),
            CommitmentConfig::processed(),
        );
        (cli, kp)
    };

    let fee_router = client.program(fee_router::ID);
    let cp_amm     = client.program(mock_cp_amm::ID);
    let streamflow = client.program(mock_streamflow::ID);

    /* ------------ create pool ----------------- */
    let pool = Keypair::new();
    cp_amm
        .request()
        .accounts(mock_cp_amm::accounts::InitPool {
            pool: pool.pubkey(),
            payer: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(mock_cp_amm::instruction::InitPool {
            quote: Pubkey::new_unique(),
            base:  Pubkey::new_unique(),
        })
        .signer(&payer)
        .signer(&pool)
        .send()?;

    /* ------------ create honorary position ---- */
    let pos_owner_seed = b"pos_owner";
    let (pos_owner_pda, _bump) =
        Pubkey::find_program_address(&[pos_owner_seed], &fee_router::ID);

    let position = Keypair::new();
    fee_router
        .request()
        .accounts(fee_router::accounts::InitHonoraryPosition {
            payer: payer.pubkey(),
            investor_fee_pos_owner_pda: pos_owner_pda,
            new_position: position.pubkey(),
            system_program: system_program::ID,
            token_program: spl_token::ID,
            cp_amm_program: cp_amm.id(),
            cp_amm_pool: pool.pubkey(),
            rent: sysvar::rent::ID,
        })
        .args(fee_router::instruction::InitHonoraryPosition {})
        .signer(&payer)
        .signer(&position)
        .send()?;

    /* ------------ add investor streams -------- */
    let investor_a = Keypair::new();
    let investor_b = Keypair::new();
    let stream_a   = Keypair::new();
    let stream_b   = Keypair::new();

    let locked_a = 100;
    let locked_b = 300;

    for (stream, beneficiary, locked) in [
        (&stream_a, &investor_a, locked_a),
        (&stream_b, &investor_b, locked_b),
    ] {
        streamflow
            .request()
            .accounts(mock_streamflow::accounts::InitStream {
                stream: stream.pubkey(),
                beneficiary: beneficiary.pubkey(),
                payer: payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(mock_streamflow::instruction::InitStream { locked })
            .signer(&payer)
            .signer(stream)
            .send()?;
    }

    /* ------------ simulate quote fees --------- */
    let fee_amount = 1_000_000u64;
    cp_amm
        .request()
        .accounts(mock_cp_amm::accounts::UpdateFee {
            position: position.pubkey(),
        })
        .args(mock_cp_amm::instruction::AddQuoteFee { amount: fee_amount })
        .send()?;

    /* ------------ crank distribution ---------- */
    let today = unix_day(Clock::get()?.unix_timestamp);
    let progress_seed = [
        b"progress",
        &today.to_le_bytes(),
    ]
    .concat();
    let (progress_pda, _bump) =
        Pubkey::find_program_address(&progress_seed, &fee_router::ID);

    let policy_seed = b"policy";
    let (policy_pda, _bump) =
        Pubkey::find_program_address(&[policy_seed], &fee_router::ID);

    // Create & seed a Policy account manually
    let policy_data = fee_router::state::Policy {
        y0: locked_a + locked_b,
        investor_fee_share_bps: 5000, // 50% max
        daily_cap_quote: 0,
        min_payout_lamports: 1,
        bump: 255,
    };
    let lamports = client
        .rpc()
        .get_minimum_balance_for_rent_exemption(std::mem::size_of_val(&policy_data))?;
    client
        .rpc()
        .request_airdrop(&payer.pubkey(), lamports)?;

    client.rpc().send_and_confirm_transaction(
        &solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[solana_sdk::system_instruction::create_account(
                &payer.pubkey(),
                &policy_pda,
                lamports,
                policy_data.try_to_vec()?.len() as u64,
                &fee_router::ID,
            )],
            Some(&payer.pubkey()),
            &[&payer],
            client.rpc().get_latest_blockhash()?,
        ),
    )?;

    fee_router
        .request()
        .accounts(fee_router::accounts::CrankPage {
            payer: payer.pubkey(),
            progress_pda,
            policy_pda,
            investor_fee_pos_owner_pda: pos_owner_pda,
            honorary_position: position.pubkey(),
            quote_treasury_ata: Pubkey::new_unique(),
            creator_quote_ata:  Pubkey::new_unique(),
            system_program: system_program::ID,
            token_program: spl_token::ID,
            cp_amm_program: cp_amm.id(),
            streamflow_program: streamflow.id(),
            remaining_accounts: vec![
                stream_a.pubkey().into(),
                stream_b.pubkey().into(),
            ],
        })
        .args(fee_router::instruction::CrankPage { page_len: 2 })
        .signer(&payer)
        .send()?;

    /* ------------ read streams, assert payouts - 50% total  ------------ */
    let a: mock_streamflow::Stream = streamflow.account(stream_a.pubkey())?;
    let b: mock_streamflow::Stream = streamflow.account(stream_b.pubkey())?;
    let invest_total = a.paid_quote + b.paid_quote;
    assert_eq!(invest_total, fee_amount / 2);              // 50 % to investors
    assert_eq!(a.paid_quote, fee_amount / 2 * locked_a / (locked_a + locked_b));
    assert_eq!(b.paid_quote, fee_amount / 2 * locked_b / (locked_a + locked_b));

    /* ------------- inject base fee, expect fail ------------------------ */
    cp_amm
        .request()
        .accounts(mock_cp_amm::accounts::UpdateFee {
            position: position.pubkey(),
        })
        .args(mock_cp_amm::instruction::AddBaseFee { amount: 10 })
        .send()?;

    let crank_res = fee_router
        .request()
        .accounts(fee_router::accounts::CrankPage {
            payer: payer.pubkey(),
            progress_pda,
            policy_pda,
            investor_fee_pos_owner_pda: pos_owner_pda,
            honorary_position: position.pubkey(),
            quote_treasury_ata: Pubkey::new_unique(),
            creator_quote_ata:  Pubkey::new_unique(),
            system_program: system_program::ID,
            token_program: spl_token::ID,
            cp_amm_program: cp_amm.id(),
            streamflow_program: streamflow.id(),
            remaining_accounts: vec![
                stream_a.pubkey().into(),
                stream_b.pubkey().into(),
            ],
        })
        .args(fee_router::instruction::CrankPage { page_len: 2 })
        .signer(&payer)
        .send();

    assert!(crank_res.is_err()); // must reject due to base fees
    Ok(())
}
// END tests/fee_router_bankrun.rs
