use std::cmp::PartialEq;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less, Greater};
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Clock {
    hour :u8,
    minute :u8
}

impl Clock {
    const MINUTES_PER_DAY : i32 = 24*60;
    const MINUTES_PER_HOUR : i32 = 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut  tot_min = minutes + hours * Clock::MINUTES_PER_HOUR;
        tot_min = tot_min % (Clock::MINUTES_PER_DAY);
        if tot_min < 0 { tot_min += Clock::MINUTES_PER_DAY};

        Clock {hour:(tot_min/Clock::MINUTES_PER_HOUR) as u8, minute: (tot_min % Clock::MINUTES_PER_HOUR) as u8 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hour as i32, self.minute as i32 + minutes)
    }
    pub fn sub_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hour as i32, self.minute as i32 - minutes)
    }
    pub fn tot_minutes(&self) -> i32 {
        (self.hour as i32) *60 + (self.minute as i32)
    }


    pub fn to_string(self) -> String {
        let mut ret = String::with_capacity(5);
        let hour:u32 = self.hour as u32;
        let minute:u32 = self.minute as u32;
        ret.push(char::from_digit(  hour/10,10).unwrap_or('-'));
        ret.push(char::from_digit(  hour%10,10).unwrap_or('-'));
        ret.push(':');
        ret.push(char::from_digit(  minute/10,10).unwrap_or('-'));
        ret.push(char::from_digit(minute%10, 10).unwrap_or('-'));
        return ret;
    }
}

impl Default for Clock{
    fn default() -> Self {
        return Clock::new(0,0);
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.to_string())
    }
}

impl PartialEq<Clock> for Clock {
    fn eq(&self, other: &Clock)-> bool {
        self.hour == other.hour && self.minute == other.minute}
    //fn ne(&self, other: &Clock)-> bool {!eq(self,RHS)}
}

impl PartialOrd for Clock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {return Some(Equal)}
        if self.lt(other) {return Some(Less)}
        if self.gt(other) {return Some(Greater)}
        None
    }

    fn lt(&self, other: &Self) -> bool {
        if self.hour == other.hour {
            self.minute < other.minute
        }  else {
            self.hour < other.hour
        }
    }

    fn le(&self, other: &Self) -> bool {
        self.eq(other) || self.lt(other)
    }

    fn gt(&self, other: &Self) -> bool {
        !(self.le(other))
    }

    fn ge(&self, other: &Self) -> bool {
        !(self.lt(other))
    }
}

impl Add<i32> for Clock {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        self.add_minutes(other)
    }

}

impl Add<Clock> for Clock {
    type Output = Self;

    fn add(self, other: Clock) -> Self {
        self.add_minutes(other.tot_minutes())
    }

}

impl Sub<i32> for Clock {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        self.sub_minutes(other)
    }

}

impl Sub<Clock> for Clock {
    type Output = Self;

    fn sub(self, other: Clock) -> Self {
        self.sub_minutes(other.tot_minutes())
    }

}

