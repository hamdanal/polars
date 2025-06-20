use super::*;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "dsl-schema", derive(schemars::JsonSchema))]
pub enum RollingFunctionBy {
    MinBy(RollingOptionsDynamicWindow),
    MaxBy(RollingOptionsDynamicWindow),
    MeanBy(RollingOptionsDynamicWindow),
    SumBy(RollingOptionsDynamicWindow),
    QuantileBy(RollingOptionsDynamicWindow),
    VarBy(RollingOptionsDynamicWindow),
    StdBy(RollingOptionsDynamicWindow),
}

impl Display for RollingFunctionBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use RollingFunctionBy::*;

        let name = match self {
            MinBy(_) => "rolling_min_by",
            MaxBy(_) => "rolling_max_by",
            MeanBy(_) => "rolling_mean_by",
            SumBy(_) => "rolling_sum_by",
            QuantileBy(_) => "rolling_quantile_by",
            VarBy(_) => "rolling_var_by",
            StdBy(_) => "rolling_std_by",
        };

        write!(f, "{name}")
    }
}

impl Hash for RollingFunctionBy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
