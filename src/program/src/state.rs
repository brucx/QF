use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use num_enum::TryFromPrimitive;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use spl_math::uint::U256;

/// Round status
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
pub enum RoundStatus {
    Uninitialized,
    Ongoing,
    Finished,
}

impl Default for RoundStatus {
    fn default() -> Self {
        RoundStatus::Uninitialized
    }
}

/// Round
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Round {
    pub status: RoundStatus,
    pub ratio: u8,
    pub fund: u64,
    pub fee: u64,
    pub project_number: u64,
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub area: U256,
    pub total_area: U256,
    pub top_area: U256,
    pub min_area: U256,
    pub min_area_p: Pubkey,
}
impl Sealed for Round {}
impl IsInitialized for Round {
    fn is_initialized(&self) -> bool {
        self.status != RoundStatus::Uninitialized
    }
}
impl Pack for Round {
    const LEN: usize = 250;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 250];
        let (
            status,
            ratio,
            fund,
            fee,
            project_number,
            vault,
            owner,
            area,
            total_area,
            top_area,
            min_area,
            min_area_p,
        ) = array_refs![src, 1, 1, 8, 8, 8, 32, 32, 32, 32, 32, 32, 32];
        Ok(Round {
            status: RoundStatus::try_from_primitive(status[0])
                .or(Err(ProgramError::InvalidAccountData))?,
            ratio: u8::from_le_bytes(*ratio),
            fund: u64::from_le_bytes(*fund),
            fee: u64::from_le_bytes(*fee),
            project_number: u64::from_le_bytes(*project_number),
            vault: Pubkey::new_from_array(*vault),
            owner: Pubkey::new_from_array(*owner),
            area: U256::from_little_endian(area),
            total_area: U256::from_little_endian(total_area),
            top_area: U256::from_little_endian(top_area),
            min_area: U256::from_little_endian(min_area),
            min_area_p: Pubkey::new_from_array(*min_area_p),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 250];
        let (
            status_dst,
            ratio_dst,
            fund_dst,
            fee_dst,
            project_number_dst,
            vault_dst,
            owner_dst,
            area_dst,
            total_area_dst,
            top_area_dst,
            min_area_dst,
            min_area_p_dst,
        ) = mut_array_refs![dst, 1, 1, 8, 8, 8, 32, 32, 32, 32, 32, 32, 32];
        let &Round {
            status,
            ratio,
            fund,
            fee,
            project_number,
            ref owner,
            ref vault,
            area,
            total_area,
            top_area,
            min_area,
            min_area_p,
        } = self;
        status_dst[0] = status as u8;
        *ratio_dst = ratio.to_le_bytes();
        *fund_dst = fund.to_le_bytes();
        *fee_dst = fee.to_le_bytes();
        *project_number_dst = project_number.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        vault_dst.copy_from_slice(vault.as_ref());
        area.to_little_endian(area_dst);
        total_area.to_little_endian(total_area_dst);
        top_area.to_little_endian(top_area_dst);
        min_area.to_little_endian(min_area_dst);
        min_area_p_dst.copy_from_slice(min_area_p.as_ref());
    }
}

/// Project
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Project {
    pub round: Pubkey,
    pub owner: Pubkey,
    pub withdraw: bool,
    pub votes: u64,
    pub area: U256,
    pub area_sqrt: U256,
}
impl Sealed for Project {}
impl IsInitialized for Project {
    fn is_initialized(&self) -> bool {
        self.round != Pubkey::default()
    }
}
impl Pack for Project {
    const LEN: usize = 137;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 137];
        let (round, owner, withdraw, votes, area, area_sqrt) =
            array_refs![src, 32, 32, 1, 8, 32, 32];
        Ok(Project {
            round: Pubkey::new_from_array(*round),
            owner: Pubkey::new_from_array(*owner),
            withdraw: match withdraw {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            votes: u64::from_le_bytes(*votes),
            area: U256::from_little_endian(area),
            area_sqrt: U256::from_little_endian(area_sqrt),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 137];
        let (round_dst, owner_dst, withdraw_dst, votes_dst, area_dst, area_sqrt_dst) =
            mut_array_refs![dst, 32, 32, 1, 8, 32, 32];
        let &Project {
            ref round,
            ref owner,
            withdraw,
            votes,
            area,
            area_sqrt,
        } = self;
        round_dst.copy_from_slice(round.as_ref());
        owner_dst.copy_from_slice(owner.as_ref());
        withdraw_dst[0] = withdraw as u8;
        *votes_dst = votes.to_le_bytes();
        area.to_little_endian(area_dst);
        area_sqrt.to_little_endian(area_sqrt_dst);
    }
}

/// Voter
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Voter {
    pub is_initialized: bool,
    pub votes: u64,
    pub votes_sqrt: U256,
}
impl Sealed for Voter {}
impl IsInitialized for Voter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for Voter {
    const LEN: usize = 41;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 41];
        let (is_initialized, votes, votes_sqrt) = array_refs![src, 1, 8, 32];
        Ok(Voter {
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            votes: u64::from_le_bytes(*votes),
            votes_sqrt: U256::from_little_endian(votes_sqrt),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 41];
        let (is_initialized_dst, votes_dst, votes_sqrt_dst) = mut_array_refs![dst, 1, 8, 32];
        let &Voter {
            is_initialized,
            votes,
            votes_sqrt,
        } = self;
        is_initialized_dst[0] = is_initialized as u8;
        *votes_dst = votes.to_le_bytes();
        votes_sqrt.to_little_endian(votes_sqrt_dst);
    }
}
