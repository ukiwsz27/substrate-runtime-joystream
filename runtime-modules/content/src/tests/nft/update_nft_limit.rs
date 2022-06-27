#![cfg(test)]
use crate::tests::curators;
use crate::tests::curators::add_curator_to_new_group_with_permissions;
use crate::tests::fixtures::*;
use crate::tests::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok};
use frame_system::RawOrigin;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::iter::FromIterator;

#[test]
fn update_nft_limits_works_as_expected_for_global_limits() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);

        let lead = RawOrigin::Signed(LEAD_ACCOUNT_ID);
        let root = RawOrigin::Root;
        let member = RawOrigin::Signed(DEFAULT_MEMBER_ACCOUNT_ID);

        for period in [NftLimitPeriod::Daily, NftLimitPeriod::Weekly].iter() {
            update_global_nft_limit_test_helper(root.clone(), *period, Ok(()));
            update_global_nft_limit_test_helper(
                lead.clone(),
                *period,
                Err(DispatchError::BadOrigin),
            );
            update_global_nft_limit_test_helper(
                member.clone(),
                *period,
                Err(DispatchError::BadOrigin),
            );
        }
    })
}

#[test]
fn update_nft_limits_works_as_expected_for_channel_limits() {
    with_default_mock_builder(|| {
        ContentTest::with_member_channel().setup();

        let authorized_group_id = add_curator_to_new_group_with_permissions(
            DEFAULT_CURATOR_ID,
            BTreeMap::from_iter(vec![(
                0,
                BTreeSet::from_iter(vec![ContentModerationAction::UpdateChannelNftLimits]),
            )]),
        );
        let unauthorized_group_id = add_curator_to_new_group_with_permissions(
            DEFAULT_CURATOR_ID,
            BTreeMap::from_iter(vec![(0, BTreeSet::new())]),
        );
        let lead = RawOrigin::Signed(LEAD_ACCOUNT_ID);
        let curator = RawOrigin::Signed(DEFAULT_CURATOR_ACCOUNT_ID);
        let root = RawOrigin::Root;
        let member = RawOrigin::Signed(DEFAULT_MEMBER_ACCOUNT_ID);
        let channel_id = 1;

        for period in [NftLimitPeriod::Daily, NftLimitPeriod::Weekly].iter() {
            update_channel_nft_limit_test_helper(
                lead.clone(),
                ContentActor::Lead,
                *period,
                channel_id,
                Ok(()),
            );
            update_channel_nft_limit_test_helper(
                curator.clone(),
                ContentActor::Lead,
                *period,
                channel_id,
                Err(Error::<Test>::LeadAuthFailed.into()),
            );
            update_channel_nft_limit_test_helper(
                curator.clone(),
                ContentActor::Curator(authorized_group_id, DEFAULT_CURATOR_ID),
                *period,
                channel_id,
                Ok(()),
            );
            update_channel_nft_limit_test_helper(
                curator.clone(),
                ContentActor::Curator(unauthorized_group_id, DEFAULT_CURATOR_ID),
                *period,
                channel_id,
                Err(Error::<Test>::CuratorModerationActionNotAllowed.into()),
            );
            update_channel_nft_limit_test_helper(
                member.clone(),
                ContentActor::Curator(authorized_group_id, DEFAULT_CURATOR_ID),
                *period,
                channel_id,
                Err(Error::<Test>::CuratorAuthFailed.into()),
            );
            update_channel_nft_limit_test_helper(
                member.clone(),
                ContentActor::Member(DEFAULT_MEMBER_ID),
                *period,
                channel_id,
                Err(Error::<Test>::ActorNotAuthorized.into()),
            );
            update_channel_nft_limit_test_helper(
                root.clone(),
                ContentActor::Lead,
                *period,
                channel_id,
                Err(DispatchError::BadOrigin),
            );
        }
    })
}

fn update_global_nft_limit_test_helper(
    origin: RawOrigin<AccountId>,
    period: NftLimitPeriod,
    expected_result: DispatchResult,
) {
    let new_limit: u64 = 7777;

    UpdateGlobalNftLimitFixture::new()
        .with_origin(origin)
        .with_period(period)
        .with_limit(new_limit)
        .call_and_assert(expected_result);
}

fn update_channel_nft_limit_test_helper(
    origin: RawOrigin<AccountId>,
    actor: ContentActor<CuratorGroupId, CuratorId, MemberId>,
    period: NftLimitPeriod,
    channel_id: ChannelId,
    expected_result: DispatchResult,
) {
    let new_limit: u64 = 7777;

    UpdateChannelNftLimitFixture::new()
        .with_origin(origin)
        .with_actor(actor)
        .with_period(period)
        .with_channel_id(channel_id)
        .with_limit(new_limit)
        .call_and_assert(expected_result);
}

// channel creation tests
#[test]
fn default_channel_nft_limits_set_successfully() {
    with_default_mock_builder(|| {
        // Run to block one to see emitted events
        run_to_block(1);
        set_dynamic_bag_creation_policy_for_storage_numbers(0);
        create_initial_storage_buckets_helper();

        CreateChannelFixture::default()
            .with_default_storage_buckets()
            .with_sender(DEFAULT_MEMBER_ACCOUNT_ID)
            .with_channel_owner(ChannelOwner::Member(DEFAULT_MEMBER_ID))
            .call_and_assert(Ok(()));

        let channel_id = 1;
        let channel = Content::channel_by_id(channel_id);
        assert_eq!(channel.daily_nft_limit, DefaultChannelDailyNftLimit::get());
        assert_eq!(
            channel.weekly_nft_limit,
            DefaultChannelWeeklyNftLimit::get()
        );
    })
}

#[test]
fn toggle_nft_limits_fails_with_unauthorized_actor() {
    with_default_mock_builder(|| {
        run_to_block(1);
        let result_with_member_context = Content::toggle_nft_limits(
            Origin::signed(DEFAULT_MEMBER_ACCOUNT_ID),
            ContentActor::Member(DEFAULT_MEMBER_ID),
        );

        assert_err!(
            result_with_member_context,
            Error::<Test>::ActorNotAuthorized
        );

        let curator_group_id = curators::create_curator_group(BTreeMap::new());
        let result_with_curator_context = Content::toggle_nft_limits(
            Origin::signed(DEFAULT_CURATOR_ACCOUNT_ID),
            ContentActor::Curator(curator_group_id, DEFAULT_CURATOR_ID),
        );

        assert_err!(
            result_with_curator_context,
            Error::<Test>::ActorNotAuthorized,
        );
    })
}

#[test]
fn toggle_nft_limits_fails_with_authentication_error() {
    with_default_mock_builder(|| {
        run_to_block(1);
        let result = Content::toggle_nft_limits(
            Origin::signed(DEFAULT_MEMBER_ACCOUNT_ID),
            ContentActor::Lead,
        );

        assert_err!(result, Error::<Test>::LeadAuthFailed);
    })
}

#[test]
fn toggle_nft_limits_ok_with_event_deposited_and_status_changed() {
    with_default_mock_builder(|| {
        run_to_block(1);
        // nft limits is true by chainspec configuration, changing to false..
        assert_ok!(Content::toggle_nft_limits(
            Origin::signed(LEAD_ACCOUNT_ID),
            ContentActor::Lead,
        ));

        assert_eq!(
            System::events().last().unwrap().event,
            MetaEvent::Content(RawEvent::LeadToggledNftLimitsEnabled(false)),
        );
        assert_eq!(Content::nft_limits_enabled(), false);
    })
}

#[test]
fn toggle_nft_limits_ok_with_status_unchanged_after_toggling_twice() {
    with_default_mock_builder(|| {
        run_to_block(1);
        // nft limits is true by chainspec configuration
        assert_ok!(Content::toggle_nft_limits(
            Origin::signed(LEAD_ACCOUNT_ID),
            ContentActor::Lead,
        ));
        assert_ok!(Content::toggle_nft_limits(
            Origin::signed(LEAD_ACCOUNT_ID),
            ContentActor::Lead,
        ));

        assert_eq!(Content::nft_limits_enabled(), true);
    })
}
