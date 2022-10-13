use crate::{self as oracle_event_feed, Config, Error};
use frame_support::{assert_noop, assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        VecSet: vec_set::{Module, Call, Storage, Event<T>},
        GenericEvent: oracle_event_feed::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl vec_set::Config for TestRuntime {
    type Event = Event;
}

impl Config for TestRuntime {
    type Event = Event;
    type MembershipSource = VecSet;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
    pub fn build() -> TestExternalities {
        let storage = frame_system::GenesisConfig::default()
            .build_storage::<TestRuntime>()
            .unwrap();
        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

#[test]
fn test() {
    ExternalityBuilder::build().execute_with(|| {
        let input = [0u8; 100];
        let auth_acc_id = 2; // assumed to be authorised account
        let unauth_acc_id = 1; // assumed to be unauthorised account

        // add only authorised account to membership set
        assert_ok!(VecSet::add_member(Origin::signed(auth_acc_id)));

        // negative test for unauthorised account (not in membership set)
        assert_noop!(
            GenericEvent::oracle_event_feed(Origin::signed(unauth_acc_id), input.to_vec()),
            Error::<TestRuntime>::NotAMember
        );

        // test with authorised account
        assert_ok!(GenericEvent::oracle_event_feed(
            Origin::signed(auth_acc_id),
            input.to_vec()
        ));

        // test whether input bytes is same as that in storage after event is posted
        let new_oracle_feed = GenericEvent::get_oracle_feed();
        assert_eq!(new_oracle_feed[0], input.to_vec());

        // construct event that should be emitted in the method call directly above
        let expected_event =
            Event::oracle_event_feed(oracle_event_feed::Event::EmitEvent(input.to_vec()));

        // test the expected event is same as the event in System events
        assert_eq!(System::events()[1].event, expected_event,);
    })
}
