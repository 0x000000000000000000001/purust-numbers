pub fn Data_Number_isFinite(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_bool(a0.unwrap_number().is_finite())
}

pub fn Data_Number_floor(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.unwrap_number().floor())
}

pub fn Data_Number_ceil(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.unwrap_number().ceil())
}

pub fn Data_Number_round(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.unwrap_number().round())
}

pub fn Data_Number_abs(mut a0: crate::UnknownType) -> crate::UnknownType {
    crate::mk_number(a0.unwrap_number().abs())
}

pub fn Data_Number_acos(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().acos()) }
pub fn Data_Number_asin(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().asin()) }
pub fn Data_Number_atan(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().atan()) }
pub fn Data_Number_atan2(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().atan2(a1.unwrap_number())) }
pub fn Data_Number_cos(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().cos()) }
pub fn Data_Number_exp(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().exp()) }
pub fn Data_Number_infinity() -> crate::UnknownType { crate::mk_number(std::f64::INFINITY) }
pub fn Data_Number_isNaN(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_bool(a0.unwrap_number().is_nan()) }
pub fn Data_Number_log(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().ln()) }
pub fn Data_Number_max(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().max(a1.unwrap_number())) }
pub fn Data_Number_min(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().min(a1.unwrap_number())) }
pub fn Data_Number_nan() -> crate::UnknownType { crate::mk_number(std::f64::NAN) }
pub fn Data_Number_pow(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().powf(a1.unwrap_number())) }
pub fn Data_Number_remainder(mut a0: crate::UnknownType, mut a1: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number() % a1.unwrap_number()) }
pub fn Data_Number_sign(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().signum()) }
pub fn Data_Number_sin(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().sin()) }
pub fn Data_Number_sqrt(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().sqrt()) }
pub fn Data_Number_tan(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().tan()) }
pub fn Data_Number_trunc(mut a0: crate::UnknownType) -> crate::UnknownType { crate::mk_number(a0.unwrap_number().trunc()) }

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
                                    if let Ok(parsed) = str_c3.unwrap_string().parse::<f64>() {
                                        let num = crate::mk_number(parsed);
                                        let is_fin = isF_c2.unwrap_func()(num.clone());
                                        if is_fin.unwrap_bool() {
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
