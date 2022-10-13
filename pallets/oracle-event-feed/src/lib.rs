//! pallet for an oracle event feed of arbitrary length bytes with access controls and storage

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use account_set::AccountSet;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Vec};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// A type that will supply a set of members to check access control against
        type MembershipSource: AccountSet<AccountId = Self::AccountId>;

        /// The ubiquitous event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The caller is not a member
        NotAMember,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    #[pallet::getter(fn get_oracle_feed)]
    pub type OracleFeed<T: Config> = StorageValue<_, Vec<Vec<u8>>, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Some input was sent
        EmitEvent(Vec<u8>),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// A call to check access control and add feed to storage
        #[pallet::weight(10_000)]
        pub fn oracle_event_feed(
            origin: OriginFor<T>,
            input: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            // Get the members from the `vec-set` pallet
            let members = T::MembershipSource::accounts();

            // Check whether the caller is a member
            ensure!(members.contains(&user), Error::<T>::NotAMember);

            // add the arbitrary length bytes to Oracle Feed store
            let new_number = input;
            let mut oracle_feed = OracleFeed::<T>::get();
            oracle_feed.insert(oracle_feed.len(), new_number.clone());
            OracleFeed::<T>::put(oracle_feed);

            Self::deposit_event(Event::EmitEvent(new_number));

            Ok(().into())
        }
    }
}
