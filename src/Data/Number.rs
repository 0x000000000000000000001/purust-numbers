pub fn Data_Number_isFinite(a0: f64) -> bool {
    a0.is_finite()
}

pub fn Data_Number_floor(a0: f64) -> f64 {
    a0.floor()
}

pub fn Data_Number_ceil(a0: f64) -> f64 {
    a0.ceil()
}

pub fn Data_Number_round(a0: f64) -> f64 {
    a0.round()
}

pub fn Data_Number_abs(a0: f64) -> f64 {
    a0.abs()
}

pub fn Data_Number_acos(a0: f64) -> f64 { a0.acos() }
pub fn Data_Number_asin(a0: f64) -> f64 { a0.asin() }
pub fn Data_Number_atan(a0: f64) -> f64 { a0.atan() }
pub fn Data_Number_atan2(a0: f64, a1: f64) -> f64 { a0.atan2(a1) }
pub fn Data_Number_cos(a0: f64) -> f64 { a0.cos() }
pub fn Data_Number_exp(a0: f64) -> f64 { a0.exp() }
pub fn Data_Number_infinity() -> f64 { std::f64::INFINITY }
pub fn Data_Number_isNaN(a0: f64) -> bool { a0.is_nan() }
pub fn Data_Number_log(a0: f64) -> f64 { a0.ln() }
pub fn Data_Number_max(a0: f64, a1: f64) -> f64 { a0.max(a1) }
pub fn Data_Number_min(a0: f64, a1: f64) -> f64 { a0.min(a1) }
pub fn Data_Number_nan() -> f64 { std::f64::NAN }
pub fn Data_Number_pow(a0: f64, a1: f64) -> f64 { a0.powf(a1) }
pub fn Data_Number_remainder(a0: f64, a1: f64) -> f64 { a0 % a1 }
pub fn Data_Number_sign(a0: f64) -> f64 { a0.signum() }
pub fn Data_Number_sin(a0: f64) -> f64 { a0.sin() }
pub fn Data_Number_sqrt(a0: f64) -> f64 { a0.sqrt() }
pub fn Data_Number_tan(a0: f64) -> f64 { a0.tan() }
pub fn Data_Number_trunc(a0: f64) -> f64 { a0.trunc() }

pub fn Data_Number_fromStringImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut str_val: crate::UnknownType| -> crate::UnknownType {
        let str_c1 = str_val.clone();
        crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut isFinite: crate::UnknownType| -> crate::UnknownType {
            let str_c2 = str_c1.clone();
            let isF_c1 = isFinite.clone();
            crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut just: crate::UnknownType| -> crate::UnknownType {
                let str_c3 = str_c2.clone();
                let isF_c2 = isF_c1.clone();
                let just_c1 = just.clone();
                crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut nothing: crate::UnknownType| -> crate::UnknownType {
                    if let Ok(parsed) = str_c3.unwrap_string().parse::<f64>() {
                        let num = crate::mk_number(parsed);
                        let is_fin = isF_c2.unwrap_func1()(num.clone());
                        if is_fin.unwrap_bool() {
                            return just_c1.unwrap_func1()(num);
                        }
                    }
                    nothing.clone()
                })))
            })))
        })))
    })))
}
