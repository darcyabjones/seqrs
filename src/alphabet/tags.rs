use crate::matcher::{Match, RedundantAlphabet};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CodonTag {
    Start,
    Res,
    StartRes,
    Stop,
    StartStop,
    StopRes,
    Any,
}

impl CodonTag {
    fn cardinality() -> usize {
        7
    }

    fn variants() -> Vec<Self> {
        use super::CodonTag::*;
        vec![Start, Res, StartRes, Stop, StartStop, StopRes, Any]
    }

    fn redundancy(&self) -> Vec<Self> {
        use super::CodonTag::*;
        match self {
            StartRes => vec![Start, Res],
            StartStop => vec![Start, Stop],
            StopRes => vec![Res, Stop],
            Any => vec![Start, Res, StartRes, Stop, StartStop, StopRes],
            _ => Vec::new(), // NB won't allocate until element pushed onto it.
        }
    }
}

impl Match<CodonTag> for CodonTag {
    fn matches(&self, other: &CodonTag) -> bool {
        match (self, other) {
            (CodonTag::Start, CodonTag::Start) => true,
            (CodonTag::Start, CodonTag::StartRes) => true,
            (CodonTag::Start, CodonTag::StartStop) => true,
            (CodonTag::Res, CodonTag::Res) => true,
            (CodonTag::Res, CodonTag::StartRes) => true,
            (CodonTag::Res, CodonTag::StopRes) => true,
            (CodonTag::StartRes, CodonTag::Stop) => false,
            (CodonTag::StartRes, _) => true,
            (CodonTag::Stop, CodonTag::Stop) => true,
            (CodonTag::Stop, CodonTag::StartStop) => true,
            (CodonTag::Stop, CodonTag::StopRes) => true,
            (CodonTag::StartStop, CodonTag::Res) => false,
            (CodonTag::StartStop, _) => true,
            (CodonTag::StopRes, CodonTag::Start) => false,
            (CodonTag::StopRes, _) => true,
            (CodonTag::Any, _) => true,
            (_, CodonTag::Any) => true,
            _ => false,
        }
    }
}

impl RedundantAlphabet for CodonTag {
    fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (CodonTag::Start, CodonTag::Start) => CodonTag::Start,
            (CodonTag::Start, CodonTag::Res) => CodonTag::StartRes,
            (CodonTag::Start, CodonTag::StartRes) => CodonTag::StartRes,
            (CodonTag::Start, CodonTag::Stop) => CodonTag::StartStop,
            (CodonTag::Start, CodonTag::StartStop) => CodonTag::StartStop,
            (CodonTag::Res, CodonTag::Start) => CodonTag::StartRes,
            (CodonTag::Res, CodonTag::Res) => CodonTag::Res,
            (CodonTag::Res, CodonTag::StartRes) => CodonTag::StartRes,
            (CodonTag::Res, CodonTag::Stop) => CodonTag::StopRes,
            (CodonTag::Res, CodonTag::StopRes) => CodonTag::StopRes,
            (CodonTag::StartRes, CodonTag::Start) => CodonTag::StartRes,
            (CodonTag::StartRes, CodonTag::Res) => CodonTag::Res,
            (CodonTag::StartRes, CodonTag::StartRes) => CodonTag::StartRes,
            (CodonTag::Stop, CodonTag::Start) => CodonTag::StartStop,
            (CodonTag::Stop, CodonTag::Res) => CodonTag::StopRes,
            (CodonTag::Stop, CodonTag::Stop) => CodonTag::Stop,
            (CodonTag::Stop, CodonTag::StartStop) => CodonTag::StartStop,
            (CodonTag::Stop, CodonTag::StopRes) => CodonTag::StopRes,
            (CodonTag::StartStop, CodonTag::Start) => CodonTag::StartStop,
            (CodonTag::StartStop, CodonTag::Stop) => CodonTag::StartStop,
            (CodonTag::StartStop, CodonTag::StartStop) => CodonTag::StartStop,
            (CodonTag::StopRes, CodonTag::Res) => CodonTag::StopRes,
            (CodonTag::StopRes, CodonTag::Stop) => CodonTag::StopRes,
            (CodonTag::StopRes, CodonTag::StopRes) => CodonTag::StopRes,
            _ => CodonTag::Any,
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (CodonTag::Start, CodonTag::Start) => Some(CodonTag::Start),
            (CodonTag::Start, CodonTag::Res) => None,
            (CodonTag::Start, CodonTag::StartRes) => Some(CodonTag::Start),
            (CodonTag::Start, CodonTag::Stop) => None,
            (CodonTag::Start, CodonTag::StartStop) => Some(CodonTag::Start),
            (CodonTag::Start, CodonTag::StopRes) => None,
            (CodonTag::Start, CodonTag::Any) => Some(CodonTag::Start),
            (CodonTag::Res, CodonTag::Start) => None,
            (CodonTag::Res, CodonTag::Res) => Some(CodonTag::Res),
            (CodonTag::Res, CodonTag::StartRes) => Some(CodonTag::Res),
            (CodonTag::Res, CodonTag::Stop) => None,
            (CodonTag::Res, CodonTag::StartStop) => None,
            (CodonTag::Res, CodonTag::StopRes) => Some(CodonTag::Res),
            (CodonTag::Res, CodonTag::Any) => Some(CodonTag::Res),
            (CodonTag::StartRes, CodonTag::Start) => Some(CodonTag::Start),
            (CodonTag::StartRes, CodonTag::Res) => Some(CodonTag::Res),
            (CodonTag::StartRes, CodonTag::StartRes) => Some(CodonTag::StartRes),
            (CodonTag::StartRes, CodonTag::Stop) => None,
            (CodonTag::StartRes, CodonTag::StartStop) => Some(CodonTag::Start),
            (CodonTag::StartRes, CodonTag::StopRes) => Some(CodonTag::Res),
            (CodonTag::StartRes, CodonTag::Any) => Some(CodonTag::StartRes),
            (CodonTag::Stop, CodonTag::Start) => None,
            (CodonTag::Stop, CodonTag::Res) => None,
            (CodonTag::Stop, CodonTag::StartRes) => None,
            (CodonTag::Stop, CodonTag::Stop) => Some(CodonTag::Stop),
            (CodonTag::Stop, CodonTag::StartStop) => Some(CodonTag::Stop),
            (CodonTag::Stop, CodonTag::StopRes) => Some(CodonTag::Stop),
            (CodonTag::Stop, CodonTag::Any) => Some(CodonTag::Stop),
            (CodonTag::StartStop, CodonTag::Start) => Some(CodonTag::Start),
            (CodonTag::StartStop, CodonTag::Res) => None,
            (CodonTag::StartStop, CodonTag::StartRes) => Some(CodonTag::Start),
            (CodonTag::StartStop, CodonTag::Stop) => Some(CodonTag::Stop),
            (CodonTag::StartStop, CodonTag::StartStop) => Some(CodonTag::StartRes),
            (CodonTag::StartStop, CodonTag::StopRes) => Some(CodonTag::Stop),
            (CodonTag::StartStop, CodonTag::Any) => Some(CodonTag::StartStop),
            (CodonTag::StopRes, CodonTag::Start) => None,
            (CodonTag::StopRes, CodonTag::Res) => Some(CodonTag::Res),
            (CodonTag::StopRes, CodonTag::StartRes) => Some(CodonTag::Res),
            (CodonTag::StopRes, CodonTag::Stop) => Some(CodonTag::Stop),
            (CodonTag::StopRes, CodonTag::StartStop) => Some(CodonTag::Stop),
            (CodonTag::StopRes, CodonTag::StopRes) => Some(CodonTag::StopRes),
            (CodonTag::StopRes, CodonTag::Any) => Some(CodonTag::StopRes),
            (CodonTag::Any, x) => Some(*x),
        }
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (CodonTag::Start, CodonTag::Res) => Some(CodonTag::Start),
            (CodonTag::Start, CodonTag::Stop) => Some(CodonTag::Start),
            (CodonTag::Start, CodonTag::StopRes) => Some(CodonTag::Start),
            (CodonTag::Res, CodonTag::Start) => Some(CodonTag::Res),
            (CodonTag::Res, CodonTag::Stop) => Some(CodonTag::Res),
            (CodonTag::Res, CodonTag::StartStop) => Some(CodonTag::Res),
            (CodonTag::StartRes, CodonTag::Start) => Some(CodonTag::Res),
            (CodonTag::StartRes, CodonTag::Res) => Some(CodonTag::Start),
            (CodonTag::StartRes, CodonTag::Stop) => Some(CodonTag::StartRes),
            (CodonTag::StartRes, CodonTag::StartStop) => Some(CodonTag::Res),
            (CodonTag::StartRes, CodonTag::StopRes) => Some(CodonTag::Start),
            (CodonTag::Stop, CodonTag::Start) => Some(CodonTag::Stop),
            (CodonTag::Stop, CodonTag::StartRes) => Some(CodonTag::Stop),
            (CodonTag::Stop, CodonTag::Res) => Some(CodonTag::Stop),
            (CodonTag::StartStop, CodonTag::Start) => Some(CodonTag::Stop),
            (CodonTag::StartStop, CodonTag::Res) => Some(CodonTag::Stop),
            (CodonTag::StartStop, CodonTag::StartRes) => Some(CodonTag::Res),
            (CodonTag::StartStop, CodonTag::Stop) => Some(CodonTag::Start),
            (CodonTag::StartStop, CodonTag::StopRes) => Some(CodonTag::Start),
            (CodonTag::StopRes, CodonTag::Start) => Some(CodonTag::StopRes),
            (CodonTag::StopRes, CodonTag::Res) => Some(CodonTag::Stop),
            (CodonTag::StopRes, CodonTag::StartRes) => Some(CodonTag::Stop),
            (CodonTag::StopRes, CodonTag::Stop) => Some(CodonTag::Res),
            (CodonTag::StopRes, CodonTag::StartStop) => Some(CodonTag::Res),
            (CodonTag::Any, CodonTag::Start) => Some(CodonTag::StopRes),
            (CodonTag::Any, CodonTag::Res) => Some(CodonTag::StartStop),
            (CodonTag::Any, CodonTag::StartRes) => Some(CodonTag::Stop),
            (CodonTag::Any, CodonTag::Stop) => Some(CodonTag::StartRes),
            (CodonTag::Any, CodonTag::StartStop) => Some(CodonTag::Res),
            (CodonTag::Any, CodonTag::StopRes) => Some(CodonTag::Start),
            (_, CodonTag::Any) => None,
            _ => None,
        }
    }

    fn is_redundant(&self) -> bool {
        match self {
            CodonTag::Start | CodonTag::Res | CodonTag::Stop => false,
            _ => true,
        }
    }
}
