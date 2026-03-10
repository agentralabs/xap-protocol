use serde::{Serialize, Deserialize};
use verity_kernel::{CanonicalTimestamp, VerityError};

/// The seven possible outcomes of a Verity decision.
/// These match the VerityReceipt schema outcome_classification enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutcomeClassification {
    Success,
    Fail,
    Unknown,
    Disputed,
    Reversed,
    Timeout,
    Partial,
}

impl std::fmt::Display for OutcomeClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Fail => write!(f, "FAIL"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::Disputed => write!(f, "DISPUTED"),
            Self::Reversed => write!(f, "REVERSED"),
            Self::Timeout => write!(f, "TIMEOUT"),
            Self::Partial => write!(f, "PARTIAL"),
        }
    }
}

/// The seven kinds of decisions Verity captures.
/// These match the VerityReceipt schema decision_type enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionType {
    ConditionVerification,
    SplitCalculation,
    TimeoutResolution,
    DisputeInitiation,
    DisputeResolution,
    ReversalExecution,
    ReputationUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeTransition {
    pub from: OutcomeClassification,
    pub to: OutcomeClassification,
    pub timestamp: CanonicalTimestamp,
    pub reason: String,
}

pub struct OutcomeStateMachine {
    current: OutcomeClassification,
    history: Vec<OutcomeTransition>,
}

impl OutcomeStateMachine {
    pub fn new() -> Self {
        Self {
            current: OutcomeClassification::Unknown,
            history: Vec::new(),
        }
    }

    pub fn transition(
        &mut self,
        to: OutcomeClassification,
        reason: String,
    ) -> Result<(), VerityError> {
        if !Self::is_valid_transition(&self.current, &to) {
            return Err(VerityError::InvalidTransition {
                from: format!("{}", self.current),
                to: format!("{}", to),
                reason: "transition not allowed by outcome state machine".to_string(),
            });
        }

        let transition = OutcomeTransition {
            from: self.current.clone(),
            to: to.clone(),
            timestamp: CanonicalTimestamp::now(),
            reason,
        };
        self.history.push(transition);
        self.current = to;
        Ok(())
    }

    pub fn current(&self) -> &OutcomeClassification {
        &self.current
    }

    pub fn history(&self) -> &[OutcomeTransition] {
        &self.history
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.current, OutcomeClassification::Reversed)
    }

    fn is_valid_transition(from: &OutcomeClassification, to: &OutcomeClassification) -> bool {
        use OutcomeClassification::*;
        matches!(
            (from, to),
            // From UNKNOWN
            (Unknown, Success)
            | (Unknown, Fail)
            | (Unknown, Partial)
            | (Unknown, Timeout)
            | (Unknown, Disputed)
            | (Unknown, Unknown)
            // From SUCCESS/FAIL/PARTIAL → REVERSED
            | (Success, Reversed)
            | (Fail, Reversed)
            | (Partial, Reversed)
            // From DISPUTED → resolution
            | (Disputed, Success)
            | (Disputed, Fail)
            | (Disputed, Partial)
            // TIMEOUT → DISPUTED (appeal)
            | (Timeout, Disputed)
        )
    }
}

impl Default for OutcomeStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state_is_unknown() {
        let sm = OutcomeStateMachine::new();
        assert_eq!(*sm.current(), OutcomeClassification::Unknown);
    }

    #[test]
    fn test_unknown_to_success() {
        let mut sm = OutcomeStateMachine::new();
        assert!(sm.transition(OutcomeClassification::Success, "conditions met".into()).is_ok());
        assert_eq!(*sm.current(), OutcomeClassification::Success);
    }

    #[test]
    fn test_unknown_to_fail() {
        let mut sm = OutcomeStateMachine::new();
        assert!(sm.transition(OutcomeClassification::Fail, "conditions failed".into()).is_ok());
    }

    #[test]
    fn test_unknown_to_partial() {
        let mut sm = OutcomeStateMachine::new();
        assert!(sm.transition(OutcomeClassification::Partial, "some conditions met".into()).is_ok());
    }

    #[test]
    fn test_unknown_to_timeout() {
        let mut sm = OutcomeStateMachine::new();
        assert!(sm.transition(OutcomeClassification::Timeout, "deadline expired".into()).is_ok());
    }

    #[test]
    fn test_unknown_to_disputed() {
        let mut sm = OutcomeStateMachine::new();
        assert!(sm.transition(OutcomeClassification::Disputed, "agent disputes".into()).is_ok());
    }

    #[test]
    fn test_unknown_to_unknown() {
        let mut sm = OutcomeStateMachine::new();
        assert!(sm.transition(OutcomeClassification::Unknown, "re-evaluation".into()).is_ok());
    }

    #[test]
    fn test_success_to_reversed() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Success, "ok".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Reversed, "chargeback".into()).is_ok());
    }

    #[test]
    fn test_fail_to_reversed() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Fail, "failed".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Reversed, "incorrect failure".into()).is_ok());
    }

    #[test]
    fn test_partial_to_reversed() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Partial, "partial".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Reversed, "reversal".into()).is_ok());
    }

    #[test]
    fn test_disputed_to_success() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Disputed, "dispute".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Success, "dispute resolved".into()).is_ok());
    }

    #[test]
    fn test_disputed_to_fail() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Disputed, "dispute".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Fail, "dispute resolved against".into()).is_ok());
    }

    #[test]
    fn test_disputed_to_partial() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Disputed, "dispute".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Partial, "partial resolution".into()).is_ok());
    }

    #[test]
    fn test_timeout_to_disputed() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Timeout, "timed out".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Disputed, "appeal".into()).is_ok());
    }

    // Invalid transitions
    #[test]
    fn test_success_to_fail_invalid() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Success, "ok".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Fail, "nope".into()).is_err());
    }

    #[test]
    fn test_fail_to_success_invalid() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Fail, "failed".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Success, "nope".into()).is_err());
    }

    #[test]
    fn test_timeout_to_success_invalid() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Timeout, "timed out".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Success, "nope".into()).is_err());
    }

    #[test]
    fn test_reversed_is_terminal() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Success, "ok".into()).unwrap();
        sm.transition(OutcomeClassification::Reversed, "chargeback".into()).unwrap();
        assert!(sm.is_terminal());
        assert!(sm.transition(OutcomeClassification::Success, "nope".into()).is_err());
        assert!(sm.transition(OutcomeClassification::Fail, "nope".into()).is_err());
        assert!(sm.transition(OutcomeClassification::Reversed, "nope".into()).is_err());
    }

    #[test]
    fn test_timeout_only_allows_disputed() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Timeout, "timed out".into()).unwrap();
        assert!(sm.transition(OutcomeClassification::Success, "nope".into()).is_err());
        assert!(sm.transition(OutcomeClassification::Fail, "nope".into()).is_err());
        assert!(sm.transition(OutcomeClassification::Partial, "nope".into()).is_err());
        assert!(sm.transition(OutcomeClassification::Reversed, "nope".into()).is_err());
    }

    #[test]
    fn test_history_records_transitions() {
        let mut sm = OutcomeStateMachine::new();
        sm.transition(OutcomeClassification::Disputed, "dispute".into()).unwrap();
        sm.transition(OutcomeClassification::Success, "resolved".into()).unwrap();
        assert_eq!(sm.history().len(), 2);
        assert_eq!(sm.history()[0].from, OutcomeClassification::Unknown);
        assert_eq!(sm.history()[0].to, OutcomeClassification::Disputed);
        assert_eq!(sm.history()[1].from, OutcomeClassification::Disputed);
        assert_eq!(sm.history()[1].to, OutcomeClassification::Success);
    }

    #[test]
    fn test_unknown_not_equal_success() {
        assert_ne!(OutcomeClassification::Unknown, OutcomeClassification::Success);
    }
}
