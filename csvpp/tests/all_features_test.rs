use csvpp::Template;
mod common;

#[test]
fn test_all_features_csv() {
    let s = common::Setup::new(r#"
# Welcome to the all_features.csvpp test. this is a comment

fn foo_fn(a, b, c) a + b * c

# a variable
bar := 42

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
               ,                                   numberformat=currency         ,[[numberformat=currency]]123456
               ,                                   numberformat=date             ,[[numberformat=date]]123456
               ,                                   numberformat=datetime         ,[[numberformat=datetime]]123456
               ,                                   numberformat=number           ,[[numberformat=number]]123456
               ,                                   numberformat=percent          ,[[numberformat=percent]]123456
               ,                                   numberformat=text             ,[[numberformat=text]]123456
               ,                                   numberformat=time             ,[[numberformat=time]]123456
               ,                                   numberformat=scientific       ,[[numberformat=scientific]]123456
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

"#);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    assert!(target.write(&template).is_ok());
}

// TODO: fn test_all_features_shorthand_csv() {
