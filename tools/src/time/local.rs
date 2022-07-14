use chrono::{FixedOffset, TimeZone};

#[derive(Clone)]
pub struct Local;

impl TimeZone for Local {
    type Offset = FixedOffset;

    fn from_offset(offset: &Self::Offset) -> Self {
        todo!()
    }

    fn offset_from_local_date(
        &self,
        local: &chrono::NaiveDate,
    ) -> chrono::LocalResult<Self::Offset> {
        todo!()
    }

    fn offset_from_local_datetime(
        &self,
        local: &chrono::NaiveDateTime,
    ) -> chrono::LocalResult<Self::Offset> {
        todo!()
    }

    fn offset_from_utc_date(&self, utc: &chrono::NaiveDate) -> Self::Offset {
        todo!()
    }

    fn offset_from_utc_datetime(&self, utc: &chrono::NaiveDateTime) -> Self::Offset {
        todo!()
    }
}
