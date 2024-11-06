use chrono::{NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;

pub trait FromTimeStamp<T> {
    fn from_stamp(secs: i64) -> Option<T>;
}

impl FromTimeStamp<NaiveDateTime> for NaiveDateTime {
    fn from_stamp(secs: i64) -> Option<NaiveDateTime> {
        NaiveDateTime::from_timestamp_opt(secs, 0)
    }
}

impl FromTimeStamp<NaiveDate> for NaiveDate {
    fn from_stamp(secs: i64) -> Option<NaiveDate> {
        NaiveDateTime::from_stamp(secs).map(|x| x.date())
    }
}

struct TimePeriod {
    real_time: NaiveDateTime,
    ulina_time: NaiveDateTime,
    time_difference: i64,
}

macro_rules! date {
    ($year: expr, $month: expr) => {
        NaiveDate::from_ymd($year, $month, 1).and_hms(0, 0, 0)
    };
}

lazy_static! {
    static ref OLD_TIME: TimePeriod = TimePeriod {
        real_time: date!(2019, 6),
        ulina_time: date!(2026, 1),
        time_difference: 12
    };
    static ref NEW_TIME: TimePeriod = TimePeriod {
        real_time: date!(2021, 1),
        ulina_time: date!(2045, 1),
        time_difference: 4
    };
}

#[derive(Debug, Clone)]
pub enum TimeError {
    OutOfRange,
    InvalidDate,
}

impl ToString for TimeError {
    fn to_string(&self) -> String {
        match self {
            TimeError::OutOfRange => "Given time was out of range",
            TimeError::InvalidDate => "Give date was invalid",
        }
        .to_string()
    }
}

macro_rules! time_period {
    ($name: tt, $field: tt) => {
        fn $name(time: i64) -> Result<&'static TimePeriod, TimeError> {
            if time <= OLD_TIME.$field.timestamp() {
                Err(TimeError::OutOfRange)
            } else if time <= NEW_TIME.$field.timestamp() {
                Ok(&OLD_TIME)
            } else {
                Ok(&NEW_TIME)
            }
        }
    };
}

time_period!(time_period_real, real_time);
time_period!(time_period_ulina, ulina_time);

fn calculate_to_ulina(real: i64, period: &'static TimePeriod) -> Option<i64> {
    let real_time_passed = real.checked_sub(period.real_time.timestamp())?;
    let ulina_time_passed = real_time_passed.checked_mul(period.time_difference)?;

    ulina_time_passed.checked_add(period.ulina_time.timestamp())
}

fn calculate_to_real(ulina: i64, period: &'static TimePeriod) -> Option<i64> {
    let ulina_time_passed = ulina.checked_sub(period.ulina_time.timestamp())?;
    let real_time_passed = ulina_time_passed.checked_div(period.time_difference)?;

    real_time_passed.checked_add(period.real_time.timestamp())
}

macro_rules! converter {
    ($name: tt, $period: tt, $calculate: tt) => {
        pub fn $name<T>(real: i64) -> Result<T, TimeError>
        where
            T: FromTimeStamp<T>,
        {
            let period = $period(real)?;

            let ulina_time = $calculate(real, period).ok_or(TimeError::OutOfRange)?;

            T::from_stamp(ulina_time).ok_or(TimeError::InvalidDate)
        }
    };
}

converter!(to_real, time_period_ulina, calculate_to_real);
converter!(to_ulina, time_period_real, calculate_to_ulina);

pub const DATE_FORMAT: &str = "%A %e %B %Y";
pub const DATE_TIME_FORMAT: &str = "%Y_%m_%d_%H_%M_%S";
