use crate::actions::{Action, ActionType};
use crate::level_table;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Crafter {
    pub(crate) cls: u32,
    pub(crate) craftsmanship: u32,
    pub(crate) control: u32,
    pub(crate) craft_points: u32,
    pub(crate) level: u32,
    pub(crate) specialist: u32,
    pub actions: Vec<Action>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub(crate) base_level: u32,
    pub(crate) level: u32,
    pub(crate) difficulty: u32,
    pub(crate) durability: u32,
    pub(crate) start_quality: u32,
    pub(crate) max_quality: u32,
    pub(crate) suggested_craftsmanship: u32,
    pub(crate) suggested_control: u32,
    pub(crate) progress_divider: f64,
    pub(crate) progress_modifier: Option<u32>,
    pub(crate) quality_divider: f64,
    pub(crate) quality_modifier: Option<u32>,
    pub(crate) stars: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SolverVars {
    solve_for_completion: bool,
    #[serde(rename = "remainderCPFitnessValue")]
    remainder_cp_fitness_value: bool,
    remainder_dur_fitness_value: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Synth {
    pub(crate) crafter: Crafter,
    pub(crate) recipe: Recipe,
    pub(crate) max_trick_uses: i32,
    pub(crate) reliability_index: u32,
    pub(crate) max_length: u32,
    pub(crate) solver_vars: SolverVars,
}

impl Synth {
    fn calculate_base_progress_increase(&self, eff_crafter_level: u32, craftsmanship: u32) -> u32 {
        let base_value: f64 = (craftsmanship as f64 * 10.0) / self.recipe.progress_divider + 2.0;
        if eff_crafter_level <= self.recipe.level {
            (base_value * (self.recipe.progress_modifier.unwrap_or(100) as f64) / 100.0) as u32
        } else {
            base_value as u32
        }
    }

    fn calculate_base_quality_increase(&self, eff_crafter_level: u32, control: u32) -> u32 {
        let base_value: f64 = (control as f64 * 10.0) / self.recipe.quality_divider + 35.0;
        if eff_crafter_level <= self.recipe.base_level {
            (base_value * (self.recipe.quality_modifier.unwrap_or(100) as f64) / 100.0).floor() as u32
        } else {
            base_value as u32
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Condition {
    Good,
    Normal,
    Excellent,
    Poor,
}

impl Default for Condition {
    fn default() -> Self {
        Condition::Good
    }
}

impl Condition {
    fn check_good_or_excellent(&self) -> bool {
        match self {
            Condition::Good => true,
            Condition::Excellent => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Effects {
    count_downs: BTreeMap<Action, i32>,
    count_ups: BTreeMap<Action, i32>,
    // still used?
    indefinites: BTreeMap<Action, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct State {
    pub synth: Synth,
    pub step: u32,
    pub last_step: u32,
    pub action: Option<Action>, // Action leading to this state
    pub durability_state: i32,
    pub cp_state: i32,
    pub bonus_max_cp: i32,
    pub quality_state: i32,
    pub progress_state: i32,
    pub wasted_actions: f64,
    pub trick_uses: i32,
    pub name_of_element_uses: i32,
    pub reliability: i32,
    pub effects: Effects,
    pub condition: Condition,

    // Advancedtouch combo stuff
    pub touch_combo_step: i32,

    // Internal state variables set after each step.
    pub iq_cnt: i32,
    pub control: i32,
    pub quality_gain: i32,
    pub progress_gain: bool,
    pub b_quality_gain: bool, // Rustversion: for some reason these are almost the same name?
    pub success: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Violations {
    progress_ok: bool,
    cp_ok: bool,
    durability_ok: bool,
    trick_ok: bool,
    reliability_ok: bool,
}

impl Violations {
    pub fn is_okay(&self) -> bool {
        self.durability_ok && self.cp_ok && self.progress_ok && self.reliability_ok && self.trick_ok
    }
}

impl State {
    pub fn check_violations(&self) -> Violations {

        let progress_ok = self.progress_state >= self.synth.recipe.difficulty as i32;
        let cp_ok = self.cp_state >= 0;
        let durability_ok = if self.durability_state >= -5
            && self.progress_state >= self.synth.recipe.difficulty as i32
        {
            if let Some(action) = self.action {
                if action.details().durability_cost == 10 || self.durability_state >= 0 {
                    true
                } else {
                    false
                }
            } else {
                self.durability_state >= 0
            }
        } else {
            false
        };

        let trick_ok = self.trick_uses <= self.synth.max_trick_uses;
        let reliability_ok = self.reliability > self.synth.reliability_index as i32;
        Violations {
            progress_ok,
            cp_ok,
            durability_ok,
            trick_ok,
            reliability_ok,
        }
    }
}

impl From<&Synth> for State {
    fn from(synth: &Synth) -> Self {
        State {
            synth: synth.clone(),
            step: 0,
            last_step: 0,
            action: None,
            effects: Effects {
                count_ups: [(Action::InnerQuiet, 1)].into_iter().collect(),
                ..Default::default()
            },
            reliability: 1,
            cp_state: synth.crafter.craft_points as i32,
            condition: Condition::Normal,
            durability_state: synth.recipe.durability as i32,
            ..Default::default()
        }
    }
}

fn prob_good_for_synth(synth: &Synth) -> f64 {
    let recipe_level = synth.recipe.level;
    let quality_assurance = synth.crafter.level >= 63;
    if recipe_level >= 300 {
        // 70+
        match quality_assurance {
            true => 0.11,
            false => 0.10,
        }
    } else if recipe_level >= 276 {
        // 65+
        match quality_assurance {
            true => 0.17,
            false => 0.15,
        }
    } else if recipe_level >= 255 {
        // 61+
        match quality_assurance {
            true => 0.22,
            false => 0.20,
        }
    } else if recipe_level >= 150 {
        // 55+
        match quality_assurance {
            true => 0.17,
            false => 0.15,
        }
    } else {
        match quality_assurance {
            true => 0.27,
            false => 0.25,
        }
    }
}

fn prob_excellent_for_synth(synth: &Synth) -> f64 {
    let recipe_level = synth.recipe.level;
    if recipe_level >= 300 {
        // 70*+
        0.01
    } else if recipe_level >= 255 {
        // 65+
        0.02
    } else if recipe_level >= 150 {
        // 60+
        0.01
    } else {
        0.02
    }
}

fn get_effective_crafter_level(synth: &Synth) -> u32 {
    let eff_crafter_level = synth.crafter.level;
    level_table::level_table_lookup(eff_crafter_level)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModifierResult {
    craftsmanship: u32,
    control: u32,
    eff_crafter_level: u32,
    eff_recipe_level: u32,
    level_difference: u32,
    success_probability: f64,
    quality_increase_multiplier: f64,
    progress_gain: f64,
    quality_gain: u32,
    durability_cost: f64,
    cp_cost: i32,
}

/// I could just do the functions that the JS uses, but I did it this way and I'm too lazy to change it now.
enum SimulationCondition {
    MonteCarlo { ignore_condition_req: bool },
}

impl SimulationCondition {
    fn check_good_or_excellent(&self, state: &State) -> bool {
        match self {
            SimulationCondition::MonteCarlo {
                ignore_condition_req,
            } => {
                if *ignore_condition_req {
                    true
                } else {
                    state.condition.check_good_or_excellent()
                }
            }
        }
    }

    fn p_good_or_excellent(&self, state: &State) -> f64 {
        match self {
            SimulationCondition::MonteCarlo { .. } => 1.0,
        }
    }
}

impl State {
    fn apply_modifiers(&mut self, action: Action, condition: &SimulationCondition) -> ModifierResult {
        let craftsmanship = self.synth.crafter.craftsmanship;
        let mut control = self.synth.crafter.control;
        let mut cp_cost = action.details().cp_cost;

        // Effects modifying level difference
        let eff_crafter_level = get_effective_crafter_level(&self.synth);
        let eff_recipe_level = self.synth.recipe.level;
        let level_difference = eff_crafter_level - eff_recipe_level;
        let original_level_difference = eff_crafter_level - eff_recipe_level;
        let pure_level_difference = self.synth.crafter.level - self.synth.recipe.base_level;
        let recipe_level = eff_recipe_level;
        let stars = self.synth.recipe.stars;

        // Effects modifying probability
        let action_details = action.details();
        let mut success_probability = action_details.success_probability;
        if action.eq(&Action::FocusedSynthesis) || action.eq(&Action::FocusedTouch) {
            if let Some(sa) = &self.action {
                if sa.eq(&Action::Observe) {
                    success_probability = 1.0;
                }
            }
        }

        success_probability = success_probability.min(1.0);

        // Advanced Touch Combo
        if action.eq(&Action::AdvancedTouch) {
            if let Some(sa) = &self.action {
                if *sa == Action::StandardTouch && self.touch_combo_step == 1 {
                    self.touch_combo_step = 0;
                    cp_cost = 18;
                }
            }
        }
        // Add combo bonus following Basic Touch
        if action.eq(&Action::StandardTouch) {
            if let Some(sa) = &self.action {
                if *sa == Action::BasicTouch {
                    cp_cost = 18;
                    self.wasted_actions -= 0.05;
                    self.touch_combo_step = 1;
                }
                if *sa == Action::StandardTouch {
                    self.wasted_actions += 0.1;
                }
            }
        }

        // Penalize use of WasteNot during solveforcompletion runs

        if action == Action::WasteNot
            || action == Action::WasteNot2 && self.synth.solver_vars.solve_for_completion
        {
            self.wasted_actions += 50.0;
        }

        // Effects modifying progress increase multiplier
        let mut progress_increase_multiplier = 1.0;

        if (action_details.progress_increase_multiplier > 0.0)
            && (self.effects.count_downs.contains_key(&Action::MuscleMemory))
        {
            progress_increase_multiplier += 1.0;
            //delete state.effects.count_downs[AllActions.muscleMemory.shortName];
        }

        if self.effects.count_downs.contains_key(&Action::Veneration) {
            progress_increase_multiplier += 0.5;
        }

        if action.eq(&Action::MuscleMemory) {
            if self.step != 1 {
                self.wasted_actions += 1.0;
                progress_increase_multiplier = 0.0;
                cp_cost = 0;
            }
        }
        // TODO do we need to be applying the durability cost from waste not to this?
        if self.durability_state < action_details.durability_cost {
            if action == Action::Groundwork || action == Action::Groundwork2 {
                progress_increase_multiplier *= 0.5;
            }
        }

        // Effects modifying quality increase multiplier
        let mut quality_increase_multiplier = 1.0;
        let mut quality_increase_multiplier_iq = 1.0; // This is calculated seperately because it's multiplicative instead of additive! See: how teamcrafting does it

        if self.effects.count_downs.contains_key(&Action::GreatStrides)
            && quality_increase_multiplier > 0.0
        {
            quality_increase_multiplier += 1.0;
        }

        if self.effects.count_downs.contains_key(&Action::Innovation) {
            quality_increase_multiplier += 0.5;
        }

        if let Some(inner_quiet_value) = self.effects.count_ups.get(&Action::InnerQuiet) {
            quality_increase_multiplier_iq += 0.1 * (inner_quiet_value + 1) as f64
            // +1 because buffs start incrementing from 0
        }

        // We can only use Byregot actions when we have at least 1 stacks of inner quiet
        if action == Action::ByregotsBlessing {
            let num_inner_quiets = *self
                .effects
                .count_ups
                .get(&Action::InnerQuiet)
                .unwrap_or(&0);
            if self
                .effects
                .count_ups
                .get(&Action::InnerQuiet)
                .map(|c| c >= &1)
                .unwrap_or_default()
            {
                quality_increase_multiplier *= 1.0 + (0.2 * (num_inner_quiets + 1) as f64).min(3.0);
            } else {
                quality_increase_multiplier = 0.0;
            }
        }

        // Calculate base and modified progress gain
        let progress_gain = self
            .synth
            .calculate_base_progress_increase(eff_crafter_level, craftsmanship);

        let mut progress_gain = progress_gain as f64
            * action_details.progress_increase_multiplier
            * progress_increase_multiplier;

        // Calculate base and modified quality gain
        let mut quality_gain = self
            .synth
            .calculate_base_quality_increase(eff_crafter_level, control);
        // conversion back to u32 from f64 is equivalent to .floor().
        quality_gain = (quality_gain as f64
            * action_details.quality_increase_multiplier
            * quality_increase_multiplier
            * quality_increase_multiplier_iq) as u32;

        // Trained finesse
        if action.eq(&Action::TrainedFinesse) {
            // Not at 10 stacks of IQ -> wasted action
            if *self
                .effects
                .count_ups
                .get(&Action::InnerQuiet)
                .unwrap_or(&0)
                != 9
            {
                self.wasted_actions += 1.0;
                quality_gain = 0;
            }
        }

        // Effects modifying durability cost
        let mut durability_cost = action_details.durability_cost as f64;
        if self.effects.count_downs.contains_key(&Action::WasteNot)
            || self.effects.count_downs.contains_key(&Action::WasteNot2)
        {
            if action.eq(&Action::PrudentTouch) {
                quality_gain = 0;
                self.wasted_actions += 1.0;
            } else if action.eq(&Action::PrudentSynthesis) {
                progress_gain = 0.0;
                self.wasted_actions += 1.0;
            } else {
                durability_cost *= 0.5;
            }
        }

        // Effects modifying quality gain directly
        if action.eq(&Action::TrainedEye) {
            if self.step == 1 && pure_level_difference >= 10 && !self.synth.recipe.stars {
                quality_gain = self.synth.recipe.max_quality;
            } else {
                self.wasted_actions += 1.0;
                quality_gain = 0;
                cp_cost = 0;
            }
        }

        // We can only use Precise Touch when state material condition is Good or Excellent. Default is true for probabilistic method.
        if action.eq(&Action::PreciseTouch) {
            if condition.check_good_or_excellent(self) {
                quality_gain *= condition.p_good_or_excellent(self) as u32;
            } else {
                self.wasted_actions += 1.0;
                quality_gain = 0;
                cp_cost = 0;
            }
        }

        if action.eq(&Action::Reflect) {
            if self.step != 1 {
                self.wasted_actions += 1.0;
                control = 0;
                quality_gain = 0;
                cp_cost = 0;
            }
        }

        ModifierResult {
            craftsmanship,
            control,
            eff_crafter_level,
            eff_recipe_level,
            level_difference,
            success_probability,
            quality_increase_multiplier,
            progress_gain,
            quality_gain,
            durability_cost,
            cp_cost,
        }
    }

    fn use_conditional_action(&mut self, condition: &SimulationCondition) -> bool {
        if self.cp_state > 0 && condition.check_good_or_excellent(self) {
            self.trick_uses += 1;
            return true;
        } else {
            self.wasted_actions += 1.0;
            return false;
        }
    }

    fn apply_special_action_effects(&mut self, action: Action, condition: &SimulationCondition) {
        // STEP_02
        // Effect management
        //==================================
        // Special Effect Actions
        if action == Action::MastersMend {
            self.durability_state += 30;
            if self.synth.solver_vars.solve_for_completion {
                self.wasted_actions += 50.0; // Bad code, but it works. We don't want dur increase in solveforcompletion.
            }
        }

        if self.effects.count_downs.contains_key(&Action::Manipulation)
            && self.durability_state > 0
            && action != Action::Manipulation
        {
            self.durability_state += 5;
            if self.synth.solver_vars.solve_for_completion {
                self.wasted_actions += 50.0; // Bad code, but it works. We don't want dur increase in solveforcompletion.
            }
        }

        if action == Action::ByregotsBlessing {
            if self.effects.count_ups.contains_key(&Action::InnerQuiet) {
                self.effects.count_ups.remove(&Action::InnerQuiet);
            } else {
                self.wasted_actions += 1.0;
            }
        }

        if action == Action::Reflect {
            if self.step == 1 {
                if let Some(mut count) = self.effects.count_ups.get_mut(&Action::InnerQuiet) {
                    *count += 1;
                } else {
                    self.effects.count_ups.insert(Action::InnerQuiet, 0); // what does this even get inserted as?
                }
            } else {
                self.wasted_actions += 1.0;
            }
        }
        let action_details = action.details();
        if action_details.quality_increase_multiplier > 0.0
            && self.effects.count_downs.contains_key(&Action::GreatStrides)
        {
            self.effects.count_downs.remove(&Action::GreatStrides);
        }

        // Manage effects with conditional requirements
        if action_details.on_excellent || action_details.on_good {
            if self.use_conditional_action(condition) {
                if action == Action::TricksOfTheTrade {
                    self.cp_state += (20.0 * condition.p_good_or_excellent(self)) as i32;
                }
            }
        }

        if action == Action::Veneration
            && self.effects.count_downs.contains_key(&Action::Veneration)
        {
            self.wasted_actions += 1.0
        }
        if action == Action::Innovation
            && self.effects.count_downs.contains_key(&Action::Innovation)
        {
            self.wasted_actions += 1.0
        }
    }

    fn update_effects_counters(
        &mut self,
        action: Action,
        condition: &SimulationCondition,
        success_probability: f64,
    ) {
        // STEP_03
        // Countdown / Countup Management
        //===============================
        // Decrement countdowns
        let mut remove_values = vec![];
        let action_details = action.details();
        for (action, count) in &mut self.effects.count_downs {
            *count -= 1;
            if *count <= 0 {
                remove_values.push(*action);
            }
        }
        for value in remove_values {
            self.effects.count_downs.remove_entry(&value);
        }

        if self.effects.count_ups.contains_key(&Action::InnerQuiet) {
            // Increment inner quiet countups that have conditional requirements
            if action == Action::PreparatoryTouch {
                if let Some(quiet) = self.effects.count_ups.get_mut(&Action::InnerQuiet) {
                    *quiet += 2;
                }
            }
            // Increment inner quiet countups that have conditional requirements
            else if action == Action::PreciseTouch && condition.check_good_or_excellent(self) {
                let quiet_increment =
                    (2.0 * success_probability * condition.p_good_or_excellent(self)) as i32;
                if let Some(quiet) = self.effects.count_ups.get_mut(&Action::InnerQuiet) {
                    *quiet += quiet_increment;
                }
            }
            // Increment all other inner quiet count ups
            else if action.details().quality_increase_multiplier > 0.0
                && action != Action::Reflect
                && action != Action::TrainedFinesse
            {
                if let Some(quiet) = self.effects.count_ups.get_mut(&Action::InnerQuiet) {
                    *quiet += (1.0 * success_probability) as i32;
                }
            }

            // Cap inner quiet stacks at 9 (10)
            if let Some(quiet) = self.effects.count_ups.get_mut(&Action::InnerQuiet) {
                *quiet = (*quiet).min(9);
            }
        }

        // Initialize new effects after countdowns are managed to reset them properly
        if action_details.action_type == ActionType::CountUp {
            self.effects.count_ups.insert(action, 0);
        }

        if action_details.action_type == ActionType::Immediate {
            /* TODO is this action even a thing?
            if isActionEq(action, AllActions.initialPreparations) {
                if s.step == 1 {
                    s.effects.indefinites[action.shortName] = true;
                }
                else {
                    s.wastedActions += 1;
                }
            }
            else {
                s.effects.indefinites[action.shortName] = true;
            }*/
        }

        if let ActionType::Countdown { active_turns } = action_details.action_type {
            /* TODO AGAIN, what??
            if (action.shortName.indexOf('nameOf') >= 0) {
                if (s.nameOfElementUses == 0) {
                    s.effects.countDowns[action.shortName] = action.activeTurns;
                    s.nameOfElementUses += 1;
                }
                else {
                    s.wastedActions += 1;
                }
            }*/
            if action == Action::MuscleMemory && self.step != 1 {
                self.wasted_actions += 1.0;
            } else {
                self.effects.count_downs.insert(action, active_turns);
                //s.effects.countDowns[action.shortName] = action.activeTurns;
            }
        }
    }
    fn update_state(
        &mut self,
        action: Action,
        progress_gain: i32,
        quality_gain: i32,
        durability_cost: i32,
        cp_cost: i32,
        condition: &SimulationCondition,
        success_probability: f64,
    ) {
        // State tracking
        self.progress_state += progress_gain;
        self.quality_state += quality_gain;
        self.durability_state -= durability_cost;
        self.cp_state -= cp_cost;
        self.last_step += 1;
        self.apply_special_action_effects(action, condition);
        self.update_effects_counters(action, condition, success_probability);

        // Sanity checks for state variables
        /* Removing this for solveForCompletion, hopefully it doesn't cause issues! :)
        if ((s.durabilityState >= -5) && (s.progressState >= s.synth.recipe.difficulty)) {
            //s.durabilityState = 0;
        }
        */
        self.durability_state = self
            .durability_state
            .min(self.synth.recipe.durability as i32);
        self.cp_state = self
            .cp_state
            .min(self.synth.crafter.craft_points as i32 + self.bonus_max_cp);
    }
    pub fn add_action(&mut self, action: Action) -> State {
        let mut state = self.clone();
        // TODO figure out how to handle simulation condition *better*
        let result = self.apply_modifiers(action, &SimulationCondition::MonteCarlo {
            ignore_condition_req: false
        });
        // add progress, TODO the js version had two different versions of this. I will ignore this for now. :)
        state.cp_state -= result.cp_cost;
        state.progress_state += result.progress_gain as i32;
        if result.progress_gain > 0.0 {
            state.reliability += 1;
        }
        state.durability_state -= result.durability_cost as i32;
        state.quality_state += result.quality_gain as i32;
        state
    }
}

/*fn sim_synth(individual: String, start_state: State, assume_success: bool, verbose: bool, debug: bool, log_output: Option<String>) -> State {

    let logger = Logger(logOutput);

    // Clone startState to keep startState immutable
    let state = start_state.clone();

    // Conditions
    let pGood = prob_good_for_synth(&state.synth);
    let pExcellent = prob_excellent_for_synth(&state.synth);
    let ignoreConditionReq = !state.synth.useConditions;

    // Step 1 is always normal
    let mut ppGood = 0;
    let mut ppExcellent = 0;
    let mut ppPoor = 0;
    let mut ppNormal = 1 - (ppGood + ppExcellent + ppPoor);



    // Check for null or empty individuals
    if individual.empty() {
        return State::from(&state.synth)
    }

    if debug {
      //logger.log('%-2s %30s %-5s %-5s %-8s %-8s %-5s %-8s %-8s %-5s %-5s %-5s', '#', 'Action', 'DUR', 'CP', 'EQUA', 'EPRG', 'IQ', 'CTL', 'QINC', 'BPRG', 'BQUA', 'WAC');
      //logger.log('%2d %30s %5.0f %5.0f %8.1f %8.1f %5.1f %8.1f %8.1f %5.0f %5.0f %5.0f', s.step, '', s.durabilityState, s.cpState, s.qualityState, s.progressState, 0, s.synth.crafter.control, 0, 0, 0, 0);
    }
    else if verbose {
      // logger.log('%-2s %30s %-5s %-5s %-8s %-8s %-5s', '#', 'Action', 'DUR', 'CP', 'EQUA', 'EPRG', 'IQ');
      // logger.log('%2d %30s %5.0f %5.0f %8.1f %8.1f %5.1f', s.step, '', s.durabilityState, s.cpState, s.qualityState, s.progressState, 0);

    }
    for (let i = 0; i < individual.length; i++) {
    // var action = individual[i];

    // Ranged edit -- Combo actions. Basically do everything twice over if there's a combo action. Woo.
    let actionsArray = vec![];

    if (individual[i].isCombo){
    actionsArray[0] = getComboAction(individual[i].comboName1);
    actionsArray[1] = getComboAction(individual[i].comboName2);
    } else {
    actionsArray[0] = individual[i];
    }
    for (var j = 0; j < actionsArray.length; j++) {
    var action = actionsArray[j];


    // Occur regardless of dummy actions
    //==================================
    state.step += 1;

    // Condition Calculation
    var condQualityIncreaseMultiplier = 1;
    if (!ignoreConditionReq) {
    condQualityIncreaseMultiplier *= (ppNormal + 1.5 * ppGood * Math.pow(1 - (ppGood + pGood) / 2, state.synth.maxTrickUses) + 4 * ppExcellent + 0.5 * ppPoor);
    }

    // Calculate Progress, Quality and Durability gains and losses under effect of modifiers
    var r = ApplyModifiers(state, action, SimCondition);

    // Calculate final gains / losses
    var successProbability = r.successProbability;
    if (assume_success) {
    successProbability = 1;
    }
    var progressGain = r.bProgressGain;
    if (progressGain > 0) {
    state.reliability = state.reliability * successProbability;
    }

    var qualityGain = condQualityIncreaseMultiplier * r.bQualityGain;

    // Floor gains at final stage before calculating expected value
    progressGain = successProbability * Math.floor(progressGain);
    qualityGain = successProbability * Math.floor(qualityGain);

    // Occur if a wasted action
    //==================================
    if (((state.progressState >= state.synth.recipe.difficulty) || (state.durabilityState <= 0) || (state.cpState < 0)) && (action != AllActions.dummyAction)) {
    state.wastedActions += 1;
    }

    // Occur if not a wasted action
    //==================================
    else {

    UpdateState(state, action, progressGain, qualityGain, r.durabilityCost, r.cpCost, SimCondition, successProbability);

    // Ending condition update
    if (!ignoreConditionReq) {
    ppPoor = ppExcellent;
    ppGood = pGood * ppNormal;
    ppExcellent = pExcellent * ppNormal;
    ppNormal = 1 - (ppGood + ppExcellent + ppPoor);
    }

    }

    var iqCnt = 0;
    if (AllActions.innerQuiet.shortName in state.effects.countUps) {
    iqCnt = state.effects.countUps[AllActions.innerQuiet.shortName];
    }
    if (debug) {
    logger.log('%2d %30s %5.0f %5.0f %8.1f %8.1f %5.1f %8.1f %8.1f %5.0f %5.0f %5.0f', state.step, action.name, state.durabilityState, state.cpState, state.qualityState, state.progressState, iqCnt, r.control, qualityGain, Math.floor(r.bProgressGain), Math.floor(r.bQualityGain), state.wastedActions);
    }
    else if (verbose) {
    logger.log('%2d %30s %5.0f %5.0f %8.1f %8.1f %5.1f', state.step, action.name, state.durabilityState, state.cpState, state.qualityState, state.progressState, iqCnt);
    }

    state.action = action.shortName
    }

    }

    // Check for feasibility violations
    var chk = state.checkViolations();

    if (debug) {
    logger.log('Progress Check: %state, Durability Check: %state, CP Check: %state, Tricks Check: %state, Reliability Check: %state, Wasted Actions: %d', chk.progressOk, chk.durabilityOk, chk.cpOk, chk.trickOk, chk.reliabilityOk, state.wastedActions);
    }
    else if (verbose) {
    logger.log('Progress Check: %state, Durability Check: %state, CP Check: %state, Tricks Check: %state, Reliability Check: %state, Wasted Actions: %d', chk.progressOk, chk.durabilityOk, chk.cpOk, chk.trickOk, chk.reliabilityOk, state.wastedActions);
    }

    // Return final state
    state.action = individual[individual.length-1].shortName;
    return state;

}*/
