use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ActionType {
    Immediate,
    CountUp,
    Countdown {
        active_turns: i32, // number of turns this countdown is active for
    },
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Immediate
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Combo {
    action_1: Action,
    action_2: Action,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionDetails<'a> {
    pub short_name: &'a str,
    pub full_name: &'a str,
    pub durability_cost: i32,
    pub cp_cost: i32,
    pub success_probability: f32,
    pub quality_increase_multiplier: f32,
    pub progress_increase_multiplier: f32,
    pub action_type: ActionType, // action types
    pub class: &'a str,
    pub level: i32,
    pub on_good: bool,
    pub on_excellent: bool,
    pub combo: Option<Combo>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Observe,
    BasicSynth,
    BasicSynth2,
    CarefulSynthesis,
    RapidSynthesis,
    BasicTouch,
    StandardTouch,
    HastyTouch,
    ByregotsBlessing,
    MastersMend,
    TricksOfTheTrade,
    InnerQuiet,
    Manipulation,
    WasteNot,
    WasteNot2,
    Veneration,
    Innovation,
    GreatStrides,
    PreciseTouch,
    MuscleMemory,
    RapidSynthesis2,
    PrudentTouch,
    FocusedSynthesis,
    FocusedTouch,
    Reflect,
    PreparatoryTouch,
    Groundwork,
    DelicateSynthesis,
    IntensiveSynthesis,
    TrainedEye,
    CarefulSynthesis2,
    Groundwork2,
    AdvancedTouch,
    PrudentSynthesis,
    TrainedFinesse,
    FinalAppraisal,
    FocusedTouchCombo,
    FocusedSynthesisCombo,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details().full_name)
    }
}

impl Action {
    pub fn details(&self) -> &ActionDetails {
        static OBSERVE : ActionDetails = ActionDetails {
            short_name: "observe",
            full_name: "Observe",
            durability_cost: 0,
            cp_cost: 7,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 13,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static BASIC_SYNTHESIS : ActionDetails = ActionDetails {
            //             shortName	              fullName	        dur	cp	Prob	 QIM	 PIM	 Type	          t	  cls	           lvl
            //          'basicSynth'	           'Basic Synthesis'	10	0	1	0	1	 'immediate'	1	  'All'	           1)
            short_name: "basicSynth",
            full_name: "Basic Synthesis",
            durability_cost: 10,
            cp_cost: 0,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 1.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 1,

            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static BASIC_SYNTH_2 : ActionDetails = ActionDetails {
            short_name: "basicSynth2",
            full_name: "Basic Synthesis",
            durability_cost: 10,
            cp_cost: 0,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 1.2,
            action_type: ActionType::Immediate,
            class: "All",
            level: 31,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static CAREFUL_SYNTHESIS : ActionDetails = ActionDetails {
            short_name: "carefulSynthesis",
            full_name: "Careful Synthesis",
            durability_cost: 10,
            cp_cost: 7,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 1.2,
            action_type: ActionType::Immediate,
            class: "All",
            level: 62,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static RAPID_SYNTHESIS : ActionDetails = ActionDetails {
            short_name: "rapidSynthesis",
            full_name: "Rapid Synthesis",
            durability_cost: 10,
            cp_cost: 0,
            success_probability: 0.5,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 2.5,
            action_type: ActionType::Immediate,
            class: "All",
            level: 9,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static BASIC_TOUCH : ActionDetails = ActionDetails {
            short_name: "basicTouch",
            full_name: "Basic Touch",
            durability_cost: 10,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 1.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 18,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static STANDARD_TOUCH : ActionDetails = ActionDetails {
            short_name: "standardTouch",
            full_name: "Standard Touch",
            durability_cost: 10,
            cp_cost: 32,
            success_probability: 1.0,
            quality_increase_multiplier: 1.25,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 18,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static HASTY_TOUCH : ActionDetails = ActionDetails {
            short_name: "hastyTouch",
            full_name: "Basic Touch",
            durability_cost: 10,
            cp_cost: 0,
            success_probability: 0.6,
            quality_increase_multiplier: 1.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 9,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static BYREGOTS_BLESSING : ActionDetails = ActionDetails {
            short_name: "byregotsBlessing",
            full_name: "Byregot's Blessing",
            durability_cost: 10,
            cp_cost: 24,
            success_probability: 1.0,
            quality_increase_multiplier: 1.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 50,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static MASTERS_MEND : ActionDetails = ActionDetails {
            short_name: "mastersMend",
            full_name: "Master's Mend",
            durability_cost: 0,
            cp_cost: 88,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 7,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static TRICKS_OF_THE_TRADE : ActionDetails = ActionDetails {
            short_name: "tricksOfTheTrade",
            full_name: "Tricks of the Trade",
            durability_cost: 0,
            cp_cost: 0,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 13,
            on_good: true,
            on_excellent: true,
            combo: None,
        };
        static INNER_QUIET : ActionDetails = ActionDetails {
            short_name: "innerQuiet",
            full_name: "Inner Quiet",
            durability_cost: 0,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::CountUp,
            class: "All",
            level: 11,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static MANIPULATION : ActionDetails = ActionDetails {
            short_name: "manipulation",
            full_name: "Manipulation",
            durability_cost: 0,
            cp_cost: 96,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 8 },
            class: "All",
            level: 65,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static WASTENOT : ActionDetails = ActionDetails {
            short_name: "wasteNot",
            full_name: "Waste Not",
            durability_cost: 0,
            cp_cost: 56,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 4 },
            class: "All",
            level: 15,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static WASTENOT2 : ActionDetails = ActionDetails {
            short_name: "wasteNot2",
            full_name: "Waste Not II",
            durability_cost: 0,
            cp_cost: 98,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 8 },
            class: "All",
            level: 47,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static VENERATION : ActionDetails = ActionDetails {
            short_name: "veneration",
            full_name: "Veneration",
            durability_cost: 0,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 4 },
            class: "All",
            level: 15,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static INNOVATION : ActionDetails = ActionDetails {
            short_name: "innovation",
            full_name: "Innovation",
            durability_cost: 0,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 4 },
            class: "All",
            level: 26,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static GREAT_STRIDES: ActionDetails = ActionDetails {
            short_name: "greatStrides",
            full_name: "Great Strides",
            durability_cost: 0,
            cp_cost: 32,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 3 },
            class: "All",
            level: 31,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static PRECISE_TOUCH: ActionDetails = ActionDetails {
            short_name: "preciseTouch",
            full_name: "Precise Touch",
            durability_cost: 10,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 1.5,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 53,
            on_good: true,
            on_excellent: true,
            combo: None,
        };
        static MUSCLE_MEMORY: ActionDetails = ActionDetails {
            short_name: "muscleMemory",
            full_name: "Muscle Memory",
            durability_cost: 10,
            cp_cost: 6,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 3.0,
            action_type: ActionType::Countdown { active_turns: 5 },
            class: "All",
            level: 54,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static RAPID_SYNTHISIS_2: ActionDetails = ActionDetails {
            short_name: "rapidSynthesis2",
            full_name: "Rapid Synthesis",
            durability_cost: 10,
            cp_cost: 0,
            success_probability: 0.5,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 5.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 63,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static PRUDENTTOUCH : ActionDetails = ActionDetails {
            short_name: "prudentTouch",
            full_name: "Prudent Touch",
            durability_cost: 5,
            cp_cost: 25,
            success_probability: 1.0,
            quality_increase_multiplier: 1.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 66,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static FOCUSEDSYNTHESIS : ActionDetails = ActionDetails {
            short_name: "focusedSynthesis",
            full_name: "Focused Synthesis",
            durability_cost: 10,
            cp_cost: 5,
            success_probability: 0.5,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 2.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 67,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static FOCUSEDTOUCH : ActionDetails = ActionDetails {
            short_name: "focusedTouch",
            full_name: "Focused Touch",
            durability_cost: 10,
            cp_cost: 18,
            success_probability: 0.5,
            quality_increase_multiplier: 1.5,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 68,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static REFLECT : ActionDetails = ActionDetails {
            short_name: "reflect",
            full_name: "Reflect",
            durability_cost: 10,
            cp_cost: 6,
            success_probability: 1.0,
            quality_increase_multiplier: 1.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 69,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static PREPARATORYTOUCH : ActionDetails = ActionDetails {
            short_name: "preparatoryTouch",
            full_name: "Preparatory Touch",
            durability_cost: 20,
            cp_cost: 40,
            success_probability: 1.0,
            quality_increase_multiplier: 2.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 71,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static GROUNDWORK : ActionDetails = ActionDetails {
            short_name: "groundwork",
            full_name: "Groundwork",
            durability_cost: 20,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 3.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 72,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static DELICATESYNTHESIS : ActionDetails = ActionDetails {
            short_name: "delicateSynthesis",
            full_name: "Delicate Synthesis",
            durability_cost: 10,
            cp_cost: 32,
            success_probability: 1.0,
            quality_increase_multiplier: 1.0,
            progress_increase_multiplier: 1.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 76,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static INTENSIVESYNTHESIS : ActionDetails = ActionDetails {
            short_name: "intensiveSynthesis",
            full_name: "Intensive Synthesis",
            durability_cost: 10,
            cp_cost: 6,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 4.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 78,
            on_good: true,
            on_excellent: true,
            combo: None,
        };
        static TRAINEDEYE : ActionDetails = ActionDetails {
            short_name: "trainedEye",
            full_name: "Trained Eye",
            durability_cost: 10,
            cp_cost: 250,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 80,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static CAREFULSYNTHESIS2 : ActionDetails = ActionDetails {
            short_name: "carefulSynthesis2",
            full_name: "Careful Synthesis",
            durability_cost: 10,
            cp_cost: 7,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 1.8,
            action_type: ActionType::Immediate,
            class: "All",
            level: 82,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static GROUNDWORK2 : ActionDetails = ActionDetails {
            short_name: "groundwork2",
            full_name: "Groundwork",
            durability_cost: 20,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 3.6,
            action_type: ActionType::Immediate,
            class: "All",
            level: 86,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static ADVANCEDTOUCH : ActionDetails = ActionDetails {
            short_name: "advancedTouch",
            full_name: "Advanced Touch",
            durability_cost: 10,
            cp_cost: 46,
            success_probability: 1.0,
            quality_increase_multiplier: 1.5,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 84,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static PRUDENTSYNTHESIS : ActionDetails = ActionDetails {
            short_name: "prudentSynthesis",
            full_name: "Prudent Synthesis",
            durability_cost: 5,
            cp_cost: 18,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 1.8,
            action_type: ActionType::Immediate,
            class: "All",
            level: 88,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static TRAINEDFINESSE : ActionDetails = ActionDetails {
            short_name: "trainedFinesse",
            full_name: "Trained Finesse",
            durability_cost: 0,
            cp_cost: 32,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 90,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        static FOCUSEDTOUCHCOMBO : ActionDetails = ActionDetails {
            short_name: "focusedTouchCombo",
            full_name: "Focused Touch Combo",
            durability_cost: 10,
            cp_cost: 25,
            success_probability: 1.0,
            quality_increase_multiplier: 1.5,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 68,
            on_good: false,
            on_excellent: false,
            combo: Some(Combo {
                action_1: Action::Observe,
                action_2: Action::FocusedTouch,
            }),
        };
        static FOCUSEDSYNTHESISCOMBO : ActionDetails = ActionDetails {
            short_name: "focusedSynthesisCombo",
            full_name: "Focused Synthesis Combo",
            durability_cost: 10,
            cp_cost: 12,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 2.0,
            action_type: ActionType::Immediate,
            class: "All",
            level: 67,
            on_good: false,
            on_excellent: false,
            combo: Some(Combo {
                action_1: Action::Observe,
                action_2: Action::FocusedSynthesis,
            }),
        };
        static FINALAPPRAISAL : ActionDetails = ActionDetails {
            short_name: "finalAppraisal",
            full_name: "Final Appraisal",
            durability_cost: 0,
            cp_cost: 1,
            success_probability: 1.0,
            quality_increase_multiplier: 0.0,
            progress_increase_multiplier: 0.0,
            action_type: ActionType::Countdown { active_turns: 5 },
            class: "All",
            level: 42,
            on_good: false,
            on_excellent: false,
            combo: None,
        };
        match self {
            // observe: new Action(            'observe',              'Observe',               0,      7,  1.0, 0.0, 0.0, 'immediate',   1,  'All',          13),
            Action::Observe => &OBSERVE,
            // basicSynth: new Action(         'basicSynth',           'Basic Synthesis',      10,      0,  1.0, 0.0, 1.0, 'immediate',   1,  'All',           1),
            Action::BasicSynth => &BASIC_SYNTHESIS,
            // basicSynth2: new Action(        'basicSynth2',          'Basic Synthesis',      10,      0,  1.0, 0.0, 1.2, 'immediate',   1,  'All',          31),
            Action::BasicSynth2 => &BASIC_SYNTH_2,
            // carefulSynthesis: new Action(   'carefulSynthesis',     'Careful Synthesis',    10,      7,  1.0, 0.0, 1.5, 'immediate',   1,  'All',          62),
            Action::CarefulSynthesis => &CAREFUL_SYNTHESIS,
            // rapidSynthesis: new Action(     'rapidSynthesis',       'Rapid Synthesis',      10,      0,  0.5, 0.0, 2.5, 'immediate',   1,  'All',           9),
            Action::RapidSynthesis => &RAPID_SYNTHESIS,
            // basicTouch: new Action(         'basicTouch',           'Basic Touch',          10,     18,  1.0, 1.0, 0.0, 'immediate',   1,  'All',           5),
            Action::BasicTouch => &BASIC_TOUCH,
            // standardTouch: new Action(      'standardTouch',        'Standard Touch',       10,     32,  1.0, 1.25,0.0, 'immediate',   1,  'All',          18),
            Action::StandardTouch => &STANDARD_TOUCH,
            // hastyTouch: new Action(         'hastyTouch',           'Hasty Touch',          10,      0,  0.6, 1.0, 0.0, 'immediate',   1,  'All',           9),
            Action::HastyTouch => &HASTY_TOUCH,
            // byregotsBlessing: new Action(   'byregotsBlessing',     'Byregot\'s Blessing',  10,     24,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          50),
            Action::ByregotsBlessing => &BYREGOTS_BLESSING,
            // mastersMend: new Action(        'mastersMend',          'Master\'s Mend',        0,     88,  1.0, 0.0, 0.0, 'immediate',   1,  'All',           7),
            Action::MastersMend => &MASTERS_MEND,
            // tricksOfTheTrade: new Action(   'tricksOfTheTrade',     'Tricks of the Trade',   0,      0,  1.0, 0.0, 0.0, 'immediate',   1,  'All',          13,  true,       true),
            Action::TricksOfTheTrade => &TRICKS_OF_THE_TRADE,
            // innerQuiet: new Action(         'innerQuiet',           'Inner Quiet',           0,     18,  1.0, 0.0, 0.0, 'countup',     1,  'All',          11),
            Action::InnerQuiet => &INNER_QUIET,
            // manipulation: new Action(       'manipulation',         'Manipulation',          0,     96,  1.0, 0.0, 0.0, 'countdown',   8,  'All',          65),
            Action::Manipulation => &MANIPULATION,
            // wasteNot: new Action(           'wasteNot',             'Waste Not',             0,     56,  1.0, 0.0, 0.0, 'countdown',   4,  'All',          15),
            Action::WasteNot => &WASTENOT,
            // wasteNot2: new Action(          'wasteNot2',            'Waste Not II',          0,     98,  1.0, 0.0, 0.0, 'countdown',   8,  'All',          47)
            Action::WasteNot2 => &WASTENOT2,
            // veneration: new Action(         'veneration',           'Veneration',            0,     18,  1.0, 0.0, 0.0, 'countdown',   4,  'All',          15),
            Action::Veneration => &VENERATION,
            // innovation: new Action(         'innovation',           'Innovation',            0,     18,  1.0, 0.0, 0.0, 'countdown',   4,  'All',          26),
            Action::Innovation => &INNOVATION,
            // greatStrides: new Action(       'greatStrides',         'Great Strides',         0,     32,  1.0, 0.0, 0.0, 'countdown',   3,  'All',          21),
            Action::GreatStrides => &GREAT_STRIDES,
            // preciseTouch: new Action(       'preciseTouch',         'Precise Touch',        10,     18,  1.0, 1.5, 0.0, 'immediate',   1,  'All',          53,  true,       true),
            Action::PreciseTouch => &PRECISE_TOUCH,
            // muscleMemory: new Action(       'muscleMemory',         'Muscle Memory',        10,      6,  1.0, 0.0, 3.0, 'countdown',   5,  'All',          54),
            Action::MuscleMemory => &MUSCLE_MEMORY,
            // rapidSynthesis2: new Action(    'rapidSynthesis2',      'Rapid Synthesis',      10,      0,  0.5, 0.0, 5.0, 'immediate',   1,  'All',          63),
            Action::RapidSynthesis2 => &RAPID_SYNTHISIS_2,
            // prudentTouch: new Action(       'prudentTouch',         'Prudent Touch',         5,     25,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          66),
            Action::PrudentTouch => &PRUDENTTOUCH,
            // focusedSynthesis: new Action(   'focusedSynthesis',     'Focused Synthesis',    10,      5,  0.5, 0.0, 2.0, 'immediate',   1,  'All',          67),
            Action::FocusedSynthesis => &FOCUSEDSYNTHESIS,
            // focusedTouch: new Action(       'focusedTouch',         'Focused Touch',        10,     18,  0.5, 1.5, 0.0, 'immediate',   1,  'All',          68),
            Action::FocusedTouch => &FOCUSEDTOUCH,
            // reflect: new Action(            'reflect',              'Reflect',              10,     6,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          69),
            Action::Reflect => &REFLECT,
            // preparatoryTouch: new Action(   'preparatoryTouch',     'Preparatory Touch',    20,     40,  1.0, 2.0, 0.0, 'immediate',   1,  'All',          71),
            Action::PreparatoryTouch => &PREPARATORYTOUCH,
            // groundwork: new Action(         'groundwork',           'Groundwork',           20,     18,  1.0, 0.0, 3.0, 'immediate',   1,  'All',          72),
            Action::Groundwork => &GROUNDWORK,
            // delicateSynthesis: new Action(  'delicateSynthesis',    'Delicate Synthesis',   10,     32,  1.0, 1.0, 1.0, 'immediate',   1,  'All',          76),
            Action::DelicateSynthesis => &DELICATESYNTHESIS,
            // intensiveSynthesis: new Action( 'intensiveSynthesis',   'Intensive Synthesis',  10,      6,  1.0, 0.0, 4.0, 'immediate',   1,  'All',          78,  true,       true),
            Action::IntensiveSynthesis => &INTENSIVESYNTHESIS,
            // trainedEye: new Action(         'trainedEye',           'Trained Eye',          10,    250,  1.0, 0.0, 0.0, 'immediate',   1,  'All',          80),
            Action::TrainedEye => &TRAINEDEYE,
            // Endwalker
            // carefulSynthesis2: new Action(   'carefulSynthesis2',     'Careful Synthesis',  10,      7,  1.0, 0.0, 1.8, 'immediate',   1,  'All',          82),
            Action::CarefulSynthesis2 => &CAREFULSYNTHESIS2,
            // groundwork2: new Action(         'groundwork2',           'Groundwork',         20,     18,  1.0, 0.0, 3.6, 'immediate',   1,  'All',          86),
            Action::Groundwork2 => &GROUNDWORK2,
            // advancedTouch: new Action(       'advancedTouch',        'Advanced Touch',      10,     46,  1.0, 1.5, 0.0, 'immediate',   1,  'All',          84),
            Action::AdvancedTouch => &ADVANCEDTOUCH,
            // prudentSynthesis: new Action(    'prudentSynthesis',     'Prudent Synthesis',   5,      18,  1.0, 0.0, 1.8, 'immediate',   1,  'All',          88),
            Action::PrudentSynthesis => &PRUDENTSYNTHESIS,
            // trainedFinesse: new Action(       'trainedFinesse',       'Trained Finesse',    0,      32,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          90),
            Action::TrainedFinesse => &TRAINEDFINESSE,
            // COMBO!
            // focusedTouchCombo: new Action(  'focusedTouchCombo',    'Focused Touch Combo',  10,     25, 1.0,  1.5, 0.0, 'immediate',   1,  'All',         68,   false,      false,       false,     true,       'observe',      'focusedTouch'),
            Action::FocusedTouchCombo => &FOCUSEDTOUCHCOMBO,
            // focusedSynthesisCombo: new Action(  'focusedSynthesisCombo',    'Focused Synthesis Combo',  10, 12, 1.0,  0.0, 2.0, 'immediate',   1,  'All',         67,   false,      false,       false,     true,       'observe',      'focusedSynthesis'),
            Action::FocusedSynthesisCombo => &FOCUSEDSYNTHESISCOMBO,
            Action::FinalAppraisal => &FINALAPPRAISAL,
        }
    }
}

/* TABLE COPIED FROM actions.js
var AllActions = {
//                              shortName,              fullName,              dur,     cp, Prob, QIM, PIM, Type,          t,  cls,           lvl,  onGood,     onExcl,      onPoor,    isCombo,    comboName1,     comboName2
observe: new Action(            'observe',              'Observe',               0,      7,  1.0, 0.0, 0.0, 'immediate',   1,  'All',          13),

basicSynth: new Action(         'basicSynth',           'Basic Synthesis',      10,      0,  1.0, 0.0, 1.0, 'immediate',   1,  'All',           1),
basicSynth2: new Action(        'basicSynth2',          'Basic Synthesis',      10,      0,  1.0, 0.0, 1.2, 'immediate',   1,  'All',          31),
carefulSynthesis: new Action(   'carefulSynthesis',     'Careful Synthesis',    10,      7,  1.0, 0.0, 1.5, 'immediate',   1,  'All',          62),
rapidSynthesis: new Action(     'rapidSynthesis',       'Rapid Synthesis',      10,      0,  0.5, 0.0, 2.5, 'immediate',   1,  'All',           9),

basicTouch: new Action(         'basicTouch',           'Basic Touch',          10,     18,  1.0, 1.0, 0.0, 'immediate',   1,  'All',           5),
standardTouch: new Action(      'standardTouch',        'Standard Touch',       10,     32,  1.0, 1.25,0.0, 'immediate',   1,  'All',          18),
hastyTouch: new Action(         'hastyTouch',           'Hasty Touch',          10,      0,  0.6, 1.0, 0.0, 'immediate',   1,  'All',           9),
byregotsBlessing: new Action(   'byregotsBlessing',     'Byregot\'s Blessing',  10,     24,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          50),

mastersMend: new Action(        'mastersMend',          'Master\'s Mend',        0,     88,  1.0, 0.0, 0.0, 'immediate',   1,  'All',           7),
tricksOfTheTrade: new Action(   'tricksOfTheTrade',     'Tricks of the Trade',   0,      0,  1.0, 0.0, 0.0, 'immediate',   1,  'All',          13,  true,       true),

innerQuiet: new Action(         'innerQuiet',           'Inner Quiet',           0,     18,  1.0, 0.0, 0.0, 'countup',     1,  'All',          11),
manipulation: new Action(       'manipulation',         'Manipulation',          0,     96,  1.0, 0.0, 0.0, 'countdown',   8,  'All',          65),
wasteNot: new Action(           'wasteNot',             'Waste Not',             0,     56,  1.0, 0.0, 0.0, 'countdown',   4,  'All',          15),
wasteNot2: new Action(          'wasteNot2',            'Waste Not II',          0,     98,  1.0, 0.0, 0.0, 'countdown',   8,  'All',          47),
veneration: new Action(         'veneration',           'Veneration',            0,     18,  1.0, 0.0, 0.0, 'countdown',   4,  'All',          15),
innovation: new Action(         'innovation',           'Innovation',            0,     18,  1.0, 0.0, 0.0, 'countdown',   4,  'All',          26),
greatStrides: new Action(       'greatStrides',         'Great Strides',         0,     32,  1.0, 0.0, 0.0, 'countdown',   3,  'All',          21),

// Heavensward actions
preciseTouch: new Action(       'preciseTouch',         'Precise Touch',        10,     18,  1.0, 1.5, 0.0, 'immediate',   1,  'All',          53,  true,       true),
muscleMemory: new Action(       'muscleMemory',         'Muscle Memory',        10,      6,  1.0, 0.0, 3.0, 'countdown',   5,  'All',          54),

// Stormblood actions
rapidSynthesis2: new Action(    'rapidSynthesis2',      'Rapid Synthesis',      10,      0,  0.5, 0.0, 5.0, 'immediate',   1,  'All',          63),
prudentTouch: new Action(       'prudentTouch',         'Prudent Touch',         5,     25,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          66),
focusedSynthesis: new Action(   'focusedSynthesis',     'Focused Synthesis',    10,      5,  0.5, 0.0, 2.0, 'immediate',   1,  'All',          67),
focusedTouch: new Action(       'focusedTouch',         'Focused Touch',        10,     18,  0.5, 1.5, 0.0, 'immediate',   1,  'All',          68),
reflect: new Action(            'reflect',              'Reflect',              10,     6,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          69),

// ShadowBringers actions
preparatoryTouch: new Action(   'preparatoryTouch',     'Preparatory Touch',    20,     40,  1.0, 2.0, 0.0, 'immediate',   1,  'All',          71),
groundwork: new Action(         'groundwork',           'Groundwork',           20,     18,  1.0, 0.0, 3.0, 'immediate',   1,  'All',          72),
delicateSynthesis: new Action(  'delicateSynthesis',    'Delicate Synthesis',   10,     32,  1.0, 1.0, 1.0, 'immediate',   1,  'All',          76),
intensiveSynthesis: new Action( 'intensiveSynthesis',   'Intensive Synthesis',  10,      6,  1.0, 0.0, 4.0, 'immediate',   1,  'All',          78,  true,       true),
trainedEye: new Action(         'trainedEye',           'Trained Eye',          10,    250,  1.0, 0.0, 0.0, 'immediate',   1,  'All',          80),

// Endwalker
carefulSynthesis2: new Action(   'carefulSynthesis2',     'Careful Synthesis',  10,      7,  1.0, 0.0, 1.8, 'immediate',   1,  'All',          82),
groundwork2: new Action(         'groundwork2',           'Groundwork',         20,     18,  1.0, 0.0, 3.6, 'immediate',   1,  'All',          86),
advancedTouch: new Action(       'advancedTouch',        'Advanced Touch',      10,     46,  1.0, 1.5, 0.0, 'immediate',   1,  'All',          84),
prudentSynthesis: new Action(    'prudentSynthesis',     'Prudent Synthesis',   5,      18,  1.0, 0.0, 1.8, 'immediate',   1,  'All',          88),
trainedFinesse: new Action(       'trainedFinesse',       'Trained Finesse',    0,      32,  1.0, 1.0, 0.0, 'immediate',   1,  'All',          90),

// Ranged edit: special combo'd actions that are handled differently
// Combo Actions. Making new combo actions need an image, extraActionInfo, and some code in getComboAction() in ffxivcraftmodel.js
// The existence of this breaks the montecarlo simulation but idgaf about that
//                              shortName,              fullName,              dur,     cp, Prob, QIM, PIM, Type,          t,  cls,           lvl,  onGood,     onExcl,      onPoor,    isCombo,    comboName1,     comboName2
focusedTouchCombo: new Action(  'focusedTouchCombo',    'Focused Touch Combo',  10,     25, 1.0,  1.5, 0.0, 'immediate',   1,  'All',         68,   false,      false,       false,     true,       'observe',      'focusedTouch'),
focusedSynthesisCombo: new Action(  'focusedSynthesisCombo',    'Focused Synthesis Combo',  10, 12, 1.0,  0.0, 2.0, 'immediate',   1,  'All',         67,   false,      false,       false,     true,       'observe',      'focusedSynthesis'),


// Special Actions - not selectable
dummyAction: new Action(        'dummyAction',          '______________',        0,      0,  1.0, 0.0, 0.0, 'immediate',   1,  'All',           1)
};
*/
