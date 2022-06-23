use super::clock::Clock;
use std::cmp::{Ordering, max,min};
use std::collections::VecDeque;

pub struct Calendar<Clock> {
    schedule: Vec<(Clock, Clock)>,
    bounds: (Clock,Clock)
}

impl Calendar<Clock> {

    pub fn new() -> Self{
        return Calendar;
    }

    pub fn find_free_slots(self, rhs: self) -> Self {
        return self;
    }

    pub fn push_slot(&mut vec: VecDeque<(Clock,Clock)>, slot: (Clock,Clock)) {
        if     
    }

    pub fn merge_calendar(self, rhs: Self) -> Self {
        let mut new_cal = Calendar::new();
        new_cal.bounds = ((max(self.bounds.0, rhs.bounds.0),min(self.bounds.1,rhs.bounds.1) ));

        let mut  result:VecDeque<(Clock,Clock)> = VecDeque::with_capacity(max(self.schedule.len(),rhs.schedule.len()));

        let mut it1 = self.schedule.into_iter();
        let mut it2 = rhs.schedule.into_iter();
        let mut slot1 = it1.next();
        let mut slot2= it2.next();
        while slot1.is_some() && slot2.is_some() {
            let slot1 = slot1?;
            let slot2 = slot2?;

            if Calendar::can_merge_slots(&slot1,&slot2) {
                let merged = Calendar::merge_slots(slot1,slot2)?;
                if result.is_empty() {
                    result.push_back(merged);
                } else{
                    if Calendar::can_merge_slots(result.back()?,&merged){
                        result.push_back(Calendar::merge_slots(result.pop_back()?,merged)?);
                    } else {
                        result.push_back(merged);
                    }
                }

            } else {

            }

        }

        let mut sch1 = VecDeque::from(self.schedule);
        let mut sch2 = VecDeque::from(rhs.schedule);



        while !sch2.is_empty || !sch1.is_empty() {
            //let slot1 = sch1.pop_front();
            //let slot2 = sch2.pop_front();
            if !sch1.is_empty() && !sch2.is_empty(){
                let slot1 = sch1.pop_front().unwrap();
                let slot2 = sch2.pop_front().unwrap();
                if Calendar::can_merge_slots(&slot1,&slot2) {

                } else {

                }
            } else if slot1 == None {
                result.push_back(slot2);
            } else if slot2 == None {
                result.push_back(slot1);
            }

        }

        return new_cal;
    }

    fn can_merge_slots(slot1: &(Clock,Clock), slot2:&(Clock,Clock)) -> bool{
        if slot1.0 < slot2.0 {
            slot1.1>slot2.0
            //intersect!
        } else {
            slot1.0>slot2.1
            //intersect!
        }
    }

    fn merge_slots(slot1: (Clock,Clock), slot2:(Clock,Clock)) -> Option<(Clock,Clock)> {
        let intersect =  Calendar::can_merge_slots(&slot1,&slot2);

        return if !intersect{
            None
        } else {
            Some((min(slot1.0,slot2.0),max(slot1.1,slot2.1)))
        }

    }
}

impl IntoIterator for Calendar<Clock>{
    type Item = Clock;
    type IntoIter = Vec::Iter<Clock>;

    fn into_iter(self) -> Self::IntoIter {
        self.schedule.into_iter()
    }
}



