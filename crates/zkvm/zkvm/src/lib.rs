use serde::{de::DeserializeOwned, Serialize};

/// Validity proof generated by the `ZKVMHost`
#[derive(Debug, Clone)]
pub struct Proof(Vec<u8>);

impl Proof {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Verification Key required to verify proof generated from `ZKVMHost`
#[derive(Debug, Clone)]
pub struct VerificationKey(pub Vec<u8>);

impl VerificationKey {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Prover config of the ZKVM Host
#[derive(Debug, Clone, Copy)]
pub struct ProverOptions {
    pub enable_compression: bool,
    pub use_mock_prover: bool,
    pub stark_to_snark_conversion: bool,
}

/// A trait for managing inputs to a ZKVM prover. This trait provides methods for
/// adding inputs in various formats to be used during the proof generation process.
pub trait ZKVMInputBuilder<'a> {
    type Input;

    /// Creates a new instance of the `ProverInputs` struct.
    fn new() -> Self;

    /// Serializes the given item using Serde and appends it to the list of inputs.
    fn write<T: serde::Serialize>(&mut self, item: &T) -> anyhow::Result<&mut Self>;

    /// Serializes the given item using the Borsh serialization format and appends
    /// it to the list of inputs.
    fn write_borsh<T: borsh::BorshSerialize>(&mut self, item: &T) -> anyhow::Result<&mut Self>;

    /// Appends a pre-serialized byte array to the list of inputs.
    ///
    /// This method is intended for cases where the data has already been serialized
    /// outside of the zkVM's standard serialization methods. It allows you to provide
    /// serialized inputs directly, bypassing any further serialization.
    fn write_serialized(&mut self, item: &[u8]) -> anyhow::Result<&mut Self>;

    /// Adds an `AggregationInput` to the list of aggregation/composition inputs.
    ///
    /// This method is specifically used for cases where proof aggregation or composition
    /// is involved, allowing for complex proof inputs to be provided to the zkVM.
    fn write_proof(&mut self, item: AggregationInput) -> anyhow::Result<&mut Self>;

    fn build(&mut self) -> anyhow::Result<Self::Input>;
}

/// A trait implemented by the prover ("host") of a zkVM program.
pub trait ZKVMHost: Send + Sync + Clone {
    type Input<'a>: ZKVMInputBuilder<'a>;

    /// Initializes the ZKVM with the provided ELF program and prover configuration.
    fn init(guest_code: Vec<u8>, prover_options: ProverOptions) -> Self;

    /// Executes the guest code within the VM, generating and returning the validity proof.
    // TODO: Consider using custom error types instead of a generic error to capture the different
    // reasons proving can fail.
    fn prove<'a>(
        &self,
        input: <Self::Input<'a> as ZKVMInputBuilder<'a>>::Input,
    ) -> anyhow::Result<(Proof, VerificationKey)>;
}

/// A trait implemented by a verifier to decode and verify the proof generated by the prover
/// ("host").
pub trait ZKVMVerifier {
    /// Verifies the proof generated by the prover against the `program_id`.
    fn verify(verification_key: &VerificationKey, proof: &Proof) -> anyhow::Result<()>;

    /// Verifies the proof generated by the prover against the given `program_id` and
    /// `public_params`.
    fn verify_with_public_params<T: Serialize + DeserializeOwned>(
        verification_key: &VerificationKey,
        public_params: T,
        proof: &Proof,
    ) -> anyhow::Result<()>;

    fn verify_groth16(
        proof: &[u8],
        verification_key: &[u8],
        public_params_raw: &[u8],
    ) -> anyhow::Result<()>;

    /// Extracts the public output from the proof.
    fn extract_public_output<T: Serialize + DeserializeOwned>(proof: &Proof) -> anyhow::Result<T>;
}

impl Default for ProverOptions {
    fn default() -> Self {
        Self {
            enable_compression: false,
            use_mock_prover: true,
            stark_to_snark_conversion: false,
        }
    }
}

/// An input to the aggregation program.
///
/// Consists of a proof and a verification key.
#[derive(Debug, Clone)]
pub struct AggregationInput {
    proof: Proof,
    vk: VerificationKey,
}

impl AggregationInput {
    pub fn new(proof: Proof, vk: VerificationKey) -> Self {
        Self { proof, vk }
    }

    pub fn proof(&self) -> &Proof {
        &self.proof
    }

    pub fn vk(&self) -> &VerificationKey {
        &self.vk
    }
}
