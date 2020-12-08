//! Proposals integration tests - with stake, membership, governance modules.

#![cfg(test)]

mod working_group_proposals;

use crate::{BlockNumber, ProposalCancellationFee, Runtime};
use codec::Encode;
use governance::election_params::ElectionParameters;
use membership;
use proposals_codex::GeneralProposalParameters;
use proposals_engine::{
    ApprovedProposalDecision, BalanceOf, Proposal, ProposalCreationParameters, ProposalParameters,
    ProposalStatus, VoteKind, VotersParameters, VotingResults,
};

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::{Currency, OnFinalize, OnInitialize};
use frame_support::{StorageMap, StorageValue};
use frame_system::RawOrigin;
use sp_runtime::AccountId32;

use super::{increase_total_balance_issuance_using_account_id, initial_test_ext, insert_member};

use crate::CouncilManager;

pub type Balances = pallet_balances::Module<Runtime>;
pub type System = frame_system::Module<Runtime>;
pub type ProposalsEngine = proposals_engine::Module<Runtime>;
pub type Council = governance::council::Module<Runtime>;
pub type Election = governance::election::Module<Runtime>;
pub type ProposalCodex = proposals_codex::Module<Runtime>;

fn setup_members(count: u8) {
    for i in 0..count {
        let account_id: [u8; 32] = [i; 32];
        let account_id_converted: AccountId32 = account_id.clone().into();
        insert_member(account_id_converted);
    }
}

fn setup_council() {
    let councilor0 = AccountId32::default();
    let councilor1: [u8; 32] = [1; 32];
    let councilor2: [u8; 32] = [2; 32];
    let councilor3: [u8; 32] = [3; 32];
    let councilor4: [u8; 32] = [4; 32];
    let councilor5: [u8; 32] = [5; 32];
    assert!(Council::set_council(
        frame_system::RawOrigin::Root.into(),
        vec![
            councilor0,
            councilor1.into(),
            councilor2.into(),
            councilor3.into(),
            councilor4.into(),
            councilor5.into()
        ]
    )
    .is_ok());
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
fn run_to_block(n: BlockNumber) {
    while System::block_number() < n {
        <System as OnFinalize<BlockNumber>>::on_finalize(System::block_number());
        <Election as OnFinalize<BlockNumber>>::on_finalize(System::block_number());
        <ProposalsEngine as OnFinalize<BlockNumber>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
        <Election as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
        <ProposalsEngine as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
    }
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: AccountId32,
    current_account_id_seed: u8,
    current_voter_id: u64,
    pub auto_increment_voter_id: bool,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_voter_id: 0,
            current_account_id_seed: 0,
            current_account_id: AccountId32::default(),
            auto_increment_voter_id: true,
        }
    }
    fn vote_and_assert_ok(&mut self, vote_kind: VoteKind) {
        self.vote_and_assert(vote_kind, Ok(()));
    }

    fn vote_and_assert(&mut self, vote_kind: VoteKind, expected_result: DispatchResult) {
        assert_eq!(self.vote(vote_kind.clone()), expected_result);
    }

    fn vote(&mut self, vote_kind: VoteKind) -> DispatchResult {
        if self.auto_increment_voter_id {
            self.current_account_id_seed += 1;
            self.current_voter_id += 1;
            let account_id: [u8; 32] = [self.current_account_id_seed; 32];
            self.current_account_id = account_id.into();
        }

        ProposalsEngine::vote(
            frame_system::RawOrigin::Signed(self.current_account_id.clone()).into(),
            self.current_voter_id,
            self.proposal_id,
            vote_kind,
            Vec::new(),
        )
    }
}

#[derive(Clone)]
struct DummyProposalFixture {
    parameters: ProposalParameters<u32, u128>,
    account_id: AccountId32,
    proposer_id: u64,
    proposal_code: Vec<u8>,
    title: Vec<u8>,
    description: Vec<u8>,
    staking_account_id: Option<AccountId32>,
    exact_execution_block: Option<u32>,
}

impl Default for DummyProposalFixture {
    fn default() -> Self {
        let title = b"title".to_vec();
        let description = b"description".to_vec();
        let dummy_proposal =
            proposals_codex::Call::<Runtime>::execute_text_proposal(b"text".to_vec());

        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
                approval_threshold_percentage: 60,
                slashing_quorum_percentage: 60,
                slashing_threshold_percentage: 60,
                grace_period: 0,
                required_stake: None,
                constitutionality: 1,
            },
            account_id: <Runtime as frame_system::Trait>::AccountId::default(),
            proposer_id: 0,
            proposal_code: dummy_proposal.encode(),
            title,
            description,
            staking_account_id: None,
            exact_execution_block: None,
        }
    }
}

impl DummyProposalFixture {
    fn with_parameters(self, parameters: ProposalParameters<u32, u128>) -> Self {
        DummyProposalFixture { parameters, ..self }
    }

    fn with_constitutionality(&self, constitutionality: u32) -> Self {
        DummyProposalFixture {
            parameters: ProposalParameters {
                constitutionality,
                ..self.parameters
            },
            ..self.clone()
        }
    }

    fn with_account_id(self, account_id: AccountId32) -> Self {
        DummyProposalFixture { account_id, ..self }
    }

    fn with_voting_period(self, voting_period: u32) -> Self {
        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period,
                ..self.parameters
            },
            ..self
        }
    }

    fn with_stake(self, account_id: AccountId32) -> Self {
        DummyProposalFixture {
            staking_account_id: Some(account_id),
            ..self
        }
    }

    fn with_proposer(self, proposer_id: u64) -> Self {
        DummyProposalFixture {
            proposer_id,
            ..self
        }
    }

    fn create_proposal_and_assert(self, result: Result<u32, DispatchError>) -> Option<u32> {
        let proposal_id_result = ProposalsEngine::create_proposal(ProposalCreationParameters {
            account_id: self.account_id,
            proposer_id: self.proposer_id,
            proposal_parameters: self.parameters,
            title: self.title,
            description: self.description,
            staking_account_id: self.staking_account_id,
            encoded_dispatchable_call_code: self.proposal_code,
            exact_execution_block: self.exact_execution_block,
        });

        assert_eq!(proposal_id_result, result);

        proposal_id_result.ok()
    }
}

struct CancelProposalFixture {
    origin: RawOrigin<AccountId32>,
    proposal_id: u32,
    proposer_id: u64,
}

impl CancelProposalFixture {
    fn new(proposal_id: u32) -> Self {
        let account_id = <Runtime as frame_system::Trait>::AccountId::default();
        CancelProposalFixture {
            proposal_id,
            origin: RawOrigin::Signed(account_id),
            proposer_id: 0,
        }
    }

    fn with_proposer(self, proposer_id: u64) -> Self {
        CancelProposalFixture {
            proposer_id,
            ..self
        }
    }

    fn cancel_and_assert(self, expected_result: DispatchResult) {
        assert_eq!(
            ProposalsEngine::cancel_proposal(
                self.origin.into(),
                self.proposer_id,
                self.proposal_id
            ),
            expected_result
        );
    }
}

/// Main purpose of this integration test: check balance of the member on proposal finalization (cancellation)
/// It tests StakingEventsHandler integration. Also, membership module is tested during the proposal creation (ActorOriginValidator).
#[test]
fn proposal_cancellation_with_slashes_with_balance_checks_succeeds() {
    initial_test_ext().execute_with(|| {
        let account_id = <Runtime as frame_system::Trait>::AccountId::default();

        setup_members(2);
        let member_id = 0; // newly created member_id

        let stake_amount = 20000u128;
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 60,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 5,
            required_stake: Some(stake_amount),
            constitutionality: 1,
        };
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters)
            .with_account_id(account_id.clone())
            .with_stake(account_id.clone())
            .with_proposer(member_id);

        let account_balance = 500000;
        let _imbalance = Balances::deposit_creating(&account_id, account_balance);

        assert_eq!(Balances::usable_balance(&account_id), account_balance);

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();
        assert_eq!(
            Balances::usable_balance(&account_id),
            account_balance - stake_amount
        );

        let proposal = ProposalsEngine::proposals(proposal_id);

        let expected_proposal = Proposal {
            parameters,
            proposer_id: member_id,
            activated_at: 0,
            status: ProposalStatus::Active,
            voting_results: VotingResults::default(),
            exact_execution_block: None,
            nr_of_council_confirmations: 0,
            staking_account_id: Some(account_id.clone()),
        };

        assert_eq!(proposal, expected_proposal);

        let cancel_proposal_fixture =
            CancelProposalFixture::new(proposal_id).with_proposer(member_id);

        cancel_proposal_fixture.cancel_and_assert(Ok(()));

        let cancellation_fee = ProposalCancellationFee::get() as u128;
        assert_eq!(
            Balances::usable_balance(&account_id),
            account_balance - cancellation_fee
        );
    });
}

#[test]
fn proposal_reset_succeeds() {
    initial_test_ext().execute_with(|| {
        setup_members(4);
        setup_council();
        // create proposal
        let dummy_proposal = DummyProposalFixture::default().with_voting_period(100);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // create some votes
        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);

        // check
        let proposal = ProposalsEngine::proposals(proposal_id);
        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 0,
                rejections: 1,
                slashes: 1,
            }
        );

        // Ensure council was elected
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 6);

        let voted_member_id = 2;
        // Check for votes.
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, voted_member_id),
            VoteKind::Abstain
        );

        // Check proposals CouncilElected hook just trigger the election hook (empty council).
        //<Runtime as governance::election::Trait>::CouncilElected::council_elected(Vec::new(), 10);

        elect_single_councilor();

        let updated_proposal = ProposalsEngine::proposals(proposal_id);

        assert_eq!(
            updated_proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 0,
                rejections: 0,
                slashes: 0,
            }
        );

        // No votes could survive cleaning: should be default value.
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, voted_member_id),
            VoteKind::default()
        );

        // Check council CouncilElected hook. It should set current council. And we elected single councilor.
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 1);
    });
}

fn elect_single_councilor() {
    let res = Election::set_election_parameters(
        RawOrigin::Root.into(),
        ElectionParameters {
            announcing_period: 1,
            voting_period: 1,
            revealing_period: 1,
            council_size: 1,
            candidacy_limit: 10,
            new_term_duration: 2000000,
            min_council_stake: 0,
            min_voting_stake: 0,
        },
    );
    assert_eq!(res, Ok(()));

    let res = Election::force_start_election(RawOrigin::Root.into());
    assert_eq!(res, Ok(()));

    let councilor1: [u8; 32] = [1; 32];
    increase_total_balance_issuance_using_account_id(councilor1.clone().into(), 1200000000);

    let res = Election::apply(RawOrigin::Signed(councilor1.into()).into(), 0);
    assert_eq!(res, Ok(()));

    run_to_block(5);
}

struct CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    successful_call: SuccessfulCall,
    member_id: u64,
    setup_environment: bool,
    proposal_id: u32,
    run_to_block: u32,
}

impl<SuccessfulCall> CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn default_for_call(call: SuccessfulCall) -> Self {
        Self {
            successful_call: call,
            member_id: 1,
            setup_environment: true,
            proposal_id: 1,
            run_to_block: 2,
        }
    }

    fn disable_setup_enviroment(self) -> Self {
        Self {
            setup_environment: false,
            ..self
        }
    }
    fn with_setup_enviroment(self, setup_environment: bool) -> Self {
        Self {
            setup_environment,
            ..self
        }
    }

    fn with_member_id(self, member_id: u64) -> Self {
        Self { member_id, ..self }
    }

    fn with_expected_proposal_id(self, expected_proposal_id: u32) -> Self {
        Self {
            proposal_id: expected_proposal_id,
            ..self
        }
    }

    fn with_run_to_block(self, run_to_block: u32) -> Self {
        Self {
            run_to_block,
            ..self
        }
    }
}

impl<SuccessfulCall> CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn call_extrinsic_and_assert(&self) {
        let account_id: [u8; 32] = [self.member_id as u8; 32];

        if self.setup_environment {
            setup_members(15);
            setup_council();

            increase_total_balance_issuance_using_account_id(account_id.clone().into(), 1_500_000);
        }

        assert_eq!((self.successful_call)(), Ok(()));

        let mut vote_generator = VoteGenerator::new(self.proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block(self.run_to_block);
    }
}

#[test]
fn text_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
            };

            ProposalCodex::create_text_proposal(
                RawOrigin::Signed(account_id.into()).into(),
                general_proposal_parameters,
                b"text".to_vec(),
                None,
            )
        })
        .with_member_id(member_id as u64);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();
    });
}

#[test]
fn spending_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];
        let new_balance = <BalanceOf<Runtime>>::from(5555u32);

        let target_account_id: [u8; 32] = [12; 32];

        assert!(Council::set_council_mint_capacity(RawOrigin::Root.into(), new_balance).is_ok());

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
            };

            ProposalCodex::create_spending_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                general_proposal_parameters,
                new_balance,
                target_account_id.clone().into(),
                None,
            )
        })
        .with_member_id(member_id as u64);

        let converted_account_id: AccountId32 = target_account_id.clone().into();
        assert_eq!(Balances::free_balance(converted_account_id.clone()), 0);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        run_to_block(14410);

        assert_eq!(Balances::free_balance(converted_account_id), new_balance);
    });
}

#[test]
fn set_validator_count_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 1;
        let account_id: [u8; 32] = [member_id; 32];

        let new_validator_count = 8;
        assert_eq!(<pallet_staking::ValidatorCount>::get(), 0);

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
            };

            ProposalCodex::create_set_validator_count_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                general_proposal_parameters,
                new_validator_count,
                None,
            )
        });
        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        run_to_block(14410);

        assert_eq!(<pallet_staking::ValidatorCount>::get(), new_validator_count);
    });
}

#[test]
fn amend_constitution_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
            };

            ProposalCodex::create_amend_constitution_proposal(
                RawOrigin::Signed(account_id.into()).into(),
                general_proposal_parameters,
                b"Constitution text".to_vec(),
                None,
            )
        })
        .with_member_id(member_id as u64);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();
    });
}

#[test]
fn proposal_reactivation_succeeds() {
    initial_test_ext().execute_with(|| {
        let starting_block = 0;
        setup_members(5);
        setup_council();
        // create proposal
        let dummy_proposal = DummyProposalFixture::default()
            .with_voting_period(100)
            .with_constitutionality(2);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // create some votes
        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block(2);

        // check
        let proposal = ProposalsEngine::proposals(proposal_id);
        assert_eq!(
            proposal.status,
            ProposalStatus::approved(
                ApprovedProposalDecision::PendingConstitutionality,
                starting_block
            )
        );

        // Ensure council was elected
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 6);

        elect_single_councilor();

        run_to_block(10);

        let updated_proposal = ProposalsEngine::proposals(proposal_id);

        assert_eq!(updated_proposal.status, ProposalStatus::Active);

        // Check council CouncilElected hook. It should set current council. And we elected single councilor.
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 1);
    });
}
