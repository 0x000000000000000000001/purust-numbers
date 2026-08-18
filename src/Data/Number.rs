pub fn Data_Number_isFinite(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_bool(a0.init_number.unwrap().is_finite())
}

pub fn Data_Number_floor(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.init_number.unwrap().floor())
}

pub fn Data_Number_ceil(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.init_number.unwrap().ceil())
}

pub fn Data_Number_round(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.init_number.unwrap().round())
}

pub fn Data_Number_abs(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.init_number.unwrap().abs())
}

pub fn Data_Number_acos(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().acos()) }
pub fn Data_Number_asin(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().asin()) }
pub fn Data_Number_atan(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().atan()) }
pub fn Data_Number_atan2(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().atan2(a1.init_number.unwrap())) }
pub fn Data_Number_cos(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().cos()) }
pub fn Data_Number_exp(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().exp()) }
pub fn Data_Number_infinity() -> crate::UnknownType { crate::mk_number(std::f64::INFINITY) }
pub fn Data_Number_isNaN(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_bool(a0.init_number.unwrap().is_nan()) }
pub fn Data_Number_log(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().ln()) }
pub fn Data_Number_max(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().max(a1.init_number.unwrap())) }
pub fn Data_Number_min(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().min(a1.init_number.unwrap())) }
pub fn Data_Number_nan() -> crate::UnknownType { crate::mk_number(std::f64::NAN) }
pub fn Data_Number_pow(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().powf(a1.init_number.unwrap())) }
pub fn Data_Number_remainder(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap() % a1.init_number.unwrap()) }
pub fn Data_Number_sign(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().signum()) }
pub fn Data_Number_sin(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().sin()) }
pub fn Data_Number_sqrt(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().sqrt()) }
pub fn Data_Number_tan(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().tan()) }
pub fn Data_Number_trunc(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.init_number.unwrap().trunc()) }

pub fn Data_Number_fromStringImpl() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a {
        call: Some(std::rc::Rc::new(move |mut str_val: crate::UnknownType| -> crate::UnknownType {
            let str_c1 = str_val.clone();
            crate::UnknownType::new(crate::Record_a {
                call: Some(std::rc::Rc::new(move |mut isFinite: crate::UnknownType| -> crate::UnknownType {
                    let str_c2 = str_c1.clone();
                    let isF_c1 = isFinite.clone();
                    crate::UnknownType::new(crate::Record_a {
                        call: Some(std::rc::Rc::new(move |mut just: crate::UnknownType| -> crate::UnknownType {
                            let str_c3 = str_c2.clone();
                            let isF_c2 = isF_c1.clone();
                            let just_c1 = just.clone();
                            crate::UnknownType::new(crate::Record_a {
                                call: Some(std::rc::Rc::new(move |mut nothing: crate::UnknownType| -> crate::UnknownType {
                                    if let Ok(parsed) = str_c3.init_string.as_ref().unwrap().parse::<f64>() {
                                        let num = crate::mk_number(parsed);
                                        let is_fin = isF_c2.call.as_ref().unwrap()(num.clone());
                                        if is_fin.init_bool.unwrap() {
                                            return crate::UnknownType::new(crate::Record_a {
                                                tag: "Just",
                                                vals: Some(std::rc::Rc::new(vec![num])),
                                                ..Default::default()
                                            });
                                        }
                                    }
                                    crate::UnknownType::new(crate::Record_a {
                                        tag: "Nothing",
                                        vals: None,
                                        ..Default::default()
                                    })
                                })),
                                ..Default::default()
                            })
                        })),
                        ..Default::default()
                    })
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}
