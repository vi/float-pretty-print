use super::PrettyPrintFloat as P;
fn p(x:&'static str) -> f64 { x.parse().unwrap() }
fn _just_opening_brace_default_settings() {
}
#[test]
fn default_settings_1() {
    assert_eq!(format!("{}", P(p("906792980000000"))), "9.0679298e14");
    assert_eq!(format!("{}", P(p("6390900000000"))), "6.3909e12");
    assert_eq!(format!("{}", P(p("28897000"))), "2.8897e7");
    assert_eq!(format!("{}", P(p("700000"))), "7e5");
    assert_eq!(format!("{}", P(p("439620.1"))), "4.396201e5");
    assert_eq!(format!("{}", P(p("9559.407"))), "9559.407");
    assert_eq!(format!("{}", P(p("8022.2"))), "8022.2");
    assert_eq!(format!("{}", P(p("6738.111"))), "6738.111");
    assert_eq!(format!("{}", P(p("6208.24123131241232132142"))), "6208.2412313");
    assert_eq!(format!("{}", P(p("5400"))), "5400.0");
    assert_eq!(format!("{}", P(p("4741.878"))), "4741.878");
    assert_eq!(format!("{}", P(p("3620.1461"))), "3620.1461");
    assert_eq!(format!("{}", P(p("3000.23451"))), "3000.23451");
    assert_eq!(format!("{}", P(p("2175.65"))), "2175.65");
    assert_eq!(format!("{}", P(p("969.49"))), "969.49");
    assert_eq!(format!("{}", P(p("840.2056"))), "840.2056");
    assert_eq!(format!("{}", P(p("620"))), "620.0");
    assert_eq!(format!("{}", P(p("407"))), "407.0");
    assert_eq!(format!("{}", P(p("401.249"))), "401.249");
    assert_eq!(format!("{}", P(p("233.021"))), "233.021");
    assert_eq!(format!("{}", P(p("96.503326"))), "96.503326");
    assert_eq!(format!("{}", P(p("58.4"))), "58.4");
    assert_eq!(format!("{}", P(p("39.137"))), "39.137");
    assert_eq!(format!("{}", P(p("38.74"))), "38.74");
    assert_eq!(format!("{}", P(p("30"))), "30.0");
    assert_eq!(format!("{}", P(p("24.48179"))), "24.48179");
    assert_eq!(format!("{}", P(p("21.123"))), "21.123");
    assert_eq!(format!("{}", P(p("9.95016e+246"))), "9.95016e246");
    assert_eq!(format!("{}", P(p("9.8388"))), "9.8388");
    assert_eq!(format!("{}", P(p("9.8059e+35"))), "9.8059e35");
}
#[test]
fn default_settings_2() {
    assert_eq!(format!("{}", P(p("9.530609e+22"))), "9.530609e22");
    assert_eq!(format!("{}", P(p("9.46e+35"))), "9.46e35");
    assert_eq!(format!("{}", P(p("9.452105e-31"))), "9.452105e-31");
    assert_eq!(format!("{}", P(p("9e+115"))), "9e115");
    assert_eq!(format!("{}", P(p("8.785953e+42"))), "8.785953e42");
    assert_eq!(format!("{}", P(p("8.5e+20"))), "8.5e20");
    assert_eq!(format!("{}", P(p("8.3536e+30"))), "8.3536e30");
    assert_eq!(format!("{}", P(p("8.3439e+25"))), "8.3439e25");
    assert_eq!(format!("{}", P(p("8.27203e-18"))), "8.27203e-18");
    assert_eq!(format!("{}", P(p("8.271221e-219"))), "8.27122e-219");
    assert_eq!(format!("{}", P(p("8.0927985e-112"))), "8.0928e-112");
    assert_eq!(format!("{}", P(p("8.07e-53"))), "8.07e-53");
    assert_eq!(format!("{}", P(p("8.062352e+38"))), "8.062352e38");
    assert_eq!(format!("{}", P(p("8.0159e+36"))), "8.0159e36");
    assert_eq!(format!("{}", P(p("8e+29"))), "8e29");
    assert_eq!(format!("{}", P(p("8e-24"))), "8e-24");
    assert_eq!(format!("{}", P(p("7.9954287e-194"))), "7.99543e-194");
    assert_eq!(format!("{}", P(p("7.83472e-22"))), "7.83472e-22");
    assert_eq!(format!("{}", P(p("7.814"))), "7.814");
    assert_eq!(format!("{}", P(p("7.7715e+27"))), "7.7715e27");
    assert_eq!(format!("{}", P(p("7.509e+38"))), "7.509e38");
    assert_eq!(format!("{}", P(p("7.331e-31"))), "7.331e-31");
    assert_eq!(format!("{}", P(p("7.2401e+226"))), "7.2401e226");
    assert_eq!(format!("{}", P(p("7.2193e-18"))), "7.2193e-18");
    assert_eq!(format!("{}", P(p("7.2e+19"))), "7.2e19");
    assert_eq!(format!("{}", P(p("7.142849e-170"))), "7.14285e-170");
    assert_eq!(format!("{}", P(p("7.0676e-17"))), "7.0676e-17");
    assert_eq!(format!("{}", P(p("7e+39"))), "7e39");
    assert_eq!(format!("{}", P(p("7e+28"))), "7e28");
    assert_eq!(format!("{}", P(p("7"))), "7.0");
}
#[test]
fn default_settings_3() {
    assert_eq!(format!("{}", P(p("6.844e+113"))), "6.844e113");
    assert_eq!(format!("{}", P(p("6.7853e-21"))), "6.7853e-21");
    assert_eq!(format!("{}", P(p("6.75e-27"))), "6.75e-27");
    assert_eq!(format!("{}", P(p("6.56e+39"))), "6.56e39");
    assert_eq!(format!("{}", P(p("6.540688e-10"))), "6.540688e-10");
    assert_eq!(format!("{}", P(p("6.4e-08"))), "6.4e-8");
    assert_eq!(format!("{}", P(p("6.389785e-262"))), "6.38979e-262");
    assert_eq!(format!("{}", P(p("6.3508e+42"))), "6.3508e42");
    assert_eq!(format!("{}", P(p("6.3383e+47"))), "6.3383e47");
    assert_eq!(format!("{}", P(p("6.295e+53"))), "6.295e53");
    assert_eq!(format!("{}", P(p("6.049"))), "6.049");
    assert_eq!(format!("{}", P(p("6.04"))), "6.04");
    assert_eq!(format!("{}", P(p("6.01341e-20"))), "6.01341e-20");
    assert_eq!(format!("{}", P(p("5.98e+28"))), "5.98e28");
    assert_eq!(format!("{}", P(p("5.9e+43"))), "5.9e43");
    assert_eq!(format!("{}", P(p("5.865"))), "5.865");
    assert_eq!(format!("{}", P(p("5.7e-26"))), "5.7e-26");
    assert_eq!(format!("{}", P(p("5.695e-09"))), "5.695e-9");
    assert_eq!(format!("{}", P(p("5.61e-07"))), "5.61e-7");
    assert_eq!(format!("{}", P(p("5.55971e-235"))), "5.55971e-235");
    assert_eq!(format!("{}", P(p("5.4541311e-10"))), "5.454131e-10");
    assert_eq!(format!("{}", P(p("5.1415063e-11"))), "5.141506e-11");
    assert_eq!(format!("{}", P(p("5.13e+282"))), "5.13e282");
    assert_eq!(format!("{}", P(p("5.0369e+172"))), "5.0369e172");
    assert_eq!(format!("{}", P(p("5.003824e+151"))), "5.003824e151");
    assert_eq!(format!("{}", P(p("5e+263"))), "5e263");
    assert_eq!(format!("{}", P(p("5e+24"))), "5e24");
    assert_eq!(format!("{}", P(p("5e+19"))), "5e19");
    assert_eq!(format!("{}", P(p("5e+133"))), "5e133");
    assert_eq!(format!("{}", P(p("4.937e+40"))), "4.937e40");
}
#[test]
fn default_settings_4() {
    assert_eq!(format!("{}", P(p("4.9361647e-133"))), "4.93616e-133");
    assert_eq!(format!("{}", P(p("4.585e+48"))), "4.585e48");
    assert_eq!(format!("{}", P(p("4.49e+26"))), "4.49e26");
    assert_eq!(format!("{}", P(p("4.4177e-134"))), "4.4177e-134");
    assert_eq!(format!("{}", P(p("4.3e-245"))), "4.3e-245");
    assert_eq!(format!("{}", P(p("4.3e-16"))), "4.3e-16");
    assert_eq!(format!("{}", P(p("3.452077e-191"))), "3.45208e-191");
    assert_eq!(format!("{}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{}", P(p("3e+25"))), "3e25");
    assert_eq!(format!("{}", P(p("3e+149"))), "3e149");
    assert_eq!(format!("{}", P(p("2.93e+32"))), "2.93e32");
    assert_eq!(format!("{}", P(p("2.916861e+44"))), "2.916861e44");
    assert_eq!(format!("{}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{}", P(p("2.764e-83"))), "2.764e-83");
    assert_eq!(format!("{}", P(p("2.49279e+34"))), "2.49279e34");
    assert_eq!(format!("{}", P(p("2.413e-15"))), "2.413e-15");
    assert_eq!(format!("{}", P(p("2.2353e-129"))), "2.2353e-129");
    assert_eq!(format!("{}", P(p("2.166819e-23"))), "2.166819e-23");
    assert_eq!(format!("{}", P(p("2e+31"))), "2e31");
    assert_eq!(format!("{}", P(p("2e+116"))), "2e116");
    assert_eq!(format!("{}", P(p("1.8e+33"))), "1.8e33");
    assert_eq!(format!("{}", P(p("1.7e-13"))), "1.7e-13");
    assert_eq!(format!("{}", P(p("1.4580931e+62"))), "1.4580931e62");
    assert_eq!(format!("{}", P(p("1.44471e-07"))), "1.44471e-7");
    assert_eq!(format!("{}", P(p("1.393237e+46"))), "1.393237e46");
    assert_eq!(format!("{}", P(p("1.39e+295"))), "1.39e295");
    assert_eq!(format!("{}", P(p("1.375e+17"))), "1.375e17");
    assert_eq!(format!("{}", P(p("1.293e+27"))), "1.293e27");
    assert_eq!(format!("{}", P(p("1.2041e-21"))), "1.2041e-21");
    assert_eq!(format!("{}", P(p("1e+48"))), "1e48");
}
#[test]
fn default_settings_5() {
    assert_eq!(format!("{}", P(p("1"))), "1.0");
    assert_eq!(format!("{}", P(p("0.98"))), "0.98");
    assert_eq!(format!("{}", P(p("0.973"))), "0.973");
    assert_eq!(format!("{}", P(p("0.945138"))), "0.945138");
    assert_eq!(format!("{}", P(p("0.84"))), "0.84");
    assert_eq!(format!("{}", P(p("0.67"))), "0.67");
    assert_eq!(format!("{}", P(p("0.335"))), "0.335");
    assert_eq!(format!("{}", P(p("0.113"))), "0.113");
    assert_eq!(format!("{}", P(p("0.0983688"))), "0.0983688");
    assert_eq!(format!("{}", P(p("0.0906203"))), "0.0906203");
    assert_eq!(format!("{}", P(p("0.0869"))), "0.0869");
    assert_eq!(format!("{}", P(p("0.05"))), "0.05");
    assert_eq!(format!("{}", P(p("0.047255829"))), "0.047255829");
    assert_eq!(format!("{}", P(p("0.028"))), "0.028");
    assert_eq!(format!("{}", P(p("0.009146"))), "0.009146");
    assert_eq!(format!("{}", P(p("0.008581"))), "0.008581");
    assert_eq!(format!("{}", P(p("0.008166"))), "0.008166");
    assert_eq!(format!("{}", P(p("0.0081"))), "0.0081");
    assert_eq!(format!("{}", P(p("0.008001"))), "0.008001");
    assert_eq!(format!("{}", P(p("0.007889"))), "0.007889");
    assert_eq!(format!("{}", P(p("0.006703542"))), "0.006703542");
    assert_eq!(format!("{}", P(p("0.0039"))), "0.0039");
    assert_eq!(format!("{}", P(p("0.0030426"))), "0.0030426");
    assert_eq!(format!("{}", P(p("0.003"))), "0.003");
    assert_eq!(format!("{}", P(p("0.0029071"))), "0.0029071");
    assert_eq!(format!("{}", P(p("0.002191249"))), "0.002191249");
    assert_eq!(format!("{}", P(p("0.001910066"))), "0.001910066");
    assert_eq!(format!("{}", P(p("0.00092769"))), "9.2769e-4");
    assert_eq!(format!("{}", P(p("0.0004"))), "4e-4");
    assert_eq!(format!("{}", P(p("0.000383036"))), "3.83036e-4");
}
#[test]
fn default_settings_6() {
    assert_eq!(format!("{}", P(p("0.0003"))), "3e-4");
    assert_eq!(format!("{}", P(p("0.000188222"))), "1.88222e-4");
    assert_eq!(format!("{}", P(p("0.00016"))), "1.6e-4");
    assert_eq!(format!("{}", P(p("0.00014834"))), "1.4834e-4");
    assert_eq!(format!("{}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{}", P(p("-inf"))), "-inf");
    assert_eq!(format!("{}", P(p("inf"))), "inf");
    assert_eq!(format!("{}", P(p("0"))), "0.0");
    assert_eq!(format!("{}", P(p("-0.00023816"))), "-2.3816e-4");
    assert_eq!(format!("{}", P(p("-0.000274"))), "-2.74e-4");
    assert_eq!(format!("{}", P(p("-0.00031"))), "-3.1e-4");
    assert_eq!(format!("{}", P(p("-0.00038509"))), "-3.8509e-4");
    assert_eq!(format!("{}", P(p("-0.000552594"))), "-5.52594e-4");
    assert_eq!(format!("{}", P(p("-0.0006028"))), "-6.028e-4");
    assert_eq!(format!("{}", P(p("-0.000719834"))), "-7.19834e-4");
    assert_eq!(format!("{}", P(p("-0.0007820539"))), "-7.820539e-4");
    assert_eq!(format!("{}", P(p("-0.000869"))), "-8.69e-4");
    assert_eq!(format!("{}", P(p("-0.00089806422"))), "-8.980642e-4");
    assert_eq!(format!("{}", P(p("-0.0009172"))), "-9.172e-4");
    assert_eq!(format!("{}", P(p("-0.00095719"))), "-9.5719e-4");
    assert_eq!(format!("{}", P(p("-0.0060372"))), "-0.0060372");
    assert_eq!(format!("{}", P(p("-0.0064726046"))), "-0.006472605");
    assert_eq!(format!("{}", P(p("-0.0092191939"))), "-0.009219194");
    assert_eq!(format!("{}", P(p("-0.019099"))), "-0.019099");
    assert_eq!(format!("{}", P(p("-0.022925"))), "-0.022925");
    assert_eq!(format!("{}", P(p("-0.02826"))), "-0.02826");
    assert_eq!(format!("{}", P(p("-0.0546163"))), "-0.0546163");
    assert_eq!(format!("{}", P(p("-0.062"))), "-0.062");
    assert_eq!(format!("{}", P(p("-0.0764018"))), "-0.0764018");
    assert_eq!(format!("{}", P(p("-0.0929804"))), "-0.0929804");
}
#[test]
fn default_settings_7() {
    assert_eq!(format!("{}", P(p("-0.1689"))), "-0.1689");
    assert_eq!(format!("{}", P(p("-0.2"))), "-0.2");
    assert_eq!(format!("{}", P(p("-0.4946795"))), "-0.4946795");
    assert_eq!(format!("{}", P(p("-0.67014714"))), "-0.67014714");
    assert_eq!(format!("{}", P(p("-0.68"))), "-0.68");
    assert_eq!(format!("{}", P(p("-0.68815"))), "-0.68815");
    assert_eq!(format!("{}", P(p("-0.7562"))), "-0.7562");
    assert_eq!(format!("{}", P(p("-1e+29"))), "-1e29");
    assert_eq!(format!("{}", P(p("-1.064846e-26"))), "-1.06485e-26");
    assert_eq!(format!("{}", P(p("-1.51624e-234"))), "-1.5162e-234");
    assert_eq!(format!("{}", P(p("-1.6965898e-119"))), "-1.6966e-119");
    assert_eq!(format!("{}", P(p("-1.7695e-236"))), "-1.7695e-236");
    assert_eq!(format!("{}", P(p("-1.93885e+19"))), "-1.93885e19");
    assert_eq!(format!("{}", P(p("-2.31373e+31"))), "-2.31373e31");
    assert_eq!(format!("{}", P(p("-2.4527809e-306"))), "-2.4528e-306");
    assert_eq!(format!("{}", P(p("-2.6136e-09"))), "-2.6136e-9");
    assert_eq!(format!("{}", P(p("-2.691e-11"))), "-2.691e-11");
    assert_eq!(format!("{}", P(p("-2.819"))), "-2.819");
    assert_eq!(format!("{}", P(p("-2.8322"))), "-2.8322");
    assert_eq!(format!("{}", P(p("-3e+45"))), "-3e45");
    assert_eq!(format!("{}", P(p("-3e-05"))), "-3e-5");
    assert_eq!(format!("{}", P(p("-3.05e+25"))), "-3.05e25");
    assert_eq!(format!("{}", P(p("-3.108287e+278"))), "-3.10829e278");
    assert_eq!(format!("{}", P(p("-3.16584e+41"))), "-3.16584e41");
    assert_eq!(format!("{}", P(p("-3.17e-222"))), "-3.17e-222");
    assert_eq!(format!("{}", P(p("-3.2348135"))), "-3.2348135");
    assert_eq!(format!("{}", P(p("-3.465e+22"))), "-3.465e22");
    assert_eq!(format!("{}", P(p("-3.63e-135"))), "-3.63e-135");
    assert_eq!(format!("{}", P(p("-3.72e+45"))), "-3.72e45");
    assert_eq!(format!("{}", P(p("-3.9e+30"))), "-3.9e30");
}
#[test]
fn default_settings_8() {
    assert_eq!(format!("{}", P(p("-4.2"))), "-4.2");
    assert_eq!(format!("{}", P(p("-4.208329e-20"))), "-4.20833e-20");
    assert_eq!(format!("{}", P(p("-4.2888733"))), "-4.2888733");
    assert_eq!(format!("{}", P(p("-4.297e-08"))), "-4.297e-8");
    assert_eq!(format!("{}", P(p("-4.60469181924042e-321"))), "-4.6047e-321");
    assert_eq!(format!("{}", P(p("-4.7142e+36"))), "-4.7142e36");
    assert_eq!(format!("{}", P(p("-4.767"))), "-4.767");
    assert_eq!(format!("{}", P(p("-4.863526e-20"))), "-4.86353e-20");
    assert_eq!(format!("{}", P(p("-4.868e+33"))), "-4.868e33");
    assert_eq!(format!("{}", P(p("-4.96e+247"))), "-4.96e247");
    assert_eq!(format!("{}", P(p("-5e-29"))), "-5e-29");
    assert_eq!(format!("{}", P(p("-5.05e-218"))), "-5.05e-218");
    assert_eq!(format!("{}", P(p("-5.169414e+37"))), "-5.169414e37");
    assert_eq!(format!("{}", P(p("-5.20816e-06"))), "-5.20816e-6");
    assert_eq!(format!("{}", P(p("-5.263e+21"))), "-5.263e21");
    assert_eq!(format!("{}", P(p("-5.4239467e-31"))), "-5.42395e-31");
    assert_eq!(format!("{}", P(p("-5.44067e-263"))), "-5.4407e-263");
    assert_eq!(format!("{}", P(p("-5.543976e+32"))), "-5.543976e32");
    assert_eq!(format!("{}", P(p("-5.56122e-29"))), "-5.56122e-29");
    assert_eq!(format!("{}", P(p("-5.6e+35"))), "-5.6e35");
    assert_eq!(format!("{}", P(p("-5.65896e-290"))), "-5.659e-290");
    assert_eq!(format!("{}", P(p("-5.71654"))), "-5.71654");
    assert_eq!(format!("{}", P(p("-5.730186e+123"))), "-5.73019e123");
    assert_eq!(format!("{}", P(p("-5.798598e-21"))), "-5.7986e-21");
    assert_eq!(format!("{}", P(p("-5.81165e-212"))), "-5.8116e-212");
    assert_eq!(format!("{}", P(p("-6e-25"))), "-6e-25");
    assert_eq!(format!("{}", P(p("-6e-06"))), "-6e-6");
    assert_eq!(format!("{}", P(p("-6.07e-268"))), "-6.07e-268");
    assert_eq!(format!("{}", P(p("-6.074991"))), "-6.074991");
    assert_eq!(format!("{}", P(p("-6.1e+290"))), "-6.1e290");
}
#[test]
fn default_settings_9() {
    assert_eq!(format!("{}", P(p("-6.1206e-06"))), "-6.1206e-6");
    assert_eq!(format!("{}", P(p("-6.5986e+45"))), "-6.5986e45");
    assert_eq!(format!("{}", P(p("-6.785938e+26"))), "-6.785938e26");
    assert_eq!(format!("{}", P(p("-6.8e-30"))), "-6.8e-30");
    assert_eq!(format!("{}", P(p("-6.921e-22"))), "-6.921e-22");
    assert_eq!(format!("{}", P(p("-7e-57"))), "-7e-57");
    assert_eq!(format!("{}", P(p("-7.0915e+22"))), "-7.0915e22");
    assert_eq!(format!("{}", P(p("-7.15407055178125e-321"))), "-7.1541e-321");
    assert_eq!(format!("{}", P(p("-7.60733e+45"))), "-7.60733e45");
    assert_eq!(format!("{}", P(p("-7.79173e-05"))), "-7.79173e-5");
    assert_eq!(format!("{}", P(p("-8e-26"))), "-8e-26");
    assert_eq!(format!("{}", P(p("-8e+20"))), "-8e20");
    assert_eq!(format!("{}", P(p("-8.245e-145"))), "-8.245e-145");
    assert_eq!(format!("{}", P(p("-8.28043e+21"))), "-8.28043e21");
    assert_eq!(format!("{}", P(p("-8.2888989e-25"))), "-8.2889e-25");
    assert_eq!(format!("{}", P(p("-8.37662e-158"))), "-8.3766e-158");
    assert_eq!(format!("{}", P(p("-8.38e-30"))), "-8.38e-30");
    assert_eq!(format!("{}", P(p("-8.4e-23"))), "-8.4e-23");
    assert_eq!(format!("{}", P(p("-8.5e+25"))), "-8.5e25");
    assert_eq!(format!("{}", P(p("-8.8165e-23"))), "-8.8165e-23");
    assert_eq!(format!("{}", P(p("-8.821e-20"))), "-8.821e-20");
    assert_eq!(format!("{}", P(p("-8.8805774e+129"))), "-8.88058e129");
    assert_eq!(format!("{}", P(p("-8.91e+18"))), "-8.91e18");
    assert_eq!(format!("{}", P(p("-9e-19"))), "-9e-19");
    assert_eq!(format!("{}", P(p("-9.03e-286"))), "-9.03e-286");
    assert_eq!(format!("{}", P(p("-9.13"))), "-9.13");
    assert_eq!(format!("{}", P(p("-9.185594e-120"))), "-9.1856e-120");
    assert_eq!(format!("{}", P(p("-9.3e-70"))), "-9.3e-70");
    assert_eq!(format!("{}", P(p("-9.451309e-30"))), "-9.45131e-30");
    assert_eq!(format!("{}", P(p("-9.494134e+65"))), "-9.494134e65");
}
#[test]
fn default_settings_10() {
    assert_eq!(format!("{}", P(p("-9.54e-24"))), "-9.54e-24");
    assert_eq!(format!("{}", P(p("-9.57259e+240"))), "-9.57259e240");
    assert_eq!(format!("{}", P(p("-27.1783"))), "-27.1783");
    assert_eq!(format!("{}", P(p("-32.234"))), "-32.234");
    assert_eq!(format!("{}", P(p("-46.430046"))), "-46.430046");
    assert_eq!(format!("{}", P(p("-48.1"))), "-48.1");
    assert_eq!(format!("{}", P(p("-48.307"))), "-48.307");
    assert_eq!(format!("{}", P(p("-50"))), "-50.0");
    assert_eq!(format!("{}", P(p("-57.28"))), "-57.28");
    assert_eq!(format!("{}", P(p("-60.094"))), "-60.094");
    assert_eq!(format!("{}", P(p("-62.01612"))), "-62.01612");
    assert_eq!(format!("{}", P(p("-64.92"))), "-64.92");
    assert_eq!(format!("{}", P(p("-80.2234"))), "-80.2234");
    assert_eq!(format!("{}", P(p("-90"))), "-90.0");
    assert_eq!(format!("{}", P(p("-94.9"))), "-94.9");
    assert_eq!(format!("{}", P(p("-98.6783"))), "-98.6783");
    assert_eq!(format!("{}", P(p("-100"))), "-100.0");
    assert_eq!(format!("{}", P(p("-164.3687"))), "-164.3687");
    assert_eq!(format!("{}", P(p("-430.50597"))), "-430.50597");
    assert_eq!(format!("{}", P(p("-466.5085"))), "-466.5085");
    assert_eq!(format!("{}", P(p("-595.05"))), "-595.05");
    assert_eq!(format!("{}", P(p("-764.3112"))), "-764.3112");
    assert_eq!(format!("{}", P(p("-807.142"))), "-807.142");
    assert_eq!(format!("{}", P(p("-5469.471"))), "-5469.471");
    assert_eq!(format!("{}", P(p("-5651.6809"))), "-5651.6809");
    assert_eq!(format!("{}", P(p("-9706.81"))), "-9706.81");
    assert_eq!(format!("{}", P(p("-93500"))), "-93500.0");
    assert_eq!(format!("{}", P(p("-577000"))), "-5.77e5");
    assert_eq!(format!("{}", P(p("-835534"))), "-8.35534e5");
    assert_eq!(format!("{}", P(p("-6000000"))), "-6e6");
}
#[test]
fn default_settings_11() {
    assert_eq!(format!("{}", P(p("-900000000"))), "-9e8");
    assert_eq!(format!("{}", P(p("-279358770000"))), "-2.793588e11");
    assert_eq!(format!("{}", P(p("-50000000000000"))), "-5e13");
}
fn _just_opening_brace_one() {
}
#[test]
fn one_1() {
    assert_eq!(format!("{:1.1}", P(p("906792980000000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6390900000000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("28897000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("700000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("439620.1"))), "#");
    assert_eq!(format!("{:1.1}", P(p("9559.407"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8022.2"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6738.111"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6208.24123131241232132142"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5400"))), "#");
    assert_eq!(format!("{:1.1}", P(p("4741.878"))), "#");
    assert_eq!(format!("{:1.1}", P(p("3620.1461"))), "#");
    assert_eq!(format!("{:1.1}", P(p("3000.23451"))), "#");
    assert_eq!(format!("{:1.1}", P(p("2175.65"))), "#");
    assert_eq!(format!("{:1.1}", P(p("969.49"))), "#");
    assert_eq!(format!("{:1.1}", P(p("840.2056"))), "#");
    assert_eq!(format!("{:1.1}", P(p("620"))), "#");
    assert_eq!(format!("{:1.1}", P(p("407"))), "#");
    assert_eq!(format!("{:1.1}", P(p("401.249"))), "#");
    assert_eq!(format!("{:1.1}", P(p("233.021"))), "#");
    assert_eq!(format!("{:1.1}", P(p("96.503326"))), "#");
    assert_eq!(format!("{:1.1}", P(p("58.4"))), "#");
    assert_eq!(format!("{:1.1}", P(p("39.137"))), "#");
    assert_eq!(format!("{:1.1}", P(p("38.74"))), "#");
    assert_eq!(format!("{:1.1}", P(p("30"))), "#");
    assert_eq!(format!("{:1.1}", P(p("24.48179"))), "#");
    assert_eq!(format!("{:1.1}", P(p("21.123"))), "#");
    assert_eq!(format!("{:1.1}", P(p("9.95016e+246"))), "#");
    assert_eq!(format!("{:1.1}", P(p("9.8388"))), "#");
    assert_eq!(format!("{:1.1}", P(p("9.8059e+35"))), "#");
}
#[test]
fn one_2() {
    assert_eq!(format!("{:1.1}", P(p("9.530609e+22"))), "#");
    assert_eq!(format!("{:1.1}", P(p("9.46e+35"))), "#");
    assert_eq!(format!("{:1.1}", P(p("9.452105e-31"))), "0");
    assert_eq!(format!("{:1.1}", P(p("9e+115"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8.785953e+42"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8.5e+20"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8.3536e+30"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8.3439e+25"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8.27203e-18"))), "0");
    assert_eq!(format!("{:1.1}", P(p("8.271221e-219"))), "0");
    assert_eq!(format!("{:1.1}", P(p("8.0927985e-112"))), "0");
    assert_eq!(format!("{:1.1}", P(p("8.07e-53"))), "0");
    assert_eq!(format!("{:1.1}", P(p("8.062352e+38"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8.0159e+36"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8e+29"))), "#");
    assert_eq!(format!("{:1.1}", P(p("8e-24"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7.9954287e-194"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7.83472e-22"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7.814"))), "8");
    assert_eq!(format!("{:1.1}", P(p("7.7715e+27"))), "#");
    assert_eq!(format!("{:1.1}", P(p("7.509e+38"))), "#");
    assert_eq!(format!("{:1.1}", P(p("7.331e-31"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7.2401e+226"))), "#");
    assert_eq!(format!("{:1.1}", P(p("7.2193e-18"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7.2e+19"))), "#");
    assert_eq!(format!("{:1.1}", P(p("7.142849e-170"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7.0676e-17"))), "0");
    assert_eq!(format!("{:1.1}", P(p("7e+39"))), "#");
    assert_eq!(format!("{:1.1}", P(p("7e+28"))), "#");
    assert_eq!(format!("{:1.1}", P(p("7"))), "7");
}
#[test]
fn one_3() {
    assert_eq!(format!("{:1.1}", P(p("6.844e+113"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6.7853e-21"))), "0");
    assert_eq!(format!("{:1.1}", P(p("6.75e-27"))), "0");
    assert_eq!(format!("{:1.1}", P(p("6.56e+39"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6.540688e-10"))), "0");
    assert_eq!(format!("{:1.1}", P(p("6.4e-08"))), "0");
    assert_eq!(format!("{:1.1}", P(p("6.389785e-262"))), "0");
    assert_eq!(format!("{:1.1}", P(p("6.3508e+42"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6.3383e+47"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6.295e+53"))), "#");
    assert_eq!(format!("{:1.1}", P(p("6.049"))), "6");
    assert_eq!(format!("{:1.1}", P(p("6.04"))), "6");
    assert_eq!(format!("{:1.1}", P(p("6.01341e-20"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.98e+28"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5.9e+43"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5.865"))), "6");
    assert_eq!(format!("{:1.1}", P(p("5.7e-26"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.695e-09"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.61e-07"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.55971e-235"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.4541311e-10"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.1415063e-11"))), "0");
    assert_eq!(format!("{:1.1}", P(p("5.13e+282"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5.0369e+172"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5.003824e+151"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5e+263"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5e+24"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5e+19"))), "#");
    assert_eq!(format!("{:1.1}", P(p("5e+133"))), "#");
    assert_eq!(format!("{:1.1}", P(p("4.937e+40"))), "#");
}
#[test]
fn one_4() {
    assert_eq!(format!("{:1.1}", P(p("4.9361647e-133"))), "0");
    assert_eq!(format!("{:1.1}", P(p("4.585e+48"))), "#");
    assert_eq!(format!("{:1.1}", P(p("4.49e+26"))), "#");
    assert_eq!(format!("{:1.1}", P(p("4.4177e-134"))), "0");
    assert_eq!(format!("{:1.1}", P(p("4.3e-245"))), "0");
    assert_eq!(format!("{:1.1}", P(p("4.3e-16"))), "0");
    assert_eq!(format!("{:1.1}", P(p("3.452077e-191"))), "0");
    assert_eq!(format!("{:1.1}", P(p("3.2"))), "3");
    assert_eq!(format!("{:1.1}", P(p("3e+25"))), "#");
    assert_eq!(format!("{:1.1}", P(p("3e+149"))), "#");
    assert_eq!(format!("{:1.1}", P(p("2.93e+32"))), "#");
    assert_eq!(format!("{:1.1}", P(p("2.916861e+44"))), "#");
    assert_eq!(format!("{:1.1}", P(p("2.9"))), "3");
    assert_eq!(format!("{:1.1}", P(p("2.764e-83"))), "0");
    assert_eq!(format!("{:1.1}", P(p("2.49279e+34"))), "#");
    assert_eq!(format!("{:1.1}", P(p("2.413e-15"))), "0");
    assert_eq!(format!("{:1.1}", P(p("2.2353e-129"))), "0");
    assert_eq!(format!("{:1.1}", P(p("2.166819e-23"))), "0");
    assert_eq!(format!("{:1.1}", P(p("2e+31"))), "#");
    assert_eq!(format!("{:1.1}", P(p("2e+116"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.8e+33"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.7e-13"))), "0");
    assert_eq!(format!("{:1.1}", P(p("1.4580931e+62"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.44471e-07"))), "0");
    assert_eq!(format!("{:1.1}", P(p("1.393237e+46"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.39e+295"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.375e+17"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.293e+27"))), "#");
    assert_eq!(format!("{:1.1}", P(p("1.2041e-21"))), "0");
    assert_eq!(format!("{:1.1}", P(p("1e+48"))), "#");
}
#[test]
fn one_5() {
    assert_eq!(format!("{:1.1}", P(p("1"))), "1");
    assert_eq!(format!("{:1.1}", P(p("0.98"))), "1");
    assert_eq!(format!("{:1.1}", P(p("0.973"))), "1");
    assert_eq!(format!("{:1.1}", P(p("0.945138"))), "1");
    assert_eq!(format!("{:1.1}", P(p("0.84"))), "1");
    assert_eq!(format!("{:1.1}", P(p("0.67"))), "1");
    assert_eq!(format!("{:1.1}", P(p("0.335"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.113"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0983688"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0906203"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0869"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.05"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.047255829"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.028"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.009146"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.008581"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.008166"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0081"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.008001"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.007889"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.006703542"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0039"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0030426"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.003"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.0029071"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.002191249"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.001910066"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0.00092769"))), "0");
    assert_eq!(format!("{:1.1}", P(p("0.0004"))), "0");
    assert_eq!(format!("{:1.1}", P(p("0.000383036"))), "0");
}
#[test]
fn one_6() {
    assert_eq!(format!("{:1.1}", P(p("0.0003"))), "0");
    assert_eq!(format!("{:1.1}", P(p("0.000188222"))), "0");
    assert_eq!(format!("{:1.1}", P(p("0.00016"))), "0");
    assert_eq!(format!("{:1.1}", P(p("0.00014834"))), "0");
    assert_eq!(format!("{:1.1}", P(p("NaN"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-inf"))), "#");
    assert_eq!(format!("{:1.1}", P(p("inf"))), "#");
    assert_eq!(format!("{:1.1}", P(p("0"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.00023816"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.000274"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.00031"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.00038509"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.000552594"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.0006028"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.000719834"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.0007820539"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.000869"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.00089806422"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.0009172"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.00095719"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-0.0060372"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.0064726046"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.0092191939"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.019099"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.022925"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.02826"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.0546163"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.062"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.0764018"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.0929804"))), "#");
}
#[test]
fn one_7() {
    assert_eq!(format!("{:1.1}", P(p("-0.1689"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.2"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.4946795"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.67014714"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.68"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.68815"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-0.7562"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-1e+29"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-1.064846e-26"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-1.51624e-234"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-1.6965898e-119"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-1.7695e-236"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-1.93885e+19"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-2.31373e+31"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-2.4527809e-306"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-2.6136e-09"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-2.691e-11"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-2.819"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-2.8322"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3e+45"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3e-05"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-3.05e+25"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3.108287e+278"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3.16584e+41"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3.17e-222"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-3.2348135"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3.465e+22"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3.63e-135"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-3.72e+45"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-3.9e+30"))), "#");
}
#[test]
fn one_8() {
    assert_eq!(format!("{:1.1}", P(p("-4.2"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-4.208329e-20"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-4.2888733"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-4.297e-08"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-4.60469181924042e-321"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-4.7142e+36"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-4.767"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-4.863526e-20"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-4.868e+33"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-4.96e+247"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5e-29"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.05e-218"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.169414e+37"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5.20816e-06"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.263e+21"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5.4239467e-31"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.44067e-263"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.543976e+32"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5.56122e-29"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.6e+35"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5.65896e-290"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.71654"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5.730186e+123"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5.798598e-21"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-5.81165e-212"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-6e-25"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-6e-06"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-6.07e-268"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-6.074991"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-6.1e+290"))), "#");
}
#[test]
fn one_9() {
    assert_eq!(format!("{:1.1}", P(p("-6.1206e-06"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-6.5986e+45"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-6.785938e+26"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-6.8e-30"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-6.921e-22"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-7e-57"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-7.0915e+22"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-7.15407055178125e-321"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-7.60733e+45"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-7.79173e-05"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8e-26"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8e+20"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-8.245e-145"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.28043e+21"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-8.2888989e-25"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.37662e-158"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.38e-30"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.4e-23"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.5e+25"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-8.8165e-23"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.821e-20"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-8.8805774e+129"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-8.91e+18"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-9e-19"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-9.03e-286"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-9.13"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-9.185594e-120"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-9.3e-70"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-9.451309e-30"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-9.494134e+65"))), "#");
}
#[test]
fn one_10() {
    assert_eq!(format!("{:1.1}", P(p("-9.54e-24"))), "0");
    assert_eq!(format!("{:1.1}", P(p("-9.57259e+240"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-27.1783"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-32.234"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-46.430046"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-48.1"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-48.307"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-50"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-57.28"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-60.094"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-62.01612"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-64.92"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-80.2234"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-90"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-94.9"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-98.6783"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-100"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-164.3687"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-430.50597"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-466.5085"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-595.05"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-764.3112"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-807.142"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5469.471"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-5651.6809"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-9706.81"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-93500"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-577000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-835534"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-6000000"))), "#");
}
#[test]
fn one_11() {
    assert_eq!(format!("{:1.1}", P(p("-900000000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-279358770000"))), "#");
    assert_eq!(format!("{:1.1}", P(p("-50000000000000"))), "#");
}
fn _just_opening_brace_onetwo() {
}
#[test]
fn onetwo_1() {
    assert_eq!(format!("{:1.2}", P(p("906792980000000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6390900000000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("28897000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("700000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("439620.1"))), "#");
    assert_eq!(format!("{:1.2}", P(p("9559.407"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8022.2"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6738.111"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6208.24123131241232132142"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5400"))), "#");
    assert_eq!(format!("{:1.2}", P(p("4741.878"))), "#");
    assert_eq!(format!("{:1.2}", P(p("3620.1461"))), "#");
    assert_eq!(format!("{:1.2}", P(p("3000.23451"))), "#");
    assert_eq!(format!("{:1.2}", P(p("2175.65"))), "#");
    assert_eq!(format!("{:1.2}", P(p("969.49"))), "#");
    assert_eq!(format!("{:1.2}", P(p("840.2056"))), "#");
    assert_eq!(format!("{:1.2}", P(p("620"))), "#");
    assert_eq!(format!("{:1.2}", P(p("407"))), "#");
    assert_eq!(format!("{:1.2}", P(p("401.249"))), "#");
    assert_eq!(format!("{:1.2}", P(p("233.021"))), "#");
    assert_eq!(format!("{:1.2}", P(p("96.503326"))), "97");
    assert_eq!(format!("{:1.2}", P(p("58.4"))), "58");
    assert_eq!(format!("{:1.2}", P(p("39.137"))), "39");
    assert_eq!(format!("{:1.2}", P(p("38.74"))), "39");
    assert_eq!(format!("{:1.2}", P(p("30"))), "30");
    assert_eq!(format!("{:1.2}", P(p("24.48179"))), "24");
    assert_eq!(format!("{:1.2}", P(p("21.123"))), "21");
    assert_eq!(format!("{:1.2}", P(p("9.95016e+246"))), "#");
    assert_eq!(format!("{:1.2}", P(p("9.8388"))), "10");
    assert_eq!(format!("{:1.2}", P(p("9.8059e+35"))), "#");
}
#[test]
fn onetwo_2() {
    assert_eq!(format!("{:1.2}", P(p("9.530609e+22"))), "#");
    assert_eq!(format!("{:1.2}", P(p("9.46e+35"))), "#");
    assert_eq!(format!("{:1.2}", P(p("9.452105e-31"))), "0");
    assert_eq!(format!("{:1.2}", P(p("9e+115"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8.785953e+42"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8.5e+20"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8.3536e+30"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8.3439e+25"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8.27203e-18"))), "0");
    assert_eq!(format!("{:1.2}", P(p("8.271221e-219"))), "0");
    assert_eq!(format!("{:1.2}", P(p("8.0927985e-112"))), "0");
    assert_eq!(format!("{:1.2}", P(p("8.07e-53"))), "0");
    assert_eq!(format!("{:1.2}", P(p("8.062352e+38"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8.0159e+36"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8e+29"))), "#");
    assert_eq!(format!("{:1.2}", P(p("8e-24"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7.9954287e-194"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7.83472e-22"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7.814"))), "8");
    assert_eq!(format!("{:1.2}", P(p("7.7715e+27"))), "#");
    assert_eq!(format!("{:1.2}", P(p("7.509e+38"))), "#");
    assert_eq!(format!("{:1.2}", P(p("7.331e-31"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7.2401e+226"))), "#");
    assert_eq!(format!("{:1.2}", P(p("7.2193e-18"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7.2e+19"))), "#");
    assert_eq!(format!("{:1.2}", P(p("7.142849e-170"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7.0676e-17"))), "0");
    assert_eq!(format!("{:1.2}", P(p("7e+39"))), "#");
    assert_eq!(format!("{:1.2}", P(p("7e+28"))), "#");
    assert_eq!(format!("{:1.2}", P(p("7"))), "7");
}
#[test]
fn onetwo_3() {
    assert_eq!(format!("{:1.2}", P(p("6.844e+113"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6.7853e-21"))), "0");
    assert_eq!(format!("{:1.2}", P(p("6.75e-27"))), "0");
    assert_eq!(format!("{:1.2}", P(p("6.56e+39"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6.540688e-10"))), "0");
    assert_eq!(format!("{:1.2}", P(p("6.4e-08"))), "0");
    assert_eq!(format!("{:1.2}", P(p("6.389785e-262"))), "0");
    assert_eq!(format!("{:1.2}", P(p("6.3508e+42"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6.3383e+47"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6.295e+53"))), "#");
    assert_eq!(format!("{:1.2}", P(p("6.049"))), "6");
    assert_eq!(format!("{:1.2}", P(p("6.04"))), "6");
    assert_eq!(format!("{:1.2}", P(p("6.01341e-20"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.98e+28"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5.9e+43"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5.865"))), "6");
    assert_eq!(format!("{:1.2}", P(p("5.7e-26"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.695e-09"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.61e-07"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.55971e-235"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.4541311e-10"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.1415063e-11"))), "0");
    assert_eq!(format!("{:1.2}", P(p("5.13e+282"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5.0369e+172"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5.003824e+151"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5e+263"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5e+24"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5e+19"))), "#");
    assert_eq!(format!("{:1.2}", P(p("5e+133"))), "#");
    assert_eq!(format!("{:1.2}", P(p("4.937e+40"))), "#");
}
#[test]
fn onetwo_4() {
    assert_eq!(format!("{:1.2}", P(p("4.9361647e-133"))), "0");
    assert_eq!(format!("{:1.2}", P(p("4.585e+48"))), "#");
    assert_eq!(format!("{:1.2}", P(p("4.49e+26"))), "#");
    assert_eq!(format!("{:1.2}", P(p("4.4177e-134"))), "0");
    assert_eq!(format!("{:1.2}", P(p("4.3e-245"))), "0");
    assert_eq!(format!("{:1.2}", P(p("4.3e-16"))), "0");
    assert_eq!(format!("{:1.2}", P(p("3.452077e-191"))), "0");
    assert_eq!(format!("{:1.2}", P(p("3.2"))), "3");
    assert_eq!(format!("{:1.2}", P(p("3e+25"))), "#");
    assert_eq!(format!("{:1.2}", P(p("3e+149"))), "#");
    assert_eq!(format!("{:1.2}", P(p("2.93e+32"))), "#");
    assert_eq!(format!("{:1.2}", P(p("2.916861e+44"))), "#");
    assert_eq!(format!("{:1.2}", P(p("2.9"))), "3");
    assert_eq!(format!("{:1.2}", P(p("2.764e-83"))), "0");
    assert_eq!(format!("{:1.2}", P(p("2.49279e+34"))), "#");
    assert_eq!(format!("{:1.2}", P(p("2.413e-15"))), "0");
    assert_eq!(format!("{:1.2}", P(p("2.2353e-129"))), "0");
    assert_eq!(format!("{:1.2}", P(p("2.166819e-23"))), "0");
    assert_eq!(format!("{:1.2}", P(p("2e+31"))), "#");
    assert_eq!(format!("{:1.2}", P(p("2e+116"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.8e+33"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.7e-13"))), "0");
    assert_eq!(format!("{:1.2}", P(p("1.4580931e+62"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.44471e-07"))), "0");
    assert_eq!(format!("{:1.2}", P(p("1.393237e+46"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.39e+295"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.375e+17"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.293e+27"))), "#");
    assert_eq!(format!("{:1.2}", P(p("1.2041e-21"))), "0");
    assert_eq!(format!("{:1.2}", P(p("1e+48"))), "#");
}
#[test]
fn onetwo_5() {
    assert_eq!(format!("{:1.2}", P(p("1"))), "1");
    assert_eq!(format!("{:1.2}", P(p("0.98"))), "1");
    assert_eq!(format!("{:1.2}", P(p("0.973"))), "1");
    assert_eq!(format!("{:1.2}", P(p("0.945138"))), "1");
    assert_eq!(format!("{:1.2}", P(p("0.84"))), "1");
    assert_eq!(format!("{:1.2}", P(p("0.67"))), "1");
    assert_eq!(format!("{:1.2}", P(p("0.335"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.113"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0983688"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0906203"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0869"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.05"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.047255829"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.028"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.009146"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.008581"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.008166"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0081"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.008001"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.007889"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.006703542"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0039"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0030426"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.003"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.0029071"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.002191249"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.001910066"))), "#");
    assert_eq!(format!("{:1.2}", P(p("0.00092769"))), "0");
    assert_eq!(format!("{:1.2}", P(p("0.0004"))), "0");
    assert_eq!(format!("{:1.2}", P(p("0.000383036"))), "0");
}
#[test]
fn onetwo_6() {
    assert_eq!(format!("{:1.2}", P(p("0.0003"))), "0");
    assert_eq!(format!("{:1.2}", P(p("0.000188222"))), "0");
    assert_eq!(format!("{:1.2}", P(p("0.00016"))), "0");
    assert_eq!(format!("{:1.2}", P(p("0.00014834"))), "0");
    assert_eq!(format!("{:1.2}", P(p("NaN"))), "##");
    assert_eq!(format!("{:1.2}", P(p("-inf"))), "##");
    assert_eq!(format!("{:1.2}", P(p("inf"))), "##");
    assert_eq!(format!("{:1.2}", P(p("0"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.00023816"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.000274"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.00031"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.00038509"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.000552594"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.0006028"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.000719834"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.0007820539"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.000869"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.00089806422"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.0009172"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.00095719"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-0.0060372"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.0064726046"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.0092191939"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.019099"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.022925"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.02826"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.0546163"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.062"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.0764018"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.0929804"))), "-0");
}
#[test]
fn onetwo_7() {
    assert_eq!(format!("{:1.2}", P(p("-0.1689"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.2"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.4946795"))), "-0");
    assert_eq!(format!("{:1.2}", P(p("-0.67014714"))), "-1");
    assert_eq!(format!("{:1.2}", P(p("-0.68"))), "-1");
    assert_eq!(format!("{:1.2}", P(p("-0.68815"))), "-1");
    assert_eq!(format!("{:1.2}", P(p("-0.7562"))), "-1");
    assert_eq!(format!("{:1.2}", P(p("-1e+29"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-1.064846e-26"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-1.51624e-234"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-1.6965898e-119"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-1.7695e-236"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-1.93885e+19"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-2.31373e+31"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-2.4527809e-306"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-2.6136e-09"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-2.691e-11"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-2.819"))), "-3");
    assert_eq!(format!("{:1.2}", P(p("-2.8322"))), "-3");
    assert_eq!(format!("{:1.2}", P(p("-3e+45"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-3e-05"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-3.05e+25"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-3.108287e+278"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-3.16584e+41"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-3.17e-222"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-3.2348135"))), "-3");
    assert_eq!(format!("{:1.2}", P(p("-3.465e+22"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-3.63e-135"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-3.72e+45"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-3.9e+30"))), "#");
}
#[test]
fn onetwo_8() {
    assert_eq!(format!("{:1.2}", P(p("-4.2"))), "-4");
    assert_eq!(format!("{:1.2}", P(p("-4.208329e-20"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-4.2888733"))), "-4");
    assert_eq!(format!("{:1.2}", P(p("-4.297e-08"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-4.60469181924042e-321"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-4.7142e+36"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-4.767"))), "-5");
    assert_eq!(format!("{:1.2}", P(p("-4.863526e-20"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-4.868e+33"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-4.96e+247"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5e-29"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.05e-218"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.169414e+37"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5.20816e-06"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.263e+21"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5.4239467e-31"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.44067e-263"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.543976e+32"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5.56122e-29"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.6e+35"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5.65896e-290"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.71654"))), "-6");
    assert_eq!(format!("{:1.2}", P(p("-5.730186e+123"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5.798598e-21"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-5.81165e-212"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-6e-25"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-6e-06"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-6.07e-268"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-6.074991"))), "-6");
    assert_eq!(format!("{:1.2}", P(p("-6.1e+290"))), "#");
}
#[test]
fn onetwo_9() {
    assert_eq!(format!("{:1.2}", P(p("-6.1206e-06"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-6.5986e+45"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-6.785938e+26"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-6.8e-30"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-6.921e-22"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-7e-57"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-7.0915e+22"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-7.15407055178125e-321"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-7.60733e+45"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-7.79173e-05"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8e-26"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8e+20"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-8.245e-145"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.28043e+21"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-8.2888989e-25"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.37662e-158"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.38e-30"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.4e-23"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.5e+25"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-8.8165e-23"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.821e-20"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-8.8805774e+129"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-8.91e+18"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-9e-19"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-9.03e-286"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-9.13"))), "-9");
    assert_eq!(format!("{:1.2}", P(p("-9.185594e-120"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-9.3e-70"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-9.451309e-30"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-9.494134e+65"))), "#");
}
#[test]
fn onetwo_10() {
    assert_eq!(format!("{:1.2}", P(p("-9.54e-24"))), "0");
    assert_eq!(format!("{:1.2}", P(p("-9.57259e+240"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-27.1783"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-32.234"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-46.430046"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-48.1"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-48.307"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-50"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-57.28"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-60.094"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-62.01612"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-64.92"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-80.2234"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-90"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-94.9"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-98.6783"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-100"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-164.3687"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-430.50597"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-466.5085"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-595.05"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-764.3112"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-807.142"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5469.471"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-5651.6809"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-9706.81"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-93500"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-577000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-835534"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-6000000"))), "#");
}
#[test]
fn onetwo_11() {
    assert_eq!(format!("{:1.2}", P(p("-900000000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-279358770000"))), "#");
    assert_eq!(format!("{:1.2}", P(p("-50000000000000"))), "#");
}
fn _just_opening_brace_two() {
}
#[test]
fn two_1() {
    assert_eq!(format!("{:2.2}", P(p("906792980000000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6390900000000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("28897000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("700000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("439620.1"))), "##");
    assert_eq!(format!("{:2.2}", P(p("9559.407"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8022.2"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6738.111"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6208.24123131241232132142"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5400"))), "##");
    assert_eq!(format!("{:2.2}", P(p("4741.878"))), "##");
    assert_eq!(format!("{:2.2}", P(p("3620.1461"))), "##");
    assert_eq!(format!("{:2.2}", P(p("3000.23451"))), "##");
    assert_eq!(format!("{:2.2}", P(p("2175.65"))), "##");
    assert_eq!(format!("{:2.2}", P(p("969.49"))), "##");
    assert_eq!(format!("{:2.2}", P(p("840.2056"))), "##");
    assert_eq!(format!("{:2.2}", P(p("620"))), "##");
    assert_eq!(format!("{:2.2}", P(p("407"))), "##");
    assert_eq!(format!("{:2.2}", P(p("401.249"))), "##");
    assert_eq!(format!("{:2.2}", P(p("233.021"))), "##");
    assert_eq!(format!("{:2.2}", P(p("96.503326"))), "97");
    assert_eq!(format!("{:2.2}", P(p("58.4"))), "58");
    assert_eq!(format!("{:2.2}", P(p("39.137"))), "39");
    assert_eq!(format!("{:2.2}", P(p("38.74"))), "39");
    assert_eq!(format!("{:2.2}", P(p("30"))), "30");
    assert_eq!(format!("{:2.2}", P(p("24.48179"))), "24");
    assert_eq!(format!("{:2.2}", P(p("21.123"))), "21");
    assert_eq!(format!("{:2.2}", P(p("9.95016e+246"))), "##");
    assert_eq!(format!("{:2.2}", P(p("9.8388"))), "10");
    assert_eq!(format!("{:2.2}", P(p("9.8059e+35"))), "##");
}
#[test]
fn two_2() {
    assert_eq!(format!("{:2.2}", P(p("9.530609e+22"))), "##");
    assert_eq!(format!("{:2.2}", P(p("9.46e+35"))), "##");
    assert_eq!(format!("{:2.2}", P(p("9.452105e-31"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("9e+115"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8.785953e+42"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8.5e+20"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8.3536e+30"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8.3439e+25"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8.27203e-18"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("8.271221e-219"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("8.0927985e-112"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("8.07e-53"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("8.062352e+38"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8.0159e+36"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8e+29"))), "##");
    assert_eq!(format!("{:2.2}", P(p("8e-24"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7.9954287e-194"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7.83472e-22"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7.814"))), " 8");
    assert_eq!(format!("{:2.2}", P(p("7.7715e+27"))), "##");
    assert_eq!(format!("{:2.2}", P(p("7.509e+38"))), "##");
    assert_eq!(format!("{:2.2}", P(p("7.331e-31"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7.2401e+226"))), "##");
    assert_eq!(format!("{:2.2}", P(p("7.2193e-18"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7.2e+19"))), "##");
    assert_eq!(format!("{:2.2}", P(p("7.142849e-170"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7.0676e-17"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("7e+39"))), "##");
    assert_eq!(format!("{:2.2}", P(p("7e+28"))), "##");
    assert_eq!(format!("{:2.2}", P(p("7"))), " 7");
}
#[test]
fn two_3() {
    assert_eq!(format!("{:2.2}", P(p("6.844e+113"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6.7853e-21"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("6.75e-27"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("6.56e+39"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6.540688e-10"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("6.4e-08"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("6.389785e-262"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("6.3508e+42"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6.3383e+47"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6.295e+53"))), "##");
    assert_eq!(format!("{:2.2}", P(p("6.049"))), " 6");
    assert_eq!(format!("{:2.2}", P(p("6.04"))), " 6");
    assert_eq!(format!("{:2.2}", P(p("6.01341e-20"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.98e+28"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5.9e+43"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5.865"))), " 6");
    assert_eq!(format!("{:2.2}", P(p("5.7e-26"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.695e-09"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.61e-07"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.55971e-235"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.4541311e-10"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.1415063e-11"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("5.13e+282"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5.0369e+172"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5.003824e+151"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5e+263"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5e+24"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5e+19"))), "##");
    assert_eq!(format!("{:2.2}", P(p("5e+133"))), "##");
    assert_eq!(format!("{:2.2}", P(p("4.937e+40"))), "##");
}
#[test]
fn two_4() {
    assert_eq!(format!("{:2.2}", P(p("4.9361647e-133"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("4.585e+48"))), "##");
    assert_eq!(format!("{:2.2}", P(p("4.49e+26"))), "##");
    assert_eq!(format!("{:2.2}", P(p("4.4177e-134"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("4.3e-245"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("4.3e-16"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("3.452077e-191"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("3.2"))), " 3");
    assert_eq!(format!("{:2.2}", P(p("3e+25"))), "##");
    assert_eq!(format!("{:2.2}", P(p("3e+149"))), "##");
    assert_eq!(format!("{:2.2}", P(p("2.93e+32"))), "##");
    assert_eq!(format!("{:2.2}", P(p("2.916861e+44"))), "##");
    assert_eq!(format!("{:2.2}", P(p("2.9"))), " 3");
    assert_eq!(format!("{:2.2}", P(p("2.764e-83"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("2.49279e+34"))), "##");
    assert_eq!(format!("{:2.2}", P(p("2.413e-15"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("2.2353e-129"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("2.166819e-23"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("2e+31"))), "##");
    assert_eq!(format!("{:2.2}", P(p("2e+116"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.8e+33"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.7e-13"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("1.4580931e+62"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.44471e-07"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("1.393237e+46"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.39e+295"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.375e+17"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.293e+27"))), "##");
    assert_eq!(format!("{:2.2}", P(p("1.2041e-21"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("1e+48"))), "##");
}
#[test]
fn two_5() {
    assert_eq!(format!("{:2.2}", P(p("1"))), " 1");
    assert_eq!(format!("{:2.2}", P(p("0.98"))), " 1");
    assert_eq!(format!("{:2.2}", P(p("0.973"))), " 1");
    assert_eq!(format!("{:2.2}", P(p("0.945138"))), " 1");
    assert_eq!(format!("{:2.2}", P(p("0.84"))), " 1");
    assert_eq!(format!("{:2.2}", P(p("0.67"))), " 1");
    assert_eq!(format!("{:2.2}", P(p("0.335"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.113"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0983688"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0906203"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0869"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.05"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.047255829"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.028"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.009146"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.008581"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.008166"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0081"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.008001"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.007889"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.006703542"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0039"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0030426"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.003"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.0029071"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.002191249"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.001910066"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0.00092769"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("0.0004"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("0.000383036"))), " 0");
}
#[test]
fn two_6() {
    assert_eq!(format!("{:2.2}", P(p("0.0003"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("0.000188222"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("0.00016"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("0.00014834"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("NaN"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-inf"))), "##");
    assert_eq!(format!("{:2.2}", P(p("inf"))), "##");
    assert_eq!(format!("{:2.2}", P(p("0"))), "0 ");
    assert_eq!(format!("{:2.2}", P(p("-0.00023816"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.000274"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.00031"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.00038509"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.000552594"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.0006028"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.000719834"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.0007820539"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.000869"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.00089806422"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.0009172"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.00095719"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-0.0060372"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.0064726046"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.0092191939"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.019099"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.022925"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.02826"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.0546163"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.062"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.0764018"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.0929804"))), "-0");
}
#[test]
fn two_7() {
    assert_eq!(format!("{:2.2}", P(p("-0.1689"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.2"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.4946795"))), "-0");
    assert_eq!(format!("{:2.2}", P(p("-0.67014714"))), "-1");
    assert_eq!(format!("{:2.2}", P(p("-0.68"))), "-1");
    assert_eq!(format!("{:2.2}", P(p("-0.68815"))), "-1");
    assert_eq!(format!("{:2.2}", P(p("-0.7562"))), "-1");
    assert_eq!(format!("{:2.2}", P(p("-1e+29"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-1.064846e-26"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-1.51624e-234"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-1.6965898e-119"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-1.7695e-236"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-1.93885e+19"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-2.31373e+31"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-2.4527809e-306"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-2.6136e-09"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-2.691e-11"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-2.819"))), "-3");
    assert_eq!(format!("{:2.2}", P(p("-2.8322"))), "-3");
    assert_eq!(format!("{:2.2}", P(p("-3e+45"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-3e-05"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-3.05e+25"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-3.108287e+278"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-3.16584e+41"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-3.17e-222"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-3.2348135"))), "-3");
    assert_eq!(format!("{:2.2}", P(p("-3.465e+22"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-3.63e-135"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-3.72e+45"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-3.9e+30"))), "##");
}
#[test]
fn two_8() {
    assert_eq!(format!("{:2.2}", P(p("-4.2"))), "-4");
    assert_eq!(format!("{:2.2}", P(p("-4.208329e-20"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-4.2888733"))), "-4");
    assert_eq!(format!("{:2.2}", P(p("-4.297e-08"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-4.60469181924042e-321"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-4.7142e+36"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-4.767"))), "-5");
    assert_eq!(format!("{:2.2}", P(p("-4.863526e-20"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-4.868e+33"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-4.96e+247"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5e-29"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.05e-218"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.169414e+37"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5.20816e-06"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.263e+21"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5.4239467e-31"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.44067e-263"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.543976e+32"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5.56122e-29"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.6e+35"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5.65896e-290"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.71654"))), "-6");
    assert_eq!(format!("{:2.2}", P(p("-5.730186e+123"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5.798598e-21"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-5.81165e-212"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-6e-25"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-6e-06"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-6.07e-268"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-6.074991"))), "-6");
    assert_eq!(format!("{:2.2}", P(p("-6.1e+290"))), "##");
}
#[test]
fn two_9() {
    assert_eq!(format!("{:2.2}", P(p("-6.1206e-06"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-6.5986e+45"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-6.785938e+26"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-6.8e-30"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-6.921e-22"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-7e-57"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-7.0915e+22"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-7.15407055178125e-321"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-7.60733e+45"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-7.79173e-05"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8e-26"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8e+20"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-8.245e-145"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.28043e+21"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-8.2888989e-25"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.37662e-158"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.38e-30"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.4e-23"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.5e+25"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-8.8165e-23"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.821e-20"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-8.8805774e+129"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-8.91e+18"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-9e-19"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-9.03e-286"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-9.13"))), "-9");
    assert_eq!(format!("{:2.2}", P(p("-9.185594e-120"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-9.3e-70"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-9.451309e-30"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-9.494134e+65"))), "##");
}
#[test]
fn two_10() {
    assert_eq!(format!("{:2.2}", P(p("-9.54e-24"))), " 0");
    assert_eq!(format!("{:2.2}", P(p("-9.57259e+240"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-27.1783"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-32.234"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-46.430046"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-48.1"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-48.307"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-50"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-57.28"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-60.094"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-62.01612"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-64.92"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-80.2234"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-90"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-94.9"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-98.6783"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-100"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-164.3687"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-430.50597"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-466.5085"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-595.05"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-764.3112"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-807.142"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5469.471"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-5651.6809"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-9706.81"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-93500"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-577000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-835534"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-6000000"))), "##");
}
#[test]
fn two_11() {
    assert_eq!(format!("{:2.2}", P(p("-900000000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-279358770000"))), "##");
    assert_eq!(format!("{:2.2}", P(p("-50000000000000"))), "##");
}
fn _just_opening_brace_onethree() {
}
#[test]
fn onethree_1() {
    assert_eq!(format!("{:1.3}", P(p("906792980000000"))), "#");
    assert_eq!(format!("{:1.3}", P(p("6390900000000"))), "#");
    assert_eq!(format!("{:1.3}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:1.3}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:1.3}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:1.3}", P(p("9559.407"))), "1e4");
    assert_eq!(format!("{:1.3}", P(p("8022.2"))), "8e3");
    assert_eq!(format!("{:1.3}", P(p("6738.111"))), "7e3");
    assert_eq!(format!("{:1.3}", P(p("6208.24123131241232132142"))), "6e3");
    assert_eq!(format!("{:1.3}", P(p("5400"))), "5e3");
    assert_eq!(format!("{:1.3}", P(p("4741.878"))), "5e3");
    assert_eq!(format!("{:1.3}", P(p("3620.1461"))), "4e3");
    assert_eq!(format!("{:1.3}", P(p("3000.23451"))), "3e3");
    assert_eq!(format!("{:1.3}", P(p("2175.65"))), "2e3");
    assert_eq!(format!("{:1.3}", P(p("969.49"))), "969");
    assert_eq!(format!("{:1.3}", P(p("840.2056"))), "840");
    assert_eq!(format!("{:1.3}", P(p("620"))), "620");
    assert_eq!(format!("{:1.3}", P(p("407"))), "407");
    assert_eq!(format!("{:1.3}", P(p("401.249"))), "401");
    assert_eq!(format!("{:1.3}", P(p("233.021"))), "233");
    assert_eq!(format!("{:1.3}", P(p("96.503326"))), "97");
    assert_eq!(format!("{:1.3}", P(p("58.4"))), "58");
    assert_eq!(format!("{:1.3}", P(p("39.137"))), "39");
    assert_eq!(format!("{:1.3}", P(p("38.74"))), "39");
    assert_eq!(format!("{:1.3}", P(p("30"))), "30");
    assert_eq!(format!("{:1.3}", P(p("24.48179"))), "24");
    assert_eq!(format!("{:1.3}", P(p("21.123"))), "21");
    assert_eq!(format!("{:1.3}", P(p("9.95016e+246"))), "#");
    assert_eq!(format!("{:1.3}", P(p("9.8388"))), "10");
    assert_eq!(format!("{:1.3}", P(p("9.8059e+35"))), "#");
}
#[test]
fn onethree_2() {
    assert_eq!(format!("{:1.3}", P(p("9.530609e+22"))), "#");
    assert_eq!(format!("{:1.3}", P(p("9.46e+35"))), "#");
    assert_eq!(format!("{:1.3}", P(p("9.452105e-31"))), "0");
    assert_eq!(format!("{:1.3}", P(p("9e+115"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8.785953e+42"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8.5e+20"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8.3536e+30"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8.3439e+25"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8.27203e-18"))), "0");
    assert_eq!(format!("{:1.3}", P(p("8.271221e-219"))), "0");
    assert_eq!(format!("{:1.3}", P(p("8.0927985e-112"))), "0");
    assert_eq!(format!("{:1.3}", P(p("8.07e-53"))), "0");
    assert_eq!(format!("{:1.3}", P(p("8.062352e+38"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8.0159e+36"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8e+29"))), "#");
    assert_eq!(format!("{:1.3}", P(p("8e-24"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7.9954287e-194"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7.83472e-22"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7.814"))), "7.8");
    assert_eq!(format!("{:1.3}", P(p("7.7715e+27"))), "#");
    assert_eq!(format!("{:1.3}", P(p("7.509e+38"))), "#");
    assert_eq!(format!("{:1.3}", P(p("7.331e-31"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7.2401e+226"))), "#");
    assert_eq!(format!("{:1.3}", P(p("7.2193e-18"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7.2e+19"))), "#");
    assert_eq!(format!("{:1.3}", P(p("7.142849e-170"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7.0676e-17"))), "0");
    assert_eq!(format!("{:1.3}", P(p("7e+39"))), "#");
    assert_eq!(format!("{:1.3}", P(p("7e+28"))), "#");
    assert_eq!(format!("{:1.3}", P(p("7"))), "7.0");
}
#[test]
fn onethree_3() {
    assert_eq!(format!("{:1.3}", P(p("6.844e+113"))), "#");
    assert_eq!(format!("{:1.3}", P(p("6.7853e-21"))), "0");
    assert_eq!(format!("{:1.3}", P(p("6.75e-27"))), "0");
    assert_eq!(format!("{:1.3}", P(p("6.56e+39"))), "#");
    assert_eq!(format!("{:1.3}", P(p("6.540688e-10"))), "0");
    assert_eq!(format!("{:1.3}", P(p("6.4e-08"))), "0");
    assert_eq!(format!("{:1.3}", P(p("6.389785e-262"))), "0");
    assert_eq!(format!("{:1.3}", P(p("6.3508e+42"))), "#");
    assert_eq!(format!("{:1.3}", P(p("6.3383e+47"))), "#");
    assert_eq!(format!("{:1.3}", P(p("6.295e+53"))), "#");
    assert_eq!(format!("{:1.3}", P(p("6.049"))), "6.0");
    assert_eq!(format!("{:1.3}", P(p("6.04"))), "6.0");
    assert_eq!(format!("{:1.3}", P(p("6.01341e-20"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.98e+28"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5.9e+43"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5.865"))), "5.9");
    assert_eq!(format!("{:1.3}", P(p("5.7e-26"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.695e-09"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.61e-07"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.55971e-235"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.4541311e-10"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.1415063e-11"))), "0");
    assert_eq!(format!("{:1.3}", P(p("5.13e+282"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5.0369e+172"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5.003824e+151"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5e+263"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5e+24"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5e+19"))), "#");
    assert_eq!(format!("{:1.3}", P(p("5e+133"))), "#");
    assert_eq!(format!("{:1.3}", P(p("4.937e+40"))), "#");
}
#[test]
fn onethree_4() {
    assert_eq!(format!("{:1.3}", P(p("4.9361647e-133"))), "0");
    assert_eq!(format!("{:1.3}", P(p("4.585e+48"))), "#");
    assert_eq!(format!("{:1.3}", P(p("4.49e+26"))), "#");
    assert_eq!(format!("{:1.3}", P(p("4.4177e-134"))), "0");
    assert_eq!(format!("{:1.3}", P(p("4.3e-245"))), "0");
    assert_eq!(format!("{:1.3}", P(p("4.3e-16"))), "0");
    assert_eq!(format!("{:1.3}", P(p("3.452077e-191"))), "0");
    assert_eq!(format!("{:1.3}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{:1.3}", P(p("3e+25"))), "#");
    assert_eq!(format!("{:1.3}", P(p("3e+149"))), "#");
    assert_eq!(format!("{:1.3}", P(p("2.93e+32"))), "#");
    assert_eq!(format!("{:1.3}", P(p("2.916861e+44"))), "#");
    assert_eq!(format!("{:1.3}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{:1.3}", P(p("2.764e-83"))), "0");
    assert_eq!(format!("{:1.3}", P(p("2.49279e+34"))), "#");
    assert_eq!(format!("{:1.3}", P(p("2.413e-15"))), "0");
    assert_eq!(format!("{:1.3}", P(p("2.2353e-129"))), "0");
    assert_eq!(format!("{:1.3}", P(p("2.166819e-23"))), "0");
    assert_eq!(format!("{:1.3}", P(p("2e+31"))), "#");
    assert_eq!(format!("{:1.3}", P(p("2e+116"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.8e+33"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.7e-13"))), "0");
    assert_eq!(format!("{:1.3}", P(p("1.4580931e+62"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.44471e-07"))), "0");
    assert_eq!(format!("{:1.3}", P(p("1.393237e+46"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.39e+295"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.375e+17"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.293e+27"))), "#");
    assert_eq!(format!("{:1.3}", P(p("1.2041e-21"))), "0");
    assert_eq!(format!("{:1.3}", P(p("1e+48"))), "#");
}
#[test]
fn onethree_5() {
    assert_eq!(format!("{:1.3}", P(p("1"))), "1.0");
    assert_eq!(format!("{:1.3}", P(p("0.98"))), "1.0");
    assert_eq!(format!("{:1.3}", P(p("0.973"))), "1.0");
    assert_eq!(format!("{:1.3}", P(p("0.945138"))), "0.9");
    assert_eq!(format!("{:1.3}", P(p("0.84"))), "0.8");
    assert_eq!(format!("{:1.3}", P(p("0.67"))), "0.7");
    assert_eq!(format!("{:1.3}", P(p("0.335"))), "0.3");
    assert_eq!(format!("{:1.3}", P(p("0.113"))), "0.1");
    assert_eq!(format!("{:1.3}", P(p("0.0983688"))), "0.1");
    assert_eq!(format!("{:1.3}", P(p("0.0906203"))), "0.1");
    assert_eq!(format!("{:1.3}", P(p("0.0869"))), "0.1");
    assert_eq!(format!("{:1.3}", P(p("0.05"))), "0.1");
    assert_eq!(format!("{:1.3}", P(p("0.047255829"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.028"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.009146"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.008581"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.008166"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.0081"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.008001"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.007889"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.006703542"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.0039"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.0030426"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.003"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.0029071"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.002191249"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.001910066"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.00092769"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.0004"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.000383036"))), "0");
}
#[test]
fn onethree_6() {
    assert_eq!(format!("{:1.3}", P(p("0.0003"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.000188222"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.00016"))), "0");
    assert_eq!(format!("{:1.3}", P(p("0.00014834"))), "0");
    assert_eq!(format!("{:1.3}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{:1.3}", P(p("-inf"))), "###");
    assert_eq!(format!("{:1.3}", P(p("inf"))), "inf");
    assert_eq!(format!("{:1.3}", P(p("0"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.00023816"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.000274"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.00031"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.00038509"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.000552594"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.0006028"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.000719834"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.0007820539"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.000869"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.00089806422"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.0009172"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.00095719"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-0.0060372"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.0064726046"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.0092191939"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.019099"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.022925"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.02826"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.0546163"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.062"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.0764018"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.0929804"))), "-0");
}
#[test]
fn onethree_7() {
    assert_eq!(format!("{:1.3}", P(p("-0.1689"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.2"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.4946795"))), "-0");
    assert_eq!(format!("{:1.3}", P(p("-0.67014714"))), "-1");
    assert_eq!(format!("{:1.3}", P(p("-0.68"))), "-1");
    assert_eq!(format!("{:1.3}", P(p("-0.68815"))), "-1");
    assert_eq!(format!("{:1.3}", P(p("-0.7562"))), "-1");
    assert_eq!(format!("{:1.3}", P(p("-1e+29"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-1.064846e-26"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-1.51624e-234"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-1.6965898e-119"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-1.7695e-236"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-1.93885e+19"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-2.31373e+31"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-2.4527809e-306"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-2.6136e-09"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-2.691e-11"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-2.819"))), "-3");
    assert_eq!(format!("{:1.3}", P(p("-2.8322"))), "-3");
    assert_eq!(format!("{:1.3}", P(p("-3e+45"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-3e-05"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-3.05e+25"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-3.108287e+278"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-3.16584e+41"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-3.17e-222"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-3.2348135"))), "-3");
    assert_eq!(format!("{:1.3}", P(p("-3.465e+22"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-3.63e-135"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-3.72e+45"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-3.9e+30"))), "#");
}
#[test]
fn onethree_8() {
    assert_eq!(format!("{:1.3}", P(p("-4.2"))), "-4");
    assert_eq!(format!("{:1.3}", P(p("-4.208329e-20"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-4.2888733"))), "-4");
    assert_eq!(format!("{:1.3}", P(p("-4.297e-08"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-4.60469181924042e-321"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-4.7142e+36"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-4.767"))), "-5");
    assert_eq!(format!("{:1.3}", P(p("-4.863526e-20"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-4.868e+33"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-4.96e+247"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5e-29"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.05e-218"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.169414e+37"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5.20816e-06"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.263e+21"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5.4239467e-31"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.44067e-263"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.543976e+32"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5.56122e-29"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.6e+35"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5.65896e-290"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.71654"))), "-6");
    assert_eq!(format!("{:1.3}", P(p("-5.730186e+123"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5.798598e-21"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-5.81165e-212"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-6e-25"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-6e-06"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-6.07e-268"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-6.074991"))), "-6");
    assert_eq!(format!("{:1.3}", P(p("-6.1e+290"))), "#");
}
#[test]
fn onethree_9() {
    assert_eq!(format!("{:1.3}", P(p("-6.1206e-06"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-6.5986e+45"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-6.785938e+26"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-6.8e-30"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-6.921e-22"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-7e-57"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-7.0915e+22"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-7.15407055178125e-321"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-7.60733e+45"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-7.79173e-05"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8e-26"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8e+20"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-8.245e-145"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.28043e+21"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-8.2888989e-25"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.37662e-158"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.38e-30"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.4e-23"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.5e+25"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-8.8165e-23"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.821e-20"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-8.8805774e+129"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-8.91e+18"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-9e-19"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-9.03e-286"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-9.13"))), "-9");
    assert_eq!(format!("{:1.3}", P(p("-9.185594e-120"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-9.3e-70"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-9.451309e-30"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-9.494134e+65"))), "#");
}
#[test]
fn onethree_10() {
    assert_eq!(format!("{:1.3}", P(p("-9.54e-24"))), "0");
    assert_eq!(format!("{:1.3}", P(p("-9.57259e+240"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-27.1783"))), "-27");
    assert_eq!(format!("{:1.3}", P(p("-32.234"))), "-32");
    assert_eq!(format!("{:1.3}", P(p("-46.430046"))), "-46");
    assert_eq!(format!("{:1.3}", P(p("-48.1"))), "-48");
    assert_eq!(format!("{:1.3}", P(p("-48.307"))), "-48");
    assert_eq!(format!("{:1.3}", P(p("-50"))), "-50");
    assert_eq!(format!("{:1.3}", P(p("-57.28"))), "-57");
    assert_eq!(format!("{:1.3}", P(p("-60.094"))), "-60");
    assert_eq!(format!("{:1.3}", P(p("-62.01612"))), "-62");
    assert_eq!(format!("{:1.3}", P(p("-64.92"))), "-65");
    assert_eq!(format!("{:1.3}", P(p("-80.2234"))), "-80");
    assert_eq!(format!("{:1.3}", P(p("-90"))), "-90");
    assert_eq!(format!("{:1.3}", P(p("-94.9"))), "-95");
    assert_eq!(format!("{:1.3}", P(p("-98.6783"))), "-99");
    assert_eq!(format!("{:1.3}", P(p("-100"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-164.3687"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-430.50597"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-466.5085"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-595.05"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-764.3112"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-807.142"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5469.471"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-5651.6809"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-9706.81"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-93500"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-577000"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-835534"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-6000000"))), "#");
}
#[test]
fn onethree_11() {
    assert_eq!(format!("{:1.3}", P(p("-900000000"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-279358770000"))), "#");
    assert_eq!(format!("{:1.3}", P(p("-50000000000000"))), "#");
}
fn _just_opening_brace_twothree() {
}
#[test]
fn twothree_1() {
    assert_eq!(format!("{:2.3}", P(p("906792980000000"))), "##");
    assert_eq!(format!("{:2.3}", P(p("6390900000000"))), "##");
    assert_eq!(format!("{:2.3}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:2.3}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:2.3}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:2.3}", P(p("9559.407"))), "1e4");
    assert_eq!(format!("{:2.3}", P(p("8022.2"))), "8e3");
    assert_eq!(format!("{:2.3}", P(p("6738.111"))), "7e3");
    assert_eq!(format!("{:2.3}", P(p("6208.24123131241232132142"))), "6e3");
    assert_eq!(format!("{:2.3}", P(p("5400"))), "5e3");
    assert_eq!(format!("{:2.3}", P(p("4741.878"))), "5e3");
    assert_eq!(format!("{:2.3}", P(p("3620.1461"))), "4e3");
    assert_eq!(format!("{:2.3}", P(p("3000.23451"))), "3e3");
    assert_eq!(format!("{:2.3}", P(p("2175.65"))), "2e3");
    assert_eq!(format!("{:2.3}", P(p("969.49"))), "969");
    assert_eq!(format!("{:2.3}", P(p("840.2056"))), "840");
    assert_eq!(format!("{:2.3}", P(p("620"))), "620");
    assert_eq!(format!("{:2.3}", P(p("407"))), "407");
    assert_eq!(format!("{:2.3}", P(p("401.249"))), "401");
    assert_eq!(format!("{:2.3}", P(p("233.021"))), "233");
    assert_eq!(format!("{:2.3}", P(p("96.503326"))), "97");
    assert_eq!(format!("{:2.3}", P(p("58.4"))), "58");
    assert_eq!(format!("{:2.3}", P(p("39.137"))), "39");
    assert_eq!(format!("{:2.3}", P(p("38.74"))), "39");
    assert_eq!(format!("{:2.3}", P(p("30"))), "30");
    assert_eq!(format!("{:2.3}", P(p("24.48179"))), "24");
    assert_eq!(format!("{:2.3}", P(p("21.123"))), "21");
    assert_eq!(format!("{:2.3}", P(p("9.95016e+246"))), "##");
    assert_eq!(format!("{:2.3}", P(p("9.8388"))), "10");
    assert_eq!(format!("{:2.3}", P(p("9.8059e+35"))), "##");
}
#[test]
fn twothree_2() {
    assert_eq!(format!("{:2.3}", P(p("9.530609e+22"))), "##");
    assert_eq!(format!("{:2.3}", P(p("9.46e+35"))), "##");
    assert_eq!(format!("{:2.3}", P(p("9.452105e-31"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("9e+115"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8.785953e+42"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8.5e+20"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8.3536e+30"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8.3439e+25"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8.27203e-18"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("8.271221e-219"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("8.0927985e-112"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("8.07e-53"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("8.062352e+38"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8.0159e+36"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8e+29"))), "##");
    assert_eq!(format!("{:2.3}", P(p("8e-24"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7.9954287e-194"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7.83472e-22"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7.814"))), "7.8");
    assert_eq!(format!("{:2.3}", P(p("7.7715e+27"))), "##");
    assert_eq!(format!("{:2.3}", P(p("7.509e+38"))), "##");
    assert_eq!(format!("{:2.3}", P(p("7.331e-31"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7.2401e+226"))), "##");
    assert_eq!(format!("{:2.3}", P(p("7.2193e-18"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7.2e+19"))), "##");
    assert_eq!(format!("{:2.3}", P(p("7.142849e-170"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7.0676e-17"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("7e+39"))), "##");
    assert_eq!(format!("{:2.3}", P(p("7e+28"))), "##");
    assert_eq!(format!("{:2.3}", P(p("7"))), "7.0");
}
#[test]
fn twothree_3() {
    assert_eq!(format!("{:2.3}", P(p("6.844e+113"))), "##");
    assert_eq!(format!("{:2.3}", P(p("6.7853e-21"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("6.75e-27"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("6.56e+39"))), "##");
    assert_eq!(format!("{:2.3}", P(p("6.540688e-10"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("6.4e-08"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("6.389785e-262"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("6.3508e+42"))), "##");
    assert_eq!(format!("{:2.3}", P(p("6.3383e+47"))), "##");
    assert_eq!(format!("{:2.3}", P(p("6.295e+53"))), "##");
    assert_eq!(format!("{:2.3}", P(p("6.049"))), "6.0");
    assert_eq!(format!("{:2.3}", P(p("6.04"))), "6.0");
    assert_eq!(format!("{:2.3}", P(p("6.01341e-20"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.98e+28"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5.9e+43"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5.865"))), "5.9");
    assert_eq!(format!("{:2.3}", P(p("5.7e-26"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.695e-09"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.61e-07"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.55971e-235"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.4541311e-10"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.1415063e-11"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("5.13e+282"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5.0369e+172"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5.003824e+151"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5e+263"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5e+24"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5e+19"))), "##");
    assert_eq!(format!("{:2.3}", P(p("5e+133"))), "##");
    assert_eq!(format!("{:2.3}", P(p("4.937e+40"))), "##");
}
#[test]
fn twothree_4() {
    assert_eq!(format!("{:2.3}", P(p("4.9361647e-133"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("4.585e+48"))), "##");
    assert_eq!(format!("{:2.3}", P(p("4.49e+26"))), "##");
    assert_eq!(format!("{:2.3}", P(p("4.4177e-134"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("4.3e-245"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("4.3e-16"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("3.452077e-191"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{:2.3}", P(p("3e+25"))), "##");
    assert_eq!(format!("{:2.3}", P(p("3e+149"))), "##");
    assert_eq!(format!("{:2.3}", P(p("2.93e+32"))), "##");
    assert_eq!(format!("{:2.3}", P(p("2.916861e+44"))), "##");
    assert_eq!(format!("{:2.3}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{:2.3}", P(p("2.764e-83"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("2.49279e+34"))), "##");
    assert_eq!(format!("{:2.3}", P(p("2.413e-15"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("2.2353e-129"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("2.166819e-23"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("2e+31"))), "##");
    assert_eq!(format!("{:2.3}", P(p("2e+116"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.8e+33"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.7e-13"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("1.4580931e+62"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.44471e-07"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("1.393237e+46"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.39e+295"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.375e+17"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.293e+27"))), "##");
    assert_eq!(format!("{:2.3}", P(p("1.2041e-21"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("1e+48"))), "##");
}
#[test]
fn twothree_5() {
    assert_eq!(format!("{:2.3}", P(p("1"))), "1.0");
    assert_eq!(format!("{:2.3}", P(p("0.98"))), "1.0");
    assert_eq!(format!("{:2.3}", P(p("0.973"))), "1.0");
    assert_eq!(format!("{:2.3}", P(p("0.945138"))), "0.9");
    assert_eq!(format!("{:2.3}", P(p("0.84"))), "0.8");
    assert_eq!(format!("{:2.3}", P(p("0.67"))), "0.7");
    assert_eq!(format!("{:2.3}", P(p("0.335"))), "0.3");
    assert_eq!(format!("{:2.3}", P(p("0.113"))), "0.1");
    assert_eq!(format!("{:2.3}", P(p("0.0983688"))), "0.1");
    assert_eq!(format!("{:2.3}", P(p("0.0906203"))), "0.1");
    assert_eq!(format!("{:2.3}", P(p("0.0869"))), "0.1");
    assert_eq!(format!("{:2.3}", P(p("0.05"))), "0.1");
    assert_eq!(format!("{:2.3}", P(p("0.047255829"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.028"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.009146"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.008581"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.008166"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.0081"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.008001"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.007889"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.006703542"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.0039"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.0030426"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.003"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.0029071"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.002191249"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.001910066"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.00092769"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.0004"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.000383036"))), " 0");
}
#[test]
fn twothree_6() {
    assert_eq!(format!("{:2.3}", P(p("0.0003"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.000188222"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.00016"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("0.00014834"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{:2.3}", P(p("-inf"))), "###");
    assert_eq!(format!("{:2.3}", P(p("inf"))), "inf");
    assert_eq!(format!("{:2.3}", P(p("0"))), "0 ");
    assert_eq!(format!("{:2.3}", P(p("-0.00023816"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.000274"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.00031"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.00038509"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.000552594"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.0006028"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.000719834"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.0007820539"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.000869"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.00089806422"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.0009172"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.00095719"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-0.0060372"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.0064726046"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.0092191939"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.019099"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.022925"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.02826"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.0546163"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.062"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.0764018"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.0929804"))), "-0");
}
#[test]
fn twothree_7() {
    assert_eq!(format!("{:2.3}", P(p("-0.1689"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.2"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.4946795"))), "-0");
    assert_eq!(format!("{:2.3}", P(p("-0.67014714"))), "-1");
    assert_eq!(format!("{:2.3}", P(p("-0.68"))), "-1");
    assert_eq!(format!("{:2.3}", P(p("-0.68815"))), "-1");
    assert_eq!(format!("{:2.3}", P(p("-0.7562"))), "-1");
    assert_eq!(format!("{:2.3}", P(p("-1e+29"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-1.064846e-26"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-1.51624e-234"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-1.6965898e-119"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-1.7695e-236"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-1.93885e+19"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-2.31373e+31"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-2.4527809e-306"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-2.6136e-09"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-2.691e-11"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-2.819"))), "-3");
    assert_eq!(format!("{:2.3}", P(p("-2.8322"))), "-3");
    assert_eq!(format!("{:2.3}", P(p("-3e+45"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-3e-05"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-3.05e+25"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-3.108287e+278"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-3.16584e+41"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-3.17e-222"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-3.2348135"))), "-3");
    assert_eq!(format!("{:2.3}", P(p("-3.465e+22"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-3.63e-135"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-3.72e+45"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-3.9e+30"))), "##");
}
#[test]
fn twothree_8() {
    assert_eq!(format!("{:2.3}", P(p("-4.2"))), "-4");
    assert_eq!(format!("{:2.3}", P(p("-4.208329e-20"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-4.2888733"))), "-4");
    assert_eq!(format!("{:2.3}", P(p("-4.297e-08"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-4.60469181924042e-321"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-4.7142e+36"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-4.767"))), "-5");
    assert_eq!(format!("{:2.3}", P(p("-4.863526e-20"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-4.868e+33"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-4.96e+247"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5e-29"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.05e-218"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.169414e+37"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5.20816e-06"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.263e+21"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5.4239467e-31"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.44067e-263"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.543976e+32"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5.56122e-29"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.6e+35"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5.65896e-290"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.71654"))), "-6");
    assert_eq!(format!("{:2.3}", P(p("-5.730186e+123"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5.798598e-21"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-5.81165e-212"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-6e-25"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-6e-06"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-6.07e-268"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-6.074991"))), "-6");
    assert_eq!(format!("{:2.3}", P(p("-6.1e+290"))), "##");
}
#[test]
fn twothree_9() {
    assert_eq!(format!("{:2.3}", P(p("-6.1206e-06"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-6.5986e+45"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-6.785938e+26"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-6.8e-30"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-6.921e-22"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-7e-57"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-7.0915e+22"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-7.15407055178125e-321"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-7.60733e+45"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-7.79173e-05"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8e-26"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8e+20"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-8.245e-145"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.28043e+21"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-8.2888989e-25"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.37662e-158"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.38e-30"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.4e-23"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.5e+25"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-8.8165e-23"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.821e-20"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-8.8805774e+129"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-8.91e+18"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-9e-19"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-9.03e-286"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-9.13"))), "-9");
    assert_eq!(format!("{:2.3}", P(p("-9.185594e-120"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-9.3e-70"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-9.451309e-30"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-9.494134e+65"))), "##");
}
#[test]
fn twothree_10() {
    assert_eq!(format!("{:2.3}", P(p("-9.54e-24"))), " 0");
    assert_eq!(format!("{:2.3}", P(p("-9.57259e+240"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-27.1783"))), "-27");
    assert_eq!(format!("{:2.3}", P(p("-32.234"))), "-32");
    assert_eq!(format!("{:2.3}", P(p("-46.430046"))), "-46");
    assert_eq!(format!("{:2.3}", P(p("-48.1"))), "-48");
    assert_eq!(format!("{:2.3}", P(p("-48.307"))), "-48");
    assert_eq!(format!("{:2.3}", P(p("-50"))), "-50");
    assert_eq!(format!("{:2.3}", P(p("-57.28"))), "-57");
    assert_eq!(format!("{:2.3}", P(p("-60.094"))), "-60");
    assert_eq!(format!("{:2.3}", P(p("-62.01612"))), "-62");
    assert_eq!(format!("{:2.3}", P(p("-64.92"))), "-65");
    assert_eq!(format!("{:2.3}", P(p("-80.2234"))), "-80");
    assert_eq!(format!("{:2.3}", P(p("-90"))), "-90");
    assert_eq!(format!("{:2.3}", P(p("-94.9"))), "-95");
    assert_eq!(format!("{:2.3}", P(p("-98.6783"))), "-99");
    assert_eq!(format!("{:2.3}", P(p("-100"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-164.3687"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-430.50597"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-466.5085"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-595.05"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-764.3112"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-807.142"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5469.471"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-5651.6809"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-9706.81"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-93500"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-577000"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-835534"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-6000000"))), "##");
}
#[test]
fn twothree_11() {
    assert_eq!(format!("{:2.3}", P(p("-900000000"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-279358770000"))), "##");
    assert_eq!(format!("{:2.3}", P(p("-50000000000000"))), "##");
}
fn _just_opening_brace_three() {
}
#[test]
fn three_1() {
    assert_eq!(format!("{:3.3}", P(p("906792980000000"))), "###");
    assert_eq!(format!("{:3.3}", P(p("6390900000000"))), "###");
    assert_eq!(format!("{:3.3}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:3.3}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:3.3}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:3.3}", P(p("9559.407"))), "1e4");
    assert_eq!(format!("{:3.3}", P(p("8022.2"))), "8e3");
    assert_eq!(format!("{:3.3}", P(p("6738.111"))), "7e3");
    assert_eq!(format!("{:3.3}", P(p("6208.24123131241232132142"))), "6e3");
    assert_eq!(format!("{:3.3}", P(p("5400"))), "5e3");
    assert_eq!(format!("{:3.3}", P(p("4741.878"))), "5e3");
    assert_eq!(format!("{:3.3}", P(p("3620.1461"))), "4e3");
    assert_eq!(format!("{:3.3}", P(p("3000.23451"))), "3e3");
    assert_eq!(format!("{:3.3}", P(p("2175.65"))), "2e3");
    assert_eq!(format!("{:3.3}", P(p("969.49"))), "969");
    assert_eq!(format!("{:3.3}", P(p("840.2056"))), "840");
    assert_eq!(format!("{:3.3}", P(p("620"))), "620");
    assert_eq!(format!("{:3.3}", P(p("407"))), "407");
    assert_eq!(format!("{:3.3}", P(p("401.249"))), "401");
    assert_eq!(format!("{:3.3}", P(p("233.021"))), "233");
    assert_eq!(format!("{:3.3}", P(p("96.503326"))), " 97");
    assert_eq!(format!("{:3.3}", P(p("58.4"))), " 58");
    assert_eq!(format!("{:3.3}", P(p("39.137"))), " 39");
    assert_eq!(format!("{:3.3}", P(p("38.74"))), " 39");
    assert_eq!(format!("{:3.3}", P(p("30"))), " 30");
    assert_eq!(format!("{:3.3}", P(p("24.48179"))), " 24");
    assert_eq!(format!("{:3.3}", P(p("21.123"))), " 21");
    assert_eq!(format!("{:3.3}", P(p("9.95016e+246"))), "###");
    assert_eq!(format!("{:3.3}", P(p("9.8388"))), " 10");
    assert_eq!(format!("{:3.3}", P(p("9.8059e+35"))), "###");
}
#[test]
fn three_2() {
    assert_eq!(format!("{:3.3}", P(p("9.530609e+22"))), "###");
    assert_eq!(format!("{:3.3}", P(p("9.46e+35"))), "###");
    assert_eq!(format!("{:3.3}", P(p("9.452105e-31"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("9e+115"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8.785953e+42"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8.5e+20"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8.3536e+30"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8.3439e+25"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8.27203e-18"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("8.271221e-219"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("8.0927985e-112"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("8.07e-53"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("8.062352e+38"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8.0159e+36"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8e+29"))), "###");
    assert_eq!(format!("{:3.3}", P(p("8e-24"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7.9954287e-194"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7.83472e-22"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7.814"))), "7.8");
    assert_eq!(format!("{:3.3}", P(p("7.7715e+27"))), "###");
    assert_eq!(format!("{:3.3}", P(p("7.509e+38"))), "###");
    assert_eq!(format!("{:3.3}", P(p("7.331e-31"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7.2401e+226"))), "###");
    assert_eq!(format!("{:3.3}", P(p("7.2193e-18"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7.2e+19"))), "###");
    assert_eq!(format!("{:3.3}", P(p("7.142849e-170"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7.0676e-17"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("7e+39"))), "###");
    assert_eq!(format!("{:3.3}", P(p("7e+28"))), "###");
    assert_eq!(format!("{:3.3}", P(p("7"))), "7.0");
}
#[test]
fn three_3() {
    assert_eq!(format!("{:3.3}", P(p("6.844e+113"))), "###");
    assert_eq!(format!("{:3.3}", P(p("6.7853e-21"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("6.75e-27"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("6.56e+39"))), "###");
    assert_eq!(format!("{:3.3}", P(p("6.540688e-10"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("6.4e-08"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("6.389785e-262"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("6.3508e+42"))), "###");
    assert_eq!(format!("{:3.3}", P(p("6.3383e+47"))), "###");
    assert_eq!(format!("{:3.3}", P(p("6.295e+53"))), "###");
    assert_eq!(format!("{:3.3}", P(p("6.049"))), "6.0");
    assert_eq!(format!("{:3.3}", P(p("6.04"))), "6.0");
    assert_eq!(format!("{:3.3}", P(p("6.01341e-20"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.98e+28"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5.9e+43"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5.865"))), "5.9");
    assert_eq!(format!("{:3.3}", P(p("5.7e-26"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.695e-09"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.61e-07"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.55971e-235"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.4541311e-10"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.1415063e-11"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("5.13e+282"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5.0369e+172"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5.003824e+151"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5e+263"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5e+24"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5e+19"))), "###");
    assert_eq!(format!("{:3.3}", P(p("5e+133"))), "###");
    assert_eq!(format!("{:3.3}", P(p("4.937e+40"))), "###");
}
#[test]
fn three_4() {
    assert_eq!(format!("{:3.3}", P(p("4.9361647e-133"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("4.585e+48"))), "###");
    assert_eq!(format!("{:3.3}", P(p("4.49e+26"))), "###");
    assert_eq!(format!("{:3.3}", P(p("4.4177e-134"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("4.3e-245"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("4.3e-16"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("3.452077e-191"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{:3.3}", P(p("3e+25"))), "###");
    assert_eq!(format!("{:3.3}", P(p("3e+149"))), "###");
    assert_eq!(format!("{:3.3}", P(p("2.93e+32"))), "###");
    assert_eq!(format!("{:3.3}", P(p("2.916861e+44"))), "###");
    assert_eq!(format!("{:3.3}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{:3.3}", P(p("2.764e-83"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("2.49279e+34"))), "###");
    assert_eq!(format!("{:3.3}", P(p("2.413e-15"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("2.2353e-129"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("2.166819e-23"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("2e+31"))), "###");
    assert_eq!(format!("{:3.3}", P(p("2e+116"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.8e+33"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.7e-13"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("1.4580931e+62"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.44471e-07"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("1.393237e+46"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.39e+295"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.375e+17"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.293e+27"))), "###");
    assert_eq!(format!("{:3.3}", P(p("1.2041e-21"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("1e+48"))), "###");
}
#[test]
fn three_5() {
    assert_eq!(format!("{:3.3}", P(p("1"))), "1.0");
    assert_eq!(format!("{:3.3}", P(p("0.98"))), "1.0");
    assert_eq!(format!("{:3.3}", P(p("0.973"))), "1.0");
    assert_eq!(format!("{:3.3}", P(p("0.945138"))), "0.9");
    assert_eq!(format!("{:3.3}", P(p("0.84"))), "0.8");
    assert_eq!(format!("{:3.3}", P(p("0.67"))), "0.7");
    assert_eq!(format!("{:3.3}", P(p("0.335"))), "0.3");
    assert_eq!(format!("{:3.3}", P(p("0.113"))), "0.1");
    assert_eq!(format!("{:3.3}", P(p("0.0983688"))), "0.1");
    assert_eq!(format!("{:3.3}", P(p("0.0906203"))), "0.1");
    assert_eq!(format!("{:3.3}", P(p("0.0869"))), "0.1");
    assert_eq!(format!("{:3.3}", P(p("0.05"))), "0.1");
    assert_eq!(format!("{:3.3}", P(p("0.047255829"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.028"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.009146"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.008581"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.008166"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.0081"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.008001"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.007889"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.006703542"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.0039"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.0030426"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.003"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.0029071"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.002191249"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.001910066"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.00092769"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.0004"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.000383036"))), "  0");
}
#[test]
fn three_6() {
    assert_eq!(format!("{:3.3}", P(p("0.0003"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.000188222"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.00016"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("0.00014834"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{:3.3}", P(p("-inf"))), "###");
    assert_eq!(format!("{:3.3}", P(p("inf"))), "inf");
    assert_eq!(format!("{:3.3}", P(p("0"))), "0.0");
    assert_eq!(format!("{:3.3}", P(p("-0.00023816"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.000274"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.00031"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.00038509"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.000552594"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.0006028"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.000719834"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.0007820539"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.000869"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.00089806422"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.0009172"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.00095719"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-0.0060372"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.0064726046"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.0092191939"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.019099"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.022925"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.02826"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.0546163"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.062"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.0764018"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.0929804"))), " -0");
}
#[test]
fn three_7() {
    assert_eq!(format!("{:3.3}", P(p("-0.1689"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.2"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.4946795"))), " -0");
    assert_eq!(format!("{:3.3}", P(p("-0.67014714"))), " -1");
    assert_eq!(format!("{:3.3}", P(p("-0.68"))), " -1");
    assert_eq!(format!("{:3.3}", P(p("-0.68815"))), " -1");
    assert_eq!(format!("{:3.3}", P(p("-0.7562"))), " -1");
    assert_eq!(format!("{:3.3}", P(p("-1e+29"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-1.064846e-26"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-1.51624e-234"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-1.6965898e-119"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-1.7695e-236"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-1.93885e+19"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-2.31373e+31"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-2.4527809e-306"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-2.6136e-09"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-2.691e-11"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-2.819"))), " -3");
    assert_eq!(format!("{:3.3}", P(p("-2.8322"))), " -3");
    assert_eq!(format!("{:3.3}", P(p("-3e+45"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-3e-05"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-3.05e+25"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-3.108287e+278"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-3.16584e+41"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-3.17e-222"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-3.2348135"))), " -3");
    assert_eq!(format!("{:3.3}", P(p("-3.465e+22"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-3.63e-135"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-3.72e+45"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-3.9e+30"))), "###");
}
#[test]
fn three_8() {
    assert_eq!(format!("{:3.3}", P(p("-4.2"))), " -4");
    assert_eq!(format!("{:3.3}", P(p("-4.208329e-20"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-4.2888733"))), " -4");
    assert_eq!(format!("{:3.3}", P(p("-4.297e-08"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-4.60469181924042e-321"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-4.7142e+36"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-4.767"))), " -5");
    assert_eq!(format!("{:3.3}", P(p("-4.863526e-20"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-4.868e+33"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-4.96e+247"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5e-29"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.05e-218"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.169414e+37"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5.20816e-06"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.263e+21"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5.4239467e-31"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.44067e-263"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.543976e+32"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5.56122e-29"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.6e+35"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5.65896e-290"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.71654"))), " -6");
    assert_eq!(format!("{:3.3}", P(p("-5.730186e+123"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5.798598e-21"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-5.81165e-212"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-6e-25"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-6e-06"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-6.07e-268"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-6.074991"))), " -6");
    assert_eq!(format!("{:3.3}", P(p("-6.1e+290"))), "###");
}
#[test]
fn three_9() {
    assert_eq!(format!("{:3.3}", P(p("-6.1206e-06"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-6.5986e+45"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-6.785938e+26"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-6.8e-30"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-6.921e-22"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-7e-57"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-7.0915e+22"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-7.15407055178125e-321"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-7.60733e+45"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-7.79173e-05"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8e-26"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8e+20"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-8.245e-145"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.28043e+21"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-8.2888989e-25"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.37662e-158"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.38e-30"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.4e-23"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.5e+25"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-8.8165e-23"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.821e-20"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-8.8805774e+129"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-8.91e+18"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-9e-19"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-9.03e-286"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-9.13"))), " -9");
    assert_eq!(format!("{:3.3}", P(p("-9.185594e-120"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-9.3e-70"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-9.451309e-30"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-9.494134e+65"))), "###");
}
#[test]
fn three_10() {
    assert_eq!(format!("{:3.3}", P(p("-9.54e-24"))), "  0");
    assert_eq!(format!("{:3.3}", P(p("-9.57259e+240"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-27.1783"))), "-27");
    assert_eq!(format!("{:3.3}", P(p("-32.234"))), "-32");
    assert_eq!(format!("{:3.3}", P(p("-46.430046"))), "-46");
    assert_eq!(format!("{:3.3}", P(p("-48.1"))), "-48");
    assert_eq!(format!("{:3.3}", P(p("-48.307"))), "-48");
    assert_eq!(format!("{:3.3}", P(p("-50"))), "-50");
    assert_eq!(format!("{:3.3}", P(p("-57.28"))), "-57");
    assert_eq!(format!("{:3.3}", P(p("-60.094"))), "-60");
    assert_eq!(format!("{:3.3}", P(p("-62.01612"))), "-62");
    assert_eq!(format!("{:3.3}", P(p("-64.92"))), "-65");
    assert_eq!(format!("{:3.3}", P(p("-80.2234"))), "-80");
    assert_eq!(format!("{:3.3}", P(p("-90"))), "-90");
    assert_eq!(format!("{:3.3}", P(p("-94.9"))), "-95");
    assert_eq!(format!("{:3.3}", P(p("-98.6783"))), "-99");
    assert_eq!(format!("{:3.3}", P(p("-100"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-164.3687"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-430.50597"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-466.5085"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-595.05"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-764.3112"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-807.142"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5469.471"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-5651.6809"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-9706.81"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-93500"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-577000"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-835534"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-6000000"))), "###");
}
#[test]
fn three_11() {
    assert_eq!(format!("{:3.3}", P(p("-900000000"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-279358770000"))), "###");
    assert_eq!(format!("{:3.3}", P(p("-50000000000000"))), "###");
}
fn _just_opening_brace_onefour() {
}
#[test]
fn onefour_1() {
    assert_eq!(format!("{:1.4}", P(p("906792980000000"))), "9e14");
    assert_eq!(format!("{:1.4}", P(p("6390900000000"))), "6e12");
    assert_eq!(format!("{:1.4}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:1.4}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:1.4}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:1.4}", P(p("9559.407"))), "9559");
    assert_eq!(format!("{:1.4}", P(p("8022.2"))), "8022");
    assert_eq!(format!("{:1.4}", P(p("6738.111"))), "6738");
    assert_eq!(format!("{:1.4}", P(p("6208.24123131241232132142"))), "6208");
    assert_eq!(format!("{:1.4}", P(p("5400"))), "5400");
    assert_eq!(format!("{:1.4}", P(p("4741.878"))), "4742");
    assert_eq!(format!("{:1.4}", P(p("3620.1461"))), "3620");
    assert_eq!(format!("{:1.4}", P(p("3000.23451"))), "3000");
    assert_eq!(format!("{:1.4}", P(p("2175.65"))), "2176");
    assert_eq!(format!("{:1.4}", P(p("969.49"))), "969");
    assert_eq!(format!("{:1.4}", P(p("840.2056"))), "840");
    assert_eq!(format!("{:1.4}", P(p("620"))), "620");
    assert_eq!(format!("{:1.4}", P(p("407"))), "407");
    assert_eq!(format!("{:1.4}", P(p("401.249"))), "401");
    assert_eq!(format!("{:1.4}", P(p("233.021"))), "233");
    assert_eq!(format!("{:1.4}", P(p("96.503326"))), "96.5");
    assert_eq!(format!("{:1.4}", P(p("58.4"))), "58.4");
    assert_eq!(format!("{:1.4}", P(p("39.137"))), "39.1");
    assert_eq!(format!("{:1.4}", P(p("38.74"))), "38.7");
    assert_eq!(format!("{:1.4}", P(p("30"))), "30.0");
    assert_eq!(format!("{:1.4}", P(p("24.48179"))), "24.5");
    assert_eq!(format!("{:1.4}", P(p("21.123"))), "21.1");
    assert_eq!(format!("{:1.4}", P(p("9.95016e+246"))), "#");
    assert_eq!(format!("{:1.4}", P(p("9.8388"))), "9.8");
    assert_eq!(format!("{:1.4}", P(p("9.8059e+35"))), "1e36");
}
#[test]
fn onefour_2() {
    assert_eq!(format!("{:1.4}", P(p("9.530609e+22"))), "1e23");
    assert_eq!(format!("{:1.4}", P(p("9.46e+35"))), "9e35");
    assert_eq!(format!("{:1.4}", P(p("9.452105e-31"))), "0");
    assert_eq!(format!("{:1.4}", P(p("9e+115"))), "#");
    assert_eq!(format!("{:1.4}", P(p("8.785953e+42"))), "9e42");
    assert_eq!(format!("{:1.4}", P(p("8.5e+20"))), "8e20");
    assert_eq!(format!("{:1.4}", P(p("8.3536e+30"))), "8e30");
    assert_eq!(format!("{:1.4}", P(p("8.3439e+25"))), "8e25");
    assert_eq!(format!("{:1.4}", P(p("8.27203e-18"))), "0");
    assert_eq!(format!("{:1.4}", P(p("8.271221e-219"))), "0");
    assert_eq!(format!("{:1.4}", P(p("8.0927985e-112"))), "0");
    assert_eq!(format!("{:1.4}", P(p("8.07e-53"))), "0");
    assert_eq!(format!("{:1.4}", P(p("8.062352e+38"))), "8e38");
    assert_eq!(format!("{:1.4}", P(p("8.0159e+36"))), "8e36");
    assert_eq!(format!("{:1.4}", P(p("8e+29"))), "8e29");
    assert_eq!(format!("{:1.4}", P(p("8e-24"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7.9954287e-194"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7.83472e-22"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7.814"))), "7.81");
    assert_eq!(format!("{:1.4}", P(p("7.7715e+27"))), "8e27");
    assert_eq!(format!("{:1.4}", P(p("7.509e+38"))), "8e38");
    assert_eq!(format!("{:1.4}", P(p("7.331e-31"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7.2401e+226"))), "#");
    assert_eq!(format!("{:1.4}", P(p("7.2193e-18"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7.2e+19"))), "7e19");
    assert_eq!(format!("{:1.4}", P(p("7.142849e-170"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7.0676e-17"))), "0");
    assert_eq!(format!("{:1.4}", P(p("7e+39"))), "7e39");
    assert_eq!(format!("{:1.4}", P(p("7e+28"))), "7e28");
    assert_eq!(format!("{:1.4}", P(p("7"))), "7.0");
}
#[test]
fn onefour_3() {
    assert_eq!(format!("{:1.4}", P(p("6.844e+113"))), "#");
    assert_eq!(format!("{:1.4}", P(p("6.7853e-21"))), "0");
    assert_eq!(format!("{:1.4}", P(p("6.75e-27"))), "0");
    assert_eq!(format!("{:1.4}", P(p("6.56e+39"))), "7e39");
    assert_eq!(format!("{:1.4}", P(p("6.540688e-10"))), "0");
    assert_eq!(format!("{:1.4}", P(p("6.4e-08"))), "6e-8");
    assert_eq!(format!("{:1.4}", P(p("6.389785e-262"))), "0");
    assert_eq!(format!("{:1.4}", P(p("6.3508e+42"))), "6e42");
    assert_eq!(format!("{:1.4}", P(p("6.3383e+47"))), "6e47");
    assert_eq!(format!("{:1.4}", P(p("6.295e+53"))), "6e53");
    assert_eq!(format!("{:1.4}", P(p("6.049"))), "6.05");
    assert_eq!(format!("{:1.4}", P(p("6.04"))), "6.04");
    assert_eq!(format!("{:1.4}", P(p("6.01341e-20"))), "0");
    assert_eq!(format!("{:1.4}", P(p("5.98e+28"))), "6e28");
    assert_eq!(format!("{:1.4}", P(p("5.9e+43"))), "6e43");
    assert_eq!(format!("{:1.4}", P(p("5.865"))), "5.87");
    assert_eq!(format!("{:1.4}", P(p("5.7e-26"))), "0");
    assert_eq!(format!("{:1.4}", P(p("5.695e-09"))), "6e-9");
    assert_eq!(format!("{:1.4}", P(p("5.61e-07"))), "6e-7");
    assert_eq!(format!("{:1.4}", P(p("5.55971e-235"))), "0");
    assert_eq!(format!("{:1.4}", P(p("5.4541311e-10"))), "0");
    assert_eq!(format!("{:1.4}", P(p("5.1415063e-11"))), "0");
    assert_eq!(format!("{:1.4}", P(p("5.13e+282"))), "#");
    assert_eq!(format!("{:1.4}", P(p("5.0369e+172"))), "#");
    assert_eq!(format!("{:1.4}", P(p("5.003824e+151"))), "#");
    assert_eq!(format!("{:1.4}", P(p("5e+263"))), "#");
    assert_eq!(format!("{:1.4}", P(p("5e+24"))), "5e24");
    assert_eq!(format!("{:1.4}", P(p("5e+19"))), "5e19");
    assert_eq!(format!("{:1.4}", P(p("5e+133"))), "#");
    assert_eq!(format!("{:1.4}", P(p("4.937e+40"))), "5e40");
}
#[test]
fn onefour_4() {
    assert_eq!(format!("{:1.4}", P(p("4.9361647e-133"))), "0");
    assert_eq!(format!("{:1.4}", P(p("4.585e+48"))), "5e48");
    assert_eq!(format!("{:1.4}", P(p("4.49e+26"))), "4e26");
    assert_eq!(format!("{:1.4}", P(p("4.4177e-134"))), "0");
    assert_eq!(format!("{:1.4}", P(p("4.3e-245"))), "0");
    assert_eq!(format!("{:1.4}", P(p("4.3e-16"))), "0");
    assert_eq!(format!("{:1.4}", P(p("3.452077e-191"))), "0");
    assert_eq!(format!("{:1.4}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{:1.4}", P(p("3e+25"))), "3e25");
    assert_eq!(format!("{:1.4}", P(p("3e+149"))), "#");
    assert_eq!(format!("{:1.4}", P(p("2.93e+32"))), "3e32");
    assert_eq!(format!("{:1.4}", P(p("2.916861e+44"))), "3e44");
    assert_eq!(format!("{:1.4}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{:1.4}", P(p("2.764e-83"))), "0");
    assert_eq!(format!("{:1.4}", P(p("2.49279e+34"))), "2e34");
    assert_eq!(format!("{:1.4}", P(p("2.413e-15"))), "0");
    assert_eq!(format!("{:1.4}", P(p("2.2353e-129"))), "0");
    assert_eq!(format!("{:1.4}", P(p("2.166819e-23"))), "0");
    assert_eq!(format!("{:1.4}", P(p("2e+31"))), "2e31");
    assert_eq!(format!("{:1.4}", P(p("2e+116"))), "#");
    assert_eq!(format!("{:1.4}", P(p("1.8e+33"))), "2e33");
    assert_eq!(format!("{:1.4}", P(p("1.7e-13"))), "0");
    assert_eq!(format!("{:1.4}", P(p("1.4580931e+62"))), "1e62");
    assert_eq!(format!("{:1.4}", P(p("1.44471e-07"))), "1e-7");
    assert_eq!(format!("{:1.4}", P(p("1.393237e+46"))), "1e46");
    assert_eq!(format!("{:1.4}", P(p("1.39e+295"))), "#");
    assert_eq!(format!("{:1.4}", P(p("1.375e+17"))), "1e17");
    assert_eq!(format!("{:1.4}", P(p("1.293e+27"))), "1e27");
    assert_eq!(format!("{:1.4}", P(p("1.2041e-21"))), "0");
    assert_eq!(format!("{:1.4}", P(p("1e+48"))), "1e48");
}
#[test]
fn onefour_5() {
    assert_eq!(format!("{:1.4}", P(p("1"))), "1.0");
    assert_eq!(format!("{:1.4}", P(p("0.98"))), "0.98");
    assert_eq!(format!("{:1.4}", P(p("0.973"))), "0.97");
    assert_eq!(format!("{:1.4}", P(p("0.945138"))), "0.95");
    assert_eq!(format!("{:1.4}", P(p("0.84"))), "0.84");
    assert_eq!(format!("{:1.4}", P(p("0.67"))), "0.67");
    assert_eq!(format!("{:1.4}", P(p("0.335"))), "0.34");
    assert_eq!(format!("{:1.4}", P(p("0.113"))), "0.11");
    assert_eq!(format!("{:1.4}", P(p("0.0983688"))), "0.1");
    assert_eq!(format!("{:1.4}", P(p("0.0906203"))), "0.09");
    assert_eq!(format!("{:1.4}", P(p("0.0869"))), "0.09");
    assert_eq!(format!("{:1.4}", P(p("0.05"))), "0.05");
    assert_eq!(format!("{:1.4}", P(p("0.047255829"))), "0.05");
    assert_eq!(format!("{:1.4}", P(p("0.028"))), "0.03");
    assert_eq!(format!("{:1.4}", P(p("0.009146"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.008581"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.008166"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.0081"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.008001"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.007889"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.006703542"))), "0.01");
    assert_eq!(format!("{:1.4}", P(p("0.0039"))), "4e-3");
    assert_eq!(format!("{:1.4}", P(p("0.0030426"))), "3e-3");
    assert_eq!(format!("{:1.4}", P(p("0.003"))), "3e-3");
    assert_eq!(format!("{:1.4}", P(p("0.0029071"))), "3e-3");
    assert_eq!(format!("{:1.4}", P(p("0.002191249"))), "2e-3");
    assert_eq!(format!("{:1.4}", P(p("0.001910066"))), "2e-3");
    assert_eq!(format!("{:1.4}", P(p("0.00092769"))), "9e-4");
    assert_eq!(format!("{:1.4}", P(p("0.0004"))), "4e-4");
    assert_eq!(format!("{:1.4}", P(p("0.000383036"))), "4e-4");
}
#[test]
fn onefour_6() {
    assert_eq!(format!("{:1.4}", P(p("0.0003"))), "3e-4");
    assert_eq!(format!("{:1.4}", P(p("0.000188222"))), "2e-4");
    assert_eq!(format!("{:1.4}", P(p("0.00016"))), "2e-4");
    assert_eq!(format!("{:1.4}", P(p("0.00014834"))), "1e-4");
    assert_eq!(format!("{:1.4}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{:1.4}", P(p("-inf"))), "-inf");
    assert_eq!(format!("{:1.4}", P(p("inf"))), "inf");
    assert_eq!(format!("{:1.4}", P(p("0"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.00023816"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.000274"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.00031"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.00038509"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.000552594"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0006028"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.000719834"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0007820539"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.000869"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.00089806422"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0009172"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.00095719"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0060372"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0064726046"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0092191939"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.019099"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.022925"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.02826"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-0.0546163"))), "-0.1");
    assert_eq!(format!("{:1.4}", P(p("-0.062"))), "-0.1");
    assert_eq!(format!("{:1.4}", P(p("-0.0764018"))), "-0.1");
    assert_eq!(format!("{:1.4}", P(p("-0.0929804"))), "-0.1");
}
#[test]
fn onefour_7() {
    assert_eq!(format!("{:1.4}", P(p("-0.1689"))), "-0.2");
    assert_eq!(format!("{:1.4}", P(p("-0.2"))), "-0.2");
    assert_eq!(format!("{:1.4}", P(p("-0.4946795"))), "-0.5");
    assert_eq!(format!("{:1.4}", P(p("-0.67014714"))), "-0.7");
    assert_eq!(format!("{:1.4}", P(p("-0.68"))), "-0.7");
    assert_eq!(format!("{:1.4}", P(p("-0.68815"))), "-0.7");
    assert_eq!(format!("{:1.4}", P(p("-0.7562"))), "-0.8");
    assert_eq!(format!("{:1.4}", P(p("-1e+29"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-1.064846e-26"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-1.51624e-234"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-1.6965898e-119"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-1.7695e-236"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-1.93885e+19"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-2.31373e+31"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-2.4527809e-306"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-2.6136e-09"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-2.691e-11"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-2.819"))), "-2.8");
    assert_eq!(format!("{:1.4}", P(p("-2.8322"))), "-2.8");
    assert_eq!(format!("{:1.4}", P(p("-3e+45"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-3e-05"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-3.05e+25"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-3.108287e+278"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-3.16584e+41"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-3.17e-222"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-3.2348135"))), "-3.2");
    assert_eq!(format!("{:1.4}", P(p("-3.465e+22"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-3.63e-135"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-3.72e+45"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-3.9e+30"))), "#");
}
#[test]
fn onefour_8() {
    assert_eq!(format!("{:1.4}", P(p("-4.2"))), "-4.2");
    assert_eq!(format!("{:1.4}", P(p("-4.208329e-20"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-4.2888733"))), "-4.3");
    assert_eq!(format!("{:1.4}", P(p("-4.297e-08"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-4.60469181924042e-321"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-4.7142e+36"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-4.767"))), "-4.8");
    assert_eq!(format!("{:1.4}", P(p("-4.863526e-20"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-4.868e+33"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-4.96e+247"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-5e-29"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.05e-218"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.169414e+37"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-5.20816e-06"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.263e+21"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-5.4239467e-31"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.44067e-263"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.543976e+32"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-5.56122e-29"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.6e+35"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-5.65896e-290"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.71654"))), "-5.7");
    assert_eq!(format!("{:1.4}", P(p("-5.730186e+123"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-5.798598e-21"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-5.81165e-212"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-6e-25"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-6e-06"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-6.07e-268"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-6.074991"))), "-6.1");
    assert_eq!(format!("{:1.4}", P(p("-6.1e+290"))), "#");
}
#[test]
fn onefour_9() {
    assert_eq!(format!("{:1.4}", P(p("-6.1206e-06"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-6.5986e+45"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-6.785938e+26"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-6.8e-30"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-6.921e-22"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-7e-57"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-7.0915e+22"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-7.15407055178125e-321"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-7.60733e+45"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-7.79173e-05"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8e-26"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8e+20"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-8.245e-145"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.28043e+21"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-8.2888989e-25"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.37662e-158"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.38e-30"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.4e-23"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.5e+25"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-8.8165e-23"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.821e-20"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-8.8805774e+129"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-8.91e+18"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-9e-19"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-9.03e-286"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-9.13"))), "-9.1");
    assert_eq!(format!("{:1.4}", P(p("-9.185594e-120"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-9.3e-70"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-9.451309e-30"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-9.494134e+65"))), "#");
}
#[test]
fn onefour_10() {
    assert_eq!(format!("{:1.4}", P(p("-9.54e-24"))), "0");
    assert_eq!(format!("{:1.4}", P(p("-9.57259e+240"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-27.1783"))), "-27");
    assert_eq!(format!("{:1.4}", P(p("-32.234"))), "-32");
    assert_eq!(format!("{:1.4}", P(p("-46.430046"))), "-46");
    assert_eq!(format!("{:1.4}", P(p("-48.1"))), "-48");
    assert_eq!(format!("{:1.4}", P(p("-48.307"))), "-48");
    assert_eq!(format!("{:1.4}", P(p("-50"))), "-50");
    assert_eq!(format!("{:1.4}", P(p("-57.28"))), "-57");
    assert_eq!(format!("{:1.4}", P(p("-60.094"))), "-60");
    assert_eq!(format!("{:1.4}", P(p("-62.01612"))), "-62");
    assert_eq!(format!("{:1.4}", P(p("-64.92"))), "-65");
    assert_eq!(format!("{:1.4}", P(p("-80.2234"))), "-80");
    assert_eq!(format!("{:1.4}", P(p("-90"))), "-90");
    assert_eq!(format!("{:1.4}", P(p("-94.9"))), "-95");
    assert_eq!(format!("{:1.4}", P(p("-98.6783"))), "-99");
    assert_eq!(format!("{:1.4}", P(p("-100"))), "-100");
    assert_eq!(format!("{:1.4}", P(p("-164.3687"))), "-164");
    assert_eq!(format!("{:1.4}", P(p("-430.50597"))), "-431");
    assert_eq!(format!("{:1.4}", P(p("-466.5085"))), "-467");
    assert_eq!(format!("{:1.4}", P(p("-595.05"))), "-595");
    assert_eq!(format!("{:1.4}", P(p("-764.3112"))), "-764");
    assert_eq!(format!("{:1.4}", P(p("-807.142"))), "-807");
    assert_eq!(format!("{:1.4}", P(p("-5469.471"))), "-5e3");
    assert_eq!(format!("{:1.4}", P(p("-5651.6809"))), "-6e3");
    assert_eq!(format!("{:1.4}", P(p("-9706.81"))), "-1e4");
    assert_eq!(format!("{:1.4}", P(p("-93500"))), "-9e4");
    assert_eq!(format!("{:1.4}", P(p("-577000"))), "-6e5");
    assert_eq!(format!("{:1.4}", P(p("-835534"))), "-8e5");
    assert_eq!(format!("{:1.4}", P(p("-6000000"))), "-6e6");
}
#[test]
fn onefour_11() {
    assert_eq!(format!("{:1.4}", P(p("-900000000"))), "-9e8");
    assert_eq!(format!("{:1.4}", P(p("-279358770000"))), "#");
    assert_eq!(format!("{:1.4}", P(p("-50000000000000"))), "#");
}
fn _just_opening_brace_twofour() {
}
#[test]
fn twofour_1() {
    assert_eq!(format!("{:2.4}", P(p("906792980000000"))), "9e14");
    assert_eq!(format!("{:2.4}", P(p("6390900000000"))), "6e12");
    assert_eq!(format!("{:2.4}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:2.4}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:2.4}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:2.4}", P(p("9559.407"))), "9559");
    assert_eq!(format!("{:2.4}", P(p("8022.2"))), "8022");
    assert_eq!(format!("{:2.4}", P(p("6738.111"))), "6738");
    assert_eq!(format!("{:2.4}", P(p("6208.24123131241232132142"))), "6208");
    assert_eq!(format!("{:2.4}", P(p("5400"))), "5400");
    assert_eq!(format!("{:2.4}", P(p("4741.878"))), "4742");
    assert_eq!(format!("{:2.4}", P(p("3620.1461"))), "3620");
    assert_eq!(format!("{:2.4}", P(p("3000.23451"))), "3000");
    assert_eq!(format!("{:2.4}", P(p("2175.65"))), "2176");
    assert_eq!(format!("{:2.4}", P(p("969.49"))), "969");
    assert_eq!(format!("{:2.4}", P(p("840.2056"))), "840");
    assert_eq!(format!("{:2.4}", P(p("620"))), "620");
    assert_eq!(format!("{:2.4}", P(p("407"))), "407");
    assert_eq!(format!("{:2.4}", P(p("401.249"))), "401");
    assert_eq!(format!("{:2.4}", P(p("233.021"))), "233");
    assert_eq!(format!("{:2.4}", P(p("96.503326"))), "96.5");
    assert_eq!(format!("{:2.4}", P(p("58.4"))), "58.4");
    assert_eq!(format!("{:2.4}", P(p("39.137"))), "39.1");
    assert_eq!(format!("{:2.4}", P(p("38.74"))), "38.7");
    assert_eq!(format!("{:2.4}", P(p("30"))), "30.0");
    assert_eq!(format!("{:2.4}", P(p("24.48179"))), "24.5");
    assert_eq!(format!("{:2.4}", P(p("21.123"))), "21.1");
    assert_eq!(format!("{:2.4}", P(p("9.95016e+246"))), "##");
    assert_eq!(format!("{:2.4}", P(p("9.8388"))), "9.8");
    assert_eq!(format!("{:2.4}", P(p("9.8059e+35"))), "1e36");
}
#[test]
fn twofour_2() {
    assert_eq!(format!("{:2.4}", P(p("9.530609e+22"))), "1e23");
    assert_eq!(format!("{:2.4}", P(p("9.46e+35"))), "9e35");
    assert_eq!(format!("{:2.4}", P(p("9.452105e-31"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("9e+115"))), "##");
    assert_eq!(format!("{:2.4}", P(p("8.785953e+42"))), "9e42");
    assert_eq!(format!("{:2.4}", P(p("8.5e+20"))), "8e20");
    assert_eq!(format!("{:2.4}", P(p("8.3536e+30"))), "8e30");
    assert_eq!(format!("{:2.4}", P(p("8.3439e+25"))), "8e25");
    assert_eq!(format!("{:2.4}", P(p("8.27203e-18"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("8.271221e-219"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("8.0927985e-112"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("8.07e-53"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("8.062352e+38"))), "8e38");
    assert_eq!(format!("{:2.4}", P(p("8.0159e+36"))), "8e36");
    assert_eq!(format!("{:2.4}", P(p("8e+29"))), "8e29");
    assert_eq!(format!("{:2.4}", P(p("8e-24"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7.9954287e-194"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7.83472e-22"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7.814"))), "7.81");
    assert_eq!(format!("{:2.4}", P(p("7.7715e+27"))), "8e27");
    assert_eq!(format!("{:2.4}", P(p("7.509e+38"))), "8e38");
    assert_eq!(format!("{:2.4}", P(p("7.331e-31"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7.2401e+226"))), "##");
    assert_eq!(format!("{:2.4}", P(p("7.2193e-18"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7.2e+19"))), "7e19");
    assert_eq!(format!("{:2.4}", P(p("7.142849e-170"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7.0676e-17"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("7e+39"))), "7e39");
    assert_eq!(format!("{:2.4}", P(p("7e+28"))), "7e28");
    assert_eq!(format!("{:2.4}", P(p("7"))), "7.0");
}
#[test]
fn twofour_3() {
    assert_eq!(format!("{:2.4}", P(p("6.844e+113"))), "##");
    assert_eq!(format!("{:2.4}", P(p("6.7853e-21"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("6.75e-27"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("6.56e+39"))), "7e39");
    assert_eq!(format!("{:2.4}", P(p("6.540688e-10"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("6.4e-08"))), "6e-8");
    assert_eq!(format!("{:2.4}", P(p("6.389785e-262"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("6.3508e+42"))), "6e42");
    assert_eq!(format!("{:2.4}", P(p("6.3383e+47"))), "6e47");
    assert_eq!(format!("{:2.4}", P(p("6.295e+53"))), "6e53");
    assert_eq!(format!("{:2.4}", P(p("6.049"))), "6.05");
    assert_eq!(format!("{:2.4}", P(p("6.04"))), "6.04");
    assert_eq!(format!("{:2.4}", P(p("6.01341e-20"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("5.98e+28"))), "6e28");
    assert_eq!(format!("{:2.4}", P(p("5.9e+43"))), "6e43");
    assert_eq!(format!("{:2.4}", P(p("5.865"))), "5.87");
    assert_eq!(format!("{:2.4}", P(p("5.7e-26"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("5.695e-09"))), "6e-9");
    assert_eq!(format!("{:2.4}", P(p("5.61e-07"))), "6e-7");
    assert_eq!(format!("{:2.4}", P(p("5.55971e-235"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("5.4541311e-10"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("5.1415063e-11"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("5.13e+282"))), "##");
    assert_eq!(format!("{:2.4}", P(p("5.0369e+172"))), "##");
    assert_eq!(format!("{:2.4}", P(p("5.003824e+151"))), "##");
    assert_eq!(format!("{:2.4}", P(p("5e+263"))), "##");
    assert_eq!(format!("{:2.4}", P(p("5e+24"))), "5e24");
    assert_eq!(format!("{:2.4}", P(p("5e+19"))), "5e19");
    assert_eq!(format!("{:2.4}", P(p("5e+133"))), "##");
    assert_eq!(format!("{:2.4}", P(p("4.937e+40"))), "5e40");
}
#[test]
fn twofour_4() {
    assert_eq!(format!("{:2.4}", P(p("4.9361647e-133"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("4.585e+48"))), "5e48");
    assert_eq!(format!("{:2.4}", P(p("4.49e+26"))), "4e26");
    assert_eq!(format!("{:2.4}", P(p("4.4177e-134"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("4.3e-245"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("4.3e-16"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("3.452077e-191"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{:2.4}", P(p("3e+25"))), "3e25");
    assert_eq!(format!("{:2.4}", P(p("3e+149"))), "##");
    assert_eq!(format!("{:2.4}", P(p("2.93e+32"))), "3e32");
    assert_eq!(format!("{:2.4}", P(p("2.916861e+44"))), "3e44");
    assert_eq!(format!("{:2.4}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{:2.4}", P(p("2.764e-83"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("2.49279e+34"))), "2e34");
    assert_eq!(format!("{:2.4}", P(p("2.413e-15"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("2.2353e-129"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("2.166819e-23"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("2e+31"))), "2e31");
    assert_eq!(format!("{:2.4}", P(p("2e+116"))), "##");
    assert_eq!(format!("{:2.4}", P(p("1.8e+33"))), "2e33");
    assert_eq!(format!("{:2.4}", P(p("1.7e-13"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("1.4580931e+62"))), "1e62");
    assert_eq!(format!("{:2.4}", P(p("1.44471e-07"))), "1e-7");
    assert_eq!(format!("{:2.4}", P(p("1.393237e+46"))), "1e46");
    assert_eq!(format!("{:2.4}", P(p("1.39e+295"))), "##");
    assert_eq!(format!("{:2.4}", P(p("1.375e+17"))), "1e17");
    assert_eq!(format!("{:2.4}", P(p("1.293e+27"))), "1e27");
    assert_eq!(format!("{:2.4}", P(p("1.2041e-21"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("1e+48"))), "1e48");
}
#[test]
fn twofour_5() {
    assert_eq!(format!("{:2.4}", P(p("1"))), "1.0");
    assert_eq!(format!("{:2.4}", P(p("0.98"))), "0.98");
    assert_eq!(format!("{:2.4}", P(p("0.973"))), "0.97");
    assert_eq!(format!("{:2.4}", P(p("0.945138"))), "0.95");
    assert_eq!(format!("{:2.4}", P(p("0.84"))), "0.84");
    assert_eq!(format!("{:2.4}", P(p("0.67"))), "0.67");
    assert_eq!(format!("{:2.4}", P(p("0.335"))), "0.34");
    assert_eq!(format!("{:2.4}", P(p("0.113"))), "0.11");
    assert_eq!(format!("{:2.4}", P(p("0.0983688"))), "0.1");
    assert_eq!(format!("{:2.4}", P(p("0.0906203"))), "0.09");
    assert_eq!(format!("{:2.4}", P(p("0.0869"))), "0.09");
    assert_eq!(format!("{:2.4}", P(p("0.05"))), "0.05");
    assert_eq!(format!("{:2.4}", P(p("0.047255829"))), "0.05");
    assert_eq!(format!("{:2.4}", P(p("0.028"))), "0.03");
    assert_eq!(format!("{:2.4}", P(p("0.009146"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.008581"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.008166"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.0081"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.008001"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.007889"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.006703542"))), "0.01");
    assert_eq!(format!("{:2.4}", P(p("0.0039"))), "4e-3");
    assert_eq!(format!("{:2.4}", P(p("0.0030426"))), "3e-3");
    assert_eq!(format!("{:2.4}", P(p("0.003"))), "3e-3");
    assert_eq!(format!("{:2.4}", P(p("0.0029071"))), "3e-3");
    assert_eq!(format!("{:2.4}", P(p("0.002191249"))), "2e-3");
    assert_eq!(format!("{:2.4}", P(p("0.001910066"))), "2e-3");
    assert_eq!(format!("{:2.4}", P(p("0.00092769"))), "9e-4");
    assert_eq!(format!("{:2.4}", P(p("0.0004"))), "4e-4");
    assert_eq!(format!("{:2.4}", P(p("0.000383036"))), "4e-4");
}
#[test]
fn twofour_6() {
    assert_eq!(format!("{:2.4}", P(p("0.0003"))), "3e-4");
    assert_eq!(format!("{:2.4}", P(p("0.000188222"))), "2e-4");
    assert_eq!(format!("{:2.4}", P(p("0.00016"))), "2e-4");
    assert_eq!(format!("{:2.4}", P(p("0.00014834"))), "1e-4");
    assert_eq!(format!("{:2.4}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{:2.4}", P(p("-inf"))), "-inf");
    assert_eq!(format!("{:2.4}", P(p("inf"))), "inf");
    assert_eq!(format!("{:2.4}", P(p("0"))), "0 ");
    assert_eq!(format!("{:2.4}", P(p("-0.00023816"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.000274"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.00031"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.00038509"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.000552594"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0006028"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.000719834"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0007820539"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.000869"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.00089806422"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0009172"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.00095719"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0060372"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0064726046"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0092191939"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.019099"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.022925"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.02826"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-0.0546163"))), "-0.1");
    assert_eq!(format!("{:2.4}", P(p("-0.062"))), "-0.1");
    assert_eq!(format!("{:2.4}", P(p("-0.0764018"))), "-0.1");
    assert_eq!(format!("{:2.4}", P(p("-0.0929804"))), "-0.1");
}
#[test]
fn twofour_7() {
    assert_eq!(format!("{:2.4}", P(p("-0.1689"))), "-0.2");
    assert_eq!(format!("{:2.4}", P(p("-0.2"))), "-0.2");
    assert_eq!(format!("{:2.4}", P(p("-0.4946795"))), "-0.5");
    assert_eq!(format!("{:2.4}", P(p("-0.67014714"))), "-0.7");
    assert_eq!(format!("{:2.4}", P(p("-0.68"))), "-0.7");
    assert_eq!(format!("{:2.4}", P(p("-0.68815"))), "-0.7");
    assert_eq!(format!("{:2.4}", P(p("-0.7562"))), "-0.8");
    assert_eq!(format!("{:2.4}", P(p("-1e+29"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-1.064846e-26"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-1.51624e-234"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-1.6965898e-119"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-1.7695e-236"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-1.93885e+19"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-2.31373e+31"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-2.4527809e-306"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-2.6136e-09"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-2.691e-11"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-2.819"))), "-2.8");
    assert_eq!(format!("{:2.4}", P(p("-2.8322"))), "-2.8");
    assert_eq!(format!("{:2.4}", P(p("-3e+45"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-3e-05"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-3.05e+25"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-3.108287e+278"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-3.16584e+41"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-3.17e-222"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-3.2348135"))), "-3.2");
    assert_eq!(format!("{:2.4}", P(p("-3.465e+22"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-3.63e-135"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-3.72e+45"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-3.9e+30"))), "##");
}
#[test]
fn twofour_8() {
    assert_eq!(format!("{:2.4}", P(p("-4.2"))), "-4.2");
    assert_eq!(format!("{:2.4}", P(p("-4.208329e-20"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-4.2888733"))), "-4.3");
    assert_eq!(format!("{:2.4}", P(p("-4.297e-08"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-4.60469181924042e-321"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-4.7142e+36"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-4.767"))), "-4.8");
    assert_eq!(format!("{:2.4}", P(p("-4.863526e-20"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-4.868e+33"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-4.96e+247"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-5e-29"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.05e-218"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.169414e+37"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-5.20816e-06"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.263e+21"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-5.4239467e-31"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.44067e-263"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.543976e+32"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-5.56122e-29"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.6e+35"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-5.65896e-290"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.71654"))), "-5.7");
    assert_eq!(format!("{:2.4}", P(p("-5.730186e+123"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-5.798598e-21"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-5.81165e-212"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-6e-25"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-6e-06"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-6.07e-268"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-6.074991"))), "-6.1");
    assert_eq!(format!("{:2.4}", P(p("-6.1e+290"))), "##");
}
#[test]
fn twofour_9() {
    assert_eq!(format!("{:2.4}", P(p("-6.1206e-06"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-6.5986e+45"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-6.785938e+26"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-6.8e-30"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-6.921e-22"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-7e-57"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-7.0915e+22"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-7.15407055178125e-321"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-7.60733e+45"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-7.79173e-05"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8e-26"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8e+20"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-8.245e-145"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.28043e+21"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-8.2888989e-25"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.37662e-158"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.38e-30"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.4e-23"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.5e+25"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-8.8165e-23"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.821e-20"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-8.8805774e+129"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-8.91e+18"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-9e-19"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-9.03e-286"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-9.13"))), "-9.1");
    assert_eq!(format!("{:2.4}", P(p("-9.185594e-120"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-9.3e-70"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-9.451309e-30"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-9.494134e+65"))), "##");
}
#[test]
fn twofour_10() {
    assert_eq!(format!("{:2.4}", P(p("-9.54e-24"))), " 0");
    assert_eq!(format!("{:2.4}", P(p("-9.57259e+240"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-27.1783"))), "-27");
    assert_eq!(format!("{:2.4}", P(p("-32.234"))), "-32");
    assert_eq!(format!("{:2.4}", P(p("-46.430046"))), "-46");
    assert_eq!(format!("{:2.4}", P(p("-48.1"))), "-48");
    assert_eq!(format!("{:2.4}", P(p("-48.307"))), "-48");
    assert_eq!(format!("{:2.4}", P(p("-50"))), "-50");
    assert_eq!(format!("{:2.4}", P(p("-57.28"))), "-57");
    assert_eq!(format!("{:2.4}", P(p("-60.094"))), "-60");
    assert_eq!(format!("{:2.4}", P(p("-62.01612"))), "-62");
    assert_eq!(format!("{:2.4}", P(p("-64.92"))), "-65");
    assert_eq!(format!("{:2.4}", P(p("-80.2234"))), "-80");
    assert_eq!(format!("{:2.4}", P(p("-90"))), "-90");
    assert_eq!(format!("{:2.4}", P(p("-94.9"))), "-95");
    assert_eq!(format!("{:2.4}", P(p("-98.6783"))), "-99");
    assert_eq!(format!("{:2.4}", P(p("-100"))), "-100");
    assert_eq!(format!("{:2.4}", P(p("-164.3687"))), "-164");
    assert_eq!(format!("{:2.4}", P(p("-430.50597"))), "-431");
    assert_eq!(format!("{:2.4}", P(p("-466.5085"))), "-467");
    assert_eq!(format!("{:2.4}", P(p("-595.05"))), "-595");
    assert_eq!(format!("{:2.4}", P(p("-764.3112"))), "-764");
    assert_eq!(format!("{:2.4}", P(p("-807.142"))), "-807");
    assert_eq!(format!("{:2.4}", P(p("-5469.471"))), "-5e3");
    assert_eq!(format!("{:2.4}", P(p("-5651.6809"))), "-6e3");
    assert_eq!(format!("{:2.4}", P(p("-9706.81"))), "-1e4");
    assert_eq!(format!("{:2.4}", P(p("-93500"))), "-9e4");
    assert_eq!(format!("{:2.4}", P(p("-577000"))), "-6e5");
    assert_eq!(format!("{:2.4}", P(p("-835534"))), "-8e5");
    assert_eq!(format!("{:2.4}", P(p("-6000000"))), "-6e6");
}
#[test]
fn twofour_11() {
    assert_eq!(format!("{:2.4}", P(p("-900000000"))), "-9e8");
    assert_eq!(format!("{:2.4}", P(p("-279358770000"))), "##");
    assert_eq!(format!("{:2.4}", P(p("-50000000000000"))), "##");
}
fn _just_opening_brace_threefour() {
}
#[test]
fn threefour_1() {
    assert_eq!(format!("{:3.4}", P(p("906792980000000"))), "9e14");
    assert_eq!(format!("{:3.4}", P(p("6390900000000"))), "6e12");
    assert_eq!(format!("{:3.4}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:3.4}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:3.4}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:3.4}", P(p("9559.407"))), "9559");
    assert_eq!(format!("{:3.4}", P(p("8022.2"))), "8022");
    assert_eq!(format!("{:3.4}", P(p("6738.111"))), "6738");
    assert_eq!(format!("{:3.4}", P(p("6208.24123131241232132142"))), "6208");
    assert_eq!(format!("{:3.4}", P(p("5400"))), "5400");
    assert_eq!(format!("{:3.4}", P(p("4741.878"))), "4742");
    assert_eq!(format!("{:3.4}", P(p("3620.1461"))), "3620");
    assert_eq!(format!("{:3.4}", P(p("3000.23451"))), "3000");
    assert_eq!(format!("{:3.4}", P(p("2175.65"))), "2176");
    assert_eq!(format!("{:3.4}", P(p("969.49"))), "969");
    assert_eq!(format!("{:3.4}", P(p("840.2056"))), "840");
    assert_eq!(format!("{:3.4}", P(p("620"))), "620");
    assert_eq!(format!("{:3.4}", P(p("407"))), "407");
    assert_eq!(format!("{:3.4}", P(p("401.249"))), "401");
    assert_eq!(format!("{:3.4}", P(p("233.021"))), "233");
    assert_eq!(format!("{:3.4}", P(p("96.503326"))), "96.5");
    assert_eq!(format!("{:3.4}", P(p("58.4"))), "58.4");
    assert_eq!(format!("{:3.4}", P(p("39.137"))), "39.1");
    assert_eq!(format!("{:3.4}", P(p("38.74"))), "38.7");
    assert_eq!(format!("{:3.4}", P(p("30"))), "30.0");
    assert_eq!(format!("{:3.4}", P(p("24.48179"))), "24.5");
    assert_eq!(format!("{:3.4}", P(p("21.123"))), "21.1");
    assert_eq!(format!("{:3.4}", P(p("9.95016e+246"))), "###");
    assert_eq!(format!("{:3.4}", P(p("9.8388"))), "9.8");
    assert_eq!(format!("{:3.4}", P(p("9.8059e+35"))), "1e36");
}
#[test]
fn threefour_2() {
    assert_eq!(format!("{:3.4}", P(p("9.530609e+22"))), "1e23");
    assert_eq!(format!("{:3.4}", P(p("9.46e+35"))), "9e35");
    assert_eq!(format!("{:3.4}", P(p("9.452105e-31"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("9e+115"))), "###");
    assert_eq!(format!("{:3.4}", P(p("8.785953e+42"))), "9e42");
    assert_eq!(format!("{:3.4}", P(p("8.5e+20"))), "8e20");
    assert_eq!(format!("{:3.4}", P(p("8.3536e+30"))), "8e30");
    assert_eq!(format!("{:3.4}", P(p("8.3439e+25"))), "8e25");
    assert_eq!(format!("{:3.4}", P(p("8.27203e-18"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("8.271221e-219"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("8.0927985e-112"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("8.07e-53"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("8.062352e+38"))), "8e38");
    assert_eq!(format!("{:3.4}", P(p("8.0159e+36"))), "8e36");
    assert_eq!(format!("{:3.4}", P(p("8e+29"))), "8e29");
    assert_eq!(format!("{:3.4}", P(p("8e-24"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7.9954287e-194"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7.83472e-22"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7.814"))), "7.81");
    assert_eq!(format!("{:3.4}", P(p("7.7715e+27"))), "8e27");
    assert_eq!(format!("{:3.4}", P(p("7.509e+38"))), "8e38");
    assert_eq!(format!("{:3.4}", P(p("7.331e-31"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7.2401e+226"))), "###");
    assert_eq!(format!("{:3.4}", P(p("7.2193e-18"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7.2e+19"))), "7e19");
    assert_eq!(format!("{:3.4}", P(p("7.142849e-170"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7.0676e-17"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("7e+39"))), "7e39");
    assert_eq!(format!("{:3.4}", P(p("7e+28"))), "7e28");
    assert_eq!(format!("{:3.4}", P(p("7"))), "7.0");
}
#[test]
fn threefour_3() {
    assert_eq!(format!("{:3.4}", P(p("6.844e+113"))), "###");
    assert_eq!(format!("{:3.4}", P(p("6.7853e-21"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("6.75e-27"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("6.56e+39"))), "7e39");
    assert_eq!(format!("{:3.4}", P(p("6.540688e-10"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("6.4e-08"))), "6e-8");
    assert_eq!(format!("{:3.4}", P(p("6.389785e-262"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("6.3508e+42"))), "6e42");
    assert_eq!(format!("{:3.4}", P(p("6.3383e+47"))), "6e47");
    assert_eq!(format!("{:3.4}", P(p("6.295e+53"))), "6e53");
    assert_eq!(format!("{:3.4}", P(p("6.049"))), "6.05");
    assert_eq!(format!("{:3.4}", P(p("6.04"))), "6.04");
    assert_eq!(format!("{:3.4}", P(p("6.01341e-20"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("5.98e+28"))), "6e28");
    assert_eq!(format!("{:3.4}", P(p("5.9e+43"))), "6e43");
    assert_eq!(format!("{:3.4}", P(p("5.865"))), "5.87");
    assert_eq!(format!("{:3.4}", P(p("5.7e-26"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("5.695e-09"))), "6e-9");
    assert_eq!(format!("{:3.4}", P(p("5.61e-07"))), "6e-7");
    assert_eq!(format!("{:3.4}", P(p("5.55971e-235"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("5.4541311e-10"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("5.1415063e-11"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("5.13e+282"))), "###");
    assert_eq!(format!("{:3.4}", P(p("5.0369e+172"))), "###");
    assert_eq!(format!("{:3.4}", P(p("5.003824e+151"))), "###");
    assert_eq!(format!("{:3.4}", P(p("5e+263"))), "###");
    assert_eq!(format!("{:3.4}", P(p("5e+24"))), "5e24");
    assert_eq!(format!("{:3.4}", P(p("5e+19"))), "5e19");
    assert_eq!(format!("{:3.4}", P(p("5e+133"))), "###");
    assert_eq!(format!("{:3.4}", P(p("4.937e+40"))), "5e40");
}
#[test]
fn threefour_4() {
    assert_eq!(format!("{:3.4}", P(p("4.9361647e-133"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("4.585e+48"))), "5e48");
    assert_eq!(format!("{:3.4}", P(p("4.49e+26"))), "4e26");
    assert_eq!(format!("{:3.4}", P(p("4.4177e-134"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("4.3e-245"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("4.3e-16"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("3.452077e-191"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("3.2"))), "3.2");
    assert_eq!(format!("{:3.4}", P(p("3e+25"))), "3e25");
    assert_eq!(format!("{:3.4}", P(p("3e+149"))), "###");
    assert_eq!(format!("{:3.4}", P(p("2.93e+32"))), "3e32");
    assert_eq!(format!("{:3.4}", P(p("2.916861e+44"))), "3e44");
    assert_eq!(format!("{:3.4}", P(p("2.9"))), "2.9");
    assert_eq!(format!("{:3.4}", P(p("2.764e-83"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("2.49279e+34"))), "2e34");
    assert_eq!(format!("{:3.4}", P(p("2.413e-15"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("2.2353e-129"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("2.166819e-23"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("2e+31"))), "2e31");
    assert_eq!(format!("{:3.4}", P(p("2e+116"))), "###");
    assert_eq!(format!("{:3.4}", P(p("1.8e+33"))), "2e33");
    assert_eq!(format!("{:3.4}", P(p("1.7e-13"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("1.4580931e+62"))), "1e62");
    assert_eq!(format!("{:3.4}", P(p("1.44471e-07"))), "1e-7");
    assert_eq!(format!("{:3.4}", P(p("1.393237e+46"))), "1e46");
    assert_eq!(format!("{:3.4}", P(p("1.39e+295"))), "###");
    assert_eq!(format!("{:3.4}", P(p("1.375e+17"))), "1e17");
    assert_eq!(format!("{:3.4}", P(p("1.293e+27"))), "1e27");
    assert_eq!(format!("{:3.4}", P(p("1.2041e-21"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("1e+48"))), "1e48");
}
#[test]
fn threefour_5() {
    assert_eq!(format!("{:3.4}", P(p("1"))), "1.0");
    assert_eq!(format!("{:3.4}", P(p("0.98"))), "0.98");
    assert_eq!(format!("{:3.4}", P(p("0.973"))), "0.97");
    assert_eq!(format!("{:3.4}", P(p("0.945138"))), "0.95");
    assert_eq!(format!("{:3.4}", P(p("0.84"))), "0.84");
    assert_eq!(format!("{:3.4}", P(p("0.67"))), "0.67");
    assert_eq!(format!("{:3.4}", P(p("0.335"))), "0.34");
    assert_eq!(format!("{:3.4}", P(p("0.113"))), "0.11");
    assert_eq!(format!("{:3.4}", P(p("0.0983688"))), "0.1");
    assert_eq!(format!("{:3.4}", P(p("0.0906203"))), "0.09");
    assert_eq!(format!("{:3.4}", P(p("0.0869"))), "0.09");
    assert_eq!(format!("{:3.4}", P(p("0.05"))), "0.05");
    assert_eq!(format!("{:3.4}", P(p("0.047255829"))), "0.05");
    assert_eq!(format!("{:3.4}", P(p("0.028"))), "0.03");
    assert_eq!(format!("{:3.4}", P(p("0.009146"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.008581"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.008166"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.0081"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.008001"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.007889"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.006703542"))), "0.01");
    assert_eq!(format!("{:3.4}", P(p("0.0039"))), "4e-3");
    assert_eq!(format!("{:3.4}", P(p("0.0030426"))), "3e-3");
    assert_eq!(format!("{:3.4}", P(p("0.003"))), "3e-3");
    assert_eq!(format!("{:3.4}", P(p("0.0029071"))), "3e-3");
    assert_eq!(format!("{:3.4}", P(p("0.002191249"))), "2e-3");
    assert_eq!(format!("{:3.4}", P(p("0.001910066"))), "2e-3");
    assert_eq!(format!("{:3.4}", P(p("0.00092769"))), "9e-4");
    assert_eq!(format!("{:3.4}", P(p("0.0004"))), "4e-4");
    assert_eq!(format!("{:3.4}", P(p("0.000383036"))), "4e-4");
}
#[test]
fn threefour_6() {
    assert_eq!(format!("{:3.4}", P(p("0.0003"))), "3e-4");
    assert_eq!(format!("{:3.4}", P(p("0.000188222"))), "2e-4");
    assert_eq!(format!("{:3.4}", P(p("0.00016"))), "2e-4");
    assert_eq!(format!("{:3.4}", P(p("0.00014834"))), "1e-4");
    assert_eq!(format!("{:3.4}", P(p("NaN"))), "NaN");
    assert_eq!(format!("{:3.4}", P(p("-inf"))), "-inf");
    assert_eq!(format!("{:3.4}", P(p("inf"))), "inf");
    assert_eq!(format!("{:3.4}", P(p("0"))), "0.0");
    assert_eq!(format!("{:3.4}", P(p("-0.00023816"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.000274"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.00031"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.00038509"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.000552594"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0006028"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.000719834"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0007820539"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.000869"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.00089806422"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0009172"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.00095719"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0060372"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0064726046"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0092191939"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.019099"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.022925"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.02826"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-0.0546163"))), "-0.1");
    assert_eq!(format!("{:3.4}", P(p("-0.062"))), "-0.1");
    assert_eq!(format!("{:3.4}", P(p("-0.0764018"))), "-0.1");
    assert_eq!(format!("{:3.4}", P(p("-0.0929804"))), "-0.1");
}
#[test]
fn threefour_7() {
    assert_eq!(format!("{:3.4}", P(p("-0.1689"))), "-0.2");
    assert_eq!(format!("{:3.4}", P(p("-0.2"))), "-0.2");
    assert_eq!(format!("{:3.4}", P(p("-0.4946795"))), "-0.5");
    assert_eq!(format!("{:3.4}", P(p("-0.67014714"))), "-0.7");
    assert_eq!(format!("{:3.4}", P(p("-0.68"))), "-0.7");
    assert_eq!(format!("{:3.4}", P(p("-0.68815"))), "-0.7");
    assert_eq!(format!("{:3.4}", P(p("-0.7562"))), "-0.8");
    assert_eq!(format!("{:3.4}", P(p("-1e+29"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-1.064846e-26"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-1.51624e-234"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-1.6965898e-119"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-1.7695e-236"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-1.93885e+19"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-2.31373e+31"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-2.4527809e-306"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-2.6136e-09"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-2.691e-11"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-2.819"))), "-2.8");
    assert_eq!(format!("{:3.4}", P(p("-2.8322"))), "-2.8");
    assert_eq!(format!("{:3.4}", P(p("-3e+45"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-3e-05"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-3.05e+25"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-3.108287e+278"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-3.16584e+41"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-3.17e-222"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-3.2348135"))), "-3.2");
    assert_eq!(format!("{:3.4}", P(p("-3.465e+22"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-3.63e-135"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-3.72e+45"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-3.9e+30"))), "###");
}
#[test]
fn threefour_8() {
    assert_eq!(format!("{:3.4}", P(p("-4.2"))), "-4.2");
    assert_eq!(format!("{:3.4}", P(p("-4.208329e-20"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-4.2888733"))), "-4.3");
    assert_eq!(format!("{:3.4}", P(p("-4.297e-08"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-4.60469181924042e-321"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-4.7142e+36"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-4.767"))), "-4.8");
    assert_eq!(format!("{:3.4}", P(p("-4.863526e-20"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-4.868e+33"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-4.96e+247"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-5e-29"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.05e-218"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.169414e+37"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-5.20816e-06"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.263e+21"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-5.4239467e-31"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.44067e-263"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.543976e+32"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-5.56122e-29"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.6e+35"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-5.65896e-290"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.71654"))), "-5.7");
    assert_eq!(format!("{:3.4}", P(p("-5.730186e+123"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-5.798598e-21"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-5.81165e-212"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-6e-25"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-6e-06"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-6.07e-268"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-6.074991"))), "-6.1");
    assert_eq!(format!("{:3.4}", P(p("-6.1e+290"))), "###");
}
#[test]
fn threefour_9() {
    assert_eq!(format!("{:3.4}", P(p("-6.1206e-06"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-6.5986e+45"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-6.785938e+26"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-6.8e-30"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-6.921e-22"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-7e-57"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-7.0915e+22"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-7.15407055178125e-321"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-7.60733e+45"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-7.79173e-05"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8e-26"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8e+20"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-8.245e-145"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.28043e+21"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-8.2888989e-25"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.37662e-158"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.38e-30"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.4e-23"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.5e+25"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-8.8165e-23"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.821e-20"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-8.8805774e+129"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-8.91e+18"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-9e-19"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-9.03e-286"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-9.13"))), "-9.1");
    assert_eq!(format!("{:3.4}", P(p("-9.185594e-120"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-9.3e-70"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-9.451309e-30"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-9.494134e+65"))), "###");
}
#[test]
fn threefour_10() {
    assert_eq!(format!("{:3.4}", P(p("-9.54e-24"))), "  0");
    assert_eq!(format!("{:3.4}", P(p("-9.57259e+240"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-27.1783"))), "-27");
    assert_eq!(format!("{:3.4}", P(p("-32.234"))), "-32");
    assert_eq!(format!("{:3.4}", P(p("-46.430046"))), "-46");
    assert_eq!(format!("{:3.4}", P(p("-48.1"))), "-48");
    assert_eq!(format!("{:3.4}", P(p("-48.307"))), "-48");
    assert_eq!(format!("{:3.4}", P(p("-50"))), "-50");
    assert_eq!(format!("{:3.4}", P(p("-57.28"))), "-57");
    assert_eq!(format!("{:3.4}", P(p("-60.094"))), "-60");
    assert_eq!(format!("{:3.4}", P(p("-62.01612"))), "-62");
    assert_eq!(format!("{:3.4}", P(p("-64.92"))), "-65");
    assert_eq!(format!("{:3.4}", P(p("-80.2234"))), "-80");
    assert_eq!(format!("{:3.4}", P(p("-90"))), "-90");
    assert_eq!(format!("{:3.4}", P(p("-94.9"))), "-95");
    assert_eq!(format!("{:3.4}", P(p("-98.6783"))), "-99");
    assert_eq!(format!("{:3.4}", P(p("-100"))), "-100");
    assert_eq!(format!("{:3.4}", P(p("-164.3687"))), "-164");
    assert_eq!(format!("{:3.4}", P(p("-430.50597"))), "-431");
    assert_eq!(format!("{:3.4}", P(p("-466.5085"))), "-467");
    assert_eq!(format!("{:3.4}", P(p("-595.05"))), "-595");
    assert_eq!(format!("{:3.4}", P(p("-764.3112"))), "-764");
    assert_eq!(format!("{:3.4}", P(p("-807.142"))), "-807");
    assert_eq!(format!("{:3.4}", P(p("-5469.471"))), "-5e3");
    assert_eq!(format!("{:3.4}", P(p("-5651.6809"))), "-6e3");
    assert_eq!(format!("{:3.4}", P(p("-9706.81"))), "-1e4");
    assert_eq!(format!("{:3.4}", P(p("-93500"))), "-9e4");
    assert_eq!(format!("{:3.4}", P(p("-577000"))), "-6e5");
    assert_eq!(format!("{:3.4}", P(p("-835534"))), "-8e5");
    assert_eq!(format!("{:3.4}", P(p("-6000000"))), "-6e6");
}
#[test]
fn threefour_11() {
    assert_eq!(format!("{:3.4}", P(p("-900000000"))), "-9e8");
    assert_eq!(format!("{:3.4}", P(p("-279358770000"))), "###");
    assert_eq!(format!("{:3.4}", P(p("-50000000000000"))), "###");
}
fn _just_opening_brace_four() {
}
#[test]
fn four_1() {
    assert_eq!(format!("{:4.4}", P(p("906792980000000"))), "9e14");
    assert_eq!(format!("{:4.4}", P(p("6390900000000"))), "6e12");
    assert_eq!(format!("{:4.4}", P(p("28897000"))), "3e7");
    assert_eq!(format!("{:4.4}", P(p("700000"))), "7e5");
    assert_eq!(format!("{:4.4}", P(p("439620.1"))), "4e5");
    assert_eq!(format!("{:4.4}", P(p("9559.407"))), "9559");
    assert_eq!(format!("{:4.4}", P(p("8022.2"))), "8022");
    assert_eq!(format!("{:4.4}", P(p("6738.111"))), "6738");
    assert_eq!(format!("{:4.4}", P(p("6208.24123131241232132142"))), "6208");
    assert_eq!(format!("{:4.4}", P(p("5400"))), "5400");
    assert_eq!(format!("{:4.4}", P(p("4741.878"))), "4742");
    assert_eq!(format!("{:4.4}", P(p("3620.1461"))), "3620");
    assert_eq!(format!("{:4.4}", P(p("3000.23451"))), "3000");
    assert_eq!(format!("{:4.4}", P(p("2175.65"))), "2176");
    assert_eq!(format!("{:4.4}", P(p("969.49"))), " 969");
    assert_eq!(format!("{:4.4}", P(p("840.2056"))), " 840");
    assert_eq!(format!("{:4.4}", P(p("620"))), " 620");
    assert_eq!(format!("{:4.4}", P(p("407"))), " 407");
    assert_eq!(format!("{:4.4}", P(p("401.249"))), " 401");
    assert_eq!(format!("{:4.4}", P(p("233.021"))), " 233");
    assert_eq!(format!("{:4.4}", P(p("96.503326"))), "96.5");
    assert_eq!(format!("{:4.4}", P(p("58.4"))), "58.4");
    assert_eq!(format!("{:4.4}", P(p("39.137"))), "39.1");
    assert_eq!(format!("{:4.4}", P(p("38.74"))), "38.7");
    assert_eq!(format!("{:4.4}", P(p("30"))), "30.0");
    assert_eq!(format!("{:4.4}", P(p("24.48179"))), "24.5");
    assert_eq!(format!("{:4.4}", P(p("21.123"))), "21.1");
    assert_eq!(format!("{:4.4}", P(p("9.95016e+246"))), "####");
    assert_eq!(format!("{:4.4}", P(p("9.8388"))), "9.8");
    assert_eq!(format!("{:4.4}", P(p("9.8059e+35"))), "1e36");
}
#[test]
fn four_2() {
    assert_eq!(format!("{:4.4}", P(p("9.530609e+22"))), "1e23");
    assert_eq!(format!("{:4.4}", P(p("9.46e+35"))), "9e35");
    assert_eq!(format!("{:4.4}", P(p("9.452105e-31"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("9e+115"))), "####");
    assert_eq!(format!("{:4.4}", P(p("8.785953e+42"))), "9e42");
    assert_eq!(format!("{:4.4}", P(p("8.5e+20"))), "8e20");
    assert_eq!(format!("{:4.4}", P(p("8.3536e+30"))), "8e30");
    assert_eq!(format!("{:4.4}", P(p("8.3439e+25"))), "8e25");
    assert_eq!(format!("{:4.4}", P(p("8.27203e-18"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("8.271221e-219"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("8.0927985e-112"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("8.07e-53"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("8.062352e+38"))), "8e38");
    assert_eq!(format!("{:4.4}", P(p("8.0159e+36"))), "8e36");
    assert_eq!(format!("{:4.4}", P(p("8e+29"))), "8e29");
    assert_eq!(format!("{:4.4}", P(p("8e-24"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7.9954287e-194"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7.83472e-22"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7.814"))), "7.81");
    assert_eq!(format!("{:4.4}", P(p("7.7715e+27"))), "8e27");
    assert_eq!(format!("{:4.4}", P(p("7.509e+38"))), "8e38");
    assert_eq!(format!("{:4.4}", P(p("7.331e-31"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7.2401e+226"))), "####");
    assert_eq!(format!("{:4.4}", P(p("7.2193e-18"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7.2e+19"))), "7e19");
    assert_eq!(format!("{:4.4}", P(p("7.142849e-170"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7.0676e-17"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("7e+39"))), "7e39");
    assert_eq!(format!("{:4.4}", P(p("7e+28"))), "7e28");
    assert_eq!(format!("{:4.4}", P(p("7"))), "7.00");
}
#[test]
fn four_3() {
    assert_eq!(format!("{:4.4}", P(p("6.844e+113"))), "####");
    assert_eq!(format!("{:4.4}", P(p("6.7853e-21"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("6.75e-27"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("6.56e+39"))), "7e39");
    assert_eq!(format!("{:4.4}", P(p("6.540688e-10"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("6.4e-08"))), "6e-8");
    assert_eq!(format!("{:4.4}", P(p("6.389785e-262"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("6.3508e+42"))), "6e42");
    assert_eq!(format!("{:4.4}", P(p("6.3383e+47"))), "6e47");
    assert_eq!(format!("{:4.4}", P(p("6.295e+53"))), "6e53");
    assert_eq!(format!("{:4.4}", P(p("6.049"))), "6.05");
    assert_eq!(format!("{:4.4}", P(p("6.04"))), "6.04");
    assert_eq!(format!("{:4.4}", P(p("6.01341e-20"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("5.98e+28"))), "6e28");
    assert_eq!(format!("{:4.4}", P(p("5.9e+43"))), "6e43");
    assert_eq!(format!("{:4.4}", P(p("5.865"))), "5.87");
    assert_eq!(format!("{:4.4}", P(p("5.7e-26"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("5.695e-09"))), "6e-9");
    assert_eq!(format!("{:4.4}", P(p("5.61e-07"))), "6e-7");
    assert_eq!(format!("{:4.4}", P(p("5.55971e-235"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("5.4541311e-10"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("5.1415063e-11"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("5.13e+282"))), "####");
    assert_eq!(format!("{:4.4}", P(p("5.0369e+172"))), "####");
    assert_eq!(format!("{:4.4}", P(p("5.003824e+151"))), "####");
    assert_eq!(format!("{:4.4}", P(p("5e+263"))), "####");
    assert_eq!(format!("{:4.4}", P(p("5e+24"))), "5e24");
    assert_eq!(format!("{:4.4}", P(p("5e+19"))), "5e19");
    assert_eq!(format!("{:4.4}", P(p("5e+133"))), "####");
    assert_eq!(format!("{:4.4}", P(p("4.937e+40"))), "5e40");
}
#[test]
fn four_4() {
    assert_eq!(format!("{:4.4}", P(p("4.9361647e-133"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("4.585e+48"))), "5e48");
    assert_eq!(format!("{:4.4}", P(p("4.49e+26"))), "4e26");
    assert_eq!(format!("{:4.4}", P(p("4.4177e-134"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("4.3e-245"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("4.3e-16"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("3.452077e-191"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("3.2"))), "3.20");
    assert_eq!(format!("{:4.4}", P(p("3e+25"))), "3e25");
    assert_eq!(format!("{:4.4}", P(p("3e+149"))), "####");
    assert_eq!(format!("{:4.4}", P(p("2.93e+32"))), "3e32");
    assert_eq!(format!("{:4.4}", P(p("2.916861e+44"))), "3e44");
    assert_eq!(format!("{:4.4}", P(p("2.9"))), "2.90");
    assert_eq!(format!("{:4.4}", P(p("2.764e-83"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("2.49279e+34"))), "2e34");
    assert_eq!(format!("{:4.4}", P(p("2.413e-15"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("2.2353e-129"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("2.166819e-23"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("2e+31"))), "2e31");
    assert_eq!(format!("{:4.4}", P(p("2e+116"))), "####");
    assert_eq!(format!("{:4.4}", P(p("1.8e+33"))), "2e33");
    assert_eq!(format!("{:4.4}", P(p("1.7e-13"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("1.4580931e+62"))), "1e62");
    assert_eq!(format!("{:4.4}", P(p("1.44471e-07"))), "1e-7");
    assert_eq!(format!("{:4.4}", P(p("1.393237e+46"))), "1e46");
    assert_eq!(format!("{:4.4}", P(p("1.39e+295"))), "####");
    assert_eq!(format!("{:4.4}", P(p("1.375e+17"))), "1e17");
    assert_eq!(format!("{:4.4}", P(p("1.293e+27"))), "1e27");
    assert_eq!(format!("{:4.4}", P(p("1.2041e-21"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("1e+48"))), "1e48");
}
#[test]
fn four_5() {
    assert_eq!(format!("{:4.4}", P(p("1"))), "1.00");
    assert_eq!(format!("{:4.4}", P(p("0.98"))), "0.98");
    assert_eq!(format!("{:4.4}", P(p("0.973"))), "0.97");
    assert_eq!(format!("{:4.4}", P(p("0.945138"))), "0.95");
    assert_eq!(format!("{:4.4}", P(p("0.84"))), "0.84");
    assert_eq!(format!("{:4.4}", P(p("0.67"))), "0.67");
    assert_eq!(format!("{:4.4}", P(p("0.335"))), "0.34");
    assert_eq!(format!("{:4.4}", P(p("0.113"))), "0.11");
    assert_eq!(format!("{:4.4}", P(p("0.0983688"))), "0.10");
    assert_eq!(format!("{:4.4}", P(p("0.0906203"))), "0.09");
    assert_eq!(format!("{:4.4}", P(p("0.0869"))), "0.09");
    assert_eq!(format!("{:4.4}", P(p("0.05"))), "0.05");
    assert_eq!(format!("{:4.4}", P(p("0.047255829"))), "0.05");
    assert_eq!(format!("{:4.4}", P(p("0.028"))), "0.03");
    assert_eq!(format!("{:4.4}", P(p("0.009146"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.008581"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.008166"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.0081"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.008001"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.007889"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.006703542"))), "0.01");
    assert_eq!(format!("{:4.4}", P(p("0.0039"))), "4e-3");
    assert_eq!(format!("{:4.4}", P(p("0.0030426"))), "3e-3");
    assert_eq!(format!("{:4.4}", P(p("0.003"))), "3e-3");
    assert_eq!(format!("{:4.4}", P(p("0.0029071"))), "3e-3");
    assert_eq!(format!("{:4.4}", P(p("0.002191249"))), "2e-3");
    assert_eq!(format!("{:4.4}", P(p("0.001910066"))), "2e-3");
    assert_eq!(format!("{:4.4}", P(p("0.00092769"))), "9e-4");
    assert_eq!(format!("{:4.4}", P(p("0.0004"))), "4e-4");
    assert_eq!(format!("{:4.4}", P(p("0.000383036"))), "4e-4");
}
#[test]
fn four_6() {
    assert_eq!(format!("{:4.4}", P(p("0.0003"))), "3e-4");
    assert_eq!(format!("{:4.4}", P(p("0.000188222"))), "2e-4");
    assert_eq!(format!("{:4.4}", P(p("0.00016"))), "2e-4");
    assert_eq!(format!("{:4.4}", P(p("0.00014834"))), "1e-4");
    assert_eq!(format!("{:4.4}", P(p("NaN"))), "NaN ");
    assert_eq!(format!("{:4.4}", P(p("-inf"))), "-inf");
    assert_eq!(format!("{:4.4}", P(p("inf"))), "inf ");
    assert_eq!(format!("{:4.4}", P(p("0"))), "0.00");
    assert_eq!(format!("{:4.4}", P(p("-0.00023816"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.000274"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.00031"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.00038509"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.000552594"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0006028"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.000719834"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0007820539"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.000869"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.00089806422"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0009172"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.00095719"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0060372"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0064726046"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0092191939"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.019099"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.022925"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.02826"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-0.0546163"))), "-0.1");
    assert_eq!(format!("{:4.4}", P(p("-0.062"))), "-0.1");
    assert_eq!(format!("{:4.4}", P(p("-0.0764018"))), "-0.1");
    assert_eq!(format!("{:4.4}", P(p("-0.0929804"))), "-0.1");
}
#[test]
fn four_7() {
    assert_eq!(format!("{:4.4}", P(p("-0.1689"))), "-0.2");
    assert_eq!(format!("{:4.4}", P(p("-0.2"))), "-0.2");
    assert_eq!(format!("{:4.4}", P(p("-0.4946795"))), "-0.5");
    assert_eq!(format!("{:4.4}", P(p("-0.67014714"))), "-0.7");
    assert_eq!(format!("{:4.4}", P(p("-0.68"))), "-0.7");
    assert_eq!(format!("{:4.4}", P(p("-0.68815"))), "-0.7");
    assert_eq!(format!("{:4.4}", P(p("-0.7562"))), "-0.8");
    assert_eq!(format!("{:4.4}", P(p("-1e+29"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-1.064846e-26"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-1.51624e-234"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-1.6965898e-119"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-1.7695e-236"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-1.93885e+19"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-2.31373e+31"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-2.4527809e-306"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-2.6136e-09"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-2.691e-11"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-2.819"))), "-2.8");
    assert_eq!(format!("{:4.4}", P(p("-2.8322"))), "-2.8");
    assert_eq!(format!("{:4.4}", P(p("-3e+45"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-3e-05"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-3.05e+25"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-3.108287e+278"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-3.16584e+41"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-3.17e-222"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-3.2348135"))), "-3.2");
    assert_eq!(format!("{:4.4}", P(p("-3.465e+22"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-3.63e-135"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-3.72e+45"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-3.9e+30"))), "####");
}
#[test]
fn four_8() {
    assert_eq!(format!("{:4.4}", P(p("-4.2"))), "-4.2");
    assert_eq!(format!("{:4.4}", P(p("-4.208329e-20"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-4.2888733"))), "-4.3");
    assert_eq!(format!("{:4.4}", P(p("-4.297e-08"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-4.60469181924042e-321"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-4.7142e+36"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-4.767"))), "-4.8");
    assert_eq!(format!("{:4.4}", P(p("-4.863526e-20"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-4.868e+33"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-4.96e+247"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-5e-29"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.05e-218"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.169414e+37"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-5.20816e-06"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.263e+21"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-5.4239467e-31"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.44067e-263"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.543976e+32"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-5.56122e-29"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.6e+35"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-5.65896e-290"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.71654"))), "-5.7");
    assert_eq!(format!("{:4.4}", P(p("-5.730186e+123"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-5.798598e-21"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-5.81165e-212"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-6e-25"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-6e-06"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-6.07e-268"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-6.074991"))), "-6.1");
    assert_eq!(format!("{:4.4}", P(p("-6.1e+290"))), "####");
}
#[test]
fn four_9() {
    assert_eq!(format!("{:4.4}", P(p("-6.1206e-06"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-6.5986e+45"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-6.785938e+26"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-6.8e-30"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-6.921e-22"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-7e-57"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-7.0915e+22"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-7.15407055178125e-321"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-7.60733e+45"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-7.79173e-05"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8e-26"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8e+20"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-8.245e-145"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.28043e+21"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-8.2888989e-25"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.37662e-158"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.38e-30"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.4e-23"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.5e+25"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-8.8165e-23"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.821e-20"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-8.8805774e+129"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-8.91e+18"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-9e-19"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-9.03e-286"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-9.13"))), "-9.1");
    assert_eq!(format!("{:4.4}", P(p("-9.185594e-120"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-9.3e-70"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-9.451309e-30"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-9.494134e+65"))), "####");
}
#[test]
fn four_10() {
    assert_eq!(format!("{:4.4}", P(p("-9.54e-24"))), "   0");
    assert_eq!(format!("{:4.4}", P(p("-9.57259e+240"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-27.1783"))), " -27");
    assert_eq!(format!("{:4.4}", P(p("-32.234"))), " -32");
    assert_eq!(format!("{:4.4}", P(p("-46.430046"))), " -46");
    assert_eq!(format!("{:4.4}", P(p("-48.1"))), " -48");
    assert_eq!(format!("{:4.4}", P(p("-48.307"))), " -48");
    assert_eq!(format!("{:4.4}", P(p("-50"))), " -50");
    assert_eq!(format!("{:4.4}", P(p("-57.28"))), " -57");
    assert_eq!(format!("{:4.4}", P(p("-60.094"))), " -60");
    assert_eq!(format!("{:4.4}", P(p("-62.01612"))), " -62");
    assert_eq!(format!("{:4.4}", P(p("-64.92"))), " -65");
    assert_eq!(format!("{:4.4}", P(p("-80.2234"))), " -80");
    assert_eq!(format!("{:4.4}", P(p("-90"))), " -90");
    assert_eq!(format!("{:4.4}", P(p("-94.9"))), " -95");
    assert_eq!(format!("{:4.4}", P(p("-98.6783"))), " -99");
    assert_eq!(format!("{:4.4}", P(p("-100"))), "-100");
    assert_eq!(format!("{:4.4}", P(p("-164.3687"))), "-164");
    assert_eq!(format!("{:4.4}", P(p("-430.50597"))), "-431");
    assert_eq!(format!("{:4.4}", P(p("-466.5085"))), "-467");
    assert_eq!(format!("{:4.4}", P(p("-595.05"))), "-595");
    assert_eq!(format!("{:4.4}", P(p("-764.3112"))), "-764");
    assert_eq!(format!("{:4.4}", P(p("-807.142"))), "-807");
    assert_eq!(format!("{:4.4}", P(p("-5469.471"))), "-5e3");
    assert_eq!(format!("{:4.4}", P(p("-5651.6809"))), "-6e3");
    assert_eq!(format!("{:4.4}", P(p("-9706.81"))), "-1e4");
    assert_eq!(format!("{:4.4}", P(p("-93500"))), "-9e4");
    assert_eq!(format!("{:4.4}", P(p("-577000"))), "-6e5");
    assert_eq!(format!("{:4.4}", P(p("-835534"))), "-8e5");
    assert_eq!(format!("{:4.4}", P(p("-6000000"))), "-6e6");
}
#[test]
fn four_11() {
    assert_eq!(format!("{:4.4}", P(p("-900000000"))), "-9e8");
    assert_eq!(format!("{:4.4}", P(p("-279358770000"))), "####");
    assert_eq!(format!("{:4.4}", P(p("-50000000000000"))), "####");
}
fn _just_opening_brace_five() {
}
#[test]
fn five_1() {
    assert_eq!(format!("{:5.5}", P(p("906792980000000"))), "9e14");
    assert_eq!(format!("{:5.5}", P(p("6390900000000"))), "6e12");
    assert_eq!(format!("{:5.5}", P(p("28897000"))), "2.9e7");
    assert_eq!(format!("{:5.5}", P(p("700000"))), "7.0e5");
    assert_eq!(format!("{:5.5}", P(p("439620.1"))), "4.4e5");
    assert_eq!(format!("{:5.5}", P(p("9559.407"))), " 9559");
    assert_eq!(format!("{:5.5}", P(p("8022.2"))), " 8022");
    assert_eq!(format!("{:5.5}", P(p("6738.111"))), " 6738");
    assert_eq!(format!("{:5.5}", P(p("6208.24123131241232132142"))), " 6208");
    assert_eq!(format!("{:5.5}", P(p("5400"))), " 5400");
    assert_eq!(format!("{:5.5}", P(p("4741.878"))), " 4742");
    assert_eq!(format!("{:5.5}", P(p("3620.1461"))), " 3620");
    assert_eq!(format!("{:5.5}", P(p("3000.23451"))), " 3000");
    assert_eq!(format!("{:5.5}", P(p("2175.65"))), " 2176");
    assert_eq!(format!("{:5.5}", P(p("969.49"))), "969.5");
    assert_eq!(format!("{:5.5}", P(p("840.2056"))), "840.2");
    assert_eq!(format!("{:5.5}", P(p("620"))), "620.0");
    assert_eq!(format!("{:5.5}", P(p("407"))), "407.0");
    assert_eq!(format!("{:5.5}", P(p("401.249"))), "401.2");
    assert_eq!(format!("{:5.5}", P(p("233.021"))), "233.0");
    assert_eq!(format!("{:5.5}", P(p("96.503326"))), "96.50");
    assert_eq!(format!("{:5.5}", P(p("58.4"))), "58.40");
    assert_eq!(format!("{:5.5}", P(p("39.137"))), "39.14");
    assert_eq!(format!("{:5.5}", P(p("38.74"))), "38.74");
    assert_eq!(format!("{:5.5}", P(p("30"))), "30.00");
    assert_eq!(format!("{:5.5}", P(p("24.48179"))), "24.48");
    assert_eq!(format!("{:5.5}", P(p("21.123"))), "21.12");
    assert_eq!(format!("{:5.5}", P(p("9.95016e+246"))), "1e247");
    assert_eq!(format!("{:5.5}", P(p("9.8388"))), "9.84");
    assert_eq!(format!("{:5.5}", P(p("9.8059e+35"))), "1e36");
}
#[test]
fn five_2() {
    assert_eq!(format!("{:5.5}", P(p("9.530609e+22"))), "1e23");
    assert_eq!(format!("{:5.5}", P(p("9.46e+35"))), "9e35");
    assert_eq!(format!("{:5.5}", P(p("9.452105e-31"))), "9e-31");
    assert_eq!(format!("{:5.5}", P(p("9e+115"))), "9e115");
    assert_eq!(format!("{:5.5}", P(p("8.785953e+42"))), "9e42");
    assert_eq!(format!("{:5.5}", P(p("8.5e+20"))), "8e20");
    assert_eq!(format!("{:5.5}", P(p("8.3536e+30"))), "8e30");
    assert_eq!(format!("{:5.5}", P(p("8.3439e+25"))), "8e25");
    assert_eq!(format!("{:5.5}", P(p("8.27203e-18"))), "8e-18");
    assert_eq!(format!("{:5.5}", P(p("8.271221e-219"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("8.0927985e-112"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("8.07e-53"))), "8e-53");
    assert_eq!(format!("{:5.5}", P(p("8.062352e+38"))), "8e38");
    assert_eq!(format!("{:5.5}", P(p("8.0159e+36"))), "8e36");
    assert_eq!(format!("{:5.5}", P(p("8e+29"))), "8e29");
    assert_eq!(format!("{:5.5}", P(p("8e-24"))), "8e-24");
    assert_eq!(format!("{:5.5}", P(p("7.9954287e-194"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("7.83472e-22"))), "8e-22");
    assert_eq!(format!("{:5.5}", P(p("7.814"))), "7.814");
    assert_eq!(format!("{:5.5}", P(p("7.7715e+27"))), "8e27");
    assert_eq!(format!("{:5.5}", P(p("7.509e+38"))), "8e38");
    assert_eq!(format!("{:5.5}", P(p("7.331e-31"))), "7e-31");
    assert_eq!(format!("{:5.5}", P(p("7.2401e+226"))), "7e226");
    assert_eq!(format!("{:5.5}", P(p("7.2193e-18"))), "7e-18");
    assert_eq!(format!("{:5.5}", P(p("7.2e+19"))), "7e19");
    assert_eq!(format!("{:5.5}", P(p("7.142849e-170"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("7.0676e-17"))), "7e-17");
    assert_eq!(format!("{:5.5}", P(p("7e+39"))), "7e39");
    assert_eq!(format!("{:5.5}", P(p("7e+28"))), "7e28");
    assert_eq!(format!("{:5.5}", P(p("7"))), "7.000");
}
#[test]
fn five_3() {
    assert_eq!(format!("{:5.5}", P(p("6.844e+113"))), "7e113");
    assert_eq!(format!("{:5.5}", P(p("6.7853e-21"))), "7e-21");
    assert_eq!(format!("{:5.5}", P(p("6.75e-27"))), "7e-27");
    assert_eq!(format!("{:5.5}", P(p("6.56e+39"))), "7e39");
    assert_eq!(format!("{:5.5}", P(p("6.540688e-10"))), "7e-10");
    assert_eq!(format!("{:5.5}", P(p("6.4e-08"))), "6e-8");
    assert_eq!(format!("{:5.5}", P(p("6.389785e-262"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("6.3508e+42"))), "6e42");
    assert_eq!(format!("{:5.5}", P(p("6.3383e+47"))), "6e47");
    assert_eq!(format!("{:5.5}", P(p("6.295e+53"))), "6e53");
    assert_eq!(format!("{:5.5}", P(p("6.049"))), "6.049");
    assert_eq!(format!("{:5.5}", P(p("6.04"))), "6.040");
    assert_eq!(format!("{:5.5}", P(p("6.01341e-20"))), "6e-20");
    assert_eq!(format!("{:5.5}", P(p("5.98e+28"))), "6e28");
    assert_eq!(format!("{:5.5}", P(p("5.9e+43"))), "6e43");
    assert_eq!(format!("{:5.5}", P(p("5.865"))), "5.865");
    assert_eq!(format!("{:5.5}", P(p("5.7e-26"))), "6e-26");
    assert_eq!(format!("{:5.5}", P(p("5.695e-09"))), "6e-9");
    assert_eq!(format!("{:5.5}", P(p("5.61e-07"))), "6e-7");
    assert_eq!(format!("{:5.5}", P(p("5.55971e-235"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("5.4541311e-10"))), "5e-10");
    assert_eq!(format!("{:5.5}", P(p("5.1415063e-11"))), "5e-11");
    assert_eq!(format!("{:5.5}", P(p("5.13e+282"))), "5e282");
    assert_eq!(format!("{:5.5}", P(p("5.0369e+172"))), "5e172");
    assert_eq!(format!("{:5.5}", P(p("5.003824e+151"))), "5e151");
    assert_eq!(format!("{:5.5}", P(p("5e+263"))), "5e263");
    assert_eq!(format!("{:5.5}", P(p("5e+24"))), "5e24");
    assert_eq!(format!("{:5.5}", P(p("5e+19"))), "5e19");
    assert_eq!(format!("{:5.5}", P(p("5e+133"))), "5e133");
    assert_eq!(format!("{:5.5}", P(p("4.937e+40"))), "5e40");
}
#[test]
fn five_4() {
    assert_eq!(format!("{:5.5}", P(p("4.9361647e-133"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("4.585e+48"))), "5e48");
    assert_eq!(format!("{:5.5}", P(p("4.49e+26"))), "4e26");
    assert_eq!(format!("{:5.5}", P(p("4.4177e-134"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("4.3e-245"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("4.3e-16"))), "4e-16");
    assert_eq!(format!("{:5.5}", P(p("3.452077e-191"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("3.2"))), "3.200");
    assert_eq!(format!("{:5.5}", P(p("3e+25"))), "3e25");
    assert_eq!(format!("{:5.5}", P(p("3e+149"))), "3e149");
    assert_eq!(format!("{:5.5}", P(p("2.93e+32"))), "3e32");
    assert_eq!(format!("{:5.5}", P(p("2.916861e+44"))), "3e44");
    assert_eq!(format!("{:5.5}", P(p("2.9"))), "2.900");
    assert_eq!(format!("{:5.5}", P(p("2.764e-83"))), "3e-83");
    assert_eq!(format!("{:5.5}", P(p("2.49279e+34"))), "2e34");
    assert_eq!(format!("{:5.5}", P(p("2.413e-15"))), "2e-15");
    assert_eq!(format!("{:5.5}", P(p("2.2353e-129"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("2.166819e-23"))), "2e-23");
    assert_eq!(format!("{:5.5}", P(p("2e+31"))), "2e31");
    assert_eq!(format!("{:5.5}", P(p("2e+116"))), "2e116");
    assert_eq!(format!("{:5.5}", P(p("1.8e+33"))), "2e33");
    assert_eq!(format!("{:5.5}", P(p("1.7e-13"))), "2e-13");
    assert_eq!(format!("{:5.5}", P(p("1.4580931e+62"))), "1e62");
    assert_eq!(format!("{:5.5}", P(p("1.44471e-07"))), "1e-7");
    assert_eq!(format!("{:5.5}", P(p("1.393237e+46"))), "1e46");
    assert_eq!(format!("{:5.5}", P(p("1.39e+295"))), "1e295");
    assert_eq!(format!("{:5.5}", P(p("1.375e+17"))), "1e17");
    assert_eq!(format!("{:5.5}", P(p("1.293e+27"))), "1e27");
    assert_eq!(format!("{:5.5}", P(p("1.2041e-21"))), "1e-21");
    assert_eq!(format!("{:5.5}", P(p("1e+48"))), "1e48");
}
#[test]
fn five_5() {
    assert_eq!(format!("{:5.5}", P(p("1"))), "1.000");
    assert_eq!(format!("{:5.5}", P(p("0.98"))), "0.980");
    assert_eq!(format!("{:5.5}", P(p("0.973"))), "0.973");
    assert_eq!(format!("{:5.5}", P(p("0.945138"))), "0.945");
    assert_eq!(format!("{:5.5}", P(p("0.84"))), "0.840");
    assert_eq!(format!("{:5.5}", P(p("0.67"))), "0.670");
    assert_eq!(format!("{:5.5}", P(p("0.335"))), "0.335");
    assert_eq!(format!("{:5.5}", P(p("0.113"))), "0.113");
    assert_eq!(format!("{:5.5}", P(p("0.0983688"))), "0.098");
    assert_eq!(format!("{:5.5}", P(p("0.0906203"))), "0.091");
    assert_eq!(format!("{:5.5}", P(p("0.0869"))), "0.087");
    assert_eq!(format!("{:5.5}", P(p("0.05"))), "0.050");
    assert_eq!(format!("{:5.5}", P(p("0.047255829"))), "0.047");
    assert_eq!(format!("{:5.5}", P(p("0.028"))), "0.028");
    assert_eq!(format!("{:5.5}", P(p("0.009146"))), "0.009");
    assert_eq!(format!("{:5.5}", P(p("0.008581"))), "0.009");
    assert_eq!(format!("{:5.5}", P(p("0.008166"))), "0.008");
    assert_eq!(format!("{:5.5}", P(p("0.0081"))), "0.008");
    assert_eq!(format!("{:5.5}", P(p("0.008001"))), "0.008");
    assert_eq!(format!("{:5.5}", P(p("0.007889"))), "0.008");
    assert_eq!(format!("{:5.5}", P(p("0.006703542"))), "0.007");
    assert_eq!(format!("{:5.5}", P(p("0.0039"))), "0.004");
    assert_eq!(format!("{:5.5}", P(p("0.0030426"))), "0.003");
    assert_eq!(format!("{:5.5}", P(p("0.003"))), "0.003");
    assert_eq!(format!("{:5.5}", P(p("0.0029071"))), "0.003");
    assert_eq!(format!("{:5.5}", P(p("0.002191249"))), "0.002");
    assert_eq!(format!("{:5.5}", P(p("0.001910066"))), "0.002");
    assert_eq!(format!("{:5.5}", P(p("0.00092769"))), "9e-4");
    assert_eq!(format!("{:5.5}", P(p("0.0004"))), "4e-4");
    assert_eq!(format!("{:5.5}", P(p("0.000383036"))), "4e-4");
}
#[test]
fn five_6() {
    assert_eq!(format!("{:5.5}", P(p("0.0003"))), "3e-4");
    assert_eq!(format!("{:5.5}", P(p("0.000188222"))), "2e-4");
    assert_eq!(format!("{:5.5}", P(p("0.00016"))), "2e-4");
    assert_eq!(format!("{:5.5}", P(p("0.00014834"))), "1e-4");
    assert_eq!(format!("{:5.5}", P(p("NaN"))), "NaN  ");
    assert_eq!(format!("{:5.5}", P(p("-inf"))), "-inf ");
    assert_eq!(format!("{:5.5}", P(p("inf"))), "inf  ");
    assert_eq!(format!("{:5.5}", P(p("0"))), "0.000");
    assert_eq!(format!("{:5.5}", P(p("-0.00023816"))), "-2e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.000274"))), "-3e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.00031"))), "-3e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.00038509"))), "-4e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.000552594"))), "-6e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.0006028"))), "-6e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.000719834"))), "-7e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.0007820539"))), "-8e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.000869"))), "-9e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.00089806422"))), "-9e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.0009172"))), "-9e-4");
    assert_eq!(format!("{:5.5}", P(p("-0.00095719"))), "-1e-3");
    assert_eq!(format!("{:5.5}", P(p("-0.0060372"))), "-0.01");
    assert_eq!(format!("{:5.5}", P(p("-0.0064726046"))), "-0.01");
    assert_eq!(format!("{:5.5}", P(p("-0.0092191939"))), "-0.01");
    assert_eq!(format!("{:5.5}", P(p("-0.019099"))), "-0.02");
    assert_eq!(format!("{:5.5}", P(p("-0.022925"))), "-0.02");
    assert_eq!(format!("{:5.5}", P(p("-0.02826"))), "-0.03");
    assert_eq!(format!("{:5.5}", P(p("-0.0546163"))), "-0.05");
    assert_eq!(format!("{:5.5}", P(p("-0.062"))), "-0.06");
    assert_eq!(format!("{:5.5}", P(p("-0.0764018"))), "-0.08");
    assert_eq!(format!("{:5.5}", P(p("-0.0929804"))), "-0.09");
}
#[test]
fn five_7() {
    assert_eq!(format!("{:5.5}", P(p("-0.1689"))), "-0.17");
    assert_eq!(format!("{:5.5}", P(p("-0.2"))), "-0.20");
    assert_eq!(format!("{:5.5}", P(p("-0.4946795"))), "-0.49");
    assert_eq!(format!("{:5.5}", P(p("-0.67014714"))), "-0.67");
    assert_eq!(format!("{:5.5}", P(p("-0.68"))), "-0.68");
    assert_eq!(format!("{:5.5}", P(p("-0.68815"))), "-0.69");
    assert_eq!(format!("{:5.5}", P(p("-0.7562"))), "-0.76");
    assert_eq!(format!("{:5.5}", P(p("-1e+29"))), "-1e29");
    assert_eq!(format!("{:5.5}", P(p("-1.064846e-26"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-1.51624e-234"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-1.6965898e-119"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-1.7695e-236"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-1.93885e+19"))), "-2e19");
    assert_eq!(format!("{:5.5}", P(p("-2.31373e+31"))), "-2e31");
    assert_eq!(format!("{:5.5}", P(p("-2.4527809e-306"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-2.6136e-09"))), "-3e-9");
    assert_eq!(format!("{:5.5}", P(p("-2.691e-11"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-2.819"))), "-2.82");
    assert_eq!(format!("{:5.5}", P(p("-2.8322"))), "-2.83");
    assert_eq!(format!("{:5.5}", P(p("-3e+45"))), "-3e45");
    assert_eq!(format!("{:5.5}", P(p("-3e-05"))), "-3e-5");
    assert_eq!(format!("{:5.5}", P(p("-3.05e+25"))), "-3e25");
    assert_eq!(format!("{:5.5}", P(p("-3.108287e+278"))), "#####");
    assert_eq!(format!("{:5.5}", P(p("-3.16584e+41"))), "-3e41");
    assert_eq!(format!("{:5.5}", P(p("-3.17e-222"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-3.2348135"))), "-3.23");
    assert_eq!(format!("{:5.5}", P(p("-3.465e+22"))), "-3e22");
    assert_eq!(format!("{:5.5}", P(p("-3.63e-135"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-3.72e+45"))), "-4e45");
    assert_eq!(format!("{:5.5}", P(p("-3.9e+30"))), "-4e30");
}
#[test]
fn five_8() {
    assert_eq!(format!("{:5.5}", P(p("-4.2"))), "-4.20");
    assert_eq!(format!("{:5.5}", P(p("-4.208329e-20"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-4.2888733"))), "-4.29");
    assert_eq!(format!("{:5.5}", P(p("-4.297e-08"))), "-4e-8");
    assert_eq!(format!("{:5.5}", P(p("-4.60469181924042e-321"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-4.7142e+36"))), "-5e36");
    assert_eq!(format!("{:5.5}", P(p("-4.767"))), "-4.77");
    assert_eq!(format!("{:5.5}", P(p("-4.863526e-20"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-4.868e+33"))), "-5e33");
    assert_eq!(format!("{:5.5}", P(p("-4.96e+247"))), "#####");
    assert_eq!(format!("{:5.5}", P(p("-5e-29"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.05e-218"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.169414e+37"))), "-5e37");
    assert_eq!(format!("{:5.5}", P(p("-5.20816e-06"))), "-5e-6");
    assert_eq!(format!("{:5.5}", P(p("-5.263e+21"))), "-5e21");
    assert_eq!(format!("{:5.5}", P(p("-5.4239467e-31"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.44067e-263"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.543976e+32"))), "-6e32");
    assert_eq!(format!("{:5.5}", P(p("-5.56122e-29"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.6e+35"))), "-6e35");
    assert_eq!(format!("{:5.5}", P(p("-5.65896e-290"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.71654"))), "-5.72");
    assert_eq!(format!("{:5.5}", P(p("-5.730186e+123"))), "#####");
    assert_eq!(format!("{:5.5}", P(p("-5.798598e-21"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-5.81165e-212"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-6e-25"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-6e-06"))), "-6e-6");
    assert_eq!(format!("{:5.5}", P(p("-6.07e-268"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-6.074991"))), "-6.07");
    assert_eq!(format!("{:5.5}", P(p("-6.1e+290"))), "#####");
}
#[test]
fn five_9() {
    assert_eq!(format!("{:5.5}", P(p("-6.1206e-06"))), "-6e-6");
    assert_eq!(format!("{:5.5}", P(p("-6.5986e+45"))), "-7e45");
    assert_eq!(format!("{:5.5}", P(p("-6.785938e+26"))), "-7e26");
    assert_eq!(format!("{:5.5}", P(p("-6.8e-30"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-6.921e-22"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-7e-57"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-7.0915e+22"))), "-7e22");
    assert_eq!(format!("{:5.5}", P(p("-7.15407055178125e-321"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-7.60733e+45"))), "-8e45");
    assert_eq!(format!("{:5.5}", P(p("-7.79173e-05"))), "-8e-5");
    assert_eq!(format!("{:5.5}", P(p("-8e-26"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8e+20"))), "-8e20");
    assert_eq!(format!("{:5.5}", P(p("-8.245e-145"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.28043e+21"))), "-8e21");
    assert_eq!(format!("{:5.5}", P(p("-8.2888989e-25"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.37662e-158"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.38e-30"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.4e-23"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.5e+25"))), "-9e25");
    assert_eq!(format!("{:5.5}", P(p("-8.8165e-23"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.821e-20"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-8.8805774e+129"))), "#####");
    assert_eq!(format!("{:5.5}", P(p("-8.91e+18"))), "-9e18");
    assert_eq!(format!("{:5.5}", P(p("-9e-19"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-9.03e-286"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-9.13"))), "-9.13");
    assert_eq!(format!("{:5.5}", P(p("-9.185594e-120"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-9.3e-70"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-9.451309e-30"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-9.494134e+65"))), "-9e65");
}
#[test]
fn five_10() {
    assert_eq!(format!("{:5.5}", P(p("-9.54e-24"))), "    0");
    assert_eq!(format!("{:5.5}", P(p("-9.57259e+240"))), "#####");
    assert_eq!(format!("{:5.5}", P(p("-27.1783"))), "-27.2");
    assert_eq!(format!("{:5.5}", P(p("-32.234"))), "-32.2");
    assert_eq!(format!("{:5.5}", P(p("-46.430046"))), "-46.4");
    assert_eq!(format!("{:5.5}", P(p("-48.1"))), "-48.1");
    assert_eq!(format!("{:5.5}", P(p("-48.307"))), "-48.3");
    assert_eq!(format!("{:5.5}", P(p("-50"))), "-50.0");
    assert_eq!(format!("{:5.5}", P(p("-57.28"))), "-57.3");
    assert_eq!(format!("{:5.5}", P(p("-60.094"))), "-60.1");
    assert_eq!(format!("{:5.5}", P(p("-62.01612"))), "-62.0");
    assert_eq!(format!("{:5.5}", P(p("-64.92"))), "-64.9");
    assert_eq!(format!("{:5.5}", P(p("-80.2234"))), "-80.2");
    assert_eq!(format!("{:5.5}", P(p("-90"))), "-90.0");
    assert_eq!(format!("{:5.5}", P(p("-94.9"))), "-94.9");
    assert_eq!(format!("{:5.5}", P(p("-98.6783"))), "-98.7");
    assert_eq!(format!("{:5.5}", P(p("-100"))), " -100");
    assert_eq!(format!("{:5.5}", P(p("-164.3687"))), " -164");
    assert_eq!(format!("{:5.5}", P(p("-430.50597"))), " -431");
    assert_eq!(format!("{:5.5}", P(p("-466.5085"))), " -467");
    assert_eq!(format!("{:5.5}", P(p("-595.05"))), " -595");
    assert_eq!(format!("{:5.5}", P(p("-764.3112"))), " -764");
    assert_eq!(format!("{:5.5}", P(p("-807.142"))), " -807");
    assert_eq!(format!("{:5.5}", P(p("-5469.471"))), "-5469");
    assert_eq!(format!("{:5.5}", P(p("-5651.6809"))), "-5652");
    assert_eq!(format!("{:5.5}", P(p("-9706.81"))), "-9707");
    assert_eq!(format!("{:5.5}", P(p("-93500"))), "-9e4");
    assert_eq!(format!("{:5.5}", P(p("-577000"))), "-6e5");
    assert_eq!(format!("{:5.5}", P(p("-835534"))), "-8e5");
    assert_eq!(format!("{:5.5}", P(p("-6000000"))), "-6e6");
}
#[test]
fn five_11() {
    assert_eq!(format!("{:5.5}", P(p("-900000000"))), "-9e8");
    assert_eq!(format!("{:5.5}", P(p("-279358770000"))), "-3e11");
    assert_eq!(format!("{:5.5}", P(p("-50000000000000"))), "-5e13");
}
fn _just_opening_brace_six() {
}
#[test]
fn six_1() {
    assert_eq!(format!("{:6.6}", P(p("906792980000000"))), "9.1e14");
    assert_eq!(format!("{:6.6}", P(p("6390900000000"))), "6.4e12");
    assert_eq!(format!("{:6.6}", P(p("28897000"))), "2.89e7");
    assert_eq!(format!("{:6.6}", P(p("700000"))), "7.00e5");
    assert_eq!(format!("{:6.6}", P(p("439620.1"))), "4.40e5");
    assert_eq!(format!("{:6.6}", P(p("9559.407"))), "9559.4");
    assert_eq!(format!("{:6.6}", P(p("8022.2"))), "8022.2");
    assert_eq!(format!("{:6.6}", P(p("6738.111"))), "6738.1");
    assert_eq!(format!("{:6.6}", P(p("6208.24123131241232132142"))), "6208.2");
    assert_eq!(format!("{:6.6}", P(p("5400"))), "5400.0");
    assert_eq!(format!("{:6.6}", P(p("4741.878"))), "4741.9");
    assert_eq!(format!("{:6.6}", P(p("3620.1461"))), "3620.1");
    assert_eq!(format!("{:6.6}", P(p("3000.23451"))), "3000.2");
    assert_eq!(format!("{:6.6}", P(p("2175.65"))), "2175.7");
    assert_eq!(format!("{:6.6}", P(p("969.49"))), "969.49");
    assert_eq!(format!("{:6.6}", P(p("840.2056"))), "840.21");
    assert_eq!(format!("{:6.6}", P(p("620"))), "620.00");
    assert_eq!(format!("{:6.6}", P(p("407"))), "407.00");
    assert_eq!(format!("{:6.6}", P(p("401.249"))), "401.25");
    assert_eq!(format!("{:6.6}", P(p("233.021"))), "233.02");
    assert_eq!(format!("{:6.6}", P(p("96.503326"))), "96.503");
    assert_eq!(format!("{:6.6}", P(p("58.4"))), "58.400");
    assert_eq!(format!("{:6.6}", P(p("39.137"))), "39.137");
    assert_eq!(format!("{:6.6}", P(p("38.74"))), "38.740");
    assert_eq!(format!("{:6.6}", P(p("30"))), "30.000");
    assert_eq!(format!("{:6.6}", P(p("24.48179"))), "24.482");
    assert_eq!(format!("{:6.6}", P(p("21.123"))), "21.123");
    assert_eq!(format!("{:6.6}", P(p("9.95016e+246"))), "1e247");
    assert_eq!(format!("{:6.6}", P(p("9.8388"))), "9.839");
    assert_eq!(format!("{:6.6}", P(p("9.8059e+35"))), "9.8e35");
}
#[test]
fn six_2() {
    assert_eq!(format!("{:6.6}", P(p("9.530609e+22"))), "9.5e22");
    assert_eq!(format!("{:6.6}", P(p("9.46e+35"))), "9.5e35");
    assert_eq!(format!("{:6.6}", P(p("9.452105e-31"))), "9e-31");
    assert_eq!(format!("{:6.6}", P(p("9e+115"))), "9e115");
    assert_eq!(format!("{:6.6}", P(p("8.785953e+42"))), "8.8e42");
    assert_eq!(format!("{:6.6}", P(p("8.5e+20"))), "8.5e20");
    assert_eq!(format!("{:6.6}", P(p("8.3536e+30"))), "8.4e30");
    assert_eq!(format!("{:6.6}", P(p("8.3439e+25"))), "8.3e25");
    assert_eq!(format!("{:6.6}", P(p("8.27203e-18"))), "8e-18");
    assert_eq!(format!("{:6.6}", P(p("8.271221e-219"))), "8e-219");
    assert_eq!(format!("{:6.6}", P(p("8.0927985e-112"))), "8e-112");
    assert_eq!(format!("{:6.6}", P(p("8.07e-53"))), "8e-53");
    assert_eq!(format!("{:6.6}", P(p("8.062352e+38"))), "8.1e38");
    assert_eq!(format!("{:6.6}", P(p("8.0159e+36"))), "8.0e36");
    assert_eq!(format!("{:6.6}", P(p("8e+29"))), "8.0e29");
    assert_eq!(format!("{:6.6}", P(p("8e-24"))), "8e-24");
    assert_eq!(format!("{:6.6}", P(p("7.9954287e-194"))), "8e-194");
    assert_eq!(format!("{:6.6}", P(p("7.83472e-22"))), "8e-22");
    assert_eq!(format!("{:6.6}", P(p("7.814"))), "7.8140");
    assert_eq!(format!("{:6.6}", P(p("7.7715e+27"))), "7.8e27");
    assert_eq!(format!("{:6.6}", P(p("7.509e+38"))), "7.5e38");
    assert_eq!(format!("{:6.6}", P(p("7.331e-31"))), "7e-31");
    assert_eq!(format!("{:6.6}", P(p("7.2401e+226"))), "7e226");
    assert_eq!(format!("{:6.6}", P(p("7.2193e-18"))), "7e-18");
    assert_eq!(format!("{:6.6}", P(p("7.2e+19"))), "7.2e19");
    assert_eq!(format!("{:6.6}", P(p("7.142849e-170"))), "7e-170");
    assert_eq!(format!("{:6.6}", P(p("7.0676e-17"))), "7e-17");
    assert_eq!(format!("{:6.6}", P(p("7e+39"))), "7.0e39");
    assert_eq!(format!("{:6.6}", P(p("7e+28"))), "7.0e28");
    assert_eq!(format!("{:6.6}", P(p("7"))), "7.0000");
}
#[test]
fn six_3() {
    assert_eq!(format!("{:6.6}", P(p("6.844e+113"))), "7e113");
    assert_eq!(format!("{:6.6}", P(p("6.7853e-21"))), "7e-21");
    assert_eq!(format!("{:6.6}", P(p("6.75e-27"))), "7e-27");
    assert_eq!(format!("{:6.6}", P(p("6.56e+39"))), "6.6e39");
    assert_eq!(format!("{:6.6}", P(p("6.540688e-10"))), "7e-10");
    assert_eq!(format!("{:6.6}", P(p("6.4e-08"))), "6.4e-8");
    assert_eq!(format!("{:6.6}", P(p("6.389785e-262"))), "6e-262");
    assert_eq!(format!("{:6.6}", P(p("6.3508e+42"))), "6.4e42");
    assert_eq!(format!("{:6.6}", P(p("6.3383e+47"))), "6.3e47");
    assert_eq!(format!("{:6.6}", P(p("6.295e+53"))), "6.3e53");
    assert_eq!(format!("{:6.6}", P(p("6.049"))), "6.0490");
    assert_eq!(format!("{:6.6}", P(p("6.04"))), "6.0400");
    assert_eq!(format!("{:6.6}", P(p("6.01341e-20"))), "6e-20");
    assert_eq!(format!("{:6.6}", P(p("5.98e+28"))), "6.0e28");
    assert_eq!(format!("{:6.6}", P(p("5.9e+43"))), "5.9e43");
    assert_eq!(format!("{:6.6}", P(p("5.865"))), "5.8650");
    assert_eq!(format!("{:6.6}", P(p("5.7e-26"))), "6e-26");
    assert_eq!(format!("{:6.6}", P(p("5.695e-09"))), "5.7e-9");
    assert_eq!(format!("{:6.6}", P(p("5.61e-07"))), "5.6e-7");
    assert_eq!(format!("{:6.6}", P(p("5.55971e-235"))), "6e-235");
    assert_eq!(format!("{:6.6}", P(p("5.4541311e-10"))), "5e-10");
    assert_eq!(format!("{:6.6}", P(p("5.1415063e-11"))), "5e-11");
    assert_eq!(format!("{:6.6}", P(p("5.13e+282"))), "5e282");
    assert_eq!(format!("{:6.6}", P(p("5.0369e+172"))), "5e172");
    assert_eq!(format!("{:6.6}", P(p("5.003824e+151"))), "5e151");
    assert_eq!(format!("{:6.6}", P(p("5e+263"))), "5e263");
    assert_eq!(format!("{:6.6}", P(p("5e+24"))), "5.0e24");
    assert_eq!(format!("{:6.6}", P(p("5e+19"))), "5.0e19");
    assert_eq!(format!("{:6.6}", P(p("5e+133"))), "5e133");
    assert_eq!(format!("{:6.6}", P(p("4.937e+40"))), "4.9e40");
}
#[test]
fn six_4() {
    assert_eq!(format!("{:6.6}", P(p("4.9361647e-133"))), "5e-133");
    assert_eq!(format!("{:6.6}", P(p("4.585e+48"))), "4.6e48");
    assert_eq!(format!("{:6.6}", P(p("4.49e+26"))), "4.5e26");
    assert_eq!(format!("{:6.6}", P(p("4.4177e-134"))), "4e-134");
    assert_eq!(format!("{:6.6}", P(p("4.3e-245"))), "4e-245");
    assert_eq!(format!("{:6.6}", P(p("4.3e-16"))), "4e-16");
    assert_eq!(format!("{:6.6}", P(p("3.452077e-191"))), "3e-191");
    assert_eq!(format!("{:6.6}", P(p("3.2"))), "3.2000");
    assert_eq!(format!("{:6.6}", P(p("3e+25"))), "3.0e25");
    assert_eq!(format!("{:6.6}", P(p("3e+149"))), "3e149");
    assert_eq!(format!("{:6.6}", P(p("2.93e+32"))), "2.9e32");
    assert_eq!(format!("{:6.6}", P(p("2.916861e+44"))), "2.9e44");
    assert_eq!(format!("{:6.6}", P(p("2.9"))), "2.9000");
    assert_eq!(format!("{:6.6}", P(p("2.764e-83"))), "3e-83");
    assert_eq!(format!("{:6.6}", P(p("2.49279e+34"))), "2.5e34");
    assert_eq!(format!("{:6.6}", P(p("2.413e-15"))), "2e-15");
    assert_eq!(format!("{:6.6}", P(p("2.2353e-129"))), "2e-129");
    assert_eq!(format!("{:6.6}", P(p("2.166819e-23"))), "2e-23");
    assert_eq!(format!("{:6.6}", P(p("2e+31"))), "2.0e31");
    assert_eq!(format!("{:6.6}", P(p("2e+116"))), "2e116");
    assert_eq!(format!("{:6.6}", P(p("1.8e+33"))), "1.8e33");
    assert_eq!(format!("{:6.6}", P(p("1.7e-13"))), "2e-13");
    assert_eq!(format!("{:6.6}", P(p("1.4580931e+62"))), "1.5e62");
    assert_eq!(format!("{:6.6}", P(p("1.44471e-07"))), "1.4e-7");
    assert_eq!(format!("{:6.6}", P(p("1.393237e+46"))), "1.4e46");
    assert_eq!(format!("{:6.6}", P(p("1.39e+295"))), "1e295");
    assert_eq!(format!("{:6.6}", P(p("1.375e+17"))), "1.4e17");
    assert_eq!(format!("{:6.6}", P(p("1.293e+27"))), "1.3e27");
    assert_eq!(format!("{:6.6}", P(p("1.2041e-21"))), "1e-21");
    assert_eq!(format!("{:6.6}", P(p("1e+48"))), "1.0e48");
}
#[test]
fn six_5() {
    assert_eq!(format!("{:6.6}", P(p("1"))), "1.0000");
    assert_eq!(format!("{:6.6}", P(p("0.98"))), "0.9800");
    assert_eq!(format!("{:6.6}", P(p("0.973"))), "0.9730");
    assert_eq!(format!("{:6.6}", P(p("0.945138"))), "0.9451");
    assert_eq!(format!("{:6.6}", P(p("0.84"))), "0.8400");
    assert_eq!(format!("{:6.6}", P(p("0.67"))), "0.6700");
    assert_eq!(format!("{:6.6}", P(p("0.335"))), "0.3350");
    assert_eq!(format!("{:6.6}", P(p("0.113"))), "0.1130");
    assert_eq!(format!("{:6.6}", P(p("0.0983688"))), "0.0984");
    assert_eq!(format!("{:6.6}", P(p("0.0906203"))), "0.0906");
    assert_eq!(format!("{:6.6}", P(p("0.0869"))), "0.0869");
    assert_eq!(format!("{:6.6}", P(p("0.05"))), "0.0500");
    assert_eq!(format!("{:6.6}", P(p("0.047255829"))), "0.0473");
    assert_eq!(format!("{:6.6}", P(p("0.028"))), "0.0280");
    assert_eq!(format!("{:6.6}", P(p("0.009146"))), "0.0091");
    assert_eq!(format!("{:6.6}", P(p("0.008581"))), "0.0086");
    assert_eq!(format!("{:6.6}", P(p("0.008166"))), "0.0082");
    assert_eq!(format!("{:6.6}", P(p("0.0081"))), "0.0081");
    assert_eq!(format!("{:6.6}", P(p("0.008001"))), "0.0080");
    assert_eq!(format!("{:6.6}", P(p("0.007889"))), "0.0079");
    assert_eq!(format!("{:6.6}", P(p("0.006703542"))), "0.0067");
    assert_eq!(format!("{:6.6}", P(p("0.0039"))), "0.0039");
    assert_eq!(format!("{:6.6}", P(p("0.0030426"))), "0.0030");
    assert_eq!(format!("{:6.6}", P(p("0.003"))), "0.0030");
    assert_eq!(format!("{:6.6}", P(p("0.0029071"))), "0.0029");
    assert_eq!(format!("{:6.6}", P(p("0.002191249"))), "0.0022");
    assert_eq!(format!("{:6.6}", P(p("0.001910066"))), "0.0019");
    assert_eq!(format!("{:6.6}", P(p("0.00092769"))), "9.3e-4");
    assert_eq!(format!("{:6.6}", P(p("0.0004"))), "4.0e-4");
    assert_eq!(format!("{:6.6}", P(p("0.000383036"))), "3.8e-4");
}
#[test]
fn six_6() {
    assert_eq!(format!("{:6.6}", P(p("0.0003"))), "3.0e-4");
    assert_eq!(format!("{:6.6}", P(p("0.000188222"))), "1.9e-4");
    assert_eq!(format!("{:6.6}", P(p("0.00016"))), "1.6e-4");
    assert_eq!(format!("{:6.6}", P(p("0.00014834"))), "1.5e-4");
    assert_eq!(format!("{:6.6}", P(p("NaN"))), "NaN   ");
    assert_eq!(format!("{:6.6}", P(p("-inf"))), "-inf  ");
    assert_eq!(format!("{:6.6}", P(p("inf"))), "inf   ");
    assert_eq!(format!("{:6.6}", P(p("0"))), "0.0000");
    assert_eq!(format!("{:6.6}", P(p("-0.00023816"))), "-2e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.000274"))), "-3e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.00031"))), "-3e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.00038509"))), "-4e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.000552594"))), "-6e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.0006028"))), "-6e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.000719834"))), "-7e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.0007820539"))), "-8e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.000869"))), "-9e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.00089806422"))), "-9e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.0009172"))), "-9e-4");
    assert_eq!(format!("{:6.6}", P(p("-0.00095719"))), "-1e-3");
    assert_eq!(format!("{:6.6}", P(p("-0.0060372"))), "-0.006");
    assert_eq!(format!("{:6.6}", P(p("-0.0064726046"))), "-0.006");
    assert_eq!(format!("{:6.6}", P(p("-0.0092191939"))), "-0.009");
    assert_eq!(format!("{:6.6}", P(p("-0.019099"))), "-0.019");
    assert_eq!(format!("{:6.6}", P(p("-0.022925"))), "-0.023");
    assert_eq!(format!("{:6.6}", P(p("-0.02826"))), "-0.028");
    assert_eq!(format!("{:6.6}", P(p("-0.0546163"))), "-0.055");
    assert_eq!(format!("{:6.6}", P(p("-0.062"))), "-0.062");
    assert_eq!(format!("{:6.6}", P(p("-0.0764018"))), "-0.076");
    assert_eq!(format!("{:6.6}", P(p("-0.0929804"))), "-0.093");
}
#[test]
fn six_7() {
    assert_eq!(format!("{:6.6}", P(p("-0.1689"))), "-0.169");
    assert_eq!(format!("{:6.6}", P(p("-0.2"))), "-0.200");
    assert_eq!(format!("{:6.6}", P(p("-0.4946795"))), "-0.495");
    assert_eq!(format!("{:6.6}", P(p("-0.67014714"))), "-0.670");
    assert_eq!(format!("{:6.6}", P(p("-0.68"))), "-0.680");
    assert_eq!(format!("{:6.6}", P(p("-0.68815"))), "-0.688");
    assert_eq!(format!("{:6.6}", P(p("-0.7562"))), "-0.756");
    assert_eq!(format!("{:6.6}", P(p("-1e+29"))), "-1e29");
    assert_eq!(format!("{:6.6}", P(p("-1.064846e-26"))), "-1e-26");
    assert_eq!(format!("{:6.6}", P(p("-1.51624e-234"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-1.6965898e-119"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-1.7695e-236"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-1.93885e+19"))), "-2e19");
    assert_eq!(format!("{:6.6}", P(p("-2.31373e+31"))), "-2e31");
    assert_eq!(format!("{:6.6}", P(p("-2.4527809e-306"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-2.6136e-09"))), "-3e-9");
    assert_eq!(format!("{:6.6}", P(p("-2.691e-11"))), "-3e-11");
    assert_eq!(format!("{:6.6}", P(p("-2.819"))), "-2.819");
    assert_eq!(format!("{:6.6}", P(p("-2.8322"))), "-2.832");
    assert_eq!(format!("{:6.6}", P(p("-3e+45"))), "-3e45");
    assert_eq!(format!("{:6.6}", P(p("-3e-05"))), "-3e-5");
    assert_eq!(format!("{:6.6}", P(p("-3.05e+25"))), "-3e25");
    assert_eq!(format!("{:6.6}", P(p("-3.108287e+278"))), "-3e278");
    assert_eq!(format!("{:6.6}", P(p("-3.16584e+41"))), "-3e41");
    assert_eq!(format!("{:6.6}", P(p("-3.17e-222"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-3.2348135"))), "-3.235");
    assert_eq!(format!("{:6.6}", P(p("-3.465e+22"))), "-3e22");
    assert_eq!(format!("{:6.6}", P(p("-3.63e-135"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-3.72e+45"))), "-4e45");
    assert_eq!(format!("{:6.6}", P(p("-3.9e+30"))), "-4e30");
}
#[test]
fn six_8() {
    assert_eq!(format!("{:6.6}", P(p("-4.2"))), "-4.200");
    assert_eq!(format!("{:6.6}", P(p("-4.208329e-20"))), "-4e-20");
    assert_eq!(format!("{:6.6}", P(p("-4.2888733"))), "-4.289");
    assert_eq!(format!("{:6.6}", P(p("-4.297e-08"))), "-4e-8");
    assert_eq!(format!("{:6.6}", P(p("-4.60469181924042e-321"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-4.7142e+36"))), "-5e36");
    assert_eq!(format!("{:6.6}", P(p("-4.767"))), "-4.767");
    assert_eq!(format!("{:6.6}", P(p("-4.863526e-20"))), "-5e-20");
    assert_eq!(format!("{:6.6}", P(p("-4.868e+33"))), "-5e33");
    assert_eq!(format!("{:6.6}", P(p("-4.96e+247"))), "-5e247");
    assert_eq!(format!("{:6.6}", P(p("-5e-29"))), "-5e-29");
    assert_eq!(format!("{:6.6}", P(p("-5.05e-218"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-5.169414e+37"))), "-5e37");
    assert_eq!(format!("{:6.6}", P(p("-5.20816e-06"))), "-5e-6");
    assert_eq!(format!("{:6.6}", P(p("-5.263e+21"))), "-5e21");
    assert_eq!(format!("{:6.6}", P(p("-5.4239467e-31"))), "-5e-31");
    assert_eq!(format!("{:6.6}", P(p("-5.44067e-263"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-5.543976e+32"))), "-6e32");
    assert_eq!(format!("{:6.6}", P(p("-5.56122e-29"))), "-6e-29");
    assert_eq!(format!("{:6.6}", P(p("-5.6e+35"))), "-6e35");
    assert_eq!(format!("{:6.6}", P(p("-5.65896e-290"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-5.71654"))), "-5.717");
    assert_eq!(format!("{:6.6}", P(p("-5.730186e+123"))), "-6e123");
    assert_eq!(format!("{:6.6}", P(p("-5.798598e-21"))), "-6e-21");
    assert_eq!(format!("{:6.6}", P(p("-5.81165e-212"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-6e-25"))), "-6e-25");
    assert_eq!(format!("{:6.6}", P(p("-6e-06"))), "-6e-6");
    assert_eq!(format!("{:6.6}", P(p("-6.07e-268"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-6.074991"))), "-6.075");
    assert_eq!(format!("{:6.6}", P(p("-6.1e+290"))), "-6e290");
}
#[test]
fn six_9() {
    assert_eq!(format!("{:6.6}", P(p("-6.1206e-06"))), "-6e-6");
    assert_eq!(format!("{:6.6}", P(p("-6.5986e+45"))), "-7e45");
    assert_eq!(format!("{:6.6}", P(p("-6.785938e+26"))), "-7e26");
    assert_eq!(format!("{:6.6}", P(p("-6.8e-30"))), "-7e-30");
    assert_eq!(format!("{:6.6}", P(p("-6.921e-22"))), "-7e-22");
    assert_eq!(format!("{:6.6}", P(p("-7e-57"))), "-7e-57");
    assert_eq!(format!("{:6.6}", P(p("-7.0915e+22"))), "-7e22");
    assert_eq!(format!("{:6.6}", P(p("-7.15407055178125e-321"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-7.60733e+45"))), "-8e45");
    assert_eq!(format!("{:6.6}", P(p("-7.79173e-05"))), "-8e-5");
    assert_eq!(format!("{:6.6}", P(p("-8e-26"))), "-8e-26");
    assert_eq!(format!("{:6.6}", P(p("-8e+20"))), "-8e20");
    assert_eq!(format!("{:6.6}", P(p("-8.245e-145"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-8.28043e+21"))), "-8e21");
    assert_eq!(format!("{:6.6}", P(p("-8.2888989e-25"))), "-8e-25");
    assert_eq!(format!("{:6.6}", P(p("-8.37662e-158"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-8.38e-30"))), "-8e-30");
    assert_eq!(format!("{:6.6}", P(p("-8.4e-23"))), "-8e-23");
    assert_eq!(format!("{:6.6}", P(p("-8.5e+25"))), "-9e25");
    assert_eq!(format!("{:6.6}", P(p("-8.8165e-23"))), "-9e-23");
    assert_eq!(format!("{:6.6}", P(p("-8.821e-20"))), "-9e-20");
    assert_eq!(format!("{:6.6}", P(p("-8.8805774e+129"))), "-9e129");
    assert_eq!(format!("{:6.6}", P(p("-8.91e+18"))), "-9e18");
    assert_eq!(format!("{:6.6}", P(p("-9e-19"))), "-9e-19");
    assert_eq!(format!("{:6.6}", P(p("-9.03e-286"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-9.13"))), "-9.130");
    assert_eq!(format!("{:6.6}", P(p("-9.185594e-120"))), "     0");
    assert_eq!(format!("{:6.6}", P(p("-9.3e-70"))), "-9e-70");
    assert_eq!(format!("{:6.6}", P(p("-9.451309e-30"))), "-9e-30");
    assert_eq!(format!("{:6.6}", P(p("-9.494134e+65"))), "-9e65");
}
#[test]
fn six_10() {
    assert_eq!(format!("{:6.6}", P(p("-9.54e-24"))), "-1e-23");
    assert_eq!(format!("{:6.6}", P(p("-9.57259e+240"))), "-1e241");
    assert_eq!(format!("{:6.6}", P(p("-27.1783"))), "-27.18");
    assert_eq!(format!("{:6.6}", P(p("-32.234"))), "-32.23");
    assert_eq!(format!("{:6.6}", P(p("-46.430046"))), "-46.43");
    assert_eq!(format!("{:6.6}", P(p("-48.1"))), "-48.10");
    assert_eq!(format!("{:6.6}", P(p("-48.307"))), "-48.31");
    assert_eq!(format!("{:6.6}", P(p("-50"))), "-50.00");
    assert_eq!(format!("{:6.6}", P(p("-57.28"))), "-57.28");
    assert_eq!(format!("{:6.6}", P(p("-60.094"))), "-60.09");
    assert_eq!(format!("{:6.6}", P(p("-62.01612"))), "-62.02");
    assert_eq!(format!("{:6.6}", P(p("-64.92"))), "-64.92");
    assert_eq!(format!("{:6.6}", P(p("-80.2234"))), "-80.22");
    assert_eq!(format!("{:6.6}", P(p("-90"))), "-90.00");
    assert_eq!(format!("{:6.6}", P(p("-94.9"))), "-94.90");
    assert_eq!(format!("{:6.6}", P(p("-98.6783"))), "-98.68");
    assert_eq!(format!("{:6.6}", P(p("-100"))), "-100.0");
    assert_eq!(format!("{:6.6}", P(p("-164.3687"))), "-164.4");
    assert_eq!(format!("{:6.6}", P(p("-430.50597"))), "-430.5");
    assert_eq!(format!("{:6.6}", P(p("-466.5085"))), "-466.5");
    assert_eq!(format!("{:6.6}", P(p("-595.05"))), "-595.0");
    assert_eq!(format!("{:6.6}", P(p("-764.3112"))), "-764.3");
    assert_eq!(format!("{:6.6}", P(p("-807.142"))), "-807.1");
    assert_eq!(format!("{:6.6}", P(p("-5469.471"))), " -5469");
    assert_eq!(format!("{:6.6}", P(p("-5651.6809"))), " -5652");
    assert_eq!(format!("{:6.6}", P(p("-9706.81"))), " -9707");
    assert_eq!(format!("{:6.6}", P(p("-93500"))), "-93500");
    assert_eq!(format!("{:6.6}", P(p("-577000"))), "-5.8e5");
    assert_eq!(format!("{:6.6}", P(p("-835534"))), "-8.4e5");
    assert_eq!(format!("{:6.6}", P(p("-6000000"))), "-6.0e6");
}
#[test]
fn six_11() {
    assert_eq!(format!("{:6.6}", P(p("-900000000"))), "-9.0e8");
    assert_eq!(format!("{:6.6}", P(p("-279358770000"))), "-3e11");
    assert_eq!(format!("{:6.6}", P(p("-50000000000000"))), "-5e13");
}
fn _just_opening_brace_many1() {
}
#[test]
fn many1_1() {
    assert_eq!(format!("{:30.30}", P(p("906792980000000"))), "9.0679298000000000000000000e14");
    assert_eq!(format!("{:30.30}", P(p("6390900000000"))), "6.3909000000000000000000000e12");
    assert_eq!(format!("{:30.30}", P(p("28897000"))), "2.88970000000000000000000000e7");
    assert_eq!(format!("{:30.30}", P(p("700000"))), "7.00000000000000000000000000e5");
    assert_eq!(format!("{:30.30}", P(p("439620.1"))), "4.39620099999999976716935635e5");
    assert_eq!(format!("{:30.30}", P(p("9559.407"))), "9559.4069999999992433004081249");
    assert_eq!(format!("{:30.30}", P(p("8022.2"))), "8022.1999999999998181010596454");
    assert_eq!(format!("{:30.30}", P(p("6738.111"))), "6738.1109999999998763087205589");
    assert_eq!(format!("{:30.30}", P(p("6208.24123131241232132142"))), "6208.2412313124123102170415223");
    assert_eq!(format!("{:30.30}", P(p("5400"))), "5400.0000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("4741.878"))), "4741.8779999999997016857378185");
    assert_eq!(format!("{:30.30}", P(p("3620.1461"))), "3620.1460999999999330611899495");
    assert_eq!(format!("{:30.30}", P(p("3000.23451"))), "3000.2345099999997728446032852");
    assert_eq!(format!("{:30.30}", P(p("2175.65"))), "2175.6500000000000909494701773");
    assert_eq!(format!("{:30.30}", P(p("969.49"))), "969.49000000000000909494701773");
    assert_eq!(format!("{:30.30}", P(p("840.2056"))), "840.20560000000000400177668780");
    assert_eq!(format!("{:30.30}", P(p("620"))), "620.00000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("407"))), "407.00000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("401.249"))), "401.24900000000002364686224610");
    assert_eq!(format!("{:30.30}", P(p("233.021"))), "233.02099999999998658495314885");
    assert_eq!(format!("{:30.30}", P(p("96.503326"))), "96.503326000000001272383087780");
    assert_eq!(format!("{:30.30}", P(p("58.4"))), "58.399999999999998578914528480");
    assert_eq!(format!("{:30.30}", P(p("39.137"))), "39.137000000000000454747350886");
    assert_eq!(format!("{:30.30}", P(p("38.74"))), "38.740000000000001989519660128");
    assert_eq!(format!("{:30.30}", P(p("30"))), "30.000000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("24.48179"))), "24.481790000000000162572177942");
    assert_eq!(format!("{:30.30}", P(p("21.123"))), "21.123000000000001108446667786");
    assert_eq!(format!("{:30.30}", P(p("9.95016e+246"))), "9.950159999999999695987719e246");
    assert_eq!(format!("{:30.30}", P(p("9.8388"))), "9.838800000000000878230821399");
    assert_eq!(format!("{:30.30}", P(p("9.8059e+35"))), "9.8059000000000000650859410e35");
}
#[test]
fn many1_2() {
    assert_eq!(format!("{:30.30}", P(p("9.530609e+22"))), "9.5306090000000005177344000e22");
    assert_eq!(format!("{:30.30}", P(p("9.46e+35"))), "9.4600000000000000500732224e35");
    assert_eq!(format!("{:30.30}", P(p("9.452105e-31"))), "9.452105000000000405290247e-31");
    assert_eq!(format!("{:30.30}", P(p("9e+115"))), "8.999999999999999265134455e115");
    assert_eq!(format!("{:30.30}", P(p("8.785953e+42"))), "8.7859530000000003975921548e42");
    assert_eq!(format!("{:30.30}", P(p("8.5e+20"))), "8.5000000000000000000000000e20");
    assert_eq!(format!("{:30.30}", P(p("8.3536e+30"))), "8.3535999999999998413986689e30");
    assert_eq!(format!("{:30.30}", P(p("8.3439e+25"))), "8.3439000000000000144703488e25");
    assert_eq!(format!("{:30.30}", P(p("8.27203e-18"))), "8.272029999999999578600149e-18");
    assert_eq!(format!("{:30.30}", P(p("8.271221e-219"))), "8.27122100000000028386666e-219");
    assert_eq!(format!("{:30.30}", P(p("8.0927985e-112"))), "8.09279849999999992989648e-112");
    assert_eq!(format!("{:30.30}", P(p("8.07e-53"))), "8.070000000000000439801953e-53");
    assert_eq!(format!("{:30.30}", P(p("8.062352e+38"))), "8.0623519999999996608299961e38");
    assert_eq!(format!("{:30.30}", P(p("8.0159e+36"))), "8.0158999999999998961814099e36");
    assert_eq!(format!("{:30.30}", P(p("8e+29"))), "7.9999999999999993146520686e29");
    assert_eq!(format!("{:30.30}", P(p("8e-24"))), "7.999999999999999389603996e-24");
    assert_eq!(format!("{:30.30}", P(p("7.9954287e-194"))), "7.99542869999999939581138e-194");
    assert_eq!(format!("{:30.30}", P(p("7.83472e-22"))), "7.834720000000000122058131e-22");
    assert_eq!(format!("{:30.30}", P(p("7.814"))), "7.8140000000000000568434188608");
    assert_eq!(format!("{:30.30}", P(p("7.7715e+27"))), "7.7715000000000002188503941e27");
    assert_eq!(format!("{:30.30}", P(p("7.509e+38"))), "7.5090000000000002289591768e38");
    assert_eq!(format!("{:30.30}", P(p("7.331e-31"))), "7.331000000000000428770499e-31");
    assert_eq!(format!("{:30.30}", P(p("7.2401e+226"))), "7.240099999999999664281570e226");
    assert_eq!(format!("{:30.30}", P(p("7.2193e-18"))), "7.219299999999999788407666e-18");
    assert_eq!(format!("{:30.30}", P(p("7.2e+19"))), "7.2000000000000000000000000e19");
    assert_eq!(format!("{:30.30}", P(p("7.142849e-170"))), "7.14284900000000054532377e-170");
    assert_eq!(format!("{:30.30}", P(p("7.0676e-17"))), "7.067600000000000202722975e-17");
    assert_eq!(format!("{:30.30}", P(p("7e+39"))), "7.0000000000000003335428019e39");
    assert_eq!(format!("{:30.30}", P(p("7e+28"))), "6.9999999999999999280861413e28");
    assert_eq!(format!("{:30.30}", P(p("7"))), "7.0000000000000000000000000000");
}
#[test]
fn many1_3() {
    assert_eq!(format!("{:30.30}", P(p("6.844e+113"))), "6.844000000000000565811296e113");
    assert_eq!(format!("{:30.30}", P(p("6.7853e-21"))), "6.785300000000000246448163e-21");
    assert_eq!(format!("{:30.30}", P(p("6.75e-27"))), "6.749999999999999542375557e-27");
    assert_eq!(format!("{:30.30}", P(p("6.56e+39"))), "6.5600000000000005474542707e39");
    assert_eq!(format!("{:30.30}", P(p("6.540688e-10"))), "6.540688000000000275078426e-10");
    assert_eq!(format!("{:30.30}", P(p("6.4e-08"))), "6.4000000000000003986021853e-8");
    assert_eq!(format!("{:30.30}", P(p("6.389785e-262"))), "6.38978500000000023548601e-262");
    assert_eq!(format!("{:30.30}", P(p("6.3508e+42"))), "6.3507999999999997321342655e42");
    assert_eq!(format!("{:30.30}", P(p("6.3383e+47"))), "6.3383000000000000935004389e47");
    assert_eq!(format!("{:30.30}", P(p("6.295e+53"))), "6.2950000000000001168332314e53");
    assert_eq!(format!("{:30.30}", P(p("6.049"))), "6.0490000000000003765876499529");
    assert_eq!(format!("{:30.30}", P(p("6.04"))), "6.0400000000000000355271367880");
    assert_eq!(format!("{:30.30}", P(p("6.01341e-20"))), "6.013410000000000181098742e-20");
    assert_eq!(format!("{:30.30}", P(p("5.98e+28"))), "5.9800000000000003708301607e28");
    assert_eq!(format!("{:30.30}", P(p("5.9e+43"))), "5.8999999999999998470209415e43");
    assert_eq!(format!("{:30.30}", P(p("5.865"))), "5.8650000000000002131628207280");
    assert_eq!(format!("{:30.30}", P(p("5.7e-26"))), "5.699999999999999932434832e-26");
    assert_eq!(format!("{:30.30}", P(p("5.695e-09"))), "5.6949999999999998759628838e-9");
    assert_eq!(format!("{:30.30}", P(p("5.61e-07"))), "5.6100000000000000743621744e-7");
    assert_eq!(format!("{:30.30}", P(p("5.55971e-235"))), "5.55970999999999977101710e-235");
    assert_eq!(format!("{:30.30}", P(p("5.4541311e-10"))), "5.454131099999999795907835e-10");
    assert_eq!(format!("{:30.30}", P(p("5.1415063e-11"))), "5.141506300000000176672871e-11");
    assert_eq!(format!("{:30.30}", P(p("5.13e+282"))), "5.129999999999999942461147e282");
    assert_eq!(format!("{:30.30}", P(p("5.0369e+172"))), "5.036900000000000248676992e172");
    assert_eq!(format!("{:30.30}", P(p("5.003824e+151"))), "5.003824000000000140469742e151");
    assert_eq!(format!("{:30.30}", P(p("5e+263"))), "5.000000000000000220702595e263");
    assert_eq!(format!("{:30.30}", P(p("5e+24"))), "5.0000000000000004529848320e24");
    assert_eq!(format!("{:30.30}", P(p("5e+19"))), "5.0000000000000000000000000e19");
    assert_eq!(format!("{:30.30}", P(p("5e+133"))), "4.999999999999999607410182e133");
    assert_eq!(format!("{:30.30}", P(p("4.937e+40"))), "4.9369999999999999627165528e40");
}
#[test]
fn many1_4() {
    assert_eq!(format!("{:30.30}", P(p("4.9361647e-133"))), "4.93616469999999981519129e-133");
    assert_eq!(format!("{:30.30}", P(p("4.585e+48"))), "4.5849999999999999292489017e48");
    assert_eq!(format!("{:30.30}", P(p("4.49e+26"))), "4.4899999999999996830783898e26");
    assert_eq!(format!("{:30.30}", P(p("4.4177e-134"))), "4.41770000000000038517943e-134");
    assert_eq!(format!("{:30.30}", P(p("4.3e-245"))), "4.30000000000000002567622e-245");
    assert_eq!(format!("{:30.30}", P(p("4.3e-16"))), "4.299999999999999860816676e-16");
    assert_eq!(format!("{:30.30}", P(p("3.452077e-191"))), "3.45207700000000026239703e-191");
    assert_eq!(format!("{:30.30}", P(p("3.2"))), "3.2000000000000001776356839400");
    assert_eq!(format!("{:30.30}", P(p("3e+25"))), "3.0000000000000000570425344e25");
    assert_eq!(format!("{:30.30}", P(p("3e+149"))), "3.000000000000000033361629e149");
    assert_eq!(format!("{:30.30}", P(p("2.93e+32"))), "2.9300000000000001230011954e32");
    assert_eq!(format!("{:30.30}", P(p("2.916861e+44"))), "2.9168609999999998849669702e44");
    assert_eq!(format!("{:30.30}", P(p("2.9"))), "2.8999999999999999111821580300");
    assert_eq!(format!("{:30.30}", P(p("2.764e-83"))), "2.763999999999999927321731e-83");
    assert_eq!(format!("{:30.30}", P(p("2.49279e+34"))), "2.4927899999999999095653553e34");
    assert_eq!(format!("{:30.30}", P(p("2.413e-15"))), "2.412999999999999963861063e-15");
    assert_eq!(format!("{:30.30}", P(p("2.2353e-129"))), "2.23529999999999986071675e-129");
    assert_eq!(format!("{:30.30}", P(p("2.166819e-23"))), "2.166819000000000116188078e-23");
    assert_eq!(format!("{:30.30}", P(p("2e+31"))), "1.9999999999999999271792590e31");
    assert_eq!(format!("{:30.30}", P(p("2e+116"))), "2.000000000000000031118832e116");
    assert_eq!(format!("{:30.30}", P(p("1.8e+33"))), "1.8000000000000001326197167e33");
    assert_eq!(format!("{:30.30}", P(p("1.7e-13"))), "1.700000000000000076878916e-13");
    assert_eq!(format!("{:30.30}", P(p("1.4580931e+62"))), "1.4580930999999999358622713e62");
    assert_eq!(format!("{:30.30}", P(p("1.44471e-07"))), "1.4447099999999999710571493e-7");
    assert_eq!(format!("{:30.30}", P(p("1.393237e+46"))), "1.3932369999999999686428617e46");
    assert_eq!(format!("{:30.30}", P(p("1.39e+295"))), "1.389999999999999937770538e295");
    assert_eq!(format!("{:30.30}", P(p("1.375e+17"))), "1.3750000000000000000000000e17");
    assert_eq!(format!("{:30.30}", P(p("1.293e+27"))), "1.2930000000000000421946982e27");
    assert_eq!(format!("{:30.30}", P(p("1.2041e-21"))), "1.204100000000000035687276e-21");
    assert_eq!(format!("{:30.30}", P(p("1e+48"))), "1.0000000000000000438458430e48");
}
#[test]
fn many1_5() {
    assert_eq!(format!("{:30.30}", P(p("1"))), "1.0000000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("0.98"))), "0.9799999999999999822364316060");
    assert_eq!(format!("{:30.30}", P(p("0.973"))), "0.9729999999999999760191826681");
    assert_eq!(format!("{:30.30}", P(p("0.945138"))), "0.9451380000000000336513039656");
    assert_eq!(format!("{:30.30}", P(p("0.84"))), "0.8399999999999999689137553105");
    assert_eq!(format!("{:30.30}", P(p("0.67"))), "0.6700000000000000399680288865");
    assert_eq!(format!("{:30.30}", P(p("0.335"))), "0.3350000000000000199840144433");
    assert_eq!(format!("{:30.30}", P(p("0.113"))), "0.1130000000000000032196467714");
    assert_eq!(format!("{:30.30}", P(p("0.0983688"))), "0.0983688000000000062339466922");
    assert_eq!(format!("{:30.30}", P(p("0.0906203"))), "0.0906203000000000008506972904");
    assert_eq!(format!("{:30.30}", P(p("0.0869"))), "0.0869000000000000050182080713");
    assert_eq!(format!("{:30.30}", P(p("0.05"))), "0.0500000000000000027755575616");
    assert_eq!(format!("{:30.30}", P(p("0.047255829"))), "0.0472558289999999991959356294");
    assert_eq!(format!("{:30.30}", P(p("0.028"))), "0.0280000000000000005828670879");
    assert_eq!(format!("{:30.30}", P(p("0.009146"))), "0.0091459999999999996161959004");
    assert_eq!(format!("{:30.30}", P(p("0.008581"))), "0.0085810000000000000858202398");
    assert_eq!(format!("{:30.30}", P(p("0.008166"))), "0.0081659999999999996478372566");
    assert_eq!(format!("{:30.30}", P(p("0.0081"))), "0.0080999999999999995614619053");
    assert_eq!(format!("{:30.30}", P(p("0.008001"))), "0.0080009999999999994318988783");
    assert_eq!(format!("{:30.30}", P(p("0.007889"))), "0.0078890000000000001789679516");
    assert_eq!(format!("{:30.30}", P(p("0.006703542"))), "0.0067035419999999998341699836");
    assert_eq!(format!("{:30.30}", P(p("0.0039"))), "0.0038999999999999998209765373");
    assert_eq!(format!("{:30.30}", P(p("0.0030426"))), "0.0030425999999999999053035271");
    assert_eq!(format!("{:30.30}", P(p("0.003"))), "0.0030000000000000000624500451");
    assert_eq!(format!("{:30.30}", P(p("0.0029071"))), "0.0029071000000000001353694934");
    assert_eq!(format!("{:30.30}", P(p("0.002191249"))), "0.0021912490000000001480473522");
    assert_eq!(format!("{:30.30}", P(p("0.001910066"))), "0.0019100659999999999773961923");
    assert_eq!(format!("{:30.30}", P(p("0.00092769"))), "9.2769000000000000332595063e-4");
    assert_eq!(format!("{:30.30}", P(p("0.0004"))), "4.0000000000000001916869441e-4");
    assert_eq!(format!("{:30.30}", P(p("0.000383036"))), "3.8303600000000001014888174e-4");
}
#[test]
fn many1_6() {
    assert_eq!(format!("{:30.30}", P(p("0.0003"))), "2.9999999999999997371893934e-4");
    assert_eq!(format!("{:30.30}", P(p("0.000188222"))), "1.8822199999999998784247501e-4");
    assert_eq!(format!("{:30.30}", P(p("0.00016"))), "1.6000000000000001308848863e-4");
    assert_eq!(format!("{:30.30}", P(p("0.00014834"))), "1.4834000000000000614105988e-4");
    assert_eq!(format!("{:30.30}", P(p("NaN"))), "NaN                           ");
    assert_eq!(format!("{:30.30}", P(p("-inf"))), "-inf                          ");
    assert_eq!(format!("{:30.30}", P(p("inf"))), "inf                           ");
    assert_eq!(format!("{:30.30}", P(p("0"))), "0.0000000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("-0.00023816"))), "-2.381600000000000101445241e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.000274"))), "-2.739999999999999922596639e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.00031"))), "-3.099999999999999999479583e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.00038509"))), "-3.850899999999999874338019e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.000552594"))), "-5.525940000000000018598456e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.0006028"))), "-6.028000000000000154973256e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.000719834"))), "-7.198340000000000337213035e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.0007820539"))), "-7.820538999999999832737907e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.000869"))), "-8.689999999999999807931417e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.00089806422"))), "-8.980642199999999469514633e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.0009172"))), "-9.171999999999999597655176e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.00095719"))), "-9.571900000000000156494262e-4");
    assert_eq!(format!("{:30.30}", P(p("-0.0060372"))), "-0.006037200000000000378597154");
    assert_eq!(format!("{:30.30}", P(p("-0.0064726046"))), "-0.006472604600000000062143268");
    assert_eq!(format!("{:30.30}", P(p("-0.0092191939"))), "-0.009219193900000000399530542");
    assert_eq!(format!("{:30.30}", P(p("-0.019099"))), "-0.019099000000000001392441717");
    assert_eq!(format!("{:30.30}", P(p("-0.022925"))), "-0.022925000000000000932587341");
    assert_eq!(format!("{:30.30}", P(p("-0.02826"))), "-0.028260000000000000397459843");
    assert_eq!(format!("{:30.30}", P(p("-0.0546163"))), "-0.054616299999999999570388098");
    assert_eq!(format!("{:30.30}", P(p("-0.062"))), "-0.061999999999999999555910790");
    assert_eq!(format!("{:30.30}", P(p("-0.0764018"))), "-0.076401800000000005819167370");
    assert_eq!(format!("{:30.30}", P(p("-0.0929804"))), "-0.092980400000000004712141788");
}
#[test]
fn many1_7() {
    assert_eq!(format!("{:30.30}", P(p("-0.1689"))), "-0.168899999999999994582111640");
    assert_eq!(format!("{:30.30}", P(p("-0.2"))), "-0.200000000000000011102230246");
    assert_eq!(format!("{:30.30}", P(p("-0.4946795"))), "-0.494679499999999994219734845");
    assert_eq!(format!("{:30.30}", P(p("-0.67014714"))), "-0.670147139999999974513400502");
    assert_eq!(format!("{:30.30}", P(p("-0.68"))), "-0.680000000000000048849813084");
    assert_eq!(format!("{:30.30}", P(p("-0.68815"))), "-0.688150000000000039435121835");
    assert_eq!(format!("{:30.30}", P(p("-0.7562"))), "-0.756199999999999983302245710");
    assert_eq!(format!("{:30.30}", P(p("-1e+29"))), "-9.999999999999999143315086e28");
    assert_eq!(format!("{:30.30}", P(p("-1.064846e-26"))), "-1.06484600000000003902238e-26");
    assert_eq!(format!("{:30.30}", P(p("-1.51624e-234"))), "-1.5162399999999998890814e-234");
    assert_eq!(format!("{:30.30}", P(p("-1.6965898e-119"))), "-1.6965897999999999514119e-119");
    assert_eq!(format!("{:30.30}", P(p("-1.7695e-236"))), "-1.7694999999999999964657e-236");
    assert_eq!(format!("{:30.30}", P(p("-1.93885e+19"))), "-1.938850000000000000000000e19");
    assert_eq!(format!("{:30.30}", P(p("-2.31373e+31"))), "-2.313730000000000051994626e31");
    assert_eq!(format!("{:30.30}", P(p("-2.4527809e-306"))), "-2.4527808999999999848708e-306");
    assert_eq!(format!("{:30.30}", P(p("-2.6136e-09"))), "-2.613600000000000156327159e-9");
    assert_eq!(format!("{:30.30}", P(p("-2.691e-11"))), "-2.69100000000000006482257e-11");
    assert_eq!(format!("{:30.30}", P(p("-2.819"))), "-2.818999999999999950262008497");
    assert_eq!(format!("{:30.30}", P(p("-2.8322"))), "-2.832199999999999828759200682");
    assert_eq!(format!("{:30.30}", P(p("-3e+45"))), "-3.000000000000000106184517e45");
    assert_eq!(format!("{:30.30}", P(p("-3e-05"))), "-3.000000000000000076002572e-5");
    assert_eq!(format!("{:30.30}", P(p("-3.05e+25"))), "-3.050000000000000136734310e25");
    assert_eq!(format!("{:30.30}", P(p("-3.108287e+278"))), "-3.10828699999999973939223e278");
    assert_eq!(format!("{:30.30}", P(p("-3.16584e+41"))), "-3.165839999999999982815240e41");
    assert_eq!(format!("{:30.30}", P(p("-3.17e-222"))), "-3.1700000000000000574634e-222");
    assert_eq!(format!("{:30.30}", P(p("-3.2348135"))), "-3.234813500000000008327560863");
    assert_eq!(format!("{:30.30}", P(p("-3.465e+22"))), "-3.465000000000000157286400e22");
    assert_eq!(format!("{:30.30}", P(p("-3.63e-135"))), "-3.6299999999999996981271e-135");
    assert_eq!(format!("{:30.30}", P(p("-3.72e+45"))), "-3.720000000000000106315789e45");
    assert_eq!(format!("{:30.30}", P(p("-3.9e+30"))), "-3.900000000000000105697535e30");
}
#[test]
fn many1_8() {
    assert_eq!(format!("{:30.30}", P(p("-4.2"))), "-4.200000000000000177635683940");
    assert_eq!(format!("{:30.30}", P(p("-4.208329e-20"))), "-4.20832899999999997705329e-20");
    assert_eq!(format!("{:30.30}", P(p("-4.2888733"))), "-4.288873299999999666454186809");
    assert_eq!(format!("{:30.30}", P(p("-4.297e-08"))), "-4.297000000000000030016368e-8");
    assert_eq!(format!("{:30.30}", P(p("-4.60469181924042e-321"))), "-4.6046918192404177917256e-321");
    assert_eq!(format!("{:30.30}", P(p("-4.7142e+36"))), "-4.714199999999999812951921e36");
    assert_eq!(format!("{:30.30}", P(p("-4.767"))), "-4.767000000000000348165940522");
    assert_eq!(format!("{:30.30}", P(p("-4.863526e-20"))), "-4.86352600000000003334239e-20");
    assert_eq!(format!("{:30.30}", P(p("-4.868e+33"))), "-4.868000000000000037125659e33");
    assert_eq!(format!("{:30.30}", P(p("-4.96e+247"))), "-4.95999999999999981233067e247");
    assert_eq!(format!("{:30.30}", P(p("-5e-29"))), "-4.99999999999999985616272e-29");
    assert_eq!(format!("{:30.30}", P(p("-5.05e-218"))), "-5.0499999999999997198085e-218");
    assert_eq!(format!("{:30.30}", P(p("-5.169414e+37"))), "-5.169413999999999842999574e37");
    assert_eq!(format!("{:30.30}", P(p("-5.20816e-06"))), "-5.208160000000000123632302e-6");
    assert_eq!(format!("{:30.30}", P(p("-5.263e+21"))), "-5.263000000000000262144000e21");
    assert_eq!(format!("{:30.30}", P(p("-5.4239467e-31"))), "-5.42394669999999988285251e-31");
    assert_eq!(format!("{:30.30}", P(p("-5.44067e-263"))), "-5.4406700000000000333436e-263");
    assert_eq!(format!("{:30.30}", P(p("-5.543976e+32"))), "-5.543975999999999949242952e32");
    assert_eq!(format!("{:30.30}", P(p("-5.56122e-29"))), "-5.56122000000000013330625e-29");
    assert_eq!(format!("{:30.30}", P(p("-5.6e+35"))), "-5.599999999999999971922455e35");
    assert_eq!(format!("{:30.30}", P(p("-5.65896e-290"))), "-5.6589599999999997701477e-290");
    assert_eq!(format!("{:30.30}", P(p("-5.71654"))), "-5.716540000000000176783032657");
    assert_eq!(format!("{:30.30}", P(p("-5.730186e+123"))), "-5.73018599999999996685772e123");
    assert_eq!(format!("{:30.30}", P(p("-5.798598e-21"))), "-5.79859800000000029281036e-21");
    assert_eq!(format!("{:30.30}", P(p("-5.81165e-212"))), "-5.8116499999999996095642e-212");
    assert_eq!(format!("{:30.30}", P(p("-6e-25"))), "-5.99999999999999954220300e-25");
    assert_eq!(format!("{:30.30}", P(p("-6e-06"))), "-6.000000000000000152005145e-6");
    assert_eq!(format!("{:30.30}", P(p("-6.07e-268"))), "-6.0699999999999994679386e-268");
    assert_eq!(format!("{:30.30}", P(p("-6.074991"))), "-6.074990999999999807812400832");
    assert_eq!(format!("{:30.30}", P(p("-6.1e+290"))), "-6.09999999999999977064935e290");
}
#[test]
fn many1_9() {
    assert_eq!(format!("{:30.30}", P(p("-6.1206e-06"))), "-6.120600000000000318707213e-6");
    assert_eq!(format!("{:30.30}", P(p("-6.5986e+45"))), "-6.598599999999999367201710e45");
    assert_eq!(format!("{:30.30}", P(p("-6.785938e+26"))), "-6.785938000000000163892429e26");
    assert_eq!(format!("{:30.30}", P(p("-6.8e-30"))), "-6.80000000000000056668766e-30");
    assert_eq!(format!("{:30.30}", P(p("-6.921e-22"))), "-6.92099999999999980713072e-22");
    assert_eq!(format!("{:30.30}", P(p("-7e-57"))), "-6.99999999999999982619713e-57");
    assert_eq!(format!("{:30.30}", P(p("-7.0915e+22"))), "-7.091499999999999711641600e22");
    assert_eq!(format!("{:30.30}", P(p("-7.15407055178125e-321"))), "-7.1540705517812499596767e-321");
    assert_eq!(format!("{:30.30}", P(p("-7.60733e+45"))), "-7.607330000000000332551903e45");
    assert_eq!(format!("{:30.30}", P(p("-7.79173e-05"))), "-7.791730000000000020982799e-5");
    assert_eq!(format!("{:30.30}", P(p("-8e-26"))), "-8.00000000000000030795896e-26");
    assert_eq!(format!("{:30.30}", P(p("-8e+20"))), "-8.000000000000000000000000e20");
    assert_eq!(format!("{:30.30}", P(p("-8.245e-145"))), "-8.2449999999999996940877e-145");
    assert_eq!(format!("{:30.30}", P(p("-8.28043e+21"))), "-8.280430000000000327680000e21");
    assert_eq!(format!("{:30.30}", P(p("-8.2888989e-25"))), "-8.28889889999999973031440e-25");
    assert_eq!(format!("{:30.30}", P(p("-8.37662e-158"))), "-8.3766199999999995871842e-158");
    assert_eq!(format!("{:30.30}", P(p("-8.38e-30"))), "-8.38000000000000013783982e-30");
    assert_eq!(format!("{:30.30}", P(p("-8.4e-23"))), "-8.40000000000000031417336e-23");
    assert_eq!(format!("{:30.30}", P(p("-8.5e+25"))), "-8.500000000000000662700032e25");
    assert_eq!(format!("{:30.30}", P(p("-8.8165e-23"))), "-8.81650000000000010836635e-23");
    assert_eq!(format!("{:30.30}", P(p("-8.821e-20"))), "-8.82099999999999961264397e-20");
    assert_eq!(format!("{:30.30}", P(p("-8.8805774e+129"))), "-8.88057740000000003603910e129");
    assert_eq!(format!("{:30.30}", P(p("-8.91e+18"))), "-8.910000000000000000000000e18");
    assert_eq!(format!("{:30.30}", P(p("-9e-19"))), "-9.00000000000000025869583e-19");
    assert_eq!(format!("{:30.30}", P(p("-9.03e-286"))), "-9.0300000000000000857069e-286");
    assert_eq!(format!("{:30.30}", P(p("-9.13"))), "-9.130000000000000781597009336");
    assert_eq!(format!("{:30.30}", P(p("-9.185594e-120"))), "-9.1855940000000001648391e-120");
    assert_eq!(format!("{:30.30}", P(p("-9.3e-70"))), "-9.29999999999999981491905e-70");
    assert_eq!(format!("{:30.30}", P(p("-9.451309e-30"))), "-9.45130899999999931512581e-30");
    assert_eq!(format!("{:30.30}", P(p("-9.494134e+65"))), "-9.494134000000000816130636e65");
}
#[test]
fn many1_10() {
    assert_eq!(format!("{:30.30}", P(p("-9.54e-24"))), "-9.54000000000000017209063e-24");
    assert_eq!(format!("{:30.30}", P(p("-9.57259e+240"))), "-9.57258999999999942068014e240");
    assert_eq!(format!("{:30.30}", P(p("-27.1783"))), "-27.17830000000000012505552149");
    assert_eq!(format!("{:30.30}", P(p("-32.234"))), "-32.23400000000000176214598469");
    assert_eq!(format!("{:30.30}", P(p("-46.430046"))), "-46.43004599999999726378518972");
    assert_eq!(format!("{:30.30}", P(p("-48.1"))), "-48.10000000000000142108547152");
    assert_eq!(format!("{:30.30}", P(p("-48.307"))), "-48.30700000000000216004991671");
    assert_eq!(format!("{:30.30}", P(p("-50"))), "-50.00000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("-57.28"))), "-57.28000000000000113686837722");
    assert_eq!(format!("{:30.30}", P(p("-60.094"))), "-60.09400000000000119371179608");
    assert_eq!(format!("{:30.30}", P(p("-62.01612"))), "-62.01612000000000080035533756");
    assert_eq!(format!("{:30.30}", P(p("-64.92"))), "-64.92000000000000170530256582");
    assert_eq!(format!("{:30.30}", P(p("-80.2234"))), "-80.22339999999999804458639119");
    assert_eq!(format!("{:30.30}", P(p("-90"))), "-90.00000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("-94.9"))), "-94.90000000000000568434188608");
    assert_eq!(format!("{:30.30}", P(p("-98.6783"))), "-98.67829999999999301962816389");
    assert_eq!(format!("{:30.30}", P(p("-100"))), "-100.0000000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("-164.3687"))), "-164.3686999999999898136593401");
    assert_eq!(format!("{:30.30}", P(p("-430.50597"))), "-430.5059699999999907049641479");
    assert_eq!(format!("{:30.30}", P(p("-466.5085"))), "-466.5085000000000263753463514");
    assert_eq!(format!("{:30.30}", P(p("-595.05"))), "-595.0499999999999545252649114");
    assert_eq!(format!("{:30.30}", P(p("-764.3112"))), "-764.3111999999999852661858313");
    assert_eq!(format!("{:30.30}", P(p("-807.142"))), "-807.1420000000000527506927028");
    assert_eq!(format!("{:30.30}", P(p("-5469.471"))), "-5469.470999999999548890627921");
    assert_eq!(format!("{:30.30}", P(p("-5651.6809"))), "-5651.680900000000292493496090");
    assert_eq!(format!("{:30.30}", P(p("-9706.81"))), "-9706.809999999999490682967007");
    assert_eq!(format!("{:30.30}", P(p("-93500"))), "-93500.00000000000000000000000");
    assert_eq!(format!("{:30.30}", P(p("-577000"))), "-5.7700000000000000000000000e5");
    assert_eq!(format!("{:30.30}", P(p("-835534"))), "-8.3553400000000000000000000e5");
    assert_eq!(format!("{:30.30}", P(p("-6000000"))), "-6.0000000000000000000000000e6");
}
#[test]
fn many1_11() {
    assert_eq!(format!("{:30.30}", P(p("-900000000"))), "-9.0000000000000000000000000e8");
    assert_eq!(format!("{:30.30}", P(p("-279358770000"))), "-2.793587700000000000000000e11");
    assert_eq!(format!("{:30.30}", P(p("-50000000000000"))), "-5.000000000000000000000000e13");
}
fn _just_opening_brace_many2() {
}
#[test]
fn many2_1() {
    assert_eq!(format!("{:20.30}", P(p("906792980000000"))), "9.067929800000000e14");
    assert_eq!(format!("{:20.30}", P(p("6390900000000"))), "6.390900000000000e12");
    assert_eq!(format!("{:20.30}", P(p("28897000"))), "2.8897000000000000e7");
    assert_eq!(format!("{:20.30}", P(p("700000"))), "7.0000000000000000e5");
    assert_eq!(format!("{:20.30}", P(p("439620.1"))), "4.39620099999999976716935635e5");
    assert_eq!(format!("{:20.30}", P(p("9559.407"))), "9559.4069999999992433004081249");
    assert_eq!(format!("{:20.30}", P(p("8022.2"))), "8022.1999999999998181010596454");
    assert_eq!(format!("{:20.30}", P(p("6738.111"))), "6738.1109999999998763087205589");
    assert_eq!(format!("{:20.30}", P(p("6208.24123131241232132142"))), "6208.2412313124123102170415223");
    assert_eq!(format!("{:20.30}", P(p("5400"))), "5400.000000000000000");
    assert_eq!(format!("{:20.30}", P(p("4741.878"))), "4741.8779999999997016857378185");
    assert_eq!(format!("{:20.30}", P(p("3620.1461"))), "3620.1460999999999330611899495");
    assert_eq!(format!("{:20.30}", P(p("3000.23451"))), "3000.2345099999997728446032852");
    assert_eq!(format!("{:20.30}", P(p("2175.65"))), "2175.6500000000000909494701773");
    assert_eq!(format!("{:20.30}", P(p("969.49"))), "969.49000000000000909494701773");
    assert_eq!(format!("{:20.30}", P(p("840.2056"))), "840.2056000000000040017766878");
    assert_eq!(format!("{:20.30}", P(p("620"))), "620.0000000000000000");
    assert_eq!(format!("{:20.30}", P(p("407"))), "407.0000000000000000");
    assert_eq!(format!("{:20.30}", P(p("401.249"))), "401.2490000000000236468622461");
    assert_eq!(format!("{:20.30}", P(p("233.021"))), "233.02099999999998658495314885");
    assert_eq!(format!("{:20.30}", P(p("96.503326"))), "96.50332600000000127238308778");
    assert_eq!(format!("{:20.30}", P(p("58.4"))), "58.39999999999999857891452848");
    assert_eq!(format!("{:20.30}", P(p("39.137"))), "39.137000000000000454747350886");
    assert_eq!(format!("{:20.30}", P(p("38.74"))), "38.740000000000001989519660128");
    assert_eq!(format!("{:20.30}", P(p("30"))), "30.00000000000000000");
    assert_eq!(format!("{:20.30}", P(p("24.48179"))), "24.481790000000000162572177942");
    assert_eq!(format!("{:20.30}", P(p("21.123"))), "21.123000000000001108446667786");
    assert_eq!(format!("{:20.30}", P(p("9.95016e+246"))), "9.950159999999999695987719e246");
    assert_eq!(format!("{:20.30}", P(p("9.8388"))), "9.838800000000000878230821399");
    assert_eq!(format!("{:20.30}", P(p("9.8059e+35"))), "9.805900000000000065085941e35");
}
#[test]
fn many2_2() {
    assert_eq!(format!("{:20.30}", P(p("9.530609e+22"))), "9.5306090000000005177344e22");
    assert_eq!(format!("{:20.30}", P(p("9.46e+35"))), "9.4600000000000000500732224e35");
    assert_eq!(format!("{:20.30}", P(p("9.452105e-31"))), "9.452105000000000405290247e-31");
    assert_eq!(format!("{:20.30}", P(p("9e+115"))), "8.999999999999999265134455e115");
    assert_eq!(format!("{:20.30}", P(p("8.785953e+42"))), "8.7859530000000003975921548e42");
    assert_eq!(format!("{:20.30}", P(p("8.5e+20"))), "8.500000000000000e20");
    assert_eq!(format!("{:20.30}", P(p("8.3536e+30"))), "8.3535999999999998413986689e30");
    assert_eq!(format!("{:20.30}", P(p("8.3439e+25"))), "8.3439000000000000144703488e25");
    assert_eq!(format!("{:20.30}", P(p("8.27203e-18"))), "8.272029999999999578600149e-18");
    assert_eq!(format!("{:20.30}", P(p("8.271221e-219"))), "8.27122100000000028386666e-219");
    assert_eq!(format!("{:20.30}", P(p("8.0927985e-112"))), "8.09279849999999992989648e-112");
    assert_eq!(format!("{:20.30}", P(p("8.07e-53"))), "8.070000000000000439801953e-53");
    assert_eq!(format!("{:20.30}", P(p("8.062352e+38"))), "8.0623519999999996608299961e38");
    assert_eq!(format!("{:20.30}", P(p("8.0159e+36"))), "8.0158999999999998961814099e36");
    assert_eq!(format!("{:20.30}", P(p("8e+29"))), "7.9999999999999993146520686e29");
    assert_eq!(format!("{:20.30}", P(p("8e-24"))), "7.999999999999999389603996e-24");
    assert_eq!(format!("{:20.30}", P(p("7.9954287e-194"))), "7.99542869999999939581138e-194");
    assert_eq!(format!("{:20.30}", P(p("7.83472e-22"))), "7.834720000000000122058131e-22");
    assert_eq!(format!("{:20.30}", P(p("7.814"))), "7.8140000000000000568434188608");
    assert_eq!(format!("{:20.30}", P(p("7.7715e+27"))), "7.7715000000000002188503941e27");
    assert_eq!(format!("{:20.30}", P(p("7.509e+38"))), "7.5090000000000002289591768e38");
    assert_eq!(format!("{:20.30}", P(p("7.331e-31"))), "7.331000000000000428770499e-31");
    assert_eq!(format!("{:20.30}", P(p("7.2401e+226"))), "7.24009999999999966428157e226");
    assert_eq!(format!("{:20.30}", P(p("7.2193e-18"))), "7.219299999999999788407666e-18");
    assert_eq!(format!("{:20.30}", P(p("7.2e+19"))), "7.200000000000000e19");
    assert_eq!(format!("{:20.30}", P(p("7.142849e-170"))), "7.14284900000000054532377e-170");
    assert_eq!(format!("{:20.30}", P(p("7.0676e-17"))), "7.067600000000000202722975e-17");
    assert_eq!(format!("{:20.30}", P(p("7e+39"))), "7.0000000000000003335428019e39");
    assert_eq!(format!("{:20.30}", P(p("7e+28"))), "6.9999999999999999280861413e28");
    assert_eq!(format!("{:20.30}", P(p("7"))), "7.000000000000000000");
}
#[test]
fn many2_3() {
    assert_eq!(format!("{:20.30}", P(p("6.844e+113"))), "6.844000000000000565811296e113");
    assert_eq!(format!("{:20.30}", P(p("6.7853e-21"))), "6.785300000000000246448163e-21");
    assert_eq!(format!("{:20.30}", P(p("6.75e-27"))), "6.749999999999999542375557e-27");
    assert_eq!(format!("{:20.30}", P(p("6.56e+39"))), "6.5600000000000005474542707e39");
    assert_eq!(format!("{:20.30}", P(p("6.540688e-10"))), "6.540688000000000275078426e-10");
    assert_eq!(format!("{:20.30}", P(p("6.4e-08"))), "6.4000000000000003986021853e-8");
    assert_eq!(format!("{:20.30}", P(p("6.389785e-262"))), "6.38978500000000023548601e-262");
    assert_eq!(format!("{:20.30}", P(p("6.3508e+42"))), "6.3507999999999997321342655e42");
    assert_eq!(format!("{:20.30}", P(p("6.3383e+47"))), "6.3383000000000000935004389e47");
    assert_eq!(format!("{:20.30}", P(p("6.295e+53"))), "6.2950000000000001168332314e53");
    assert_eq!(format!("{:20.30}", P(p("6.049"))), "6.0490000000000003765876499529");
    assert_eq!(format!("{:20.30}", P(p("6.04"))), "6.040000000000000035527136788");
    assert_eq!(format!("{:20.30}", P(p("6.01341e-20"))), "6.013410000000000181098742e-20");
    assert_eq!(format!("{:20.30}", P(p("5.98e+28"))), "5.9800000000000003708301607e28");
    assert_eq!(format!("{:20.30}", P(p("5.9e+43"))), "5.8999999999999998470209415e43");
    assert_eq!(format!("{:20.30}", P(p("5.865"))), "5.865000000000000213162820728");
    assert_eq!(format!("{:20.30}", P(p("5.7e-26"))), "5.699999999999999932434832e-26");
    assert_eq!(format!("{:20.30}", P(p("5.695e-09"))), "5.6949999999999998759628838e-9");
    assert_eq!(format!("{:20.30}", P(p("5.61e-07"))), "5.6100000000000000743621744e-7");
    assert_eq!(format!("{:20.30}", P(p("5.55971e-235"))), "5.5597099999999997710171e-235");
    assert_eq!(format!("{:20.30}", P(p("5.4541311e-10"))), "5.454131099999999795907835e-10");
    assert_eq!(format!("{:20.30}", P(p("5.1415063e-11"))), "5.141506300000000176672871e-11");
    assert_eq!(format!("{:20.30}", P(p("5.13e+282"))), "5.129999999999999942461147e282");
    assert_eq!(format!("{:20.30}", P(p("5.0369e+172"))), "5.036900000000000248676992e172");
    assert_eq!(format!("{:20.30}", P(p("5.003824e+151"))), "5.003824000000000140469742e151");
    assert_eq!(format!("{:20.30}", P(p("5e+263"))), "5.000000000000000220702595e263");
    assert_eq!(format!("{:20.30}", P(p("5e+24"))), "5.000000000000000452984832e24");
    assert_eq!(format!("{:20.30}", P(p("5e+19"))), "5.000000000000000e19");
    assert_eq!(format!("{:20.30}", P(p("5e+133"))), "4.999999999999999607410182e133");
    assert_eq!(format!("{:20.30}", P(p("4.937e+40"))), "4.9369999999999999627165528e40");
}
#[test]
fn many2_4() {
    assert_eq!(format!("{:20.30}", P(p("4.9361647e-133"))), "4.93616469999999981519129e-133");
    assert_eq!(format!("{:20.30}", P(p("4.585e+48"))), "4.5849999999999999292489017e48");
    assert_eq!(format!("{:20.30}", P(p("4.49e+26"))), "4.4899999999999996830783898e26");
    assert_eq!(format!("{:20.30}", P(p("4.4177e-134"))), "4.41770000000000038517943e-134");
    assert_eq!(format!("{:20.30}", P(p("4.3e-245"))), "4.30000000000000002567622e-245");
    assert_eq!(format!("{:20.30}", P(p("4.3e-16"))), "4.299999999999999860816676e-16");
    assert_eq!(format!("{:20.30}", P(p("3.452077e-191"))), "3.45207700000000026239703e-191");
    assert_eq!(format!("{:20.30}", P(p("3.2"))), "3.20000000000000017763568394");
    assert_eq!(format!("{:20.30}", P(p("3e+25"))), "3.0000000000000000570425344e25");
    assert_eq!(format!("{:20.30}", P(p("3e+149"))), "3.000000000000000033361629e149");
    assert_eq!(format!("{:20.30}", P(p("2.93e+32"))), "2.9300000000000001230011954e32");
    assert_eq!(format!("{:20.30}", P(p("2.916861e+44"))), "2.9168609999999998849669702e44");
    assert_eq!(format!("{:20.30}", P(p("2.9"))), "2.89999999999999991118215803");
    assert_eq!(format!("{:20.30}", P(p("2.764e-83"))), "2.763999999999999927321731e-83");
    assert_eq!(format!("{:20.30}", P(p("2.49279e+34"))), "2.4927899999999999095653553e34");
    assert_eq!(format!("{:20.30}", P(p("2.413e-15"))), "2.412999999999999963861063e-15");
    assert_eq!(format!("{:20.30}", P(p("2.2353e-129"))), "2.23529999999999986071675e-129");
    assert_eq!(format!("{:20.30}", P(p("2.166819e-23"))), "2.166819000000000116188078e-23");
    assert_eq!(format!("{:20.30}", P(p("2e+31"))), "1.999999999999999927179259e31");
    assert_eq!(format!("{:20.30}", P(p("2e+116"))), "2.000000000000000031118832e116");
    assert_eq!(format!("{:20.30}", P(p("1.8e+33"))), "1.8000000000000001326197167e33");
    assert_eq!(format!("{:20.30}", P(p("1.7e-13"))), "1.700000000000000076878916e-13");
    assert_eq!(format!("{:20.30}", P(p("1.4580931e+62"))), "1.4580930999999999358622713e62");
    assert_eq!(format!("{:20.30}", P(p("1.44471e-07"))), "1.4447099999999999710571493e-7");
    assert_eq!(format!("{:20.30}", P(p("1.393237e+46"))), "1.3932369999999999686428617e46");
    assert_eq!(format!("{:20.30}", P(p("1.39e+295"))), "1.389999999999999937770538e295");
    assert_eq!(format!("{:20.30}", P(p("1.375e+17"))), "1.375000000000000e17");
    assert_eq!(format!("{:20.30}", P(p("1.293e+27"))), "1.2930000000000000421946982e27");
    assert_eq!(format!("{:20.30}", P(p("1.2041e-21"))), "1.204100000000000035687276e-21");
    assert_eq!(format!("{:20.30}", P(p("1e+48"))), "1.000000000000000043845843e48");
}
#[test]
fn many2_5() {
    assert_eq!(format!("{:20.30}", P(p("1"))), "1.000000000000000000");
    assert_eq!(format!("{:20.30}", P(p("0.98"))), "0.979999999999999982236431606");
    assert_eq!(format!("{:20.30}", P(p("0.973"))), "0.9729999999999999760191826681");
    assert_eq!(format!("{:20.30}", P(p("0.945138"))), "0.9451380000000000336513039656");
    assert_eq!(format!("{:20.30}", P(p("0.84"))), "0.8399999999999999689137553105");
    assert_eq!(format!("{:20.30}", P(p("0.67"))), "0.6700000000000000399680288865");
    assert_eq!(format!("{:20.30}", P(p("0.335"))), "0.3350000000000000199840144433");
    assert_eq!(format!("{:20.30}", P(p("0.113"))), "0.1130000000000000032196467714");
    assert_eq!(format!("{:20.30}", P(p("0.0983688"))), "0.0983688000000000062339466922");
    assert_eq!(format!("{:20.30}", P(p("0.0906203"))), "0.0906203000000000008506972904");
    assert_eq!(format!("{:20.30}", P(p("0.0869"))), "0.0869000000000000050182080713");
    assert_eq!(format!("{:20.30}", P(p("0.05"))), "0.0500000000000000027755575616");
    assert_eq!(format!("{:20.30}", P(p("0.047255829"))), "0.0472558289999999991959356294");
    assert_eq!(format!("{:20.30}", P(p("0.028"))), "0.0280000000000000005828670879");
    assert_eq!(format!("{:20.30}", P(p("0.009146"))), "0.0091459999999999996161959004");
    assert_eq!(format!("{:20.30}", P(p("0.008581"))), "0.0085810000000000000858202398");
    assert_eq!(format!("{:20.30}", P(p("0.008166"))), "0.0081659999999999996478372566");
    assert_eq!(format!("{:20.30}", P(p("0.0081"))), "0.0080999999999999995614619053");
    assert_eq!(format!("{:20.30}", P(p("0.008001"))), "0.0080009999999999994318988783");
    assert_eq!(format!("{:20.30}", P(p("0.007889"))), "0.0078890000000000001789679516");
    assert_eq!(format!("{:20.30}", P(p("0.006703542"))), "0.0067035419999999998341699836");
    assert_eq!(format!("{:20.30}", P(p("0.0039"))), "0.0038999999999999998209765373");
    assert_eq!(format!("{:20.30}", P(p("0.0030426"))), "0.0030425999999999999053035271");
    assert_eq!(format!("{:20.30}", P(p("0.003"))), "0.0030000000000000000624500451");
    assert_eq!(format!("{:20.30}", P(p("0.0029071"))), "0.0029071000000000001353694934");
    assert_eq!(format!("{:20.30}", P(p("0.002191249"))), "0.0021912490000000001480473522");
    assert_eq!(format!("{:20.30}", P(p("0.001910066"))), "0.0019100659999999999773961923");
    assert_eq!(format!("{:20.30}", P(p("0.00092769"))), "9.2769000000000000332595063e-4");
    assert_eq!(format!("{:20.30}", P(p("0.0004"))), "4.0000000000000001916869441e-4");
    assert_eq!(format!("{:20.30}", P(p("0.000383036"))), "3.8303600000000001014888174e-4");
}
#[test]
fn many2_6() {
    assert_eq!(format!("{:20.30}", P(p("0.0003"))), "2.9999999999999997371893934e-4");
    assert_eq!(format!("{:20.30}", P(p("0.000188222"))), "1.8822199999999998784247501e-4");
    assert_eq!(format!("{:20.30}", P(p("0.00016"))), "1.6000000000000001308848863e-4");
    assert_eq!(format!("{:20.30}", P(p("0.00014834"))), "1.4834000000000000614105988e-4");
    assert_eq!(format!("{:20.30}", P(p("NaN"))), "NaN                 ");
    assert_eq!(format!("{:20.30}", P(p("-inf"))), "-inf                ");
    assert_eq!(format!("{:20.30}", P(p("inf"))), "inf                 ");
    assert_eq!(format!("{:20.30}", P(p("0"))), "0.000000000000000000");
    assert_eq!(format!("{:20.30}", P(p("-0.00023816"))), "-2.381600000000000101445241e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.000274"))), "-2.739999999999999922596639e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.00031"))), "-3.099999999999999999479583e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.00038509"))), "-3.850899999999999874338019e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.000552594"))), "-5.525940000000000018598456e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.0006028"))), "-6.028000000000000154973256e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.000719834"))), "-7.198340000000000337213035e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.0007820539"))), "-7.820538999999999832737907e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.000869"))), "-8.689999999999999807931417e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.00089806422"))), "-8.980642199999999469514633e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.0009172"))), "-9.171999999999999597655176e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.00095719"))), "-9.571900000000000156494262e-4");
    assert_eq!(format!("{:20.30}", P(p("-0.0060372"))), "-0.006037200000000000378597154");
    assert_eq!(format!("{:20.30}", P(p("-0.0064726046"))), "-0.006472604600000000062143268");
    assert_eq!(format!("{:20.30}", P(p("-0.0092191939"))), "-0.009219193900000000399530542");
    assert_eq!(format!("{:20.30}", P(p("-0.019099"))), "-0.019099000000000001392441717");
    assert_eq!(format!("{:20.30}", P(p("-0.022925"))), "-0.022925000000000000932587341");
    assert_eq!(format!("{:20.30}", P(p("-0.02826"))), "-0.028260000000000000397459843");
    assert_eq!(format!("{:20.30}", P(p("-0.0546163"))), "-0.054616299999999999570388098");
    assert_eq!(format!("{:20.30}", P(p("-0.062"))), "-0.06199999999999999955591079");
    assert_eq!(format!("{:20.30}", P(p("-0.0764018"))), "-0.07640180000000000581916737");
    assert_eq!(format!("{:20.30}", P(p("-0.0929804"))), "-0.092980400000000004712141788");
}
#[test]
fn many2_7() {
    assert_eq!(format!("{:20.30}", P(p("-0.1689"))), "-0.16889999999999999458211164");
    assert_eq!(format!("{:20.30}", P(p("-0.2"))), "-0.200000000000000011102230246");
    assert_eq!(format!("{:20.30}", P(p("-0.4946795"))), "-0.494679499999999994219734845");
    assert_eq!(format!("{:20.30}", P(p("-0.67014714"))), "-0.670147139999999974513400502");
    assert_eq!(format!("{:20.30}", P(p("-0.68"))), "-0.680000000000000048849813084");
    assert_eq!(format!("{:20.30}", P(p("-0.68815"))), "-0.688150000000000039435121835");
    assert_eq!(format!("{:20.30}", P(p("-0.7562"))), "-0.75619999999999998330224571");
    assert_eq!(format!("{:20.30}", P(p("-1e+29"))), "-9.999999999999999143315086e28");
    assert_eq!(format!("{:20.30}", P(p("-1.064846e-26"))), "-1.06484600000000003902238e-26");
    assert_eq!(format!("{:20.30}", P(p("-1.51624e-234"))), "-1.5162399999999998890814e-234");
    assert_eq!(format!("{:20.30}", P(p("-1.6965898e-119"))), "-1.6965897999999999514119e-119");
    assert_eq!(format!("{:20.30}", P(p("-1.7695e-236"))), "-1.7694999999999999964657e-236");
    assert_eq!(format!("{:20.30}", P(p("-1.93885e+19"))), "-1.93885000000000e19");
    assert_eq!(format!("{:20.30}", P(p("-2.31373e+31"))), "-2.313730000000000051994626e31");
    assert_eq!(format!("{:20.30}", P(p("-2.4527809e-306"))), "-2.4527808999999999848708e-306");
    assert_eq!(format!("{:20.30}", P(p("-2.6136e-09"))), "-2.613600000000000156327159e-9");
    assert_eq!(format!("{:20.30}", P(p("-2.691e-11"))), "-2.69100000000000006482257e-11");
    assert_eq!(format!("{:20.30}", P(p("-2.819"))), "-2.818999999999999950262008497");
    assert_eq!(format!("{:20.30}", P(p("-2.8322"))), "-2.832199999999999828759200682");
    assert_eq!(format!("{:20.30}", P(p("-3e+45"))), "-3.000000000000000106184517e45");
    assert_eq!(format!("{:20.30}", P(p("-3e-05"))), "-3.000000000000000076002572e-5");
    assert_eq!(format!("{:20.30}", P(p("-3.05e+25"))), "-3.05000000000000013673431e25");
    assert_eq!(format!("{:20.30}", P(p("-3.108287e+278"))), "-3.10828699999999973939223e278");
    assert_eq!(format!("{:20.30}", P(p("-3.16584e+41"))), "-3.16583999999999998281524e41");
    assert_eq!(format!("{:20.30}", P(p("-3.17e-222"))), "-3.1700000000000000574634e-222");
    assert_eq!(format!("{:20.30}", P(p("-3.2348135"))), "-3.234813500000000008327560863");
    assert_eq!(format!("{:20.30}", P(p("-3.465e+22"))), "-3.4650000000000001572864e22");
    assert_eq!(format!("{:20.30}", P(p("-3.63e-135"))), "-3.6299999999999996981271e-135");
    assert_eq!(format!("{:20.30}", P(p("-3.72e+45"))), "-3.720000000000000106315789e45");
    assert_eq!(format!("{:20.30}", P(p("-3.9e+30"))), "-3.900000000000000105697535e30");
}
#[test]
fn many2_8() {
    assert_eq!(format!("{:20.30}", P(p("-4.2"))), "-4.20000000000000017763568394");
    assert_eq!(format!("{:20.30}", P(p("-4.208329e-20"))), "-4.20832899999999997705329e-20");
    assert_eq!(format!("{:20.30}", P(p("-4.2888733"))), "-4.288873299999999666454186809");
    assert_eq!(format!("{:20.30}", P(p("-4.297e-08"))), "-4.297000000000000030016368e-8");
    assert_eq!(format!("{:20.30}", P(p("-4.60469181924042e-321"))), "-4.6046918192404177917256e-321");
    assert_eq!(format!("{:20.30}", P(p("-4.7142e+36"))), "-4.714199999999999812951921e36");
    assert_eq!(format!("{:20.30}", P(p("-4.767"))), "-4.767000000000000348165940522");
    assert_eq!(format!("{:20.30}", P(p("-4.863526e-20"))), "-4.86352600000000003334239e-20");
    assert_eq!(format!("{:20.30}", P(p("-4.868e+33"))), "-4.868000000000000037125659e33");
    assert_eq!(format!("{:20.30}", P(p("-4.96e+247"))), "-4.95999999999999981233067e247");
    assert_eq!(format!("{:20.30}", P(p("-5e-29"))), "-4.99999999999999985616272e-29");
    assert_eq!(format!("{:20.30}", P(p("-5.05e-218"))), "-5.0499999999999997198085e-218");
    assert_eq!(format!("{:20.30}", P(p("-5.169414e+37"))), "-5.169413999999999842999574e37");
    assert_eq!(format!("{:20.30}", P(p("-5.20816e-06"))), "-5.208160000000000123632302e-6");
    assert_eq!(format!("{:20.30}", P(p("-5.263e+21"))), "-5.263000000000000262144e21");
    assert_eq!(format!("{:20.30}", P(p("-5.4239467e-31"))), "-5.42394669999999988285251e-31");
    assert_eq!(format!("{:20.30}", P(p("-5.44067e-263"))), "-5.4406700000000000333436e-263");
    assert_eq!(format!("{:20.30}", P(p("-5.543976e+32"))), "-5.543975999999999949242952e32");
    assert_eq!(format!("{:20.30}", P(p("-5.56122e-29"))), "-5.56122000000000013330625e-29");
    assert_eq!(format!("{:20.30}", P(p("-5.6e+35"))), "-5.599999999999999971922455e35");
    assert_eq!(format!("{:20.30}", P(p("-5.65896e-290"))), "-5.6589599999999997701477e-290");
    assert_eq!(format!("{:20.30}", P(p("-5.71654"))), "-5.716540000000000176783032657");
    assert_eq!(format!("{:20.30}", P(p("-5.730186e+123"))), "-5.73018599999999996685772e123");
    assert_eq!(format!("{:20.30}", P(p("-5.798598e-21"))), "-5.79859800000000029281036e-21");
    assert_eq!(format!("{:20.30}", P(p("-5.81165e-212"))), "-5.8116499999999996095642e-212");
    assert_eq!(format!("{:20.30}", P(p("-6e-25"))), "-5.999999999999999542203e-25");
    assert_eq!(format!("{:20.30}", P(p("-6e-06"))), "-6.000000000000000152005145e-6");
    assert_eq!(format!("{:20.30}", P(p("-6.07e-268"))), "-6.0699999999999994679386e-268");
    assert_eq!(format!("{:20.30}", P(p("-6.074991"))), "-6.074990999999999807812400832");
    assert_eq!(format!("{:20.30}", P(p("-6.1e+290"))), "-6.09999999999999977064935e290");
}
#[test]
fn many2_9() {
    assert_eq!(format!("{:20.30}", P(p("-6.1206e-06"))), "-6.120600000000000318707213e-6");
    assert_eq!(format!("{:20.30}", P(p("-6.5986e+45"))), "-6.59859999999999936720171e45");
    assert_eq!(format!("{:20.30}", P(p("-6.785938e+26"))), "-6.785938000000000163892429e26");
    assert_eq!(format!("{:20.30}", P(p("-6.8e-30"))), "-6.80000000000000056668766e-30");
    assert_eq!(format!("{:20.30}", P(p("-6.921e-22"))), "-6.92099999999999980713072e-22");
    assert_eq!(format!("{:20.30}", P(p("-7e-57"))), "-6.99999999999999982619713e-57");
    assert_eq!(format!("{:20.30}", P(p("-7.0915e+22"))), "-7.0914999999999997116416e22");
    assert_eq!(format!("{:20.30}", P(p("-7.15407055178125e-321"))), "-7.1540705517812499596767e-321");
    assert_eq!(format!("{:20.30}", P(p("-7.60733e+45"))), "-7.607330000000000332551903e45");
    assert_eq!(format!("{:20.30}", P(p("-7.79173e-05"))), "-7.791730000000000020982799e-5");
    assert_eq!(format!("{:20.30}", P(p("-8e-26"))), "-8.00000000000000030795896e-26");
    assert_eq!(format!("{:20.30}", P(p("-8e+20"))), "-8.00000000000000e20");
    assert_eq!(format!("{:20.30}", P(p("-8.245e-145"))), "-8.2449999999999996940877e-145");
    assert_eq!(format!("{:20.30}", P(p("-8.28043e+21"))), "-8.28043000000000032768e21");
    assert_eq!(format!("{:20.30}", P(p("-8.2888989e-25"))), "-8.2888988999999997303144e-25");
    assert_eq!(format!("{:20.30}", P(p("-8.37662e-158"))), "-8.3766199999999995871842e-158");
    assert_eq!(format!("{:20.30}", P(p("-8.38e-30"))), "-8.38000000000000013783982e-30");
    assert_eq!(format!("{:20.30}", P(p("-8.4e-23"))), "-8.40000000000000031417336e-23");
    assert_eq!(format!("{:20.30}", P(p("-8.5e+25"))), "-8.500000000000000662700032e25");
    assert_eq!(format!("{:20.30}", P(p("-8.8165e-23"))), "-8.81650000000000010836635e-23");
    assert_eq!(format!("{:20.30}", P(p("-8.821e-20"))), "-8.82099999999999961264397e-20");
    assert_eq!(format!("{:20.30}", P(p("-8.8805774e+129"))), "-8.8805774000000000360391e129");
    assert_eq!(format!("{:20.30}", P(p("-8.91e+18"))), "-8.91000000000000e18");
    assert_eq!(format!("{:20.30}", P(p("-9e-19"))), "-9.00000000000000025869583e-19");
    assert_eq!(format!("{:20.30}", P(p("-9.03e-286"))), "-9.0300000000000000857069e-286");
    assert_eq!(format!("{:20.30}", P(p("-9.13"))), "-9.130000000000000781597009336");
    assert_eq!(format!("{:20.30}", P(p("-9.185594e-120"))), "-9.1855940000000001648391e-120");
    assert_eq!(format!("{:20.30}", P(p("-9.3e-70"))), "-9.29999999999999981491905e-70");
    assert_eq!(format!("{:20.30}", P(p("-9.451309e-30"))), "-9.45130899999999931512581e-30");
    assert_eq!(format!("{:20.30}", P(p("-9.494134e+65"))), "-9.494134000000000816130636e65");
}
#[test]
fn many2_10() {
    assert_eq!(format!("{:20.30}", P(p("-9.54e-24"))), "-9.54000000000000017209063e-24");
    assert_eq!(format!("{:20.30}", P(p("-9.57259e+240"))), "-9.57258999999999942068014e240");
    assert_eq!(format!("{:20.30}", P(p("-27.1783"))), "-27.17830000000000012505552149");
    assert_eq!(format!("{:20.30}", P(p("-32.234"))), "-32.23400000000000176214598469");
    assert_eq!(format!("{:20.30}", P(p("-46.430046"))), "-46.43004599999999726378518972");
    assert_eq!(format!("{:20.30}", P(p("-48.1"))), "-48.10000000000000142108547152");
    assert_eq!(format!("{:20.30}", P(p("-48.307"))), "-48.30700000000000216004991671");
    assert_eq!(format!("{:20.30}", P(p("-50"))), "-50.0000000000000000");
    assert_eq!(format!("{:20.30}", P(p("-57.28"))), "-57.28000000000000113686837722");
    assert_eq!(format!("{:20.30}", P(p("-60.094"))), "-60.09400000000000119371179608");
    assert_eq!(format!("{:20.30}", P(p("-62.01612"))), "-62.01612000000000080035533756");
    assert_eq!(format!("{:20.30}", P(p("-64.92"))), "-64.92000000000000170530256582");
    assert_eq!(format!("{:20.30}", P(p("-80.2234"))), "-80.22339999999999804458639119");
    assert_eq!(format!("{:20.30}", P(p("-90"))), "-90.0000000000000000");
    assert_eq!(format!("{:20.30}", P(p("-94.9"))), "-94.90000000000000568434188608");
    assert_eq!(format!("{:20.30}", P(p("-98.6783"))), "-98.67829999999999301962816389");
    assert_eq!(format!("{:20.30}", P(p("-100"))), "-100.000000000000000");
    assert_eq!(format!("{:20.30}", P(p("-164.3687"))), "-164.3686999999999898136593401");
    assert_eq!(format!("{:20.30}", P(p("-430.50597"))), "-430.5059699999999907049641479");
    assert_eq!(format!("{:20.30}", P(p("-466.5085"))), "-466.5085000000000263753463514");
    assert_eq!(format!("{:20.30}", P(p("-595.05"))), "-595.0499999999999545252649114");
    assert_eq!(format!("{:20.30}", P(p("-764.3112"))), "-764.3111999999999852661858313");
    assert_eq!(format!("{:20.30}", P(p("-807.142"))), "-807.1420000000000527506927028");
    assert_eq!(format!("{:20.30}", P(p("-5469.471"))), "-5469.470999999999548890627921");
    assert_eq!(format!("{:20.30}", P(p("-5651.6809"))), "-5651.68090000000029249349609");
    assert_eq!(format!("{:20.30}", P(p("-9706.81"))), "-9706.809999999999490682967007");
    assert_eq!(format!("{:20.30}", P(p("-93500"))), "-93500.0000000000000");
    assert_eq!(format!("{:20.30}", P(p("-577000"))), "-5.770000000000000e5");
    assert_eq!(format!("{:20.30}", P(p("-835534"))), "-8.355340000000000e5");
    assert_eq!(format!("{:20.30}", P(p("-6000000"))), "-6.000000000000000e6");
}
#[test]
fn many2_11() {
    assert_eq!(format!("{:20.30}", P(p("-900000000"))), "-9.000000000000000e8");
    assert_eq!(format!("{:20.30}", P(p("-279358770000"))), "-2.79358770000000e11");
    assert_eq!(format!("{:20.30}", P(p("-50000000000000"))), "-5.00000000000000e13");
}
