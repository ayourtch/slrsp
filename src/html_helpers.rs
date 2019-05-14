use rspten::*;

pub fn dbh_get_dropdown(_switchtype: i32) -> HtmlSelect<i32> {
    let mut dd: HtmlSelect<i32> = Default::default();
    dd.item(" --- ".into(), -1);
    for i in 1..23 {
        dd.item(&format!("item {}", i), i);
    }
    dd
}

pub fn dbh_get_testing_dropdown(_switchtype: i32) -> HtmlSelect<i32> {
    let mut dd: HtmlSelect<i32> = Default::default();
    dd.item(" --- ".into(), -1);
    for i in 1..23 {
        dd.item(&format!("testing item {}", i), i);
    }
    dd
}
