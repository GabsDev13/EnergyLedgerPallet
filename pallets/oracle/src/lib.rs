#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, dispatch::DispatchResult};
    use frame_system::pallet_prelude::*;
    use frame_support::weights::Weight;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // =========================
    // STORAGE
    // =========================

    #[pallet::storage]
    #[pallet::getter(fn energy_ledger)]
    pub type EnergyLedger<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, u32,   // network_id
        Blake2_128Concat, u64,   // timestamp
        (u32, u32),              // (delta_input, delta_output)
        OptionQuery
    >;

    // =========================
    // EVENTS
    // =========================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DeltaStored(u32, u64, u32, u32),
    }

    // =========================
    // ERRORS
    // =========================

    #[pallet::error]
    pub enum Error<T> {
        TimestampAlreadyExists,
    }

    // =========================
    // CALLS
    // =========================

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn submit_delta(
            origin: OriginFor<T>,
            network_id: u32,
            delta_input: u32,
            delta_output: u32,
            oracle_timestamp: u64,
        ) -> DispatchResult {

            ensure_signed(origin)?;

            // Evita sobrescrever timestamp já existente
            ensure!(
                !EnergyLedger::<T>::contains_key(network_id, oracle_timestamp),
                Error::<T>::TimestampAlreadyExists
            );

            EnergyLedger::<T>::insert(
                network_id,
                oracle_timestamp,
                (delta_input, delta_output)
            );

            Self::deposit_event(Event::DeltaStored(
                network_id,
                oracle_timestamp,
                delta_input,
                delta_output,
            ));

            Ok(())
        }
    }
}
