
use chrono::prelude::*;
use chrono::{NaiveDate};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn get_cur_ymd() -> String {
  let utc: DateTime<Utc> = Utc::now();
  utc.format("%Y-%m-%d").to_string()
}

pub fn get_cur_time() -> String {
  let utc: DateTime<Utc> = Utc::now();
  utc.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn calculate_hash(t: &[u8]) -> String {
  // 没有特殊需求，使用 HashMap 默认的 Hasher 即可
  let mut h = DefaultHasher::new();
  h.write(&t);
  format!("{:x}", h.finish())
}

pub fn get_weekday_index(date: &str) -> u32 {
  let utc = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
  let day_index = utc.ordinal();
  (day_index / 7) + 1
}