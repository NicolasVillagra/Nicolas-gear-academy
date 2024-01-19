#![no_std]
use escrow_io::{EscrowState, ProgramMetadata};
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId, Vec};

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn seller(state: State) -> ActorId {
        state.seller
    }

    pub fn buyer(state: State) -> ActorId {
        state.buyer
    }

    pub fn escrow_state(state: State) -> EscrowState {
        state.state
    }
}
