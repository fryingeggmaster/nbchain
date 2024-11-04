#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use sp_std::vec;
use xcm::latest::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::Encode;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Pallet Configuration
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Origin: From<<Self as SystemConfig>::Origin>
            + Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

        type Call: From<Call<Self>> + Encode;

        type XcmSender: SendXcm;
    }

    // Counter
    #[pallet::storage]
    #[pallet::getter(fn get_counter)]
    pub type Counter<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        CounterSet(ParaId, u32),
        ErrorSettingCounter(SendError, ParaId, u32),
        CounterIncremented(ParaId),
        ErrorIncrementingCounter(SendError, ParaId),
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1000 + T::DbWeight::get().writes(1))]
        pub fn set_counter(
            origin: OriginFor<T>,
            para: ParaId,
            value: u32,
        ) -> DispatchResultWithPostInfo {
            // Set counter of Parachain Id: <ParaId> to value : <value>
            // by sending Xcm call to parachain

            ensure_root(origin)?;
            match T::XcmSender::send_xcm(
                (1, Junction::Parachain(para.into())),
                Xcm(vec![Transact {
                    origin_type: OriginKind::Native,
                    require_weight_at_most: 1_000,
                    call: <T as Config>::Call::from(Call::<T>::set_counter_value { value })
                        .encode()
                        .into(),
                }]),
            ) {
                Ok(()) => {
                    Self::deposit_event(Event::CounterSet(para, value));
                }
                Err(e) => {
                    Self::deposit_event(Event::ErrorSettingCounter(e, para, value));
                }
            }

            Ok(().into())
        }

        #[pallet::weight(1000 + T::DbWeight::get().writes(1))]
        pub fn increment_counter(
            origin: OriginFor<T>,
            para: ParaId,
            require_weight_at_most: u64,
        ) -> DispatchResultWithPostInfo {
            // Increment Counter value of parachain Id: <ParaId> with plus 1
            // by sending Xcm call to parachain::pallet::call

            ensure_root(origin)?;
            match T::XcmSender::send_xcm(
                (1, Junction::Parachain(para.into())),
                Xcm(vec![Transact {
                    origin_type: OriginKind::Native,
                    require_weight_at_most,
                    call: <T as Config>::Call::from(Call::<T>::increment_counter_value {})
                        .encode()
                        .into(),
                }]),
            ) {
                Ok(()) => {
                    Self::deposit_event(Event::CounterIncremented(para));
                }
                Err(e) => {
                    Self::deposit_event(Event::ErrorIncrementingCounter(e, para));
                }
            }

            Ok(().into())
        }

        #[pallet::weight(1000)]
        pub fn set_counter_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;
            // Update Counter Value.
            <Counter<T>>::put(value);
            Self::deposit_event(Event::CounterSet(para, value));
            Ok(())
        }

        #[pallet::weight(1000)]
        pub fn increment_counter_value(origin: OriginFor<T>) -> DispatchResult {
            let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;
            // Increment Counter with plus 1
            let count = Counter::<T>::mutate(|cnt| {
                *cnt += 1;
                *cnt
            });
            <Counter<T>>::put(count);
            Self::deposit_event(Event::CounterIncremented(para));
            Ok(())
        }
    }
}
