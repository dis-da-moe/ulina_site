use chrono::{FixedOffset, TimeZone};

#[derive(Clone)]
pub struct Local;

impl TimeZone for Local {
    type Offset = FixedOffset;

    fn from_offset(_offset: &Self::Offset) -> Self {
        todo!()
    }

    fn offset_from_local_date(
        &self,
        _local: &chrono::NaiveDate,
    ) -> chrono::LocalResult<Self::Offset> {
        todo!()
    }

    fn offset_from_local_datetime(
        &self,
        _local: &chrono::NaiveDateTime,
    ) -> chrono::LocalResult<Self::Offset> {
        todo!()
    }

    fn offset_from_utc_date(&self, _utc: &chrono::NaiveDate) -> Self::Offset {
        todo!()
    }

    fn offset_from_utc_datetime(&self, _utc: &chrono::NaiveDateTime) -> Self::Offset {
        todo!()
    }
}
