use std::env::set_var;

use getset::Getters;
use halo2_base::{
    gates::{
        circuit::{builder::BaseCircuitBuilder, BaseCircuitParams, CircuitBuilderStage},
        flex_gate::{threads::SinglePhaseCoreManager, MultiPhaseThreadBreakPoints},
        RangeChip,
    },
    utils::BigPrimeField,
    AssignedValue, Context,
};

use crate::util::{builder::CommonCircuitBuilder, gates::CommonGateManager};

use super::FIRST_PHASE;

#[derive(Getters)]
pub struct ShaCircuitBuilder<F: BigPrimeField, ThreadBuilder: CommonGateManager<F>> {
    #[getset(get = "pub", get_mut = "pub")]
    pub(crate) sha: ThreadBuilder,
    #[getset(get = "pub", get_mut = "pub")]
    pub(crate) base: BaseCircuitBuilder<F>,
}

impl<F: BigPrimeField, GateManager: CommonGateManager<F>> ShaCircuitBuilder<F, GateManager> {
    pub fn new(witness_gen_only: bool) -> Self {
        let base = BaseCircuitBuilder::new(witness_gen_only);
        Self {
            sha: GateManager::new(witness_gen_only)
                .use_copy_manager(base.core().phase_manager[FIRST_PHASE].copy_manager.clone()),
            base,
        }
    }

    pub fn from_stage(stage: CircuitBuilderStage) -> Self {
        Self::new(stage == CircuitBuilderStage::Prover)
            .unknown(stage == CircuitBuilderStage::Keygen)
    }

    pub fn unknown(mut self, use_unknown: bool) -> Self {
        self.sha = self.sha.unknown(use_unknown);
        self.base = self.base.unknown(use_unknown);
        self
    }

    /// Creates a new [ShaCircuitBuilder] with `use_unknown` of [ShaThreadBuilder] set to true.
    pub fn keygen() -> Self {
        Self::from_stage(CircuitBuilderStage::Keygen)
    }

    /// Creates a new [ShaCircuitBuilder] with `use_unknown` of [GateThreadBuilder] set to false.
    pub fn mock() -> Self {
        Self::from_stage(CircuitBuilderStage::Mock)
    }

    /// Creates a new [ShaCircuitBuilder].
    pub fn prover() -> Self {
        Self::from_stage(CircuitBuilderStage::Prover)
    }

    /// The log_2 size of the lookup table, if using.
    pub fn lookup_bits(&self) -> Option<usize> {
        self.base.lookup_bits()
    }

    /// Set lookup bits
    pub fn set_lookup_bits(&mut self, lookup_bits: usize) {
        self.base.set_lookup_bits(lookup_bits);
    }

    /// Returns new with lookup bits
    pub fn use_lookup_bits(mut self, lookup_bits: usize) -> Self {
        self.set_lookup_bits(lookup_bits);
        self
    }

    /// Set the number of instance columns. This resizes `self.base().assigned_instances`.
    pub fn set_instance_columns(&mut self, num_instance_columns: usize) {
        self.base.set_instance_columns(num_instance_columns)
    }

    /// Returns new with `self.assigned_instances` resized to specified number of instance columns.
    pub fn use_instance_columns(mut self, num_instance_columns: usize) -> Self {
        self.set_instance_columns(num_instance_columns);
        self
    }

    pub fn set_instances(&mut self, column: usize, assigned_instances: Vec<AssignedValue<F>>) {
        self.base.assigned_instances[column] = assigned_instances;
    }

    /// Returns new with `self.assigned_instances` resized to specified number of instance columns.
    pub fn use_instances(
        mut self,
        column: usize,
        assigned_instances: Vec<AssignedValue<F>>,
    ) -> Self {
        self.set_instances(column, assigned_instances);
        self
    }

    /// Sets new `k` = log2 of domain
    pub fn set_k(&mut self, k: usize) {
        self.base.set_k(k);
    }

    /// Returns new with `k` set
    pub fn use_k(mut self, k: usize) -> Self {
        self.set_k(k);
        self
    }

    /// Set config params
    pub fn set_params(&mut self, params: BaseCircuitParams) {
        self.base.set_params(params)
    }

    /// Returns new with config params
    pub fn use_params(mut self, params: BaseCircuitParams) -> Self {
        self.set_params(params);
        self
    }

    /// Sets the break points of the circuit.
    pub fn set_break_points(&mut self, break_points: MultiPhaseThreadBreakPoints) {
        self.base.set_break_points(break_points);
    }

    /// Returns new with break points
    pub fn use_break_points(mut self, break_points: MultiPhaseThreadBreakPoints) -> Self {
        self.set_break_points(break_points);
        self
    }

    /// Returns [SinglePhaseCoreManager] with the virtual region with all core threads in the given phase.
    pub fn pool(&mut self, phase: usize) -> &mut SinglePhaseCoreManager<F> {
        self.base.pool(phase)
    }

    pub fn calculate_params(&mut self, minimum_rows: Option<usize>) -> BaseCircuitParams {
        let params = self.base.calculate_params(minimum_rows);
        set_var(
            "GATE_CONFIG_PARAMS",
            serde_json::to_string(&params).unwrap(),
        );
        params
    }

    pub fn sha_contexts_pair(&mut self) -> (&mut Context<F>, GateManager::CustomContext<'_>) {
        (self.base.main(0), self.sha.custom_context())
    }

    pub fn range_chip(&mut self, lookup_bits: usize) -> RangeChip<F> {
        self.base.set_lookup_bits(lookup_bits);
        self.base.range_chip()
    }
}

impl<F: BigPrimeField, GateManager: CommonGateManager<F>> CommonCircuitBuilder<F>
    for ShaCircuitBuilder<F, GateManager>
{
    fn main(&mut self) -> &mut Context<F> {
        self.base.main(0)
    }

    fn thread_count(&self) -> usize {
        unimplemented!()
    }

    fn new_context(&self, _context_id: usize) -> Context<F> {
        unimplemented!()
    }

    fn new_thread(&mut self) -> &mut Context<F> {
        unimplemented!()
    }
}
