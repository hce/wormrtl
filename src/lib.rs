use std::collections::{ HashMap, HashSet };
use regex::{ Regex, RegexBuilder };

#[repr(C)]
pub enum RegexFlags {
    CaseInsensitive = 1,
    MultiLine = 2,
    DotMatchesNewLine = 4,
    IgnoreWhitespace = 8,
    Unicode = 16,
    Octal = 32
}

#[repr(C)]
pub struct AllocatedString{
    size: usize,
    ptr: *mut u8
}

#[repr(C)]
pub enum RtlValue {
    RtlInt(i64),
    RtlFloat(f64),
    RtlBool(bool),
    RtlString(String),
    RtlRegex(Regex),
    // RtlNone is acually an error condition
    RtlNone
}

impl RtlValue {
    pub fn compare_to(&self, other: &RtlValue) -> bool {
        match &self {
            &RtlValue::RtlInt(l) => {
                match &other {
                    &RtlValue::RtlInt(r) => l == r,
                    _ => false
                }
            },
            &RtlValue::RtlFloat(l) => {
                match &other {
                    &RtlValue::RtlFloat(r) => l == r,
                    _ => false
                }
            },
            &RtlValue::RtlBool(l) => {
                match &other {
                    &RtlValue::RtlBool(r) => l == r,
                    _ => false
                }
            },
            &RtlValue::RtlString(l) => {
                match &other {
                    &RtlValue::RtlString(r) => l == r,
                    &RtlValue::RtlRegex(r) => r.is_match(&l),
                    _ => false
                }
            },
            &RtlValue::RtlRegex(_l) => false,
            &RtlValue::RtlNone => false
        }
    }

    pub fn type_id(&self) -> u32 {
        match &self {
            &RtlValue::RtlInt(_) => 0,
            &RtlValue::RtlFloat(_) => 1,
            &RtlValue::RtlBool(_) => 2,
            &RtlValue::RtlString(_) => 3,
            &RtlValue::RtlRegex(_) => 4,
            &RtlValue::RtlNone => 5
        }
    }

    pub fn as_boolean(&self) -> bool {
        match &self {
            &RtlValue::RtlBool(b) => *b,
            _ => false
        }
    }
}

#[repr(C)]
pub struct RtlState {
    field_set: HashMap<String, RtlValue>,
    reserved_elements: u64,
    value_counter: u64,
    values: HashMap<u64, RtlValue>,
    flags: HashMap<String, RtlValue>,
    permanents: HashSet<u64>
}

#[no_mangle]
pub extern "C" fn make_state(reserved_elements: u64) -> *mut RtlState {
    let state = RtlState {
        field_set: HashMap::new(),
        reserved_elements,
        value_counter: reserved_elements,
        values: HashMap::new(),
        flags: HashMap::new(),
        permanents: HashSet::new()
    };
    let etats = Box::new(state);
    Box::into_raw(etats)
}

#[no_mangle]
pub extern "C" fn free_state(state: *mut RtlState) {
    let _state: Box<RtlState> = unsafe { Box::from_raw(state) };
}

#[no_mangle]
pub extern "C" fn alloc_string(s: usize) -> *mut AllocatedString {
    let mut v: Vec<u8> = Vec::with_capacity(s);
    for _i in 0..s {
        v.push(170);
    }
    let t = Box::into_raw(v.into_boxed_slice());
    let allocated_string = AllocatedString {
        size: s, ptr: t as *mut u8
    };
    let bx = Box::new(allocated_string);
    Box::into_raw(bx)
}

#[no_mangle]
pub extern "C" fn get_string_buf(ptr: *mut AllocatedString) -> *mut u8 {
    let a_str = unsafe { &*ptr };
    a_str.ptr
}

#[no_mangle]
pub extern "C" fn get_string_len(ptr: *mut AllocatedString) -> usize {
    let a_str = unsafe { &*ptr };
    a_str.size
}

#[no_mangle]
pub extern "C" fn make_string(
    state: *mut RtlState,
    idx: u64,
    ptr: *mut AllocatedString,
    len: usize
) -> u64 {
    let mut etats = unsafe { &mut *state };
    let a_str = unsafe { Box::from_raw(ptr) };
    let vec = unsafe { Vec::from_raw_parts(a_str.ptr, a_str.size, a_str.size) };
    let sigh = String::from_utf8_lossy(vec.as_slice());
    let (left, _rest) = sigh.split_at(len);
    let s = left.to_string();
    let rv = RtlValue::RtlString(s);
    if idx == 0 {
        let vc = etats.value_counter;
        etats.value_counter += 1;
        etats.values.insert(vc, rv);
        vc
    } else {
        etats.values.insert(idx, rv);
        idx
    }
}

#[no_mangle]
pub extern "C" fn make_regex(
    state: *mut RtlState,
    idx: u64,
    flags: u64,
    ptr: *mut AllocatedString,
    len: usize
) -> u64 {
    let mut etats = unsafe { &mut *state };
    let a_str = unsafe { Box::from_raw(ptr) };
    let vec = unsafe { Vec::from_raw_parts(a_str.ptr, a_str.size, a_str.size) };
    let sigh = String::from_utf8_lossy(vec.as_slice());
    let (left, _rest) = sigh.split_at(len);
    let mut rb: RegexBuilder = RegexBuilder::new(left);
    rb.case_insensitive((flags & RegexFlags::CaseInsensitive as u64) == (RegexFlags::CaseInsensitive as u64))
        .multi_line((flags & RegexFlags::MultiLine as u64) == (RegexFlags::MultiLine as u64))
        .dot_matches_new_line((flags & RegexFlags::DotMatchesNewLine as u64) == (RegexFlags::DotMatchesNewLine as u64))
        .ignore_whitespace((flags & RegexFlags::IgnoreWhitespace as u64) == (RegexFlags::IgnoreWhitespace as u64))
        .unicode((flags & RegexFlags::Unicode as u64) == (RegexFlags::Unicode as u64))
        .octal((flags & RegexFlags::Octal as u64) == (RegexFlags::Octal as u64));
    if let Ok(regex) = rb.build() {
        let rv = RtlValue::RtlRegex(regex);
        if idx == 0 {
            let vc = etats.value_counter;
            etats.value_counter += 1;
            etats.values.insert(vc, rv);
            vc
        } else {
            etats.values.insert(idx, rv);
            idx
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn make_bool(state: *mut RtlState, idx: u64, b: i32) -> u64 {
    let mut etats = unsafe { &mut *state };
    let rv = match b {
        0 => RtlValue::RtlBool(false),
        _ => RtlValue::RtlBool(true)
    };
    if idx == 0 {
        let vc = etats.value_counter;
        etats.value_counter += 1;
        etats.values.insert(vc, rv);
        vc
    } else {
        etats.values.insert(idx, rv);
        idx
    }
}

#[no_mangle]
pub extern "C" fn make_i64(state: *mut RtlState, idx: u64, intval: i64) -> u64 {
    let mut etats = unsafe { &mut *state };
    let rv = RtlValue::RtlInt(intval);
    if idx == 0 {
        let vc = etats.value_counter;
        etats.value_counter += 1;
        etats.values.insert(vc, rv);
        vc
    } else {
        etats.values.insert(idx, rv);
        idx
    }
}

#[no_mangle]
pub extern "C" fn make_f64(state: *mut RtlState, idx: u64, fval: f64) -> u64 {
    let mut etats = unsafe { &mut *state };
    let rv = RtlValue::RtlFloat(fval);
    if idx == 0 {
        let vc = etats.value_counter;
        etats.value_counter += 1;
        etats.values.insert(vc, rv);
        vc
    } else {
        etats.values.insert(idx, rv);
        idx
    }
}


#[no_mangle]
pub extern "C" fn rtl_eq(state: *mut RtlState, left: u64, right: u64) -> u64 {
    let mut etats = unsafe { &mut *state };
    let left1 = etats.values.get(&left).unwrap_or(&RtlValue::RtlNone);
    let right1 = etats.values.get(&right).unwrap_or(&RtlValue::RtlNone);
    let res = left1.compare_to(right1);
    let res_v = RtlValue::RtlBool(res);
    let vc = etats.value_counter;
    etats.value_counter += 1;
    etats.values.insert(vc, res_v);
    vc
}

#[no_mangle]
pub extern "C" fn rtl_and(state: *mut RtlState, left: u64, right: u64) -> u64 {
    let mut etats = unsafe { &mut *state };
    let left1 = etats.values.get(&left).unwrap_or(&RtlValue::RtlNone);
    let right1 = etats.values.get(&right).unwrap_or(&RtlValue::RtlNone);
    let res = left1.as_boolean() && right1.as_boolean();
    let res_v = RtlValue::RtlBool(res);
    let vc = etats.value_counter;
    etats.value_counter += 1;
    etats.values.insert(vc, res_v);
    vc
}

#[no_mangle]
pub extern "C" fn rtl_or(state: *mut RtlState, left: u64, right: u64) -> u64 {
    let mut etats = unsafe { &mut *state };
    let left1 = etats.values.get(&left).unwrap_or(&RtlValue::RtlNone);
    let right1 = etats.values.get(&right).unwrap_or(&RtlValue::RtlNone);
    let res = left1.as_boolean() || right1.as_boolean();
    let res_v = RtlValue::RtlBool(res);
    let vc = etats.value_counter;
    etats.value_counter += 1;
    etats.values.insert(vc, res_v);
    vc
}

#[no_mangle]
pub extern "C" fn rtl_not(state: *mut RtlState, value: u64) -> u64 {
    let mut etats = unsafe { &mut *state };
    let value1 = etats.values.get(&value).unwrap_or(&RtlValue::RtlNone);
    let res = !value1.as_boolean();
    let res_v = RtlValue::RtlBool(res);
    let vc = etats.value_counter;
    etats.value_counter += 1;
    etats.values.insert(vc, res_v);
    vc
}

#[no_mangle]
pub extern "C" fn rtl_type(state: *mut RtlState, v: u64) -> u32 {
    let etats = unsafe { &*state };
    let value = etats.values.get(&v).unwrap_or(&RtlValue::RtlNone);
    value.type_id()
}

#[no_mangle]
pub extern "C" fn rtl_get_bool(state: *mut RtlState, v: u64) -> u32 {
    let etats = unsafe { &*state };
    let value = etats.values.get(&v).unwrap_or(&RtlValue::RtlNone);
    match value.as_boolean() {
        false => 0,
        true => 1
    }
}

#[no_mangle]
pub extern "C" fn rtl_make_permanent(state: *mut RtlState, v: u64) {
    let etats = unsafe { &mut *state };
    etats.permanents.insert(v);
}

#[no_mangle]
pub extern "C" fn rtl_unmake_permanent(state: *mut RtlState, v: u64) {
    let etats = unsafe { &mut *state };
    etats.permanents.remove(&v);
}

#[no_mangle]
pub extern "C" fn rtl_is_permanent(state: *mut RtlState, v: u64) -> u32 {
    let etats = unsafe { &*state };
    if v < etats.reserved_elements {
        return 1;
    }
    if etats.permanents.contains(&v) {
        1
    } else {
        0
    }
}


#[no_mangle]
pub extern "C" fn rtl_clear(state: *mut RtlState) -> usize {
    let etats = unsafe { &mut *state };
    let mut counter = 0;
    for item in etats.reserved_elements..etats.value_counter {
        if !etats.permanents.contains(&item) {
            if etats.values.remove(&item).is_some() {
                counter += 1;
            }
        }
    }
    counter
}
