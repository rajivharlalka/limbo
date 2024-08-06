#[derive(Debug, Clone, PartialEq)]
pub enum JsonFunc {
    JSON,
}

impl ToString for JsonFunc {
    fn to_string(&self) -> String {
        match self {
            JsonFunc::JSON => "json".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AggFunc {
    Avg,
    Count,
    GroupConcat,
    Max,
    Min,
    StringAgg,
    Sum,
    Total,
}

impl AggFunc {
    pub fn to_string(&self) -> &str {
        match self {
            AggFunc::Avg => "avg",
            AggFunc::Count => "count",
            AggFunc::GroupConcat => "group_concat",
            AggFunc::Max => "max",
            AggFunc::Min => "min",
            AggFunc::StringAgg => "string_agg",
            AggFunc::Sum => "sum",
            AggFunc::Total => "total",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarFunc {
    Coalesce,
    Like,
    Abs,
    Upper,
    Lower,
    Random,
    Trim,
    LTrim,
    RTrim,
    Round,
    Length,
    Min,
    Max,
    Date,
    Time,
    Unicode,
    UnixEpoch,
}

impl ToString for ScalarFunc {
    fn to_string(&self) -> String {
        match self {
            ScalarFunc::Coalesce => "coalesce".to_string(),
            ScalarFunc::Like => "like(2)".to_string(),
            ScalarFunc::Abs => "abs".to_string(),
            ScalarFunc::Upper => "upper".to_string(),
            ScalarFunc::Lower => "lower".to_string(),
            ScalarFunc::Random => "random".to_string(),
            ScalarFunc::Trim => "trim".to_string(),
            ScalarFunc::LTrim => "ltrim".to_string(),
            ScalarFunc::RTrim => "rtrim".to_string(),
            ScalarFunc::Round => "round".to_string(),
            ScalarFunc::Length => "length".to_string(),
            ScalarFunc::Min => "min".to_string(),
            ScalarFunc::Max => "max".to_string(),
            ScalarFunc::Date => "date".to_string(),
            ScalarFunc::Time => "time".to_string(),
            ScalarFunc::Unicode => "unicode".to_string(),
            ScalarFunc::UnixEpoch => "unixepoch".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Func {
    Agg(AggFunc),
    Scalar(ScalarFunc),
    Json(JsonFunc),
}

impl Func {
    pub fn resolve_function(name: &str, arg_count: usize) -> Result<Func, ()> {
        match name {
            "avg" => Ok(Func::Agg(AggFunc::Avg)),
            "count" => Ok(Func::Agg(AggFunc::Count)),
            "group_concat" => Ok(Func::Agg(AggFunc::GroupConcat)),
            "max" if arg_count == 0 || arg_count == 1 => Ok(Func::Agg(AggFunc::Max)),
            "max" if arg_count > 1 => Ok(Func::Scalar(ScalarFunc::Max)),
            "min" if arg_count == 0 || arg_count == 1 => Ok(Func::Agg(AggFunc::Min)),
            "min" if arg_count > 1 => Ok(Func::Scalar(ScalarFunc::Min)),
            "string_agg" => Ok(Func::Agg(AggFunc::StringAgg)),
            "sum" => Ok(Func::Agg(AggFunc::Sum)),
            "total" => Ok(Func::Agg(AggFunc::Total)),
            "coalesce" => Ok(Func::Scalar(ScalarFunc::Coalesce)),
            "like" => Ok(Func::Scalar(ScalarFunc::Like)),
            "abs" => Ok(Func::Scalar(ScalarFunc::Abs)),
            "upper" => Ok(Func::Scalar(ScalarFunc::Upper)),
            "lower" => Ok(Func::Scalar(ScalarFunc::Lower)),
            "random" => Ok(Func::Scalar(ScalarFunc::Random)),
            "trim" => Ok(Func::Scalar(ScalarFunc::Trim)),
            "ltrim" => Ok(Func::Scalar(ScalarFunc::LTrim)),
            "rtrim" => Ok(Func::Scalar(ScalarFunc::RTrim)),
            "round" => Ok(Func::Scalar(ScalarFunc::Round)),
            "length" => Ok(Func::Scalar(ScalarFunc::Length)),
            "date" => Ok(Func::Scalar(ScalarFunc::Date)),
            "time" => Ok(Func::Scalar(ScalarFunc::Time)),
            "unicode" => Ok(Func::Scalar(ScalarFunc::Unicode)),
            "json" => Ok(Func::Json((JsonFunc::JSON))),
            "unixepoch" => Ok(Func::Scalar(ScalarFunc::UnixEpoch)),
            _ => Err(()),
        }
    }
}
