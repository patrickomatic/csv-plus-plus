use csvpp::Template;
mod common;

const ALL_FEATURES: &str = "
## Welcome to the all_features.csvpp test. this is a comment
##

fn foo_fn(a, b, c) a + b * c

# a variable
bar := 42

# a function that takes two args and calls another function
fn composite_fn(a, b)
    foo_fn(a, b, bar) # a trailing comment

---
 border        ,                                                                 ,
               ,[[border=top]]                     border=top                    ,
               ,[[border=right]]                   border=right                  ,
               ,[[border=bottom]]                  border=bottom                 ,
               ,[[border=left]]                    border=left                   ,
               ,[[border=all]]                     border=all                    ,
               ,                                                                 ,
 borderstyle   ,                                                                 ,
               ,[[border=all/borderstyle=dashed]]  border=all/borderstyle=dashed ,
               ,[[border=all/borderstyle=solid]]   border=all/borderstyle=solid  ,
               ,                                                                 ,
 color         ,                                                                 ,
               ,[[color=FF0000]]                   color=FF0000                  ,
               ,[[color=ABC]]                      color=ABC                     ,
               ,                                                                 ,
 expand        ,                                                                 ,
 ![[expand=3]] ,                                   expand=3                      ,
               ,                                                                 ,
 fontcolor     ,                                                                 ,
               ,[[fontcolor=FF0000]]               fontcolor=FF0000              ,
               ,[[fontcolor=ABC]]                  fontcolor=ABC                 ,
               ,                                                                 ,
 fontfamily    ,                                                                 ,
               ,[[fontfamily='Comic Sans Ms']]     fontfamily='Comic Sans MS'    ,
               ,[[fontfamily='Helvetica']]         fontfamily='Helvetica'        ,
               ,                                                                 ,
 fontsize      ,                                                                 ,
               ,[[fontsize=20]]                    fontsize=20                   ,
               ,[[fontsize=4]]                     fontsize=4                    ,
               ,                                                                 ,
 format        ,                                                                 ,
               ,[[format=bold]]                    format=bold                   ,
               ,[[format=italic]]                  format=italic                 ,
               ,[[format=underline]]               format=underline              ,
               ,[[format=strikethrough]]           format=strikethrough          ,
               ,                                                                 ,
 halign        ,                                                                 ,
               ,[[halign=left]]                    halign=left                   ,
               ,[[halign=center]]                  halign=center                 ,
               ,[[halign=right]]                   halign=right                  ,
               ,                                                                 ,
 note          ,                                                                 ,
               ,[[note='this is a note']]          note='this is a note'         ,
               ,                                                                 ,
 numberformat  ,                                                                 ,
               ,[[numberformat=currency]]          numberformat=currency         ,[[numberformat=currency]]123456
               ,[[numberformat=date]]              numberformat=date             ,[[numberformat=date]]123456
               ,[[numberformat=datetime]]          numberformat=datetime         ,[[numberformat=datetime]]123456
               ,[[numberformat=number]]            numberformat=number           ,[[numberformat=number]]123456
               ,[[numberformat=percent]]           numberformat=percent          ,[[numberformat=percent]]123456
               ,[[numberformat=text]]              numberformat=text             ,[[numberformat=text]]123456
               ,[[numberformat=time]]              numberformat=time             ,[[numberformat=time]]123456
               ,[[numberformat=scientific]]        numberformat=scientific       ,[[numberformat=scientific]]123456
               ,                                                                 ,
 valign        ,                                                                 ,
               ,[[valign=top]]                     valign=top                    ,
               ,[[valign=center]]                  valign=center                 ,
               ,[[valign=bottom]]                  valign=bottom                 ,
               ,                                                                 ,
 variables     ,                                                                 ,
               ,depends_on_another                                               ,=depends_on_another
               ,my_function()                                                    ,=my_function()
               ,simple_var                                                       ,=simple_var
";

const ALL_FEATURES_SHORTHAND: &str = "
 border        ,                                                      ,
               ,[[b=t]]                 border=top                    ,
               ,[[b=r]]                 border=right                  ,
               ,[[b=b]]                 border=bottom                 ,
               ,[[b=l]]                 border=left                   ,
               ,[[b=a]]                 border=all                    ,
               ,                                                      ,
 borderstyle   ,                                                      ,
               ,[[b=all/bs=dashed]]     border=all/borderstyle=dashed ,
               ,[[b=all/bs=solid]]      border=all/borderstyle=solid  ,
               ,                                                      ,
 color         ,                                                      ,
               ,[[c=FF0000]]            color=FF0000                  ,
               ,[[c=ABC]]               color=ABC                     ,
               ,                                                      ,
 expand        ,                                                      ,
 ![[e=3]]      ,                        expand=3                      ,
               ,                                                      ,
 fontcolor     ,                                                      ,
               ,[[fc=FF0000]]           fontcolor=FF0000              ,
               ,[[fc=ABC]]              fontcolor=ABC                 ,
               ,                                                      ,
 fontfamily    ,                                                      ,
               ,[[ff='Comic Sans Ms']]  fontfamily='Comic Sans MS'    ,
               ,[[ff='Helvetica']]      fontfamily='Helvetica'        ,
               ,                                                      ,
 fontsize      ,                                                      ,
               ,[[fs=20]]               fontsize=20                   ,
               ,[[fs=4]]                fontsize=4                    ,
               ,                                                      ,
 format        ,                                                      ,
               ,[[f=b]]                 format=bold                   ,
               ,[[f=i]]                 format=italic                 ,
               ,[[f=u]]                 format=underline              ,
               ,[[f=s]]                 format=strikethrough          ,
               ,                                                      ,
 halign        ,                                                      ,
               ,[[ha=l]]                halign=left                   ,
               ,[[ha=c]]                halign=center                 ,
               ,[[ha=r]]                halign=right                  ,
               ,                                                      ,
 note          ,                                                      ,
               ,[[n='this is a note']]  note='this is a note'         ,
               ,                                                      ,
 numberformat  ,                                                      ,
               ,[[nf=currency]]         numberformat=currency         ,[[numberformat=currency]]123456
               ,[[nf=date]]             numberformat=date             ,[[numberformat=date]]123456
               ,[[nf=datetime]]         numberformat=datetime         ,[[numberformat=datetime]]123456
               ,[[nf=number]]           numberformat=number           ,[[numberformat=number]]123456
               ,[[nf=percent]]          numberformat=percent          ,[[numberformat=percent]]123456
               ,[[nf=text]]             numberformat=text             ,[[numberformat=text]]123456
               ,[[nf=time]]             numberformat=time             ,[[numberformat=time]]123456
               ,[[nf=scientific]]       numberformat=scientific       ,[[numberformat=scientific]]123456
               ,                                                      ,
 valign        ,                                                      ,
               ,[[va=top]]              valign=top                    ,
               ,[[va=center]]           valign=center                 ,
               ,[[va=bottom]]           valign=bottom                 ,

";

#[test]
fn all_features_csv() {
    let s = common::Setup::new("all_features_csv", "csv", ALL_FEATURES);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();

    assert!(target.write(&template).is_ok());
}

#[test]
fn all_features_shorthand_csv_no_code_section() {
    let s = common::Setup::new(
        "all_features_shorthand_csv_no_code_section",
        "csv",
        ALL_FEATURES_SHORTHAND,
    );
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();

    assert!(target.write(&template).is_ok());
}

#[test]
fn all_features_excel() {
    let s = common::Setup::new("all_features_excel", "xlsx", ALL_FEATURES);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();

    assert!(target.write(&template).is_ok());
}

// TODO:
// #[test]
// fn all_features_google_sheets() {
// }
