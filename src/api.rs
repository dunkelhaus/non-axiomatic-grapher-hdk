use error::{ZomeApiError, ZomeApiResult};
use globals::*;
use holochain_core_types::{
    cas::content::Address,
    entry::{Entry, SerializedEntry},
    error::{CoreError, HolochainError, RibosomeReturnCode, ZomeApiInternalResult},
};
pub use holochain_wasm_utils::api_serialization::validation::*;
use holochain_wasm_utils::{
    api_serialization::{
        get_entry::GetEntryOptions,
        get_links::{GetLinksArgs, GetLinksResult},
        link_entries::LinkEntriesArgs,
        QueryArgs, QueryResult, ZomeFnCallArgs,
    },
    holochain_core_types::{
        hash::HashString,
        json::{JsonString, RawString},
    },
    memory_allocation::*,
    memory_serialization::*,
};
use serde_json;
use std::{convert::TryInto, os::raw::c_char};

//--------------------------------------------------------------------------------------------------
// ZOME API GLOBAL VARIABLES
//--------------------------------------------------------------------------------------------------

lazy_static! {
  /// The `name` property as taken from the DNA.
  pub static ref DNA_NAME: &'static str = &GLOBALS.dna_name;

  /// The hash of the DNA the Zome is embedded within.
  /// This is often useful as a fixed value that is known by all
  /// participants running the DNA.
  pub static ref DNA_HASH: &'static HashString = &GLOBALS.dna_hash;

  /// The identity string used when the chain was first initialized.
  pub static ref AGENT_ID_STR: &'static str = &GLOBALS.agent_id_str;

  /// The hash of your public key.
  /// This is your node address on the DHT.
  /// It can be used for node-to-node messaging with `send` and `receive` functions.
  pub static ref AGENT_ADDRESS: &'static Address = &GLOBALS.agent_address;

  /// The hash of the first identity entry on your chain (The second entry on your chain).
  /// This is your peer's identity on the DHT.
  pub static ref AGENT_INITIAL_HASH: &'static HashString = &GLOBALS.agent_initial_hash;

  #[doc(hidden)]
  /// The hash of the most recent identity entry that has been committed to your chain.
  /// Starts with the same value as AGENT_INITIAL_HASH.
  /// After a call to `update_agent` it will have the value of the hash of the newly committed identity entry.
  pub static ref AGENT_LATEST_HASH: &'static HashString = &GLOBALS.agent_latest_hash;
}

impl From<DNA_NAME> for JsonString {
    fn from(dna_name: DNA_NAME) -> JsonString {
        JsonString::from(RawString::from(dna_name.to_string()))
    }
}

trait Syllogism {
    // The implementation that the syllogistic rule functions share go in here.

    fn load(self) {
        // Accepts a function which it uses as the Rule for this instantiation of the trait.
    }
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Syllogism for fn(Params) -> u32 {
    // Functions specific to the specific syllogistic rule goes in here in case there is any variation from the above - some functions can be overloaded here from above.

    // fn load(self) {
        // Accepts a function which it uses as the Rule for this instantiation of the trait.
   //  }
}

fn syllogisticInferenceRuleA(params: Params) -> u32 {
    // Actual Rule implementation here
}

// ************* main ****************

// Call the function in this manner.
Syllogism::load(syllogisticInferenceRule as fn(Params) -> u32);
